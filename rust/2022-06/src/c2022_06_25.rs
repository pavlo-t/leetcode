#![allow(dead_code)]
//! \#665. Non-decreasing Array
//! ===========================
//!
//! Given an array `nums` with `n` integers, your task is to check if it could become
//! non-decreasing by modifying __at most one element__.
//!
//! We define an array is non-decreasing if `nums[i] <= nums[i + 1]`
//! holds for every `i` (__0-based__) such that (`0 <= i <= n - 2`).
//!
//! __Constraints:__
//!
//! - `n == nums.length`
//! - `1 <= n <= 10_000`
//! - `-100_000 <= nums[i] <= 100_000`
//!
//! <https://leetcode.com/problems/non-decreasing-array>

pub struct Solution;
impl Solution {
    pub fn check_possibility_mutate_input(mut nums: Vec<i32>) -> bool {
        let mut done_change = false;
        for i in 1..nums.len() {
            if nums[i - 1] > nums[i] {
                if done_change {
                    return false;
                }
                done_change = true;
                if i == 1 || (nums[i - 2] <= nums[i]) {
                    nums[i - 1] = nums[i];
                } else {
                    nums[i] = nums[i - 1];
                }
            }
        }
        true
    }
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut change: Option<(usize, i32)> = None;
        for i in 1..nums.len() {
            if let Some((j, val)) = change {
                if (j == i - 1 && val > nums[i]) || nums[i - 1] > nums[i] {
                    return false;
                }
            } else if nums[i - 1] > nums[i] {
                if i == 1 || (nums[i - 2] <= nums[i]) {
                    change = Some((i - 1, nums[i]));
                } else {
                    change = Some((i, nums[i - 1]));
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn t_1() { assert_eq!(Solution::check_possibility(vec![1]), true); }
    #[rustfmt::skip] #[test] fn t_1_2() { assert_eq!(Solution::check_possibility(vec![1,2]), true); }
    #[rustfmt::skip] #[test] fn t_2_1() { assert_eq!(Solution::check_possibility(vec![2,1]), true); }

    #[rustfmt::skip] #[test] fn t_1_1_1() { assert_eq!(Solution::check_possibility(vec![1,1,1]), true); }
    #[rustfmt::skip] #[test] fn t_1_5_1() { assert_eq!(Solution::check_possibility(vec![1,5,1]), true); }
    #[rustfmt::skip] #[test] fn t_5_1_1() { assert_eq!(Solution::check_possibility(vec![5,1,1]), true); }
    #[rustfmt::skip] #[test] fn t_5_2_1() { assert_eq!(Solution::check_possibility(vec![5,2,1]), false); }

    #[rustfmt::skip] #[test] fn t_2_1_1_1() { assert_eq!(Solution::check_possibility(vec![2,1,1,1]), true); }
    #[rustfmt::skip] #[test] fn t_1_2_1_1() { assert_eq!(Solution::check_possibility(vec![1,2,1,1]), true); }
    #[rustfmt::skip] #[test] fn t_1_1_2_1() { assert_eq!(Solution::check_possibility(vec![1,1,2,1]), true); }
    #[rustfmt::skip] #[test] fn t_1_1_1_2() { assert_eq!(Solution::check_possibility(vec![1,1,1,2]), true); }
    #[rustfmt::skip] #[test] fn t_2_2_2_1() { assert_eq!(Solution::check_possibility(vec![2,2,2,1]), true); }
    #[rustfmt::skip] #[test] fn t_1_2_2_1() { assert_eq!(Solution::check_possibility(vec![1,2,2,1]), true); }
    #[rustfmt::skip] #[test] fn t_1_2_1_2() { assert_eq!(Solution::check_possibility(vec![1,2,1,2]), true); }
    #[rustfmt::skip] #[test] fn t_2_1_1_2() { assert_eq!(Solution::check_possibility(vec![2,1,1,2]), true); }

    #[rustfmt::skip] #[test] fn t_2_2_1_1() { assert_eq!(Solution::check_possibility(vec![2,2,1,1]), false); }
    #[rustfmt::skip] #[test] fn t_2_1_2_1() { assert_eq!(Solution::check_possibility(vec![2,1,2,1]), false); }

    #[test]
    fn t_4_2_3() {
        assert_eq!(Solution::check_possibility(vec![4, 2, 3]), true);
        // Explanation: You could modify the first 4 to 1 to get a non-decreasing array.
    }
    #[test]
    fn t_4_2_1() {
        assert_eq!(Solution::check_possibility(vec![4, 2, 1]), false);
        // Explanation: You can't get a non-decreasing array by modify at most one element.
    }
}
