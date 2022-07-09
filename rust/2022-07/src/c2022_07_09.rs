#![allow(dead_code)]
//! \#1696. Jump Game VI
//! ====================
//!
//! You are given a __0-indexed__ integer array `nums` and an integer `k`.
//!
//! You are initially standing at index `0`.
//! In one move, you can jump at most `k` steps forward without going outside the boundaries of the array.
//! That is, you can jump from index `i` to any index in the range `[i + 1, min(n - 1, i + k)]` __inclusive__.
//!
//! You want to reach the last index of the array (index `n - 1`).
//! Your __score__ is the __sum__ of all `nums[j]` for each index `j` you visited in the array.
//!
//! Return _the __maximum score__ you can get_.
//!
//! __Constraints:__
//!
//! - `1 <= nums.length, k <= 100_000`
//! - `-10_000 nums[i] <= 10_000`
//!
//! <https://leetcode.com/problems/jump-game-vi>

pub struct Solution;
impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::BinaryHeap;
        let k = k as usize;
        let mut dp = BinaryHeap::new();
        for (i, mut n) in nums.into_iter().enumerate().rev() {
            while !dp.is_empty() && dp.peek().filter(|&&(_, j)| j > i + k).is_some() {
                dp.pop();
            }
            if let Some(&(max, _)) = dp.peek() {
                n += max;
            }
            if i == 0 {
                return n;
            } else {
                dp.push((n, i));
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1_k_2() { assert_eq!(Solution::max_result(vec![1], 2), 1); }
    #[rustfmt::skip] #[test] fn n_1_m1_2_m2_k_1() { assert_eq!(Solution::max_result(vec![1,-1,2,-2], 1), 0); }
    #[rustfmt::skip] #[test] fn n_1_m1_2_m2_k_4() { assert_eq!(Solution::max_result(vec![1,-1,2,-2], 4), 1); }

    #[test]
    fn n_1_m1_m2_4_m7_3_k_2() {
        let n = vec![1, -1, -2, 4, -7, 3];
        assert_eq!(Solution::max_result(n, 2), 7);
        // Explanation: You can choose your jumps forming the subsequence [1,-1,4,3]. The sum is 7.
    }
    #[test]
    fn n_10_m5_m2_4_0_3_k_3() {
        let n = vec![10, -5, -2, 4, 0, 3];
        assert_eq!(Solution::max_result(n, 3), 17);
        // Explanation: You can choose your jumps forming the subsequence [10,4,3]. The sum is 17.
    }
    #[test]
    fn n_1_m5_m20_4_m1_3_m6_m3_k_2() {
        let n = vec![1, -5, -20, 4, -1, 3, -6, -3];
        assert_eq!(Solution::max_result(n, 2), 0);
    }

    #[test]
    fn n_1to10000x10_k_100000() {
        let n = (0..100_000).map(|n| 1 + (n % 10_000)).collect();
        assert_eq!(Solution::max_result(n, 100_000), 500_050_000);
    }
    #[test]
    fn n_m10000to10000x5_k_100000() {
        let n = (0..100_005).map(|n| (n % 20_001) - 10_000).collect();
        assert_eq!(Solution::max_result(n, 100_000), 250_015_000);
    }
}
