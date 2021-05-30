#![allow(dead_code)]
/// Maximum Gap
/// ===========
///
/// Given an integer array `nums`,
/// return _the maximum difference between two successive elements in its sorted form_.
/// If the array contains less than two elements, return `0`.
///
/// You must write an algorithm that runs in linear time and uses linear extra space.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 10_000`
/// - `0 <= nums[i] <= 1_000_000_000`
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/602/week-5-may-29th-may-31st/3761/
struct Solution;
impl Solution {
    /// Approach 3: Buckets and The Pigeonhole Principle
    /// https://leetcode.com/problems/maximum-gap/solution/
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            0
        } else {
            let (min, max) = nums.iter().fold((i32::MAX, i32::MIN), |(min, max), &n| {
                (min.min(n), max.max(n))
            });
            let bucket_size = 1.max((max - min) as usize / (nums.len() - 1));
            let bucket_num = (max - min) as usize / bucket_size + 1;
            let mut buckets = vec![(i32::MAX, i32::MIN); bucket_num];

            for &n in &nums {
                let bi = (n - min) as usize / bucket_size;
                let (b_min, b_max) = buckets[bi];
                buckets[bi] = (b_min.min(n), b_max.max(n));
            }

            let mut prev_max = min;
            let mut max_gap = 0;
            for (b_min, b_max) in buckets {
                if b_min < i32::MAX {
                    max_gap = max_gap.max(b_min - prev_max);
                    prev_max = b_max;
                }
            }

            max_gap
        }
    }

    /// Approach 2: Radix Sort
    /// https://leetcode.com/problems/maximum-gap/solution/
    pub fn maximum_gap_radix(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            0
        } else {
            fn radix_sort(nums: &mut Vec<i32>) {
                let &max_val = nums.iter().max().unwrap();
                let mut exp = 1usize;
                let mut aux = vec![0; nums.len()];

                while max_val / exp as i32 > 0 {
                    let mut count = vec![0; 10];
                    for &n in nums.iter() {
                        count[(n as usize / exp) % 10] += 1;
                    }
                    for i in 1..count.len() {
                        count[i] += count[i - 1];
                    }
                    for &n in nums.iter().rev() {
                        let ci = (n as usize / exp) % 10;
                        count[ci] -= 1;
                        let c = count[ci];
                        aux[c] = n;
                    }
                    for i in 0..nums.len() {
                        nums[i] = aux[i];
                    }

                    exp *= 10;
                }
            }
            radix_sort(&mut nums);
            nums.iter()
                .fold((0, nums[0]), |(r, p), &c| (r.max((c - p).abs()), c))
                .0
        }
    }

    pub fn maximum_gap_log_time(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.iter()
            .fold((0, nums[0]), |(r, p), &c| (r.max((c - p).abs()), c))
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_3_6_9_1_produces_3() {
        assert_eq!(Solution::maximum_gap(vec![1, 3, 6, 9]), 3);
        // Explanation: The sorted form of the array is [1,3,6,9], either (3,6) or (6,9) has the maximum difference 3.
    }
    #[test]
    fn n_10_produces_0() {
        assert_eq!(Solution::maximum_gap(vec![10]), 0);
        // Explanation: The array contains less than 2 elements, therefore return 0.
    }
    #[test]
    fn n_10k_to_1_produces_1() {
        let nums = (1..=10_000).rev().collect();
        assert_eq!(Solution::maximum_gap(nums), 1);
    }
    #[test]
    fn n_1kkk_to_0_by_100k_produces_100k() {
        let nums = (0..=10_000).rev().map(|i| i * 100_000).collect();
        assert_eq!(Solution::maximum_gap(nums), 100_000);
    }
}
