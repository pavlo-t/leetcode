#![allow(dead_code)]

/// # Missing Number
///
/// Given an array `nums` containing `n` distinct numbers in the range `[0, n]`, return
/// _the only number in the range that is missing from the array_.
///
/// __Follow up:__ Could you implement a solution using only `O(1)` extra space complexity
/// and `O(n)` runtime complexity?
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 10_000`
/// - `0 <= nums[i] <= nums.length`
/// - All the numbers of `nums` are __unique__.
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/588/week-1-march-1st-march-7th/3659/
struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .fold(nums.len() as i32, |acc, (i, n)| acc ^ n ^ i as i32)
    }

    pub fn missing_number_sum_gauss(nums: Vec<i32>) -> i32 {
        (nums.len() * (nums.len() + 1) / 2) as i32 - nums.iter().sum::<i32>()
    }

    pub fn missing_number_sum(nums: Vec<i32>) -> i32 {
        (0..=nums.len() as i32).sum::<i32>() - nums.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_n301_should_produce_2() {
        let nums = vec![3, 0, 1];
        assert_eq!(Solution::missing_number(nums), 2);
        // Explanation:
        // n = 3 since there are 3 numbers, so all numbers are in the range [0,3].
        // 2 is the missing number in the range since it does not appear in nums.
    }

    #[test]
    fn example2_n01_should_produce_2() {
        let nums = vec![0, 1];
        assert_eq!(Solution::missing_number(nums), 2);
        // Explanation:
        // n = 2 since there are 2 numbers, so all numbers are in the range [0,2].
        // 2 is the missing number in the range since it does not appear in nums.
    }

    #[test]
    fn example3_n964235701_should_produce_8() {
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        assert_eq!(Solution::missing_number(nums), 8);
        // Explanation:
        // n = 9 since there are 9 numbers, so all numbers are in the range [0,9].
        // 8 is the missing number in the range since it does not appear in nums.
    }

    #[test]
    fn example4_n0_should_produce_1() {
        let nums = vec![0];
        assert_eq!(Solution::missing_number(nums), 1);
        // Explanation:
        // n = 1 since there is 1 number, so all numbers are in the range [0,1].
        // 1 is the missing number in the range since it does not appear in nums.
    }
}
