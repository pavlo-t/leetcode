#![allow(dead_code)]
/// Non-decreasing Array
/// ====================
///
/// Given an array `nums` with `n` integers, your task is to check if it could become
/// non-decreasing by modifying __at most one element__.
///
/// We define an array is non-decreasing if `nums[i] <= nums[i + 1]` holds
/// for every `i` (__0-based__) such that (`0 <= i <= n - 2`).
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 10_000`
/// - `-100_000 <= nums[i] <= 100_000`
///
/// https://leetcode.com/explore/featured/card/may-leetcoding-challenge-2021/598/week-1-may-1st-may-7th/3731/
struct Solution;
impl Solution {
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        let mut prev = -100_000;
        let mut modified = false;
        for i in 1..nums.len() {
            if nums[i - 1] > nums[i] {
                if modified {
                    return false;
                }
                if prev > nums[i] {
                    nums[i] = nums[i - 1];
                } else {
                    nums[i - 1] = prev;
                }
                modified = true;
            }
            prev = nums[i - 1];
        }
        true
    }

    pub fn check_possibility_change_one_and_check(mut nums: Vec<i32>) -> bool {
        let mut prev = -100_000;
        for i in 1..nums.len() {
            if nums[i - 1] > nums[i] {
                if prev > nums[i] {
                    nums[i] = nums[i - 1];
                } else {
                    nums[i - 1] = prev;
                }
                break;
            }
            prev = nums[i - 1];
        }
        nums.windows(2).all(|w| w[0] <= w[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![4, 2, 3];
        assert!(Solution::check_possibility(nums));
        // Explanation: You could modify the first 4 to 1 to get a non-decreasing array.
    }
    #[test]
    fn example2() {
        let nums = vec![4, 2, 1];
        assert!(!Solution::check_possibility(nums));
        // Explanation: You can't get a non-decreasing array by modify at most one element.
    }

    #[test]
    fn test324() {
        let nums = vec![5, 7, 1, 8];
        assert!(Solution::check_possibility(nums));
    }
}
