#![allow(dead_code)]
/// Running Sum of 1d Array
/// =======================
///
/// Given an array `nums`.
/// We define a running sum of an array as `runningSum[i] = sum(nums[0]…nums[i])`.
///
/// Return the running sum of `nums`.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 1000`
/// - `-10^6 <= nums[i] <= 10^6`
///
/// https://leetcode.com/explore/featured/card/may-leetcoding-challenge-2021/598/week-1-may-1st-may-7th/3730/
struct Solution;
impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        nums
    }

    pub fn running_sum_fold(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter().fold(Vec::new(), |mut rsf, n| {
            let last = *rsf.last().unwrap_or(&0);
            rsf.push(last + n);
            rsf
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 2, 3, 4];
        let e = vec![1, 3, 6, 10];
        assert_eq!(Solution::running_sum(nums), e);
        // Explanation: Running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4].
    }
    #[test]
    fn example2() {
        let nums = vec![1, 1, 1, 1, 1];
        let e = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::running_sum(nums), e);
        // Explanation: Running sum is obtained as follows: [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1].
    }
    #[test]
    fn example3() {
        let nums = vec![3, 1, 2, 10, 1];
        let e = vec![3, 4, 6, 16, 17];
        assert_eq!(Solution::running_sum(nums), e);
    }
}
