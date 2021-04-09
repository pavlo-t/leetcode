#![allow(dead_code)]
/// Verifying an Alien Dictionary
/// =============================
///
/// In an alien language, surprisingly they also use english lowercase letters,
/// but possibly in a different `order`.
/// The `order` of the alphabet is some permutation of lowercase letters.
///
/// Given a sequence of `words` written in the alien language, and the `order` of the alphabet,
/// return `true` if and only if the given `words` are sorted lexicographicaly in this alien language.
///
/// __Constraints:__
///
/// - `1 <= words.length <= 100`
/// - `1 <= words[i].length <= 20`
/// - `order.length == 26`
/// - All characters in `words[i]` and `order` are English lowercase letters.
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/594/week-2-april-8th-april-14th/3702/
struct Solution;
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        use std::collections::HashMap;
        let weights = order
            .chars()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, c)| {
                acc.insert(c, i);
                acc
            });

        let lte = |a: &str, b: &str| -> bool {
            for (a, b) in a.chars().zip(b.chars()) {
                if a != b {
                    return weights[&a] < weights[&b];
                }
            }
            a.len() <= b.len()
        };
        for ws in words.windows(2) {
            if !lte(&ws[0], &ws[1]) {
                return false;
            }
        }
        true
    }

    pub fn is_alien_sorted_brute_force(words: Vec<String>, order: String) -> bool {
        let lte = |a: &str, b: &str| -> bool {
            let mut a = a.chars();
            let mut b = b.chars();
            while let Some(ac) = a.next() {
                if let Some(bc) = b.next() {
                    if ac == bc {
                        continue;
                    }
                    for c in order.chars() {
                        if c == ac {
                            return true;
                        } else if c == bc {
                            return false;
                        }
                    }
                } else {
                    return false;
                }
            }
            true
        };
        for ws in words.windows(2) {
            if !lte(&ws[0], &ws[1]) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let words = vec!["hello".to_string(), "leetcode".to_string()];
        let order = "hlabcdefgijkmnopqrstuvwxyz".to_string();
        assert!(Solution::is_alien_sorted(words, order));
        // Explanation: As 'h' comes before 'l' in this language, then the sequence is sorted.
    }
    #[test]
    fn example2() {
        let words = vec!["word".to_string(), "world".to_string(), "row".to_string()];
        let order = "worldabcefghijkmnpqstuvxyz".to_string();
        assert!(!Solution::is_alien_sorted(words, order));
        // Explanation: As 'd' comes after 'l' in this language, then words[0] > words[1],
        // hence the sequence is unsorted.
    }
    #[test]
    fn example3() {
        let words = vec!["apple".to_string(), "app".to_string()];
        let order = "abcdefghijklmnopqrstuvwxyz".to_string();
        assert!(!Solution::is_alien_sorted(words, order));
        // Explanation:
        // The first three characters "app" match, and the second string is shorter (in size.)
        // According to lexicographical rules "apple" > "app", because 'l' > '∅',
        // where '∅' is defined as the blank character which is less than any other character
        // (More info: https://en.wikipedia.org/wiki/Lexicographic_order).
    }

    #[test]
    fn w_a_a_produces_true() {
        let words = vec!["a".to_string(), "a".to_string()];
        let order = "abcdefghijklmnopqrstuvwxyz".to_string();
        assert!(Solution::is_alien_sorted(words, order));
    }

    mod performance {
        use super::*;

        #[test]
        fn worst_case_produces_true() {
            let words = vec!["z".repeat(20); 100];
            let order = "abcdefghijklmnopqrstuvwxyz".to_string();
            assert!(Solution::is_alien_sorted(words, order));
        }
    }
}
