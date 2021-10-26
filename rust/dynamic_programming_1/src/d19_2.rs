#![allow(dead_code)]
/// 1143. Longest Common Subsequence
/// ================================
///
/// Given two strings `text1` and `text2`, return _the length of their longest __common subsequence___.
/// If there is no __common subsequence__, return `0`.
///
/// A __subsequence__ of a string is a new string generated from the original string with some characters (can be none)
/// deleted without changing the relative order of the remaining characters.
///
/// For example, `"ace"` is a subsequence of `"abcde"`.
/// A __common subsequence__ of two strings is a subsequence that is common to both strings.
///
/// __Constraints:__
///
/// - `1 <= text1.length, text2.length <= 1000`
/// - `text1` and `text2` consist of only lowercase English characters.
///
/// https://leetcode.com/problems/longest-common-subsequence/
struct Solution;
impl Solution {
    /// 21:27
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        println!("longest_common_subsequence({}, {})", text1, text2);
        let (t1, t2) = (text1.as_bytes(), text2.as_bytes());
        let (l1, l2) = (t1.len(), t2.len());
        let mut dp = vec![0; l2 + 1];
        for i1 in (0..l1).rev() {
            let mut prev = 0;
            for i2 in (0..l2).rev() {
                let curr = dp[i2];
                dp[i2] = if t1[i1] == t2[i2] {
                    1 + prev
                } else {
                    dp[i2].max(dp[i2 + 1])
                };
                prev = curr;
            }
        }
        dp[0]
    }
    /// 21:23-21:27
    pub fn longest_common_subsequence_dp_vec_vec(text1: String, text2: String) -> i32 {
        println!("longest_common_subsequence({}, {})", text1, text2);
        let (t1, t2) = (text1.as_bytes(), text2.as_bytes());
        let (l1, l2) = (t1.len(), t2.len());
        let mut dp = vec![vec![0; l2 + 1]; l1 + 1];
        for i1 in (0..l1).rev() {
            for i2 in (0..l2).rev() {
                dp[i1][i2] = if t1[i1] == t2[i2] {
                    1 + dp[i1 + 1][i2 + 1]
                } else {
                    dp[i1 + 1][i2].max(dp[i1][i2 + 1])
                };
            }
        }
        dp[0][0]
    }
    /// 21:17-21:23
    pub fn longest_common_subsequence_rec_with_memo(text1: String, text2: String) -> i32 {
        println!("longest_common_subsequence({}, {})", text1, text2);
        fn rec(i1: usize, i2: usize, t1: &[u8], t2: &[u8], memo: &mut Vec<Vec<i32>>) -> i32 {
            if i1 == t1.len() || i2 == t2.len() {
                0
            } else if memo[i1][i2] >= 0 {
                memo[i1][i2]
            } else {
                memo[i1][i2] = if t1[i1] == t2[i2] {
                    1 + rec(i1 + 1, i2 + 1, t1, t2, memo)
                } else {
                    rec(i1 + 1, i2, t1, t2, memo).max(rec(i1, i2 + 1, t1, t2, memo))
                };
                memo[i1][i2]
            }
        }
        let (t1, t2) = (text1.as_bytes(), text2.as_bytes());
        let mut memo = vec![vec![-1; t2.len()]; t1.len()];
        rec(0, 0, t1, t2, &mut memo)
    }
    /// 21:10-21:17
    pub fn longest_common_subsequence_rec(text1: String, text2: String) -> i32 {
        println!("longest_common_subsequence({}, {})", text1, text2);
        fn rec(i1: usize, i2: usize, t1: &[u8], t2: &[u8]) -> i32 {
            if i1 == t1.len() || i2 == t2.len() {
                0
            } else if t1[i1] == t2[i2] {
                1 + rec(i1 + 1, i2 + 1, t1, t2)
            } else {
                rec(i1 + 1, i2, t1, t2).max(rec(i1, i2 + 1, t1, t2))
            }
        }
        let (t1, t2) = (text1.as_bytes(), text2.as_bytes());
        rec(0, 0, t1, t2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1_abcde_t2_ace() {
        let t1 = "abcde".to_string();
        let t2 = "ace".to_string();
        assert_eq!(Solution::longest_common_subsequence(t1, t2), 3);
        // Explanation: The longest common subsequence is "ace" and its length is 3.
    }
    #[test]
    fn t1_abc_t2_abc() {
        let t1 = "abc".to_string();
        let t2 = "abc".to_string();
        assert_eq!(Solution::longest_common_subsequence(t1, t2), 3);
        // Explanation: The longest common subsequence is "abc" and its length is 3.
    }
    #[test]
    fn t1_abc_t2_def() {
        let t1 = "abc".to_string();
        let t2 = "def".to_string();
        assert_eq!(Solution::longest_common_subsequence(t1, t2), 0);
        // Explanation: There is no such common subsequence, so the result is 0.
    }

    #[test]
    fn t1_1000a_t2_1000a() {
        let t1 = "a".repeat(1000);
        let t2 = "a".repeat(1000);
        assert_eq!(Solution::longest_common_subsequence(t1, t2), 1000);
    }
    #[test]
    fn t1_100a_t2_1000b() {
        let t1 = "a".repeat(100);
        let t2 = "b".repeat(1000);
        assert_eq!(Solution::longest_common_subsequence(t1, t2), 0);
    }
    #[test]
    fn t1_1000a_t2_100b() {
        let t1 = "a".repeat(1000);
        let t2 = "b".repeat(100);
        assert_eq!(Solution::longest_common_subsequence(t1, t2), 0);
    }
    #[test]
    fn t1_1000a_t2_1000b() {
        let t1 = "a".repeat(1000);
        let t2 = "b".repeat(1000);
        assert_eq!(Solution::longest_common_subsequence(t1, t2), 0);
    }
}
