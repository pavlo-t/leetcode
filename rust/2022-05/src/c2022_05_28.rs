#![allow(dead_code)]
/// \#268. Missing Number
/// =====================
///
/// Given an array `nums` containing `n` distinct numbers in the range `[0, n]`,
/// return _the only number in the range that is missing from the array_.
///
/// __Constraints:__
///
/// - `n == nums.length`
/// - `1 <= n <= 10_000`
/// - `0 <= nums[i] <= n`
/// - All the numbers of `nums` are __unique__.
///
/// __Follow up:__ Could you implement a solution using only
/// `O(1)` extra space complexity and `O(n)` runtime complexity?
///
/// https://leetcode.com/problems/missing-number/
struct Solution;
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut sum = (1..=nums.len() as i32).sum();
        for n in nums {
            sum -= n;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_3_0_1() {
        let n = vec![3, 0, 1];
        assert_eq!(Solution::missing_number(n), 2);
        // Explanation: n = 3 since there are 3 numbers, so all numbers are in the range [0,3]. 2 is the missing number in the range since it does not appear in nums.
    }
    #[test]
    fn n_0_1() {
        let n = vec![0, 1];
        assert_eq!(Solution::missing_number(n), 2);
        // Explanation: n = 2 since there are 2 numbers, so all numbers are in the range [0,2]. 2 is the missing number in the range since it does not appear in nums.
    }
    #[test]
    fn n_9_6_4_2_3_5_7_0_1() {
        let n = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        assert_eq!(Solution::missing_number(n), 8);
        // Explanation: n = 9 since there are 9 numbers, so all numbers are in the range [0,9]. 8 is the missing number in the range since it does not appear in nums.
    }
}
