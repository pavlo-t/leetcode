#![allow(dead_code)]
//! \#1658. Minimum Operations to Reduce X to Zero
//! ==============================================
//!
//! You are given an integer array `nums` and an integer `x`.
//! In one operation, you can either remove the leftmost or the rightmost
//! element from the array `nums` and subtract its value from `x`.
//! Note that this __modifies__ the array for future operations.
//!
//! Return _the __minimum number__ of operations to reduce `x`
//! to __exactly__ `0` if it is possible, otherwise, return `-1`_.
//!
//! __Constraints:__
//!
//! - `1 <= nums.length <= 100_000`
//! - `1 <= nums[i] <= 10_000`
//! - `1 <= x <= 1_000_000_000`
//!
//! <https://leetcode.com/problems/minimum-operations-to-reduce-x-to-zero>

pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut target = nums.iter().sum::<i32>() - x;
        if target < 0 {
            return -1;
        }
        let mut result = i32::MAX;
        let mut curr = nums.len() as i32;
        let (mut l, mut r) = (0, 0);
        while r < nums.len() {
            while target < nums[r] && l <= r {
                target += nums[l];
                l += 1;
                curr += 1;
            }
            if nums[r] <= target {
                target -= nums[r];
                curr -= 1;
            }
            r += 1;
            if target == 0 {
                result = result.min(curr);
            }
        }

        match result {
            i32::MAX => -1,
            r => r,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_1_1_4_2_3_x_5() {
        let n = vec![1, 1, 4, 2, 3];
        assert_eq!(Solution::min_operations(n, 5), 2);
        // Explanation: The optimal solution is to remove the last two elements to reduce x to zero.
    }
    #[test]
    fn n_5_6_7_8_9_x_4() {
        let n = vec![5, 6, 7, 8, 9];
        assert_eq!(Solution::min_operations(n, 4), -1);
    }
    #[test]
    fn n_3_2_20_1_1_3_x_10() {
        let n = vec![3, 2, 20, 1, 1, 3];
        assert_eq!(Solution::min_operations(n, 10), 5);
        // Explanation: The optimal solution is to remove the last three elements and the first two elements
        // (5 operations in total) to reduce x to zero.
    }

    #[test]
    fn n_1000_1_1_2_3_x_1004() {
        assert_eq!(Solution::min_operations(vec![1000, 1, 1, 2, 3], 1004), 3);
    }

    #[test]
    fn n_1_1_x_3() {
        assert_eq!(Solution::min_operations(vec![1, 1], 3), -1);
    }
    #[test]
    fn n_1_1_1_x_3() {
        assert_eq!(Solution::min_operations(vec![1, 1, 1], 3), 3);
    }
}
