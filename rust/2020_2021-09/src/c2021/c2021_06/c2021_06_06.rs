#![allow(dead_code)]
/// Longest Consecutive Sequence
/// ============================
///
/// Given an unsorted array of integers `nums`,
/// return _the length of the longest consecutive elements sequence_.
///
/// You must write an algorithm that runs in `O(n)` time.
///
/// __Constraints:__
///
/// - `0 <= nums.length <= 100_000`
/// - `-1_000_000_000 <= nums[i] <= 1_000_000_000`
///
/// https://leetcode.com/explore/featured/card/june-leetcoding-challenge-2021/603/week-1-june-1st-june-7th/3769/
struct Solution;
impl Solution {
    /// Approach 3: HashSet and Intelligent Sequence Building
    ///
    /// https://leetcode.com/problems/longest-consecutive-sequence/solution/
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let nums = nums.into_iter().collect::<HashSet<_>>();
        nums.iter().fold(0, |rsf, &s| {
            if nums.contains(&(s - 1)) {
                rsf
            } else {
                let mut c = s + 1;
                let mut len = 1;
                while nums.contains(&c) {
                    len += 1;
                    c += 1;
                }
                rsf.max(len)
            }
        })
    }

    pub fn longest_consecutive_radix_sort(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 1 {
            0
        } else {
            fn radix_sort(ns: &mut Vec<i32>) {
                if let Some(max) = ns.iter().map(|&n| n.abs()).max() {
                    let mut buckets = vec![vec![]; 19];
                    let mut place = 1;
                    while place <= max {
                        for &n in ns.iter() {
                            let b = (n / place % 10 + 9) as usize;
                            buckets[b].push(n);
                        }

                        buckets
                            .iter()
                            .flatten()
                            .enumerate()
                            .for_each(|(i, &n)| ns[i] = n);

                        buckets.iter_mut().for_each(|b| b.clear());

                        place *= 10;
                    }
                }
            }

            radix_sort(&mut nums);

            nums.into_iter()
                .fold((1, 1, -1_000_000_001), |(max, l, p), c| {
                    let nl = if c == p + 1 {
                        l + 1
                    } else if c == p {
                        l
                    } else {
                        1
                    };
                    (max.max(nl), nl, c)
                })
                .0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nums_100_4_200_1_3_2_produces_4() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(Solution::longest_consecutive(nums), 4);
        // Explanation:
        // The longest consecutive elements sequence is [1, 2, 3, 4].
        // Therefore its length is 4.
    }
    #[test]
    fn nums_0_3_7_2_5_8_4_6_0_1_produces_9() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(Solution::longest_consecutive(nums), 9);
    }
    #[test]
    fn nums_0_1_2_3_produces_4() {
        let nums = vec![0, 1, 2, 3];
        assert_eq!(Solution::longest_consecutive(nums), 4);
    }

    #[test]
    fn nums_empty_produces_0() {
        assert_eq!(Solution::longest_consecutive(vec![]), 0);
    }
    #[test]
    fn nums_m1_0_produces_2() {
        let nums = vec![0, -1];
        assert_eq!(Solution::longest_consecutive(nums), 2);
    }
    #[test]
    fn nums_9_1_4_7_3_m1_0_5_8_m1_6_produces_7() {
        let nums = vec![9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6];
        assert_eq!(Solution::longest_consecutive(nums), 7);
    }
    #[test]
    fn nums_1_2_0_1_produces_3() {
        let nums = vec![1, 2, 0, 1];
        assert_eq!(Solution::longest_consecutive(nums), 3);
    }
    #[test]
    fn nums_10_to_1_produces_10() {
        let nums = (1..=10).rev().collect::<Vec<_>>();
        assert_eq!(Solution::longest_consecutive(nums), 10);
    }
    #[test]
    fn nums_100k_to_1_produces_100k() {
        let nums = (1..=100_000).rev().collect::<Vec<_>>();
        assert_eq!(Solution::longest_consecutive(nums), 100_000);
    }
    #[test]
    fn nums_200k_to_2_by_m2_produces_1() {
        let nums = (1..=100_000).rev().map(|i| i * 2).collect();
        assert_eq!(Solution::longest_consecutive(nums), 1);
    }
}
