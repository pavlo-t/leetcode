#![allow(dead_code)]
/// Number of Matching Subsequences
/// ===============================
///
/// Given a string `s` and an array of strings `words`,
/// return _the number of `words[i]` that is a subsequence of `s`_.
///
/// A __subsequence__ of a string is a new string generated from the original string with some
/// characters (can be none) deleted without changing the relative order of the remaining characters.
///
/// - For example, `"ace"` is a subsequence of `"abcde"`.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 50_000`
/// - `1 <= words.length <= 5000`
/// - `1 <= words[i].length <= 50`
/// - `s` and `words[i]` consist of only lowercase English letters.
///
/// https://leetcode.com/explore/challenge/card/june-leetcoding-challenge-2021/606/week-4-june-22nd-june-28th/3788/
struct Solution;
impl Solution {
    /// Approach #2: Next Letter Pointers
    ///
    /// https://leetcode.com/problems/number-of-matching-subsequences/solution/
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut result = 0;

        let mut ws = vec![vec![]; 26];
        for w in &words {
            let bs = w.as_bytes();
            ws[(bs[0] - b'a') as usize].push(&bs[1..]);
        }

        for &b in s.as_bytes() {
            let i = (b - b'a') as usize;
            let iws = ws[i].clone();
            ws[i] = vec![];
            for bs in iws {
                if bs.is_empty() {
                    result += 1;
                } else {
                    ws[(bs[0] - b'a') as usize].push(&bs[1..]);
                }
            }
        }

        result
    }

    pub fn num_matching_subseq_count_words(s: String, words: Vec<String>) -> i32 {
        use std::collections::HashMap;

        fn is_subseq(s: &[u8], w: &[u8]) -> bool {
            if w.is_empty() {
                true
            } else {
                let mut si = 0;
                let mut wi = 0;
                while si < s.len() && wi < w.len() {
                    if w[wi] == s[si] {
                        wi += 1;
                    }
                    si += 1;
                }
                wi == w.len()
            }
        }

        let mut ws = HashMap::new();

        for w in words {
            *ws.entry(w).or_insert(0) += 1;
        }
        let s = s.as_bytes();

        ws.into_iter()
            .filter(|(w, _)| is_subseq(s, w.as_bytes()))
            .map(|(_, c)| c)
            .sum()
    }

    /// Status: Time Limit Exceeded
    pub fn num_matching_subseq_bitmask_of_chars(s: String, words: Vec<String>) -> i32 {
        fn as_mask(s: &[u8]) -> i32 {
            s.iter().map(|&b| b - b'a').fold(0, |rsf, b| rsf | (1 << b))
        }
        fn is_subseq(s: &[u8], w: &[u8]) -> bool {
            if w.is_empty() {
                true
            } else {
                let mut si = 0;
                let mut wi = 0;
                while si < s.len() && wi < w.len() {
                    if w[wi] == s[si] {
                        wi += 1;
                    }
                    si += 1;
                }
                wi == w.len()
            }
        }
        // println!("s: {}", s);
        let s = s.as_bytes();
        let sm = as_mask(s);
        // println!("m: {:#028b}", sm);
        // print!("     ");
        // for b in (b'a'..=b'z').rev() {
        //     print!("{}", b as char)
        // }
        // println!();
        words
            .iter()
            .map(|w| w.as_bytes())
            .map(|w| (w, as_mask(w)))
            .filter(|&(_, m)| (sm & m) == m)
            .filter(|&(w, _)| is_subseq(s, w))
            .count() as i32
    }

    /// Status: Time Limit Exceeded
    pub fn num_matching_subseq_brute_force(s: String, words: Vec<String>) -> i32 {
        fn is_subseq(s: &[u8], w: &[u8]) -> bool {
            if w.is_empty() {
                true
            } else {
                let mut si = 0;
                let mut wi = 0;
                while si < s.len() && wi < w.len() {
                    if w[wi] == s[si] {
                        wi += 1;
                    }
                    si += 1;
                }
                wi == w.len()
            }
        }
        let s = s.as_bytes();
        words
            .into_iter()
            .filter(|w| is_subseq(s, w.as_bytes()))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($s:tt),*) => {vec![$($s.to_string()),*]};}

    #[test]
    fn example1() {
        let s = "abcde".to_string();
        let words = vs!["a", "bb", "acd", "ace"];
        assert_eq!(Solution::num_matching_subseq(s, words), 3);
        // Explanation: There are three strings in words that are a subsequence of s: "a", "acd", "ace".
    }
    #[test]
    fn example2() {
        let s = "dsahjpjauf".to_string();
        let words = vs!["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"];
        assert_eq!(Solution::num_matching_subseq(s, words), 2);
    }

    mod performance {
        use super::*;

        #[test]
        fn s_50000a_ws_5000x50a_produces_5000() {
            let s = "a".repeat(50000);
            let words = vec!["a".repeat(50); 5000];
            assert_eq!(Solution::num_matching_subseq(s, words), 5000);
        }
        #[test]
        fn s_50000b_ws_5000x50a_produces_0() {
            let s = "b".repeat(50000);
            let words = vec!["a".repeat(50); 5000];
            assert_eq!(Solution::num_matching_subseq(s, words), 0);
        }
        #[test]
        fn s_49999a1b_ws_5000x49a1b_produces_5000() {
            let mut s = "a".repeat(49999);
            s.push('b');
            let mut w = "a".repeat(49);
            w.push('b');
            let words = vec![w; 5000];
            assert_eq!(Solution::num_matching_subseq(s, words), 5000);
        }
    }
}
