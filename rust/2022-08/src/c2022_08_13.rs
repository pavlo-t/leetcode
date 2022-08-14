#![allow(dead_code)]
//! \#30. Substring with Concatenation of All Words
//! ===============================================
//!
//! <https://leetcode.com/problems/substring-with-concatenation-of-all-words>
//!
//! You are given a string `s` and an array of strings `words` of __the same length__.
//! Return all starting indices of substring(s) in `s` that is a concatenation of each word in `words`
//! __exactly once__, __in any order__, and __without any intervening characters__.
//!
//! You can return the answer in __any order__.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_08::c2022_08_13::*;
//! # use c2022_08::vs;
//! let s = "barfoothefoobarman".to_string();
//! let words = vs!["foo","bar"];
//!
//! assert_eq!(Solution::find_substring(s, words), vec![0, 9]);
//! ```
//!
//! __Explanation:__ Substrings starting at index `0` and `9` are `"barfoo"` and `"foobar"` respectively.
//! The output order does not matter, returning `[9,0]` is fine too.
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_13::*;
//! # use c2022_08::vs;
//! let s = "wordgoodgoodgoodbestword".to_string();
//! let words = vs!["word","good","best","word"];
//!
//! assert_eq!(Solution::find_substring(s, words), vec![]);
//! ```
//!
//! ###### Example 3
//!
//! ```
//! # use c2022_08::c2022_08_13::*;
//! # use c2022_08::vs;
//! let s = "barfoofoobarthefoobarman".to_string();
//! let words = vs!["bar","foo","the"];
//!
//! assert_eq!(Solution::find_substring(s, words), vec![6, 9, 12]);
//! ```
//!
//! ##### Constraints
//!
//! - `1 <= s.length <= 10_000`
//! - `1 <= words.length <= 5000`
//! - `1 <= words[i].length <= 30`
//! - `s` and `words[i]` consist of lowercase English letters.

pub struct Solution;
impl Solution {
    /// Brute-force
    pub fn find_substring_v1(s: String, words: Vec<String>) -> Vec<i32> {
        let word_len = words[0].len();
        let slice_len = words.len() * word_len;

        if slice_len > s.len() {
            return vec![];
        }

        let mut result = vec![];
        for slice_start in 0..=(s.len().saturating_sub(slice_len)) {
            let slice = &s[slice_start..slice_start + slice_len];
            let mut seen = vec![false; words.len()];
            for word_start in (0..slice.len()).step_by(word_len) {
                let word = &slice[word_start..word_start + word_len];
                if let Some(idx) = words
                    .iter()
                    .enumerate()
                    .find(|&(i, w)| !seen[i] && w == word)
                    .map(|(i, _)| i)
                {
                    seen[idx] = true;
                } else {
                    break;
                }
            }
            if seen.into_iter().all(|s| s) {
                result.push(slice_start as i32);
            }
        }
        result
    }

    /// Using `HashMap` `word_counts`
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        use std::collections::HashMap;

        let word_count = words.len();
        let word_len = words[0].len();
        let slice_len = word_count * word_len;

        if slice_len > s.len() {
            return vec![];
        }

        let mut word_counts = HashMap::new();
        for w in words {
            *word_counts.entry(w).or_insert(0usize) += 1;
        }

        let mut result = vec![];
        for slice_start in 0..=(s.len().saturating_sub(slice_len)) {
            let slice = &s[slice_start..slice_start + slice_len];
            let mut seen_counts = word_counts.clone();
            let mut seen_words = 0;
            for word_start in (0..slice.len()).step_by(word_len) {
                let word = &slice[word_start..word_start + word_len];
                if let Some(count) = seen_counts.get_mut(word).filter(|c| c > &&mut 0) {
                    *count -= 1;
                    seen_words += 1;
                } else {
                    break;
                }
            }
            if seen_words == word_count {
                result.push(slice_start as i32);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vs;

    #[rustfmt::skip] #[test] fn a_a() { assert_eq!(Solution::find_substring("a".into(), vs!["a"]), vec![0]); }
    #[rustfmt::skip] #[test] fn a_b() { assert_eq!(Solution::find_substring("a".into(), vs!["b"]), vec![]); }

    #[rustfmt::skip] #[test] fn a_a_a() { assert_eq!(Solution::find_substring("a".into(), vs!["a","a"]), vec![]); }

    #[rustfmt::skip] #[test] fn aa_a() { assert_eq!(Solution::find_substring("aa".into(), vs!["a"]), vec![0,1]); }
    #[rustfmt::skip] #[test] fn aa_a_a() { assert_eq!(Solution::find_substring("aa".into(), vs!["a", "a"]), vec![0]); }

    /// __Explanation:__ Substrings starting at index `0` and `9` are `"barfoo"` and `"foobar"` respectively.
    /// The output order does not matter, returning `[9,0]` is fine too.
    #[test]
    fn barfoothefoobarman_foo_bar() {
        let s = "barfoothefoobarman".to_string();
        let words = vs!["foo", "bar"];
        let expected = vec![0, 9];
        assert_eq!(Solution::find_substring(s, words), expected);
    }

    #[test]
    fn wordgoodgoodgoodbestword_word_good_best_word() {
        let s = "wordgoodgoodgoodbestword".to_string();
        let words = vs!["word", "good", "best", "word"];
        let expected = vec![];
        assert_eq!(Solution::find_substring(s, words), expected);
    }

    #[test]
    fn barfoofoobarthefoobarman_bar_foo_the() {
        let s = "barfoofoobarthefoobarman".to_string();
        let words = vs!["bar", "foo", "the"];
        let expected = vec![6, 9, 12];
        assert_eq!(Solution::find_substring(s, words), expected);
    }

    //#[test]
    //fn a_x_10000_aa_repeat_5000() {
    //    let s = "a".repeat(10000);
    //    let words = vec!["aa".to_string(); 5000];
    //    let expected = vec![0];
    //    assert_eq!(Solution::find_substring(s, words), expected);
    //}

    //#[test]
    //fn a_x_10000_a_x_30_repeat_333() {
    //    let s = "a".repeat(10000);
    //    let words = vec!["a".repeat(30); 333];
    //    let expected = (0..11).collect::<Vec<_>>();
    //    assert_eq!(Solution::find_substring(s, words), expected);
    //}
}
