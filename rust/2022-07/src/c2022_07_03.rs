#![allow(dead_code)]
//! \#376. Wiggle Subsequence
//! =========================
//!
//! A __wiggle sequence__ is a sequence where the differences between successive numbers
//! strictly alternate between positive and negative.
//! The first difference (if one exists) may be either positive or negative.
//! A sequence with one element and a sequence with two non-equal elements are trivially wiggle sequences.
//!
//! - For example, `[1, 7, 4, 9, 2, 5]` is a __wiggle sequence__ because the differences `(6, -3, 5, -7, 3)`
//!   alternate between positive and negative.
//! - In contrast, `[1, 4, 7, 2, 5]` and `[1, 7, 4, 5, 5]` are not wiggle sequences.
//!   The first is not because its first two differences are positive,
//!   and the second is not because its last difference is zero.
//!
//! A __subsequence__ is obtained by deleting some elements (possibly zero) from the original sequence,
//! leaving the remaining elements in their original order.
//!
//! Given an integer array `nums`, return _the length of the longest __wiggle subsequence__ of `nums`_.
//!
//! __Constraints:__
//!
//! - `1 <= nums.length <= 1000`
//! - `0 <= nums[i] <= 1000`
//!
//! __Follow up:__ Could you solve this in `O(n)` time?
//!
//! <https://leetcode.com/problems/wiggle-subsequence>

pub struct Solution;
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            1
        } else {
            let mut direction = (nums[1] - nums[0]).signum();
            let mut peaks = 1 + (direction.abs());
            for i in 2..nums.len() {
                match (nums[i] - nums[i - 1]).signum() {
                    n if n != 0 && (n == -direction || direction == 0) => {
                        direction = n;
                        peaks += 1;
                    }
                    _ => (),
                }
            }
            peaks
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1() { assert_eq!(Solution::wiggle_max_length(vec![1]), 1); }

    #[rustfmt::skip] #[test] fn n_1_2() { assert_eq!(Solution::wiggle_max_length(vec![1,2]), 2); }
    #[rustfmt::skip] #[test] fn n_2_1() { assert_eq!(Solution::wiggle_max_length(vec![2,1]), 2); }
    #[rustfmt::skip] #[test] fn n_1_1() { assert_eq!(Solution::wiggle_max_length(vec![1,1]), 1); }

    #[rustfmt::skip] #[test] fn n_1_1_1() { assert_eq!(Solution::wiggle_max_length(vec![1,1,1]), 1); }

    #[test]
    fn n_1_7_4_9_2_5() {
        let n = vec![1, 7, 4, 9, 2, 5];
        assert_eq!(Solution::wiggle_max_length(n), 6);
        // Explanation: The entire sequence is a wiggle sequence with differences (6, -3, 5, -7, 3).
    }
    #[test]
    fn n_1_17_5_10_13_15_10_5_16_8() {
        let n = vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8];
        assert_eq!(Solution::wiggle_max_length(n), 7);
        // Explanation: There are several subsequences that achieve this length.
        // One is [1, 17, 10, 13, 10, 16, 8] with differences (16, -7, 3, -3, 6, -8).
    }
    #[test]
    fn n_1_2_3_4_5_6_7_8_9() {
        let n = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(Solution::wiggle_max_length(n), 2);
    }
    #[test]
    fn n_3_3_3_2_5() {
        let n = vec![3, 3, 3, 2, 5];
        assert_eq!(Solution::wiggle_max_length(n), 3);
    }
    #[test]
    fn n_3_3_3_6_5() {
        let n = vec![3, 3, 3, 6, 5];
        assert_eq!(Solution::wiggle_max_length(n), 3);
    }
    #[test]
    fn n_3_2_2_2_5() {
        let n = vec![3, 2, 2, 2, 5];
        assert_eq!(Solution::wiggle_max_length(n), 3);
    }
    #[test]
    fn n_3_6_6_6_5() {
        let n = vec![3, 6, 6, 6, 5];
        assert_eq!(Solution::wiggle_max_length(n), 3);
    }
    #[test]
    fn n_3_6_5_5_5() {
        let n = vec![3, 6, 5, 5, 5];
        assert_eq!(Solution::wiggle_max_length(n), 3);
    }
    #[test]
    fn n_3_2_5_5_5() {
        let n = vec![3, 2, 5, 5, 5];
        assert_eq!(Solution::wiggle_max_length(n), 3);
    }
}
