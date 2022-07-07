#![allow(dead_code)]
//! \#97. Interleaving String
//! =========================
//!
//! Given strings `s1`, `s2`, and `s3`, find whether `s3` is formed by an __interleaving__ of `s1` and `s2`.
//!
//! An __interleaving__ of two strings `s` and `t` is a configuration
//! where they are divided into __non-empty__ substrings such that:
//!
//! - `s = s1 + s2 + ... + sn`
//! - `t = t1 + t2 + ... + tm`
//! - `|n - m| <= 1`
//! - The __interleaving__ is `s1 + t1 + s2 + t2 + s3 + t3 + ...` or `t1 + s1 + t2 + s2 + t3 + s3 + ...`
//!
//! __Note:__ `a + b` is the concatenation of strings `a` and `b`.
//!
//! __Constraints:__
//!
//! - `0 <= s1.length, s2.length <= 100`
//! - `0 <= s3.length <= 200`
//! - `s1`, `s2`, and `s3` consist of lowercase English letters.
//!
//! __Follow up:__ Could you solve it using only `O(s2.length)` additional memory space?
//!
//! <https://leetcode.com/problems/interleaving-string>

pub struct Solution;
impl Solution {
    pub fn is_interleave_rec_brute_force(s1: String, s2: String, s3: String) -> bool {
        fn rec(i1: usize, i2: usize, s1: &[u8], s2: &[u8], s3: &[u8]) -> bool {
            let i3 = i1 + i2;
            if i1 >= s1.len() && i2 >= s2.len() {
                true
            } else if i1 >= s1.len() {
                s2[i2] == s3[i3] && rec(i1, i2 + 1, s1, s2, s3)
            } else if i2 >= s2.len() {
                s1[i1] == s3[i3] && rec(i1 + 1, i2, s1, s2, s3)
            } else {
                (s1[i1] == s3[i3] && rec(i1 + 1, i2, s1, s2, s3))
                    || (s2[i2] == s3[i3] && rec(i1, i2 + 1, s1, s2, s3))
            }
        }
        let (bs1, bs2, bs3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        s1.len() + s2.len() == s3.len() && rec(0, 0, bs1, bs2, bs3)
    }

    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        fn rec(i1: usize, i2: usize, s1: &[u8], s2: &[u8], s3: &[u8], memo: &mut Vec<Vec<bool>>) -> bool {
            let i3 = i1 + i2;
            if !memo[i1][i2] {
                let result = if i1 >= s1.len() && i2 >= s2.len() {
                    true
                } else if i1 >= s1.len() {
                    s2[i2] == s3[i3] && rec(i1, i2 + 1, s1, s2, s3, memo)
                } else if i2 >= s2.len() {
                    s1[i1] == s3[i3] && rec(i1 + 1, i2, s1, s2, s3, memo)
                } else {
                    (s1[i1] == s3[i3] && rec(i1 + 1, i2, s1, s2, s3, memo))
                        || (s2[i2] == s3[i3] && rec(i1, i2 + 1, s1, s2, s3, memo))
                };
                if result {
                    return true;
                } else {
                    memo[i1][i2] = true;
                }
            }
            false
        }
        let (bs1, bs2, bs3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let mut memo = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        s1.len() + s2.len() == s3.len() && rec(0, 0, bs1, bs2, bs3, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aabcc_dbbca_aadbbcbcac() {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbcbcac".to_string();
        assert_eq!(Solution::is_interleave(s1, s2, s3), true);
    }
    #[test]
    fn aabcc_dbbca_aadbbbaccc() {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbbaccc".to_string();
        assert_eq!(Solution::is_interleave(s1, s2, s3), false);
    }
    #[test]
    fn empty_empty_empty() {
        let s1 = "".to_string();
        let s2 = "".to_string();
        let s3 = "".to_string();
        assert_eq!(Solution::is_interleave(s1, s2, s3), true);
    }

    #[test]
    fn test_105() {
        let s1 = "bbbcc".to_string();
        let s2 = "bbaccbbbabcacc".to_string();
        let s3 = "bbbbacbcccbcbabbacc".to_string();
        assert_eq!(Solution::is_interleave(s1, s2, s3), false);
    }
    #[test]
    fn test_106() {
        let s1 = "abababababababababababababababababababababababababababababababababababababababababababababababababbb".to_string();
        let s2 = "babababababababababababababababababababababababababababababababababababababababababababababababaaaba".to_string();
        let s3 = "abababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababbb".to_string();
        assert_eq!(Solution::is_interleave(s1, s2, s3), false);
    }

    #[test]
    fn ax100_ax100_ax200() {
        let s1 = "a".repeat(100);
        let s2 = "a".repeat(100);
        let s3 = "a".repeat(200);
        assert_eq!(Solution::is_interleave(s1, s2, s3), true);
    }
}
