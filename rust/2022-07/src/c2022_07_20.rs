#![allow(dead_code)]
//! \#792. Number of Matching Subsequences
//! ======================================
//!
//! <https://leetcode.com/problems/number-of-matching-subsequences>
//!
//! Given a string `s` and an array of strings `words`, return _the number of `words[i]` that is a subsequence of `s`_.
//!
//! A __subsequence__ of a string is a new string generated from the original string
//! with some characters (can be none) deleted without changing the relative order of the remaining characters.
//!
//! For example, `"ace"` is a subsequence of `"abcde"`.
//!
//! __Constraints:__
//!
//! - `1 <= s.length <= 50_000`
//! - `1 <= words.length <= 5000`
//! - `1 <= words[i].length <= 50`
//! - `s` and `words[i]` consist of only lowercase English letters.

pub struct Solution;
impl Solution {
    /// Brute Force:
    ///
    /// - `O(s.len() * words.len() * words[i].len())` time
    /// - `O(1)` memory
    pub fn num_matching_subseq_v1(s: String, words: Vec<String>) -> i32 {
        fn is_subsequence(word: &str, string: &str) -> bool {
            let string = string.as_bytes();
            let mut i = 0;
            for b in word.bytes() {
                while i < string.len() && string[i] != b {
                    i += 1;
                }
                if i == string.len() {
                    return false;
                }
                i += 1;
            }
            true
        }

        words
            .into_iter()
            .filter(|word| is_subsequence(word, &s))
            .count() as i32
    }

    /// Precalculation:
    ///
    /// - `O(s.len() + words.len() * words[i].len())` time
    /// - `O(s.len() * 26)` memory
    pub fn num_matching_subseq_v2(s: String, words: Vec<String>) -> i32 {
        fn to_usize(b: u8) -> usize {
            (b - b'a') as usize
        }
        fn is_subsequence(word: &str, nexts: &[[Option<usize>; 26]]) -> bool {
            let mut i = 0;
            for c in word.bytes().map(to_usize) {
                if let Some(next) = nexts[i][c] {
                    i = next;
                } else {
                    return false;
                }
            }
            true
        }

        let nexts = {
            let mut result: Vec<[Option<usize>; 26]> = vec![[None; 26]; s.len() + 1];
            let mut nexts: [Option<usize>; 26] = [None; 26];
            for (i, c) in s.bytes().map(to_usize).enumerate().rev() {
                result[i + 1] = nexts.clone();
                nexts[c] = Some(i + 1);
            }
            result[0] = nexts;

            result
        };

        words
            .into_iter()
            .filter(|word| is_subsequence(word, &nexts))
            .count() as i32
    }

    /// Lists of iterators:
    ///
    /// - `O(s.len() + words.len() * words[i].len())` time
    /// - `O(words.len())` memory
    ///
    /// <https://leetcode.com/problems/number-of-matching-subsequences/discuss/117634/Efficient-and-simple-go-through-words-in-parallel-with-explanation>
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        fn to_usize(b: u8) -> usize {
            (b - b'a') as usize
        }
        let mut waiting = vec![vec![]; 26];
        for w in &words {
            let mut iter = w.bytes().map(to_usize);
            waiting[iter.next().unwrap()].push(iter);
        }
        let mut result = 0;
        for c in s.bytes().map(to_usize) {
            waiting.push(vec![]);
            let mut advance = waiting.swap_remove(c);
            while let Some(mut iter) = advance.pop() {
                if let Some(c) = iter.next() {
                    waiting[c].push(iter);
                } else {
                    result += 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs { ($($x:expr),*) => { vec![$($x.to_string()),*] }; }

    #[rustfmt::skip] #[test] fn a_a() { assert_eq!(Solution::num_matching_subseq("a".into(), vs!["a"]), 1); }
    #[rustfmt::skip] #[test] fn a_b() { assert_eq!(Solution::num_matching_subseq("a".into(), vs!["b"]), 0); }

    #[rustfmt::skip] #[test] fn ab_ab() { assert_eq!(Solution::num_matching_subseq("ab".into(), vs!["ab"]), 1); }
    #[rustfmt::skip] #[test] fn ab_ba() { assert_eq!(Solution::num_matching_subseq("ab".into(), vs!["ba"]), 0); }
    #[rustfmt::skip] #[test] fn ab_a_b() { assert_eq!(Solution::num_matching_subseq("ab".into(), vs!["a","b"]), 2); }

    #[test]
    fn abcde_a_bb_acd_ace() {
        let s = "abcde".to_string();
        let w = vs!["a", "bb", "acd", "ace"];
        assert_eq!(Solution::num_matching_subseq(s, w), 3);
        // Explanation: There are three strings in words that are a subsequence of s: "a", "acd", "ace".
    }
    #[test]
    fn dsahjpjauf_ahjpjau_ja_ahbwzgqnuk_tnmlanowax() {
        let s = "dsahjpjauf".to_string();
        let w = vs!["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"];
        assert_eq!(Solution::num_matching_subseq(s, w), 2);
    }

    #[test]
    fn ax49bx49951_ax50x5000() {
        let s = {
            let mut result = "a".repeat(49);
            result.push_str(&"b".repeat(49951));
            result
        };
        let w = vec!["a".repeat(50); 5000];
        assert_eq!(Solution::num_matching_subseq(s, w), 0);
    }
}
