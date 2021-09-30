#![allow(dead_code)]
/// Number of Subarrays with Bounded Maximum
/// ========================================
///
/// We are given an array `nums` of positive integers,
/// and two positive integers `left` and `right` (`left <= right`).
///
/// Return the number of (contiguous, non-empty) subarrays such that the value of the maximum
/// array element in that subarray is at least `left` and at most `right`.
///
/// __Note:__
///
/// - `left`, `right`, and `nums[i]` will be an integer in the range `[0, 10^9]`.
/// - The length of `nums` will be in the range of `[1, 50000]`.
///
/// https://leetcode.com/explore/challenge/card/june-leetcoding-challenge-2021/605/week-3-june-15th-june-21st/3782/
struct Solution;
impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let mut result = 0;
        let mut m = 0;
        let mut l  = 0;
        for n in nums {
            if n > right {
                m = 0;
            } else {
                m += 1;
                result += m;
            }
            if n >= left {
                l = 0;
            } else {
                l += 1;
                result -= l;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let nums = vec![2, 1, 4, 3];
        let left = 2;
        let right = 3;
        assert_eq!(Solution::num_subarray_bounded_max(nums, left, right), 3);
        // Explanation: There are three subarrays that meet the requirements: [2], [2, 1], [3].
    }

    #[test]
    fn nums_2_2_2_l_2_r_2_produces_6() {
        let nums = vec![2, 2, 2];
        let left = 2;
        let right = 2;
        assert_eq!(Solution::num_subarray_bounded_max(nums, left, right), 6);
    }
    #[test]
    fn nums_0_0_0_l_0_r_0_produces_6() {
        let nums = vec![0, 0, 0];
        let left = 0;
        let right = 0;
        assert_eq!(Solution::num_subarray_bounded_max(nums, left, right), 6);
    }
}
