#![allow(dead_code)]

/// # Wiggle Subsequence
///
/// Given an integer array `nums`, return _the length of the longest __wiggle sequence___.
///
/// A __wiggle sequence__ is a sequence where the differences between successive numbers
/// strictly alternate between positive and negative.
/// The first difference (if one exists) may be either positive or negative.
/// A sequence with fewer than two elements is trivially a wiggle sequence.
///
/// - For example, `[1, 7, 4, 9, 2, 5]` is a __wiggle sequence__ because the differences
///   `(6, -3, 5, -7, 3)` are alternately positive and negative.
/// - In contrast, `[1, 4, 7, 2, 5]` and `[1, 7, 4, 5, 5]` are not wiggle sequences,
///   the first because its first two differences are positive
///   and the second because its last difference is zero.
///
/// A __subsequence__ is obtained by deleting some elements (eventually, also zero)
/// from the original sequence, leaving the remaining elements in their original order.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 1000`
/// - `0 <= nums[i] <= 1000`
///
/// __Follow up:__ Could you solve this in `O(n)` time?
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/590/week-3-march-15th-march-21st/3676/
struct Solution;
impl Solution {
    pub fn wiggle_max_length_2vars(nums: Vec<i32>) -> i32 {
        let mut gtz = 1;
        let mut ltz = 1;
        for w in nums.windows(2) {
            if w[0] < w[1] {
                ltz = gtz + 1;
            } else if w[0] > w[1] {
                gtz = ltz + 1;
            }
        }
        gtz.max(ltz)
    }

    pub fn wiggle_max_length_2vec(nums: Vec<i32>) -> i32 {
        let mut gtz = vec![1];
        let mut ltz = vec![1];
        for w in nums.windows(2) {
            if w[0] < w[1] {
                ltz.push(gtz.last().unwrap() + 1);
            } else if w[0] > w[1] {
                gtz.push(ltz.last().unwrap() + 1);
            }
        }
        gtz.into_iter().chain(ltz.into_iter()).max().unwrap()
    }

    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        nums.iter()
            .skip(1)
            .fold((1, 0, nums[0]), |(rsf, ps, p), &c| match (c - p).signum() {
                cs if cs == 0 || cs == ps => (rsf, ps, c),
                cs => (rsf + 1, cs, c)
            }).0
    }

    pub fn wiggle_max_length_my(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, None, None), |(rsf, p_opt, pn_opt), &c| {
                match (p_opt, pn_opt) {
                    (Some(p), Some(pn)) if c == p || (c - p).is_negative() == pn => (rsf, Some(c), pn_opt),
                    (Some(_), Some(pn)) => (rsf + 1, Some(c), Some(!pn)),
                    (Some(p), None) if c == p => (rsf, p_opt, None),
                    (Some(p), None) => (rsf + 1, Some(c), Some((c - p).is_negative())),
                    (None, None) => (1, Some(c), None),
                    _ => panic!(),
                }
            }).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_ns_1_7_4_9_2_5_produces_6() {
        let nums = vec![1, 7, 4, 9, 2, 5];
        assert_eq!(Solution::wiggle_max_length(nums), 6);
        // Explanation: The entire sequence is a wiggle sequence.
    }
    #[test]
    fn example2_ns_1_17_5_10_13_15_10_5_16_8_produces_7() {
        let nums = vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8];
        assert_eq!(Solution::wiggle_max_length(nums), 7);
        // Explanation:
        // There are several subsequences that achieve this length. One is [1,17,10,13,10,16,8].
    }
    #[test]
    fn example3_ns_1_2_3_4_5_6_7_8_9_produces_2() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(Solution::wiggle_max_length(nums), 2);
    }

    #[test]
    fn test8_ns_1_1_7_4_9_2_5_produces_6() {
        let nums = vec![1, 1, 7, 4, 9, 2, 5];
        assert_eq!(Solution::wiggle_max_length(nums), 6);
    }
    #[test]
    fn test9_ns_3_3_3_2_5_produces_3() {
        let nums = vec![3, 3, 3, 2, 5];
        assert_eq!(Solution::wiggle_max_length(nums), 3);
    }
    #[test]
    fn test10_ns_produces_67() {
        let nums = vec![
            33, 53, 12, 64, 50, 41, 45, 21, 97, 35, 47, 92, 39, 0, 93, 55, 40, 46, 69, 42, 6, 95,
            51, 68, 72, 9, 32, 84, 34, 64, 6, 2, 26, 98, 3, 43, 30, 60, 3, 68, 82, 9, 97, 19, 27,
            98, 99, 4, 30, 96, 37, 9, 78, 43, 64, 4, 65, 30, 84, 90, 87, 64, 18, 50, 60, 1, 40, 32,
            48, 50, 76, 100, 57, 29, 63, 53, 46, 57, 93, 98, 42, 80, 82, 9, 41, 55, 69, 84, 82, 79,
            30, 79, 18, 97, 67, 23, 52, 38, 74, 15,
        ];
        assert_eq!(Solution::wiggle_max_length(nums), 67);
    }

    #[test]
    fn ns_1_produces_1() {
        assert_eq!(Solution::wiggle_max_length(vec![1]), 1);
    }
    #[test]
    fn ns_1_2_produces_2() {
        assert_eq!(Solution::wiggle_max_length(vec![1, 2]), 2);
    }
    #[test]
    fn ns_0_1_0_1_0_1_0_1_produces_8() {
        assert_eq!(Solution::wiggle_max_length(vec![0, 1, 0, 1, 0, 1, 0, 1]), 8);
    }
}
