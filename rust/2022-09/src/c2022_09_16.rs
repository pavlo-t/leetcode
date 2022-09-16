#![allow(dead_code)]
//! \#1770. Maximum Score from Performing Multiplication Operations
//! ===============================================================
//!
//! <https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations>
//!
//! You are given two integer arrays `nums` and `multipliers` of size `n` and `m` respectively, where `n >= m`.
//! The arrays are __1-indexed__.
//!
//! You begin with a score of `0`.
//! You want to perform __exactly__ `m` operations.
//! On the `i`th operation (__1-indexed__), you will:
//!
//! - Choose one integer `x` from __either the start or the end__ of the array `nums`.
//! - Add `multipliers[i] * x` to your score.
//! - Remove `x` from the array `nums`.
//!
//! Return _the __maximum__ score after performing `m` operations_.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_09::c2022_09_16::*;
//! let nums = vec![1, 2, 3];
//! let multipliers = vec![3, 2, 1];
//! assert_eq!(Solution::maximum_score(nums, multipliers), 14);
//! ```
//!
//! __Explanation:__ An optimal solution is as follows:
//!
//! - Choose from the end, `[1, 2, 3]`, adding `3 * 3 = 9` to the score.
//! - Choose from the end, `[1, 2]`, adding `2 * 2 = 4` to the score.
//! - Choose from the end, `[1]`, adding `1 * 1 = 1` to the score.
//!
//! The total score is `9 + 4 + 1 = 14`.
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_09::c2022_09_16::*;
//! let nums = vec![-5, -3, -3, -2, 7, 1];
//! let multipliers = vec![-10, -5, 3, 4, 6];
//! assert_eq!(Solution::maximum_score(nums, multipliers), 102);
//! ```
//!
//! __Explanation:__ An optimal solution is as follows:
//!
//! - Choose from the start, `[-5, -3, -3, -2, 7, 1]`, adding `-5 * -10 = 50` to the score.
//! - Choose from the start, `[-3, -3, -2, 7, 1]`, adding `-3 * -5 = 15` to the score.
//! - Choose from the start, `[-3, -2, 7, 1]`, adding `-3 * 3 = -9` to the score.
//! - Choose from the end, `[-2, 7, 1]`, adding `1 * 4 = 4` to the score.
//! - Choose from the end, `[-2, 7]`, adding `7 * 6 = 42` to the score.
//!
//! The total score is `50 + 15 - 9 + 4 + 42 = 102`.
//!
//! ##### Constraints
//!
//! - `n == nums.length`
//! - `m == multipliers.length`
//! - `1 <= m <= 1000`
//! - `m <= n <= 100_000`
//! - `-1000 <= nums[i], multipliers[i] <= 1000`

pub struct Solution;
impl Solution {
    pub fn maximum_score_v1(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        fn rec(i: usize, l: usize, r: usize, nums: &[i32], mults: &[i32]) -> i32 {
            if i == mults.len() {
                0
            } else {
                let take_l = nums[l] * mults[i] + rec(i + 1, l + 1, r, nums, mults);
                let take_r = nums[r] * mults[i] + rec(i + 1, l, r.saturating_sub(1), nums, mults);

                take_l.max(take_r)
            }
        }

        rec(0, 0, nums.len() - 1, &nums, &multipliers)
    }

    pub fn maximum_score_v2(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        fn rec(i: usize, l: usize, r: usize, nums: &[i32], mults: &[i32]) -> i32 {
            if i == mults.len() {
                0
            } else {
                let take_l = rec(i + 1, l + 1, r, nums, mults) + mults[i] * nums[l];
                let ri = nums.len() - r - 1;
                let take_r = rec(i + 1, l, r + 1, nums, mults) + mults[i] * nums[ri];

                take_l.max(take_r)
            }
        }

        rec(0, 0, 0, &nums, &multipliers)
    }

    pub fn maximum_score_v3(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        fn rec(
            i: usize,
            l: usize,
            r: usize,
            nums: &[i32],
            mults: &[i32],
            memo: &mut Vec<Vec<Vec<i32>>>,
        ) -> i32 {
            if i == mults.len() {
                0
            } else {
                if memo[i][l][r] == -1 {
                    memo[i][l][r] = {
                        let take_l = rec(i + 1, l + 1, r, nums, mults, memo) + mults[i] * nums[l];
                        let ri = nums.len() - r - 1;
                        let take_r = rec(i + 1, l, r + 1, nums, mults, memo) + mults[i] * nums[ri];

                        take_l.max(take_r)
                    }
                }
                memo[i][l][r]
            }
        }

        let m = multipliers.len();
        let mut memo = vec![vec![vec![-1; m + 1]; m + 1]; m + 1];

        rec(0, 0, 0, &nums, &multipliers, &mut memo)
    }

    pub fn maximum_score_v4(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let (n, m) = (nums.len(), multipliers.len());
        let mut dp = vec![vec![vec![0; m + 1]; m + 1]; m + 1];
        for i in (0..m).rev() {
            for l in (0..=i).rev() {
                for r in (0..=i).rev() {
                    dp[i][l][r] = {
                        let take_l = dp[i + 1][l + 1][r] + multipliers[i] * nums[l];
                        let take_r = dp[i + 1][l][r + 1] + multipliers[i] * nums[n - r - 1];
                        take_l.max(take_r)
                    };
                }
            }
        }

        dp[0][0][0]
    }

    pub fn maximum_score_v5(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let (n, m) = (nums.len(), multipliers.len());
        let mut dp = vec![vec![vec![0; m + 1]; m + 1]; m + 1];
        for i in (0..m).rev() {
            for l in 0..=i {
                for r in 0..=i - l {
                    dp[i][l][r] = {
                        let take_l = dp[i + 1][l + 1][r] + multipliers[i] * nums[l];
                        let take_r = dp[i + 1][l][r + 1] + multipliers[i] * nums[n - r - 1];
                        take_l.max(take_r)
                    };
                }
            }
        }

        dp[0][0][0]
    }

    pub fn maximum_score_v6(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let (n, m) = (nums.len(), multipliers.len());
        let mut dp = vec![vec![0; m + 1]; m + 1];
        for i in (0..m).rev() {
            for l in 0..=i {
                for r in 0..=i - l {
                    dp[l][r] = {
                        let take_l = dp[l + 1][r] + multipliers[i] * nums[l];
                        let take_r = dp[l][r + 1] + multipliers[i] * nums[n - r - 1];
                        take_l.max(take_r)
                    };
                }
            }
        }

        dp[0][0]
    }

    pub fn maximum_score_v7(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let (n, m) = (nums.len(), multipliers.len());
        let mut dp = vec![vec![0; m + 1]; m + 1];
        for i in (0..m).rev() {
            for l in 0..=i {
                let r = n - (i - l) - 1;
                dp[i][l] = {
                    let take_l = dp[i + 1][l + 1] + multipliers[i] * nums[l];
                    let take_r = dp[i + 1][l] + multipliers[i] * nums[r];
                    take_l.max(take_r)
                };
            }
        }

        dp[0][0]
    }

    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let (n, m) = (nums.len(), multipliers.len());
        let mut dp = vec![0; m + 1];
        for i in (0..m).rev() {
            for l in 0..=i {
                dp[l] = {
                    let take_l = dp[l + 1] + multipliers[i] * nums[l];
                    let r = n - (i - l) - 1;
                    let take_r = dp[l] + multipliers[i] * nums[r];
                    take_l.max(take_r)
                };
            }
        }

        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nums_1_2_3_mults_3_2_1() {
        let nums = vec![1, 2, 3];
        let multipliers = vec![3, 2, 1];
        assert_eq!(Solution::maximum_score(nums, multipliers), 14);
    }

    #[test]
    fn nums_m5_m3_m3_m2_7_1_mults_m10_m5_3_4_6() {
        let nums = vec![-5, -3, -3, -2, 7, 1];
        let multipliers = vec![-10, -5, 3, 4, 6];
        assert_eq!(Solution::maximum_score(nums, multipliers), 102);
    }

    #[test]
    fn nums_1_x_10000_mults_1_x_1000() {
        let nums = vec![1; 10000];
        let multipliers = vec![1; 1000];
        assert_eq!(Solution::maximum_score(nums, multipliers), 1000);
    }
}
