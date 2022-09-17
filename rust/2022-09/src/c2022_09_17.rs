#![allow(dead_code)]
//! \#336. Palindrome Pairs
//! =======================
//!
//! <https://leetcode.com/problems/palindrome-pairs>
//!
//! Given a list of __unique__ words, return all the pairs of the __distinct__ indices `(i, j)` in the given list,
//! so that the concatenation of the two words `words[i] + words[j]` is a palindrome.
//!
//! ##### Examples
//!
//! ###### Example 1:
//!
//! ```
//! # use c2022_09::vs;
//! # use c2022_09::c2022_09_17::*;
//! let words = vs!["abcd", "dcba", "lls", "s", "sssll"];
//! let mut result = Solution::palindrome_pairs(words);
//! result.sort_unstable();
//! assert_eq!(result, [[0, 1], [1, 0], [2, 4], [3, 2]]);
//! ```
//!
//! __Explanation:__ The palindromes are `["dcbaabcd", "abcddcba", "slls", "llssssll"]`
//!
//! ###### Example 2:
//!
//! ```
//! # use c2022_09::vs;
//! # use c2022_09::c2022_09_17::*;
//! let words = vs!["bat", "tab", "cat"];
//! let mut result = Solution::palindrome_pairs(words);
//! result.sort_unstable();
//! assert_eq!(result, [[0, 1], [1, 0]]);
//! ```
//!
//! __Explanation:__ The palindromes are `["battab", "tabbat"]`
//!
//! ###### Example 3:
//!
//! ```
//! # use c2022_09::vs;
//! # use c2022_09::c2022_09_17::*;
//! let words = vs!["a", ""];
//! let mut result = Solution::palindrome_pairs(words);
//! result.sort_unstable();
//! assert_eq!(result, [[0, 1], [1, 0]]);
//! ```
//!
//! ##### Constraints
//!
//! - `1 <= words.length <= 5000`
//! - `0 <= words[i].length <= 300`
//! - `words[i]` consists of lower-case English letters.

pub struct Solution;
impl Solution {
    pub fn palindrome_pairs_v1(words: Vec<String>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        fn is_palindrome(s: &str) -> bool {
            s.is_empty() || {
                let bytes = s.as_bytes();
                let (mut l, mut r) = (0, bytes.len() - 1);
                while l < r {
                    if bytes[l] != bytes[r] {
                        return false;
                    }
                    l += 1;
                    r -= 1;
                }
                true
            }
        }

        fn valid_suffixes(w: &str) -> HashSet<String> {
            let mut result = HashSet::new();
            for i in 0..w.len() {
                if is_palindrome(&w[i..]) {
                    let suffix = w[..i].chars().rev().collect();
                    result.insert(suffix);
                }
            }
            result
        }

        fn valid_prefixes(w: &str) -> HashSet<String> {
            let mut result = HashSet::new();
            for i in 0..w.len() {
                if is_palindrome(&w[..=i]) {
                    let prefix = w[i + 1..].chars().rev().collect();
                    result.insert(prefix);
                }
            }
            result
        }

        let mut words_with_indices = words.into_iter().enumerate().collect::<Vec<_>>();
        words_with_indices.sort_unstable_by_key(|(_, w)| w.len());

        let mut results = vec![];

        while let Some((i, word)) = words_with_indices.pop() {
            let reversed = word.chars().rev().collect::<String>();
            let suffixes = valid_suffixes(&word);
            let prefixes = valid_prefixes(&word);

            for (j, w) in &words_with_indices {
                if w == &reversed {
                    results.push(vec![i as i32, *j as i32]);
                    results.push(vec![*j as i32, i as i32]);
                } else {
                    if suffixes.contains(w) {
                        results.push(vec![i as i32, *j as i32]);
                    }
                    if prefixes.contains(w) {
                        results.push(vec![*j as i32, i as i32]);
                    }
                }
            }
        }

        results
    }

    pub fn palindrome_pairs_v2(words: Vec<String>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;

        fn is_palindrome(s: &str) -> bool {
            s.is_empty() || {
                let bytes = s.as_bytes();
                let (mut l, mut r) = (0, bytes.len() - 1);
                while l < r {
                    if bytes[l] != bytes[r] {
                        return false;
                    }
                    l += 1;
                    r -= 1;
                }
                true
            }
        }

        fn valid_suffixes(w: &str) -> Vec<String> {
            let mut result = vec![];
            for i in 0..w.len() {
                if is_palindrome(&w[i..]) {
                    result.push(w[..i].chars().rev().collect());
                }
            }
            result
        }

        fn valid_prefixes(w: &str) -> Vec<String> {
            let mut result = vec![];
            for i in 0..w.len() {
                if is_palindrome(&w[..=i]) {
                    result.push(w[i + 1..].chars().rev().collect());
                }
            }
            result
        }

        let word_to_idx = {
            let mut map = HashMap::new();
            for (i, word) in words.into_iter().enumerate() {
                map.insert(word, i as i32);
            }
            map
        };

        let mut results = vec![];

        for word in word_to_idx.keys() {
            let i = *word_to_idx.get(word).unwrap();

            let reversed = word.chars().rev().collect::<String>();

            if let Some(&j) = word_to_idx.get(&reversed) {
                if i != j {
                    results.push(vec![i, j]);
                }
            }

            for suffix in valid_suffixes(&word) {
                if let Some(&j) = word_to_idx.get(&suffix) {
                    results.push(vec![i, j]);
                }
            }

            for prefix in valid_prefixes(&word) {
                if let Some(&j) = word_to_idx.get(&prefix) {
                    results.push(vec![j, i]);
                }
            }
        }

        results
    }

    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;

        fn is_palindrome(s: &str) -> bool {
            s.is_empty() || {
                let bytes = s.as_bytes();
                let (mut l, mut r) = (0, bytes.len() - 1);
                while l < r {
                    if bytes[l] != bytes[r] {
                        return false;
                    }
                    l += 1;
                    r -= 1;
                }
                true
            }
        }

        let word_to_idx = {
            let mut map = HashMap::new();
            for (i, word) in words.into_iter().enumerate() {
                map.insert(word, i as i32);
            }
            map
        };

        let mut results = vec![];

        for word in word_to_idx.keys() {
            let i = *word_to_idx.get(word).unwrap();

            let reversed = word.chars().rev().collect::<String>();

            if let Some(&j) = word_to_idx.get(&reversed) {
                if i != j {
                    results.push(vec![i, j]);
                }
            }

            for s in 0..word.len() {
                if is_palindrome(&word[s..]) {
                    let suffix = word[..s].chars().rev().collect::<String>();
                    if let Some(&j) = word_to_idx.get(&suffix) {
                        results.push(vec![i, j]);
                    }
                }
            }

            for s in 0..word.len() {
                if is_palindrome(&word[..=s]) {
                    let prefix = word[s + 1..].chars().rev().collect::<String>();
                    if let Some(&j) = word_to_idx.get(&prefix) {
                        results.push(vec![j, i]);
                    }
                }
            }
        }

        results
    }
}
