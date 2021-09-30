#![allow(dead_code)]
/// 3Sum Closest
/// ============
///
/// Given an array `nums` of _n_ integers and an integer `target`,
/// find three integers in `nums` such that the sum is closest to `target`.
/// Return the sum of the three integers.
/// You may assume that each input would have exactly one solution.
///
/// __Constraints:__
///
/// - `3 <= nums.length <= 10_000`
/// - `-10_000 <= nums[i] <= 10_000`
/// - `-100_000 <= target <= 100_000`
///
/// https://leetcode.com/explore/challenge/card/july-leetcoding-challenge-2021/611/week-4-july-22nd-july-28th/3828/
struct Solution;
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut result = 100_000;
        let mut d_min = 100_000;
        for i in 0..nums.len() - 2 {
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                let d = (target - sum).abs();
                if d < d_min {
                    result = sum;
                    d_min = d;
                }
                if sum < target {
                    l += 1;
                } else if sum > target {
                    r -= 1;
                } else {
                    return sum;
                }
            }
        }
        result
    }

    pub fn three_sum_closest_brute_force(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = 100_000;
        let mut diff = 100_000;
        for a in 0..nums.len() - 2 {
            for b in a + 1..nums.len() - 1 {
                for c in b + 1..nums.len() {
                    let r = nums[a] + nums[b] + nums[c];
                    let d = (target - r).abs();
                    if d < diff {
                        result = r;
                        diff = d;
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ns_m1p2p1m4_t_1_produces_2() {
        let nums = vec![-1, 2, 1, -4];
        assert_eq!(Solution::three_sum_closest(nums, 1), 2);
        // Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
    }
    #[test]
    fn ns_p1p2p3_t_1_produces_6() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::three_sum_closest(nums, 1), 6);
    }

    mod performance {
        use super::*;

        //#[ignore]
        #[test]
        fn ns_1to10000_t_29997_produces_29997() {
            let nums = (1..=10000).collect();
            assert_eq!(Solution::three_sum_closest(nums, 29997), 29997);
        }
    }
}
