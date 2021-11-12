#![allow(dead_code)]
/// 1143. Longest Common Subsequence
/// ================================
///
/// Given two strings `text1` and `text2`,
/// return _the length of their longest __common subsequence___.
/// If there is no __common subsequence__, return `0`.
///
/// A __subsequence__ of a string is a new string generated from the original string with some characters (can be none)
/// deleted without changing the relative order of the remaining characters.
///
/// For example, `"ace"` is a subsequence of `"abcde"`.
///
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
    /// 16:40-16:46
    pub fn longest_common_subsequence_rec(text1: String, text2: String) -> i32 {
        println!("longest_common_subsequence({}, {})", text1, text2);
        fn rec(i1: usize, i2: usize, t1: &[u8], t2: &[u8]) -> i32 {
            if i1 == t1.len() || i2 == t2.len() {
                0
            } else {
                if t1[i1] == t2[i2] {
                    1 + rec(i1 + 1, i2 + 1, t1, t2)
                } else {
                    rec(i1 + 1, i2, t1, t2).max(rec(i1, i2 + 1, t1, t2))
                }
            }
        }
        rec(0, 0, text1.as_bytes(), text2.as_bytes())
    }
    /// 16:46-16:51
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
    /// 16:51-16:56
    pub fn longest_common_subsequence_dp_vec_vec(text1: String, text2: String) -> i32 {
        println!("longest_common_subsequence({}, {})", text1, text2);
        let (t1, t2) = (text1.as_bytes(), text2.as_bytes());
        let mut dp = vec![vec![0; t2.len() + 1]; t1.len() + 1];
        for i1 in (0..t1.len()).rev() {
            for i2 in (0..t2.len()).rev() {
                dp[i1][i2] = if t1[i1] == t2[i2] {
                    1 + dp[i1 + 1][i2 + 1]
                } else {
                    dp[i1 + 1][i2].max(dp[i1][i2 + 1])
                };
            }
        }
        dp[0][0]
    }
    /// 16:56-17:01
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        println!("longest_common_subsequence({}, {})", text1, text2);
        let (t1, t2) = {
            let (t1, t2) = (text1.as_bytes(), text2.as_bytes());
            if t1.len() > t2.len() { (t1, t2) } else { (t2, t1) }
        };
        let mut curr = vec![0; t2.len() + 1];
        let mut prev = curr.clone();
        for i1 in (0..t1.len()).rev() {
            std::mem::swap(&mut curr, &mut prev);
            for i2 in (0..t2.len()).rev() {
                curr[i2] = if t1[i1] == t2[i2] {
                    1 + prev[i2 + 1]
                } else {
                    prev[i2].max(curr[i2 + 1])
                };
            }
        }
        curr[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abcde_ace() {
        let t1 = "abcde".into();
        let t2 = "ace".into();
        assert_eq!(Solution::longest_common_subsequence(t1, t2), 3);
        // Explanation: The longest common subsequence is "ace" and its length is 3.
    }
    #[test]
    fn abc_abc() {
        let t1 = "abc".into();
        let t2 = "abc".into();
        assert_eq!(Solution::longest_common_subsequence(t1, t2), 3);
        // Explanation: The longest common subsequence is "abc" and its length is 3.
    }
    #[test]
    fn abc_def() {
        let t1 = "abc".into();
        let t2 = "def".into();
        assert_eq!(Solution::longest_common_subsequence(t1, t2), 0);
        // Explanation: There is no such common subsequence, so the result is 0.
    }

    #[test]
    fn a_repeat_1000_a_repeat_1000() {
        let t1 = "a".repeat(1000);
        let t2 = "a".repeat(1000);
        assert_eq!(Solution::longest_common_subsequence(t1, t2), 1000);
    }
    #[test]
    fn a_repeat_1000_b_repeat_1000() {
        let t1 = "a".repeat(1000);
        let t2 = "b".repeat(1000);
        assert_eq!(Solution::longest_common_subsequence(t1, t2), 0);
    }
}
