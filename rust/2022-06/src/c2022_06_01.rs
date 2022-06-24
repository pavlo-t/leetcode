#![allow(dead_code)]
//! \#1480. Running Sum of 1d Array
//! ===============================
//!
//! Given an array `nums`.
//! We define a running sum of an array as `runningSum[i] = sum(nums[0]…nums[i])`.
//!
//! Return the running sum of `nums`.
//!
//! __Constraints:__
//!
//! - `1 <= nums.length <= 1000`
//! - `-1_000_000 <= nums[i] <= 10_000_000`
//!
//! <https://leetcode.com/problems/running-sum-of-1d-array>

pub struct Solution;
impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_1_2_3_4() {
        let n = vec![1, 2, 3, 4];
        let e = vec![1, 3, 6, 10];
        assert_eq!(Solution::running_sum(n), e);
        // Explanation: Running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4].
    }
    #[test]
    fn n_1_1_1_1_1() {
        let n = vec![1, 1, 1, 1, 1];
        let e = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::running_sum(n), e);
        // Explanation: Running sum is obtained as follows: [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1].
    }
    #[test]
    fn n_3_1_2_10_1() {
        let n = vec![3, 1, 2, 10, 1];
        let e = vec![3, 4, 6, 16, 17];
        assert_eq!(Solution::running_sum(n), e);
    }
}
