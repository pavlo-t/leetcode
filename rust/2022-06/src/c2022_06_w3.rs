#![allow(dead_code)]
//! \#527. Word Abbreviation
//! ========================
//!
//! Given an array of __distinct__ strings `words`, return _the minimal possible __abbreviations__ for every word_.
//!
//! The following are the rules for a string abbreviation:
//!
//! 1. Begin with the first character, and then the number of characters abbreviated, followed by the last character.
//! 2. If there is any conflict and more than one word shares the same abbreviation,
//!    a longer prefix is used instead of only the first character
//!    until making the map from word to abbreviation become unique.
//!    In other words, a final abbreviation cannot map to more than one original word.
//! 3. If the abbreviation does not make the word shorter, then keep it as the original.
//!
//! __Constraints:__
//!
//! - `1 <= words.length <= 400`
//! - `2 <= words[i].length <= 400`
//! - `words[i]` consists of lowercase English letters.
//! - All the strings of `words` are __unique__.
//!
//! <https://leetcode.com/problems/word-abbreviation>

pub struct Solution;
impl Solution {
    pub fn words_abbreviation(mut words: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;

        // group words by len and last char
        let mut len_words: HashMap<(usize, char), Vec<usize>> = HashMap::new();
        for i in 0..words.len() {
            if words[i].len() > 3 {
                let last_char = *words[i].as_bytes().last().unwrap() as char;
                let len = words[i].len();
                len_words.entry((len, last_char)).or_default().push(i);
            }
        }

        // change to vec with (curr_char_index, words_index)
        let mut curr_i_words = len_words
            .into_iter()
            .map(|(_, ws)| (0, ws))
            .collect::<Vec<_>>();

        while let Some((ci, ws)) = curr_i_words.pop() {
            let len = words[ws[0]].len();
            // regroup words by char at ci
            let mut char_words: HashMap<char, Vec<usize>> = HashMap::new();
            for &wi in ws.iter() {
                let c = words[wi].as_bytes()[ci] as char;
                char_words.entry(c).or_default().push(wi);
            }
            for (_, wis) in char_words {
                // if there is only one word in the group - we can abbreviate it
                if wis.len() == 1 {
                    let wi = wis[0];
                    let word = words[wi].as_bytes();
                    let mut abbr = String::new();
                    for i in 0..=ci {
                        abbr.push(word[i] as char);
                    }
                    abbr.push_str(&mut (word.len() - ci - 2).to_string());
                    abbr.push(word[word.len() - 1] as char);
                    words[wi] = abbr;
                } else if ci < len - 4 {
                    curr_i_words.push((ci + 1, wis));
                }
            }
        }

        words
    }

    pub fn words_abbreviation_hash_set(mut words: Vec<String>) -> Vec<String> {
        use std::collections::{HashMap, HashSet};

        // group words by len and last char
        let mut len_words: HashMap<(usize, char), HashSet<usize>> = HashMap::new();
        for i in 0..words.len() {
            if words[i].len() > 3 {
                let last_char = *words[i].as_bytes().last().unwrap() as char;
                let len = words[i].len();
                len_words.entry((len, last_char)).or_default().insert(i);
            }
        }

        // change to vec with (curr_char_index, words_index)
        let mut curr_i_words = len_words
            .into_iter()
            .map(|(_, ws)| (0, ws))
            .collect::<Vec<_>>();

        while let Some((ci, ws)) = curr_i_words.pop() {
            let len = words[*ws.iter().next().unwrap()].len();
            // regroup words by char at ci
            let mut char_words: HashMap<char, HashSet<usize>> = HashMap::new();
            for &wi in ws.iter() {
                let c = words[wi].as_bytes()[ci] as char;
                char_words.entry(c).or_default().insert(wi);
            }
            for (_, wis) in char_words {
                // if there is only one word in the group - we can abbreviate it
                if wis.len() == 1 {
                    let wi = wis.into_iter().next().unwrap();
                    let word = words[wi].as_bytes();
                    let mut abbr = String::new();
                    for i in 0..=ci {
                        abbr.push(word[i] as char);
                    }
                    abbr.push_str(&mut (word.len() - ci - 2).to_string());
                    abbr.push(word[word.len() - 1] as char);
                    words[wi] = abbr;
                } else if ci < len - 4 {
                    curr_i_words.push((ci + 1, wis));
                }
            }
        }

        words
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($x:expr),*) => {vec![$($x.to_string()),*]}}

    #[test]
    fn like_god_internal_me_internet_interval_intension_face_intrusion() {
        let w = vs![
            "like",
            "god",
            "internal",
            "me",
            "internet",
            "interval",
            "intension",
            "face",
            "intrusion"
        ];
        let e = vs!["l2e", "god", "internal", "me", "i6t", "interval", "inte4n", "f2e", "intr4n"];
        assert_eq!(Solution::words_abbreviation(w), e);
    }
    #[test]
    fn aa_aaa() {
        let w = vs!["aa", "aaa"];
        let e = vs!["aa", "aaa"];
        assert_eq!(Solution::words_abbreviation(w), e);
    }
    #[test]
    fn aaaaa_abaaa_acaaa() {
        let w = vs!["aaaaa", "abaaa", "acaaa"];
        let e = vs!["aa2a", "ab2a", "ac2a"];
        assert_eq!(Solution::words_abbreviation(w), e);
    }
    #[test]
    fn aaaaa_aaaba_aaaca() {
        let w = vs!["aaaaa", "aaaba", "aaaca"];
        let e = vs!["aaaaa", "aaaba", "aaaca"];
        assert_eq!(Solution::words_abbreviation(w), e);
    }
    #[test]
    fn aaaaaa_abaaaa_acaaaa_acbaaa() {
        let w = vs!["aaaaaa", "abaaaa", "acaaaa", "acbaaa"];
        let e = vs!["aa3a", "ab3a", "aca2a", "acb2a"];
        assert_eq!(Solution::words_abbreviation(w), e);
    }
    #[test]
    fn aaaaaa_aabaaa_acaaaa_acbaaa() {
        let w = vs!["aaaaaa", "aabaaa", "acaaaa", "acbaaa"];
        let e = vs!["aaa2a", "aab2a", "aca2a", "acb2a"];
        assert_eq!(Solution::words_abbreviation(w), e);
    }

    //#[ignore]
    #[test]
    fn test_48() {
        let w = vs![
            "abababaababababaabbbbaabbaabbbabbaaaabbaaabaabaabaaaabbbaaabbaabaaabababbabaaaaabaaaaaaaaabababababbababbaababbaaabaaaababaaabbabbbbbabaaabbabaabaabbbaababaaaaaabaabbabbaaaaaababaabaabbaabbbbaa",
            "bbbaabbbbabbabaababbbbbbbbaabaaababbabbaaababaabbaaaabba",
            "babbbbbbabaabbbaabaaaabbbbaabaaaabbbaabbaaaabbbbaaabababaabbaababbbbaaabababbabaabbaaabbaaababaababbabbabbbaaaaabababbbaabababaaaaabbbbabaaaaabaabbbbaaaabbaabaaaaaabaab",
            "abbbbaaaaabaabbababaabbabbbbbbaaaabbbbbbaaabbbbababaabbbbbbbbbabbabaababaabbaabbabaabaaaaabbbbbaaabbabbbabbbbbbaabbaabaaabaaaabbbaaaaaabbaabaabbbaabbaabbaaaabbbbaaaabaababbabbabbaabaaabaaaa",
            "baabbbbaababbaaabbbababaabbaaaaaaabbbaabbaaababbbaaaaaaabbaabbaabbaaaaabaaababbababaababbabaaaababbaabbbabbabaaababbaaabab",
            "aaaabaabaabbabbaabbbabaaabaababbabbaaaabbbaabbaababbbaabbbaabaaba",
            "aaaaabbbbbabbbaabaabbbbabbaababbbabaaabbbaaaaabbbaababbbbaaaabaaaabbababaaabaabbbbaaaaaabbbaabbbbbaaaaabbababbbbbaabbabbabbbaaabbbaaabbbbbabbaaabbaabbabbbbabbaaababababbaaabaab",
            "aaaababaaaaabbbabaabbbabbabbaaaaaaaabbaaaabbbbbbaabbabaabb",
            "aabbabbaaababbabbaabbbaaaaabaaabbabaaabbaabaaaaaaaabbaaabbababbbabbbaabbabaaba",
            "aabaabbaababbabbaabaaabaaababbbbbabaabaabbbabbbaabaaaaabbabbbbbabbbabbabbaaabbabbbbbabbbbabbbaabaaababaaabbbaabbaaabbabaaaababaaabbabbaaaabbbabbbbabbbbbababbabaabaababbbaa",
            "bbbbbabaababbababaabbaaabaabaaabbaababaaababbbabaaabaabbabbba",
            "abaaaababbbaaabbbaaabbaaabbbabaabbaababbababbabbbaaaaabbaabbbbbbbaabbbabbabbaaaaabbabbaaabaaaababaabababbabbbbbabababaabbabbaaababaabbaabaaaaaabbbbbbbbabbaabaabbabaaaabaababaaabbbaaaabbbaabaaabbbaaa",
            "aabababbbbbaaabbabbaababbaabbaaaabaabababbabababaaabaaaaabbabbabbabaaabaabbbaaabb",
            "abbbbbaabaabbbaaaabbaabaaabbbbabbaaaaaabaabbab",
            "aaaaaaabbbabbabbbabbbbbabaabbaaaaabbbabbaaaaabbaabbaaaba",
            "abaaabbbaaabbbbbbbbbbaabbbbbbbaabaaabaabaabbabbbbaabaabababbabbabaaaabbbbbbabababbbbbbbbbaabbaaaaababbbaaababbbabaababaabbaaaaaaaabaaabbaaababbabbbabbbabbaabbba",
            "abbbabaaaaaaaaabbaabbabababbbbbbbaaababaabaababbaaaababaaabaababbbbbbaaaabababaaabbabbaaaabababaaaabbbbaaaabbabbabbbbabbbaabbababaaababbbabbaaaaaabbbbaaaaabbbbababaaababbbbbbaabbbbaaababbabaa",
            "abbaaabaabaababbaaaaaaaabbabaababbbaaabaaaabaabbaababaabaaaabaabaabaaabbabbbabbbbabbbaaaaaaababaaaaabbabbaaababbbabbababbabbaaaaaaaaabbbbabbbabbbbbbabbbbbaabbaabaaabababbabaaaabaaababbababbbbaababb",
            "bbababbaaababbbababbbbbaabbbbbaaabaaaaababb",
            "baaabbabbabaabbbbabbaaaaabbaabbabaaaaabbbbabaabbabaababaababbaabbbabbabbbbabab",
            "bbbaab",
            "bbaaaaaaaabbbaaaababbaaababbaabbbababbabbaabaaaaaaabaababbbabbabababa",
            "abaababbaaaaabbaaaaabbabbbbbababbbaaababaabaaabbaaabbaaabbbbbaabbbaaaaababbbaaababa",
            "baaaab",
            "aababaaaaabaabababbaabaaababbbbbaaabbbabbbaaabaaaabbbabbbbbbaabaababbbabaa",
            "aaabbaabaabbbabaabbaaabaabbaabaaaaabaabaabbbaaaaaaaabaabababbbabaaaaaababaaabaaaabaabbbbbbbaabbbbabababababbaabbaabbababbabbbbbbbbbbbbabbbbbbababaabbbbaabbbaabbbabaaaabbbbbabbbbbbbabbbbaabaaaaababa",
            "baabbaabaababbbbaaabbababbabbabbaaaabbaaabaabaababbaaabbaaaaaabaaabbbbababbaaabaabaabbabbbabaabbabaaabbabaabbbbbbbbabbbbabababababbbbababababbbaaabbabbbbababbaababaabaabbaabbabaaabbbaaabba",
            "bbbabaabababbbbbaaaaabbbabaaabbaabbababababbbbbabbaabaaabbbaaaaabaabbbabababababaabbababbbaabaaaabaabaabbaabbabaaaabbbbaababaaabbab",
            "aaaababababaabaababbbabababb",
            "abbbbabbaaaaaaaabbbbbaaaab",
            "aaababbbabaabb",
            "bbaaabababbbaaabbbaaaaaabbaabbabaaaababaaaaabbaa",
            "aaaaaabaaaabbabbabbababababaabb",
            "bbaaabbabbbaabbbbbbabbabbababaaaaaabbabababbabbbbbbabaababbbaaaaabbaabbbbbaabbaaabbbaaabababbabbbabababaabbbbbbbabbaaaaabbabbaaaabbbbbbabbbabababbabaabbbbbbaaababaabbabbabbbaabbbbbaaaaabab",
            "bbaababbbbbabaaabaabaaabbabbbaaaaabbbaabaabbbabaabbbbababbabbb",
            "aaabababaaabbbbababbbbbabbbbbabbbaaababbabbb",
            "abbbabaababbbbbabaababbbbaaaaaabaaaabbbbbabbbababbbaabbaabababbbabaabaabababaabbaaababaaababaabbbabababbaaaababbbb",
            "aabaabbabbabaabaaaabaabaabaababbaaabbbaabaaaaaaaabbbbaaabababaaaa",
            "bbbabaaaaaaaaabab",
            "abaabaaaaabbabaaabbaaabbaabbbbbababbaabbabbbbbaabababbabbaababaabbaabaabbbbbbaabaabaaabbaaababbabaaaabbabbbabaabaabbaaababbababababbbbbbabbbbaabaaabbbabaaababababbbbaabbbbaaabaaaba",
            "babaaabaabbababaabaabbaabaabbaabbaaaaaaaabbbabbbaaabbbbbaaabaaabbbbabaabbbabaabaabaaaaabaabaaaabbaaaaaabbbaababbbaaabaaabbabbaaabaabbbaaabaaaaababbaaaabaaabaababbabbbbbbbbb",
            "baabbaaba",
            "abbabababbabbbbbbbbbbbaaaababbbbbbabbabaababbabbaabbaba",
            "aababbabaabbbaaaaaaaabbbbaaaaaaaaababbaaababbababababaabbbaaabaaabbabbbaaababbbabbaaabaaaabaab",
            "ababbab",
            "babaabbaaabbabbbbababbaababbbbaaaaaabbaababaaaaabaabbbbaaaabbbaaaaaabbbabbaaabaababbbbabbaabbbbbbaabbabbbabababbaabb",
            "bbaaabbaabbaaababbaababbbabababbbaababbbbbbbbaabbbbabaaabbbbaaaabbabbaaaaabab",
            "babbaababbbbbbbbbbabaabbabaaaabababababaaaaabbbaaabbbbbbaa",
            "abbbabbbba",
            "babaaababaabaabbaabbbbbaabbbbabaabbbbaabbabbabbbbbabbbbabbabbaaababbbaabbaaabaabbabbabaaaabbbaabbbaaabbbbabbaaaaabbabbabbbbaababaababababbaaaaaaaabbaaabaaaabababaaababbbaaaaaaaabaabaababb",
            "babbbaabbbbaababaaabababbaababbbbba",
            "baababbbbaaaaabaaabbbbbaaabbbaabbbabbbabaaaabbaabbbbbbaababaaabbbaababbbaaababbbbbababbba",
            "baaaabbaaaaaabaaababbaabbaabbaaaaabbabbbbbabbbabaaaaabbbbaaabbbaabbaaabbbbaaaaababbbbbabbabaabbbbbbbbaabbbaabbaabbaaabaaaabbbabaaaababaabaabbbabaabbbbbbabbaababaabbaaabbbabbabababababbabbaaaaabba",
            "baaaaabbabaababbabaaaabaaaaaababbababaa",
            "babaabbababababbbabaabbbbbbbbbbbbbbbabbababababbabbaaaababbbbbbbabaaababbbbbbbbaaabaaabaabbabbbbbabaaabababa",
            "bb",
            "abbbbbabbaaabbaaaaabaaabaabbbaaabbbbabababbbbbababbabaaaaaabbab",
            "baaabbaabaaababaaabbaabbbbbbbababaababbbabbabbabbaaabbabbaabaabaaaaaaabbbabbaabb",
            "bbabbbbbbabbbabbaabab",
            "bbbbbaabbbbaaaaabababababbbbbababaaabbbaabaabaaabababaaaaabaaaabbbbb",
            "bbbbbbabaabbaaababaabaababaaaababababaabbabbbbbbababbbaaabaabbabbbabbbaaaaaaabaaabaabbbbbbba",
            "baaabaaaabababbbbbbaabbbabaaabbbaaabbababaabbabbbababbaaababbabbbbaaabbaabbaaaaabababaabbbbbabbabaabbbbbbaababbbaaaaabaaaabaabaabbababab",
            "abbbbbabbbbbaabbaaabbbbabbabaaaabaabbaaaaaabbaaabaaaaabbbabaabbaaaabbbaabaaaaabbaababaaaabaabaaaabbaabbbaabbaabbabbaaba",
            "ab",
            "bbabbaabbbbbbaaaababbaaabbbaabaababbbaabaaaabbaababaaaaabbbabbbbbaabbabaabbabababbaabaaabbbbaabbabbabbaaaaaabbbbbbbbaaabbbbbbbbababbbbbbbabaabbbabaaaabbbbababbababaabbbabaabbaababbaabbabababbabbabba",
            "babbbaaaaaaabbbabbaababababaaaaaabbababbabbabbabbbbbabbbbaabbbaaabbaabababbbabaababababaabbbbabbababbbabbbbaabbabbabbaabbaaababbabaabaaabaaababbbaabbbaaaababaaaabaaaaaababaaabbbbabbbbabbaa",
            "abaabbbababbbbaabaaa",
            "baaabababbabbaabaaaabbabaabbbaaabaabaababaaaaaabaabaabaabbbabaababaabababbbababaabbaabaaabaabbabaaabababbabbabbbbbaaaaabbabbabbabbbaaabbbbabbbaaaababbbaabaaaaaabaabaaabaaabbbaabaabaababbbbb",
            "bababbbaabbbbababababbababaabaabbbbbbbabbbbbaababbaaaababbaababbbbabbabbabaaaabaababaaabbaabb",
            "bbbbabababbbbaabbababbaaabbbbbbbbabaaabbbaabbabaabbababbabbabbaabbbaaabbabbbbbbbbaaaaababbaaaaababbaaaabaaaabbbaaaabaaaabbbbaaaabbabbaabaababaaaaaababaabaaababbaaababaabababbbbabaababbbbaa",
            "bbaabbbabbbbbbbabbaaaababababaaababaaababbaabbbaaabababaaabbbbbaabaababbbabaaabaabbabbbbbbbbbbaaababaaaabaaabbababaaabbaaaaaaabbaabaababaabbbbbabaaaabaabababbabbabaaabbabbaababa",
            "baabbbbbaaabbabbbabbbbabbaaabaababaaababa",
            "abbaababaaaabbbbbabaaaaabbabababbbaabbaaabbaaaabbbbbaabbaabbbbbbbbba",
            "aabaabbbbaaabaabbaabbbbaabbbbaaaaaabbab",
            "ababbbbababaabbaabaaabaababbaaabbbabbaaabbbbaabaaabaabaabbbaaaaabbbababaaaaabbaabbbbabbbababaaaababbaaabaaaaaaabaaaa",
            "bbbaaaaabbbbabababbaabbaabaabbababaabaaabaabbbabbbbbaaaababbaaababbbabaabbaabbbbbabbaaabbaababababbabbaaaababaabbaaabaaaaabbabbbaaaaaabaababbababbaaaabaaabaaaaababaa",
            "bbbbabababbb",
            "aababababbbaaaaabbabbaabbbaabbabaaabbababaabaaaaabbaaababaaabbbababaababbababaabbbbbabaabbbaaaaaaabbbbaaaaaabaabbaababbbabaaaabaabbaaabbbbaaabbaabababbabaabaababababba",
            "bbbbbabbbbbbbbaaababaaaabbbbaabbaaaabbaaabbbbabaaaabbbbbaaaabbbabbbbaababaaabaaaababba",
            "aaababbbaaaaabbbaaabbbbbbbbabbbbbabbaabbababababaababbaabaaaababbaaabaaaabaababbbaabbabababaabbaaaaaabbbaaaabbaababaaaaabaabbbbbbaaaabbabbaabaababaaaabbaababbaaababbbaabbabbaababbabbabbbaaaaba",
            "abbbbbaabaaabbaaaaaabbabababbbaabaaaaaabaabbabbaaabbbbbabbaaaaabaababbaabaaabbabaabbaaaaaaaababaaaaabaabbbaaababbbababbbbaaabbbbaababaaaabbbbbbbbbaaaaabbaabbbabaabbbbabbabbbbabbbaaaaababbbbaaaaaba",
            "aaababbaabbabbabbbabbaaaababababb",
            "aaaababbbbbbbaabbbbaabbbababbaaaabbbaabbaaaaababbbaabbbaababbbaabbbaabbbbabaaabbbaaabaaaabbbbbbbaabaaaababbbbbaaaaababbb",
            "bbaabaaaaaabbabbbbbaabaabbabbbbaabaababaabbbbaabbaaabaabaabbabbbbbbababaabaababbaabbbabbbbbbbabaaaaabbbabaaabababbabbbabbbabbaaaabba",
            "bbaabbbbbaababbaaababaaabbabaaaaaabbaaaaaaaaabbbbabbabbbabbbbaabbaabbbbbbaabaababb",
            "bbabbaaaababaaaaabbaabbababbbaaabbbaaaaaabbabaaabbaaaabbbaabbaabbbabb",
            "bbbbbaabaa",
            "abaaaaabbabbabaaabbaaabaaababaaaabaabbabbaaabbbbbbbbbaabbbababaaaabbaaababbabbabbaababbabbabbbabaaaabbabbbbbbaabaaaababaaaaaaaaaaaaaaaaabaabababbbbaaaabaab",
            "bababaaaaabbbbaabbaabaababbabaabaababbbbabbabaaabbabaaababababbbaaaababbababbbbaabbabbbaaaabbbabbabbbaaaaabaaaabaabaababababbaaaaabaaababbaabaaabbabbbbabababababbbabbbaabbabaaaabbaabaabbaabb",
            "babbabaaabaa",
            "abbaaaababbabaaabbabaaaabaaabbaabaaaaabbbbbaabaaabbaaababbabbbabaaaaaaabbabbaababbbabbabababababaaaaabababbbabbaababbbaaab",
            "abbbabaabbabbabaaababaaaaabbaabababbbabbabbbaababaaaaabaabababbabba",
            "bbbabbabaaabaabbabaabbbabaabbaabbaaaabbb",
            "bbaaabaaaabbbabababbabbbbabbbbaaabbbaaabbbabaaabaabbbabbabaaabbbbbbabbbbaaabbabbabbaababbbbbababbaaabbbbabbbbbbbbaaabbbbabb",
            "aaaababbaababababaaaababbabaabbaabababababaabaaabaaabbabbababaaaabaaaababbababbbbbbbaabbaabababbaabbbabbbabbbbaababbaababbbbbbabbbbaaaabbabbaaabbbaaaabbbabbaabbaabaaabbb",
            "ababaabaaabbababbbbaabbaabbaaaaabababbbbbbabbabaa",
            "babaaaaabbbbbaababbaabaabbbbabbbbbaaabbbbababbabbbbbbabbbbbabaaababbbbbbbbaabababbaaababbbaaaaabbababaababababababbaaabbbbbabbababababbbbaabbaabbabaabaaaabaabbaaabaabbabbab",
            "aabaabaabbababbbbbbaaababaabaabbbbbaabbbbaaabbbababbaabaaaababbaaaaaaabbabaaaababbbbaabbaabbbbabbaabbbabbabbbbbbaabbbababbaaabbbbaaaabbbbbba",
            "abaaaabbaabbbaababbbaabaabaaaabbabbbabbbbbaaaaaaaaaaaabaaabbbbbabbabbbaabbbabbabaabaabbabaaabbbabaaaaaaaabababaaabababaabbbaabbbbabababaabaabbbbaabbaaaabbabbbaaaaabaabbbbbabaabb",
            "abaabaabbaaabbbabaabbaabaaaababbaababaaabbabaaabbababbbaabbbaababbbaaaabaabbaa",
            "abbbaabbbbaabbbbababbababbbbabbaabaabbbbbbbaaabaaabaaabaabbabbbabaabbbaaaababbaabbabbbabbbbaaabbbabbbbbbbaabbaabaaaabbbbbababaabaabbabbabaabababaaaabaabbbabababb",
            "bbababbbababbaaaaabbaabbaaababbababbabababaabaabababbbbbbbaabbbaabbaaaabaabaaabbaaaabbabaaaaaabaaaabbaabbbaabbbbbaababbbbbbbaabaaaabbbbaaaababbbbbbaaaaaaabbbbaabbabbbabbaaabaaaabbbbbbbbbbb",
            "aabaaaabbbbaabbaabbbbabbbbaababbbaababbaaabababbbbbbaaabbba",
            "aabbbbaaababbaaabbbbbbbbb",
            "abbaabaaaabababaababaaaa",
            "bbaababbaababbbbaaaaaaabbaaaabbbababbbaababbbbbbaabaaaaabbabababbbababbabbaaabaaaaaaabaaabbaaababababbabbbaaaabbbababbbbabbabbbbbaabbabaabbbbabb",
            "bababbababbabab"];
        let e = [
            "a191a",
            "b54a",
            "b166b",
            "a187a",
            "b120b",
            "aaa61a",
            "a174b",
            "a56b",
            "aa75a",
            "a169a",
            "b59a",
            "a196a",
            "a79b",
            "a44b",
            "a54a",
            "a158a",
            "a189a",
            "a195b",
            "b41b",
            "b76b",
            "bb3b",
            "b67a",
            "a81a",
            "ba3b",
            "a72a",
            "a195a",
            "baa184a",
            "b129b",
            "a26b",
            "a24b",
            "a12b",
            "b46a",
            "a29b",
            "bbaa183b",
            "b60b",
            "a42b",
            "a112b",
            "aab61a",
            "b15b",
            "a178a",
            "babaaab164b",
            "b7a",
            "a53a",
            "a92b",
            "a5b",
            "b114b",
            "b75b",
            "b56a",
            "a8a",
            "b185b",
            "b33a",
            "b87a",
            "b193a",
            "b37a",
            "b106a",
            "bb",
            "a61b",
            "b78b",
            "b19b",
            "b66b",
            "b90a",
            "b134b",
            "a117a",
            "ab",
            "b196a",
            "bab184a",
            "a18a",
            "b187b",
            "b91b",
            "bb185a",
            "b175a",
            "b39a",
            "a66a",
            "a37b",
            "a114a",
            "b163a",
            "b10b",
            "a165a",
            "b84a",
            "a190a",
            "a194a",
            "a31b",
            "a118b",
            "b130a",
            "b80b",
            "b67b",
            "b8a",
            "a153b",
            "b188b",
            "b10a",
            "a120b",
            "a65a",
            "b38b",
            "b121b",
            "a167b",
            "a47a",
            "babaaaa164b",
            "a138a",
            "a175b",
            "ab75a",
            "a159b",
            "bbab183b",
            "a57a",
            "a23b",
            "a22a",
            "b142b",
            "b13b",
        ];
        assert_eq!(Solution::words_abbreviation(w), e);
    }
}
