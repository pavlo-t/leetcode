#![allow(dead_code)]
//! \#300. Longest Increasing Subsequence
//! =====================================
//!
//! <https://leetcode.com/problems/longest-increasing-subsequence>
//!
//! Given an integer array `nums`, return the length of the longest strictly increasing subsequence.
//!
//! A __subsequence__ is a sequence that can be derived from an array by deleting some or no elements
//! without changing the order of the remaining elements.
//! For example, `[3,6,2,7]` is a subsequence of the array `[0,3,1,6,2,2,7]`.
//!
//! ##### Constraints
//!
//! - `1 <= nums.length <= 2500`
//! - `-10_000 <= nums[i] <= 10_000`
//!
//! ##### Follow up
//!
//! Can you come up with an algorithm that runs in `O(n log(n))` time complexity?
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_08::c2022_08_08::Solution;
//! let nums = vec![10,9,2,5,3,7,101,18];
//! assert_eq!(Solution::length_of_lis(nums), 4);
//! ```
//!
//! __Explanation:__ The longest increasing subsequence is `[2,3,7,101]`, therefore the length is `4`.
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_08::Solution;
//! let nums = vec![0,1,0,3,2,3];
//! assert_eq!(Solution::length_of_lis(nums), 4);
//! ```
//!
//! ###### Example 3
//!
//! ```
//! # use c2022_08::c2022_08_08::Solution;
//! let nums = vec![7,7,7,7,7,7,7];
//! assert_eq!(Solution::length_of_lis(nums), 1);
//! ```

pub struct Solution;
impl Solution {
    /// Recursion
    pub fn length_of_lis_v1(nums: Vec<i32>) -> i32 {
        fn rec(curr: usize, prev: Option<usize>, nums: &[i32]) -> i32 {
            if curr == nums.len() {
                0
            } else if prev.filter(|&prev| nums[prev] >= nums[curr]).is_some() {
                rec(curr + 1, prev, nums)
            } else {
                let skip = rec(curr + 1, prev, nums);
                let take = rec(curr + 1, Some(curr), nums) + 1;
                skip.max(take)
            }
        }

        rec(0, None, &nums)
    }

    /// Recursion with memo
    pub fn length_of_lis_v2(nums: Vec<i32>) -> i32 {
        fn rec(curr: usize, prev: usize, nums: &[i32], memo: &mut Vec<Vec<i32>>) -> i32 {
            if curr == nums.len() {
                0
            } else {
                if memo[curr][prev] == -1 {
                    let next = curr + 1;
                    let skip = rec(next, prev, nums, memo);
                    memo[curr][prev] = if prev > 0 && nums[prev - 1] >= nums[curr] {
                        skip
                    } else {
                        let take = rec(next, next, nums, memo) + 1;
                        skip.max(take)
                    }
                }

                memo[curr][prev]
            }
        }

        let n = nums.len();
        let mut memo = vec![vec![-1; n]; n];
        rec(0, 0, &nums, &mut memo)
    }

    /// DP 2 dimensions
    pub fn length_of_lis_v3(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![0; n + 1]; n + 1];
        for prev in (0..n).rev() {
            for curr in (prev..n).rev() {
                let next = curr + 1;
                dp[curr][prev] = if prev > 0 && nums[prev - 1] >= nums[curr] {
                    dp[next][prev]
                } else {
                    dp[next][prev].max(dp[next][next] + 1)
                }
            }
        }
        dp[0][0]
    }

    /// DP 1 dimension
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![0; n + 1];
        for prev in (0..n).rev() {
            for curr in (prev..n).rev() {
                dp[prev] = if prev > 0 && nums[prev - 1] >= nums[curr] {
                    dp[prev]
                } else {
                    dp[prev].max(dp[curr + 1] + 1)
                }
            }
        }
        dp[0]
    }
}
