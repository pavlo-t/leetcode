#![allow(dead_code)]
//! \#890. Find and Replace Pattern
//! ===============================
//!
//! <https://leetcode.com/problems/find-and-replace-pattern>
//!
//! Given a list of strings `words` and a string `pattern`, return _a list of `words[i]` that match pattern_.
//! You may return the answer in __any order__.
//!
//! A word matches the pattern if there exists a permutation of letters `p`
//! so that after replacing every letter `x` in the pattern with `p(x)`,
//! we get the desired word.
//!
//! Recall that a permutation of letters is a bijection from letters to letters:
//! every letter maps to another letter, and no two letters map to the same letter.
//!
//! __Constraints:__
//!
//! - `1 <= pattern.length <= 20`
//! - `1 <= words.length <= 50`
//! - `words[i].length == pattern.length`
//! - `pattern` and `words[i]` are lowercase English letters.
//!
//! # Examples
//!
//! ```
//! # use c2022_07::c2022_07_29::*;
//! # macro_rules! vs { ($($x:expr),*) => { vec![$($x.to_string()),*] }; }
//! let words    = vs!["abc", "deq", "mee", "aqq", "dkd", "ccc"];
//! let pattern  = "abb".to_string();
//! let expected = vs!["mee", "aqq"];
//! assert_eq!(Solution::find_and_replace_pattern(words, pattern), expected);
//! ```
//!
//! __Explanation:__ `"mee"` matches the pattern because there is a permutation `{a -> m, b -> e, ...}`.
//! `"ccc"` does not match the pattern because `{a -> c, b -> c, ...}` is not a permutation,
//! since `a` and `b` map to the same letter.
//!
//! ```
//! # use c2022_07::c2022_07_29::*;
//! # macro_rules! vs { ($($x:expr),*) => { vec![$($x.to_string()),*] }; }
//! let words    = vs!["a", "b", "c"];
//! let pattern  = "a".to_string();
//! let expected = vs!["a", "b", "c"];
//! assert_eq!(Solution::find_and_replace_pattern(words, pattern), expected);
//! ```

pub struct Solution;
impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        fn to_pattern(word: &str) -> Vec<usize> {
            use std::collections::HashMap;
            let mut map = HashMap::new();
            let mut next_mapping = 0;
            let mut result = vec![];
            for c in word.chars() {
                if let Some(&mapping) = map.get(&c) {
                    result.push(mapping);
                } else {
                    result.push(next_mapping);
                    map.insert(c, next_mapping);
                    next_mapping += 1;
                }
            }
            result
        }

        let pattern = to_pattern(&pattern);
        words
            .into_iter()
            .filter(|w| to_pattern(w) == pattern)
            .collect()
    }
}
