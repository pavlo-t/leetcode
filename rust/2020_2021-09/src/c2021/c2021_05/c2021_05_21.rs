#![allow(dead_code)]
/// Find and Replace Pattern
/// ========================
///
/// Given a list of strings `words` and a string `pattern`,
/// return _a list of_ `words[i]` _that match_ `pattern`.
/// You may return the answer in __any order__.
///
/// A word matches the pattern if there exists a permutation of letters `p`
/// so that after replacing every letter `x` in the pattern with `p(x)`, we get the desired word.
///
/// Recall that a permutation of letters is a bijection from letters to letters:
/// every letter maps to another letter, and no two letters map to the same letter.
///
/// __Constraints:__
///
/// - `1 <= pattern.length <= 20`
/// - `1 <= words.length <= 50`
/// - `words[i].length == pattern.length`
/// - `pattern` and `words[i]` are lowercase English letters.
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/600/week-3-may-15th-may-21st/3750/
struct Solution;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        fn matches(w: &[u8], p: &[u8]) -> bool {
            if w.len() != p.len() {
                false
            } else if w.len() < 2 {
                true
            } else {
                let mut p_to_w = vec![0; (b'z' - b'a' + 1) as usize];
                let mut w_to_p = vec![0; (b'z' - b'a' + 1) as usize];
                for i in 0..w.len() {
                    match p_to_w[(p[i] - b'a') as usize] {
                        0 => p_to_w[(p[i] - b'a') as usize] = w[i],
                        b if b != w[i] => return false,
                        _ => (),
                    }
                    match w_to_p[(w[i] - b'a') as usize] {
                        0 => w_to_p[(w[i] - b'a') as usize] = p[i],
                        b if b != p[i] => return false,
                        _ => (),
                    }
                }
                true
            }
        }
        let p = pattern.as_bytes();
        words.into_iter().filter(|w| matches(w.as_bytes(), p)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs { ($($s:tt), *) => { vec![$($s.to_string()), *] }; }

    #[test]
    fn example1() {
        let words = vs!["abc", "deq", "mee", "aqq", "dkd", "ccc"];
        let pattern = "abb".to_string();
        let e = vs!["mee", "aqq"];
        assert_eq!(Solution::find_and_replace_pattern(words, pattern), e);
        // Explanation: "mee" matches the pattern because there is a permutation {a -> m, b -> e, ...}.
        // "ccc" does not match the pattern because {a -> c, b -> c, ...} is not a permutation, since a and b map to the same letter.
    }
    #[test]
    fn example2() {
        let words = vs!["a", "b", "c"];
        let pattern = "a".to_string();
        let e = vs!["a", "b", "c"];
        assert_eq!(Solution::find_and_replace_pattern(words, pattern), e);
    }

    #[test]
    fn test10() {
        let words = vs!["badc", "abab", "dddd", "dede", "yyxx"];
        let pattern = String::from("baba");
        let e = vs!["abab", "dede"];
        assert_eq!(Solution::find_and_replace_pattern(words, pattern), e);
    }
    #[test]
    fn test11() {
        let words = vs!["bezhk", "ohmgb", "enbki", "kcxmv", "zimsl"];
        let pattern = "iusiq".to_string();
        let e: Vec<String> = vec![];
        assert_eq!(Solution::find_and_replace_pattern(words, pattern), e);
    }
    #[test]
    fn test36() {
        let words = vs!["abc", "cba", "xyx", "yxx", "yyx"];
        let pattern = "abc".to_string();
        let e = vs!["abc", "cba"];
        assert_eq!(Solution::find_and_replace_pattern(words, pattern), e);
    }
}
