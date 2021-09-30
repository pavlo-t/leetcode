#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

/// # Vowel Spellchecker
///
/// Given a `wordlist`, we want to implement a spellchecker that converts a query word into a correct word.
///
/// For a given `query` word, the spell checker handles two categories of spelling mistakes:
///
/// - Capitalization: If the query matches a word in the wordlist (__case-insensitive__),
///   then the query word is returned with the same case as the case in the wordlist.
///   - Example: `wordlist = ["yellow"]`, `query = "YellOw"`: `correct = "yellow"`
///   - Example: `wordlist = ["Yellow"]`, `query = "yellow"`: `correct = "Yellow"`
///   - Example: `wordlist = ["yellow"]`, `query = "yellow"`: `correct = "yellow"`
/// - Vowel Errors: If after replacing the vowels ('a', 'e', 'i', 'o', 'u') of the query word
///   with any vowel individually, it matches a word in the wordlist (__case-insensitive__),
///   then the query word is returned with the same case as the match in the wordlist.
///   - Example: `wordlist = ["YellOw"]`, `query = "yollow"`: `correct = "YellOw"`
///   - Example: `wordlist = ["YellOw"]`, `query = "yeellow"`: `correct = ""` (no match)
///   - Example: `wordlist = ["YellOw"]`, `query = "yllw"`: `correct = ""` (no match)
///
/// In addition, the spell checker operates under the following precedence rules:
///
/// - When the query exactly matches a word in the wordlist (__case-sensitive__), you should return the same word back.
/// - When the query matches a word up to capitalization, you should return the first such match in the wordlist.
/// - When the query matches a word up to vowel errors, you should return the first such match in the wordlist.
/// - If the query has no matches in the wordlist, you should return the empty string.
///
/// Given some `queries`, return a list of words `answer`,
/// where `answer[i]` is the correct word for `query = queries[i]`.
///
/// __Note:__
///
/// - `1 <= wordlist.length <= 5000`
/// - `1 <= queries.length <= 5000`
/// - `1 <= wordlist[i].length <= 7`
/// - `1 <= queries[i].length <= 7`
/// - All strings in `wordlist` and `queries` consist only of __english__ letters.
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/591/week-4-march-22nd-march-28th/3681/
struct Solution;
impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        fn remove_vowels(w: &str) -> String {
            let bytes = w
                .to_lowercase()
                .as_bytes()
                .iter()
                .map(|&c| match c {
                    b'a' | b'e' | b'i' | b'o' | b'u' => b'_',
                    c => c,
                })
                .collect::<Vec<u8>>();
            String::from_utf8(bytes).unwrap()
        }

        let word_set = wordlist.iter().map(|s| s.clone()).collect::<HashSet<_>>();
        let mut fixes = HashMap::with_capacity(wordlist.len() * 2);

        for w in wordlist.iter().rev() {
            fixes.insert(w.to_lowercase(), w.clone());
            let vow_key = remove_vowels(w);
            fixes.insert(vow_key, w.clone());
        }

        queries
            .into_iter()
            .map(|q| {
                if word_set.contains(&q) {
                    q
                } else if let Some(r) = fixes.get(&q.to_lowercase()) {
                    r.clone()
                } else if let Some(r) = fixes.get(&remove_vowels(&q)) {
                    r.clone()
                } else {
                    "".to_string()
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let wordlist = vec![
            "KiTe".to_string(),
            "kite".to_string(),
            "hare".to_string(),
            "Hare".to_string(),
        ];
        let queries = vec![
            "kite".to_string(),
            "Kite".to_string(),
            "KiTe".to_string(),
            "Hare".to_string(),
            "HARE".to_string(),
            "Hear".to_string(),
            "hear".to_string(),
            "keti".to_string(),
            "keet".to_string(),
            "keto".to_string(),
        ];
        let expected = vec![
            "kite".to_string(),
            "KiTe".to_string(),
            "KiTe".to_string(),
            "Hare".to_string(),
            "hare".to_string(),
            "".to_string(),
            "".to_string(),
            "KiTe".to_string(),
            "".to_string(),
            "KiTe".to_string(),
        ];
        assert_eq!(Solution::spellchecker(wordlist, queries), expected);
    }
}
