#![allow(dead_code)]
//! \#583. Delete Operation for Two Strings
//! =======================================
//!
//! Given two strings `word1` and `word2`, return
//! _the minimum number of __steps__ required to make `word1` and `word2` the same_.
//!
//! In one __step__, you can delete exactly one character in either string.
//!
//! __Constraints:__
//!
//! - `1 <= word1.length, word2.length <= 500`
//! - `word1` and `word2` consist of only lowercase English letters.
//!
//! <https://leetcode.com/problems/delete-operation-for-two-strings>

pub struct Solution;
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (w1, w2) = {
            if word1.len() > word2.len() {
                (word1.as_bytes(), word2.as_bytes())
            } else {
                (word2.as_bytes(), word1.as_bytes())
            }
        };

        let mut dp = vec![0; w2.len() + 1];

        for w2l in 0..w2.len() {
            dp[w2l] = (w2.len() - w2l) as i32;
        }
        for i1 in (0..w1.len()).rev() {
            let mut prev = dp[w2.len()];
            dp[w2.len()] = (w1.len() - i1) as i32;
            for i2 in (0..w2.len()).rev() {
                let curr = dp[i2];
                if w1[i1] == w2[i2] {
                    dp[i2] = prev;
                } else {
                    dp[i2] = 1 + dp[i2].min(dp[i2 + 1]);
                }
                prev = curr;
            }
        }

        dp[0]
    }

    pub fn min_distance_dp_vec_vec(word1: String, word2: String) -> i32 {
        let (w1, w2) = (word1.as_bytes(), word2.as_bytes());
        let mut dp = vec![vec![0; w2.len() + 1]; w1.len() + 1];
        for w1l in 0..w1.len() {
            dp[w1l][w2.len()] = (w1.len() - w1l) as i32;
        }
        for w2l in 0..w2.len() {
            dp[w1.len()][w2l] = (w2.len() - w2l) as i32;
        }
        for i1 in (0..w1.len()).rev() {
            for i2 in (0..w2.len()).rev() {
                if w1[i1] == w2[i2] {
                    dp[i1][i2] = dp[i1 + 1][i2 + 1];
                } else {
                    dp[i1][i2] = 1 + dp[i1][i2 + 1].min(dp[i1 + 1][i2]);
                }
            }
        }

        dp[0][0]
    }

    pub fn min_distance_rec_memo(word1: String, word2: String) -> i32 {
        fn rec(i1: usize, i2: usize, w1: &[u8], w2: &[u8], memo: &mut Vec<Vec<i32>>) -> i32 {
            if i1 == w1.len() {
                (w2.len() - i2) as i32
            } else if i2 == w2.len() {
                (w1.len() - i1) as i32
            } else {
                if memo[i1][i2] == -1 {
                    memo[i1][i2] = if w1[i1] == w2[i2] {
                        rec(i1 + 1, i2 + 1, w1, w2, memo)
                    } else {
                        let skip1 = rec(i1 + 1, i2, w1, w2, memo);
                        let skip2 = rec(i1, i2 + 1, w1, w2, memo);
                        1 + skip1.min(skip2)
                    };
                }
                memo[i1][i2]
            }
        }
        let (w1, w2) = (word1.as_bytes(), word2.as_bytes());
        let mut memo = vec![vec![-1; w2.len()]; w1.len()];
        rec(0, 0, w1, w2, &mut memo)
    }

    pub fn min_distance_rec(word1: String, word2: String) -> i32 {
        fn rec(i1: usize, i2: usize, w1: &[u8], w2: &[u8]) -> usize {
            if i1 == w1.len() {
                w2.len() - i2
            } else if i2 == w2.len() {
                w1.len() - i1
            } else if w1[i1] == w2[i2] {
                rec(i1 + 1, i2 + 1, w1, w2)
            } else {
                let skip1 = rec(i1 + 1, i2, w1, w2);
                let skip2 = rec(i1, i2 + 1, w1, w2);
                1 + skip1.min(skip2)
            }
        }
        rec(0, 0, word1.as_bytes(), word2.as_bytes()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn a_a() { assert_eq!(Solution::min_distance("a".into(), "a".into()), 0); }
    #[rustfmt::skip] #[test] fn a_b() { assert_eq!(Solution::min_distance("a".into(), "b".into()), 2); }
    #[rustfmt::skip] #[test] fn aab_aa() { assert_eq!(Solution::min_distance("aab".into(), "aa".into()), 1); }
    #[rustfmt::skip] #[test] fn aba_aa() { assert_eq!(Solution::min_distance("aba".into(), "aa".into()), 1); }
    #[rustfmt::skip] #[test] fn baa_aa() { assert_eq!(Solution::min_distance("baa".into(), "aa".into()), 1); }

    #[test]
    fn sea_eat() {
        let word1 = "sea".to_string();
        let word2 = "eat".to_string();
        assert_eq!(Solution::min_distance(word1, word2), 2);
        // Explanation: You need one step to make "sea" to "ea" and another step to make "eat" to "ea".
    }
    #[test]
    fn leetcode_etco() {
        let word1 = "leetcode".to_string();
        let word2 = "etco".to_string();
        assert_eq!(Solution::min_distance(word1, word2), 4);
    }
    #[test]
    fn etco_leetcode() {
        let word1 = "etco".to_string();
        let word2 = "leetcode".to_string();
        assert_eq!(Solution::min_distance(word1, word2), 4);
    }

    #[test]
    fn abcdefg_gfedcba() {
        let word1 = "abcdefg".to_string();
        let word2 = "gfedcba".to_string();
        assert_eq!(Solution::min_distance(word1, word2), 12);
    }

    #[test]
    fn a_x_500_a_x_500() {
        let word1 = "a".repeat(500);
        let word2 = "a".repeat(500);
        assert_eq!(Solution::min_distance(word1, word2), 0);
    }
    #[test]
    fn a_x_500_b_x_500() {
        let word1 = "a".repeat(500);
        let word2 = "b".repeat(500);
        assert_eq!(Solution::min_distance(word1, word2), 1000);
    }
}
