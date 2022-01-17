#![allow(dead_code)]
/// 290. Word Pattern
/// =================
///
/// Given a `pattern` and a string `s`, find if `s` follows the same pattern.
///
/// Here __follow__ means a full match,
/// such that there is a bijection between a letter in `pattern` and a __non-empty__ word in s.
///
/// __Constraints:__
///
/// - `1 <= pattern.length <= 300`
/// - `pattern` contains only lower-case English letters.
/// - `1 <= s.length <= 3000`
/// - `s` contains only lowercase English letters and spaces `' '`.
/// - `s` __does not contain__ any leading or trailing spaces.
/// - `All the words in `s` are separated by a __single space__.
///
/// https://leetcode.com/problems/word-pattern/
struct Solution;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        use std::collections::HashMap;
        let mut words_to_chars: HashMap<&str, char> = HashMap::new();
        let mut chars_to_words: HashMap<char, &str> = HashMap::new();
        let mut l = 0;
        for c in pattern.chars() {
            if l >= s.len() {
                return false;
            }
            let mut r = l;
            while r < s.len() && s.as_bytes()[r] != b' ' {
                r += 1;
            }
            let word = &s[l..r];
            match (words_to_chars.get(word), chars_to_words.get(&c)) {
                (None, None) => {
                    chars_to_words.insert(c, word);
                    words_to_chars.insert(word, c);
                }
                (Some(&sc), Some(&sw)) if sc == c && sw == word => (),
                _ => return false,
            }
            l = r + 1;
        }
        l == s.len() + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abba_dog_cat_cat_dog() {
        assert!(Solution::word_pattern(
            "abba".into(),
            "dog cat cat dog".into()
        ));
    }
    #[test]
    fn abba_dog_cat_cat_fish() {
        assert!(!Solution::word_pattern(
            "abba".into(),
            "dog cat cat fish".into()
        ));
    }
    #[test]
    fn aaaa_dog_cat_cat_dog() {
        assert!(!Solution::word_pattern(
            "aaaa".into(),
            "dog cat cat dog".into()
        ));
    }

    #[test]
    fn abbac_dog_cat_cat_dog_dog() {
        assert!(!Solution::word_pattern(
            "abbac".into(),
            "dog cat cat dog dog".into()
        ));
    }

    #[test]
    fn abbac_dog_cat_cat_dog_rat() {
        assert!(Solution::word_pattern(
            "abbac".into(),
            "dog cat cat dog rat".into()
        ));
    }
    #[test]
    fn abbac_dog_cat_cat_dog_rat_dog() {
        assert!(!Solution::word_pattern(
            "abbac".into(),
            "dog cat cat dog rat dog".into()
        ));
    }
    #[test]
    fn abbac_dog_cat_cat_dog() {
        assert!(!Solution::word_pattern(
            "abbac".into(),
            "dog cat cat dog".into()
        ));
    }
}
