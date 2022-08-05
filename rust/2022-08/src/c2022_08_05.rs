#![allow(dead_code)]
//! \#377. Combination Sum IV
//! =========================
//!
//! <https://leetcode.com/problems/combination-sum-iv>
//!
//! Given an array of __distinct__ integers `nums` and a target integer `target`,
//! return _the number of possible combinations that add up to `target`_.
//!
//! The test cases are generated so that the answer can fit in a __32-bit__ integer.
//!
//! ##### Constraints
//!
//! - `1 <= nums.length <= 200`
//! - `1 <= nums[i] <= 1000`
//! - All the elements of nums are __unique__.
//! - `1 <= target <= 1000`
//!
//! ##### Follow up
//!
//! What if negative numbers are allowed in the given array?
//! How does it change the problem?
//! What limitation we need to add to the question to allow negative numbers?
//!
//! ##### Examples
//!
//! ```
//! # use c2022_08::c2022_08_05::Solution;
//! assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
//! ```
//!
//! __Explanation:__
//!
//! The possible combination ways are:
//!
//! - `(1, 1, 1, 1)`
//! - `(1, 1, 2)`
//! - `(1, 2, 1)`
//! - `(1, 3)`
//! - `(2, 1, 1)`
//! - `(2, 2)`
//! - `(3, 1)`
//!
//! Note that different sequences are counted as different combinations.
//!
//! ```
//! # use c2022_08::c2022_08_05::Solution;
//! assert_eq!(Solution::combination_sum4(vec![9], 3), 0);
//! ```

pub struct Solution;
impl Solution {
    /// Brute-force recursion
    ///
    /// - Time: `O(2 pow target)`
    /// - Memory: `O(n max target)`
    pub fn combination_sum4_v1(nums: Vec<i32>, target: i32) -> i32 {
        fn rec(i: usize, target: i32, nums: &[i32]) -> i32 {
            if target == 0 {
                1
            } else if target < 0 || i == nums.len() {
                0
            } else {
                let skip = rec(i + 1, target, nums);
                let take = rec(0, target - nums[i], nums);
                skip + take
            }
        }
        rec(0, target, &nums)
    }

    /// Recursion with memo
    ///
    /// - Time: `O(n * target)`
    /// - Memory: `O(n * target)`
    pub fn combination_sum4_v2(nums: Vec<i32>, target: i32) -> i32 {
        fn rec(target: usize, i: usize, nums: &[i32], memo: &mut Vec<Vec<i32>>) -> i32 {
            if target == 0 {
                1
            } else if i == nums.len() {
                0
            } else {
                if memo[target][i] == -1 {
                    memo[target][i] = if nums[i] as usize > target {
                        rec(target, i + 1, nums, memo)
                    } else {
                        let skip = rec(target, i + 1, nums, memo);
                        let take = rec(target - nums[i] as usize, 0, nums, memo);
                        skip + take
                    }
                }
                memo[target][i]
            }
        }
        let target = target as usize;
        let mut memo = vec![vec![-1; nums.len()]; target + 1];

        rec(target, 0, &nums, &mut memo)
    }

    /// DP 2 dimensions
    ///
    /// - Time: `O(n * target)`
    /// - Memory: `O(n * target)`
    pub fn combination_sum4_v3(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let target = target as usize;
        let mut dp = vec![vec![1; n + 1]; target + 1];
        for t in 1..=target {
            dp[t][n] = 0;
            for i in (0..n).rev() {
                let num = nums[i] as usize;
                if t < num {
                    dp[t][i] = dp[t][i + 1];
                } else {
                    dp[t][i] = dp[t][i + 1] + dp[t - num][0];
                }
            }
        }
        dp[target][0]
    }

    /// DP 1 dimension
    ///
    /// - Time: `O(n * target)`
    /// - Memory: `O(target)`
    pub fn combination_sum4_v4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;

        let mut dp = vec![0; target + 1];
        dp[0] = 1;
        for t in 1..=target {
            for num in nums.iter().map(|&n| n as usize).filter(|&n| n <= t) {
                dp[t] += dp[t - num];
            }
        }

        dp[target]
    }

    /// DP 1 dimension using iterators and closures
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        use std::iter::{once, repeat};

        let target = target as usize;

        let nums_lte_target = |target: usize| {
            nums.iter()
                .map(|&num| num as usize)
                .filter(move |&num| num <= target)
                .map(move |num| (target, num))
        };

        let build_dp = |mut dp: Vec<i32>, (target, num): (usize, usize)| {
            dp[target] += dp[target - num];
            dp
        };

        let dp = once(1)
            .chain(repeat(0))
            .take(target + 1)
            .collect::<Vec<_>>();

        (1..=target).flat_map(nums_lte_target).fold(dp, build_dp)[target]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1_t_1() { assert_eq!(Solution::combination_sum4(vec![1], 1), 1); }
    #[rustfmt::skip] #[test] fn n_1_t_2() { assert_eq!(Solution::combination_sum4(vec![1], 2), 1); }
    #[rustfmt::skip] #[test] fn n_1_t_3() { assert_eq!(Solution::combination_sum4(vec![1], 3), 1); }

    #[rustfmt::skip] #[test] fn n_1_2_t_1() { assert_eq!(Solution::combination_sum4(vec![1,2], 1), 1); }
    #[rustfmt::skip] #[test] fn n_1_2_t_2() { assert_eq!(Solution::combination_sum4(vec![1,2], 2), 2); }
    #[rustfmt::skip] #[test] fn n_1_2_t_3() { assert_eq!(Solution::combination_sum4(vec![1,2], 3), 3); }
    #[rustfmt::skip] #[test] fn n_1_2_t_4() { assert_eq!(Solution::combination_sum4(vec![1,2], 4), 5); }

    #[rustfmt::skip] #[test] fn n_9_t_3()     { assert_eq!(Solution::combination_sum4(vec![9],       3), 0); }
    #[rustfmt::skip] #[test] fn n_1_2_3_t_4() { assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7); }

    #[rustfmt::skip] #[test] fn n_1_170_t_1000() { assert_eq!(Solution::combination_sum4(vec![1, 170], 1000), 1_169_305_696); }
}
