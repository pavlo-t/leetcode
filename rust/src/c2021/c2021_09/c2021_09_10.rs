#![allow(dead_code)]
/// Arithmetic Slices II - Subsequence
/// ==================================
///
/// Given an integer array `nums`, return _the number of all the __arithmetic subsequences__ of `nums`_.
///
/// A sequence of numbers is called arithmetic if it consists of __at least three elements__
/// and if the difference between any two consecutive elements is the same.
///
/// For example, `[1, 3, 5, 7, 9]`, `[7, 7, 7, 7]`, and `[3, -1, -5, -9]` are arithmetic sequences.
/// For example, `[1, 1, 2, 5, 7]` is not an arithmetic sequence.
///
/// A __subsequence__ of an array is a sequence that can be formed
/// by removing some elements (possibly none) of the array.
///
/// For example, `[2,5,10]` is a subsequence of `[1,2,1,2,4,1,5,10]`.
///
/// The test cases are generated so that the answer fits in __32-bit__ integer.
///
/// __Constraints:__
///
/// - `1  <= nums.length <= 1000`
/// - `-2^31 <= nums[i] <= 2^31 - 1`
///
/// https://leetcode.com/explore/challenge/card/september-leetcoding-challenge-2021/637/week-2-september-8th-september-14th/3970/
struct Solution;
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let n = nums.len();
        let mut dp = vec![HashMap::new(); n];
        let mut result = 0;
        for i in 1..n {
            for j in 0..i {
                let d = nums[i] as i64 - nums[j] as i64;
                let jd = dp[j].get(&d).map(|&d| d).unwrap_or(0);
                result += jd;
                *dp[i].entry(d).or_insert(0) += 1 + jd;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_2_4_6_8_10_produces_7() {
        let nums = vec![2, 4, 6, 8, 10];
        assert_eq!(Solution::number_of_arithmetic_slices(nums), 7);
        // Explanation: All arithmetic subsequence slices are:
        // [2,4,6]
        // [4,6,8]
        // [6,8,10]
        // [2,4,6,8]
        // [4,6,8,10]
        // [2,4,6,8,10]
        // [2,6,10]
    }
    #[test]
    fn n_7_7_7_7_7_produces_16() {
        let nums = vec![7, 7, 7, 7, 7];
        assert_eq!(Solution::number_of_arithmetic_slices(nums), 16);
        // Explanation: Any subsequence of this array is arithmetic.
    }
    #[test]
    fn n_1_produces_0() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1]), 0);
    }
    #[test]
    fn n_1_1_produces_0() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 1]), 0);
    }
    #[test]
    fn n_1_1_1_produces_1() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 1, 1]), 1);
    }
    #[test]
    fn n_1_1_1_1_produces_5() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 1, 1, 1]), 5);
    }
}
