#![allow(dead_code)]
/// 136. Single Number
/// ==================
///
/// Given a __non-empty__ array of integers `nums`, every element appears twice except for one.
/// Find that single one.
///
/// You must implement a solution with a linear runtime complexity and use only constant extra space.
///
/// Constraints:
///
/// - `1 <= nums.length <= 30_000`
/// - `-3 * 104 <= nums[i] <= 30_000`
/// - Each element in the array appears twice except for one element which appears only once.
///
/// https://leetcode.com/problems/single-number/
struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |r, n| r ^ n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_1() {
        let n = vec![1];
        assert_eq!(Solution::single_number(n), 1);
    }
    #[test]
    fn n_2_2_1() {
        let n = vec![2, 2, 1];
        assert_eq!(Solution::single_number(n), 1);
    }
    #[test]
    fn n_4_1_2_1_2() {
        let n = vec![4, 1, 2, 1, 2];
        assert_eq!(Solution::single_number(n), 4);
    }
}
