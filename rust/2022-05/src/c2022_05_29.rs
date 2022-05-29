#![allow(dead_code)]
/// \#318. Maximum Product of Word Lengths
/// ======================================
///
/// Given a string array `words`, return _the maximum value of `length(word[i]) * length(word[j])`
/// where the two words do not share common letters_.
/// If no such two words exist, return `0`.
///
/// __Constraints:__
///
/// - `2 <= words.length <= 1000`
/// - `1 <= words[i].length <= 1000`
/// - `words[i]` consists only of lowercase English letters.
///
/// https://leetcode.com/problems/maximum-product-of-word-lengths
struct Solution;
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        use std::collections::HashMap;

        let chars_lengths = {
            let mut result: HashMap<i32, i32> = HashMap::new();
            for w in words {
                let mut chars = 0;
                for b in w.bytes() {
                    chars |= 1 << ((b - b'a') as i32);
                }
                let len = w.len() as i32;
                if result.get(&chars).filter(|&&prev| prev >= len).is_none() {
                    result.insert(chars, len);
                }
            }
            result.into_iter().collect::<Vec<_>>()
        };

        let mut result = 0;
        for a in 0..chars_lengths.len() - 1 {
            for b in a + 1..chars_lengths.len() {
                let ((ac, al), (bc, bl)) = (chars_lengths[a], chars_lengths[b]);
                if ac & bc == 0 {
                    result = result.max(al * bl);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs { ($($s:expr),*) => { vec![$($s.to_string()),*] }; }

    #[test]
    fn abcw_baz_foo_bar_xtfn_abcdef() {
        let w = vs!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"];
        assert_eq!(Solution::max_product(w), 16);
        // Explanation: The two words can be "abcw", "xtfn".
    }
    #[test]
    fn a_ab_abc_d_cd_bcd_abcd() {
        let w = vs!["a", "ab", "abc", "d", "cd", "bcd", "abcd"];
        assert_eq!(Solution::max_product(w), 4);
        // Explanation: The two words can be "ab", "cd".
    }
    #[test]
    fn a_aa_aaa_aaaa() {
        let w = vs!["a", "aa", "aaa", "aaaa"];
        assert_eq!(Solution::max_product(w), 0);
        // Explanation: No such pair of words.
    }
}
