#![allow(dead_code)]
/// \#1048. Longest String Chain
/// ============================
///
/// You are given an array of `words` where each word consists of lowercase English letters.
///
/// `wordA` is a __predecessor__ of `wordB` if and only if we can insert __exactly one__ letter
/// anywhere in `wordA` __without changing the order of the other characters__ to make it equal to `wordB`.
///
/// - For example, `"abc"` is a __predecessor__ of `"abac"`, while `"cba"` is not a __predecessor__ of `"bcad"`.
///
/// A __word chain__ is a sequence of words `[word1, word2, ..., wordk]` with `k >= 1`,
/// where `word1` is a __predecessor__ of `word2`, `word2` is a __predecessor__ of `word3`, and so on.
/// A single word is trivially a __word chain__ with `k == 1`.
///
/// Return _the __length__ of the __longest possible word__ chain with words chosen from the given list of words_.
///
/// __Constraints:__
///
/// - `1 <= words.length <= 1000`
/// - `1 <= words[i].length <= 16`
/// - `words[i]` only consists of lowercase English letters.
///
/// https://leetcode.com/problems/longest-string-chain
struct Solution;
impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        fn is_predecessor(w1: &[u8], w2: &[u8]) -> bool {
            if w1.len() + 1 != w2.len() {
                return false;
            }
            let (mut i1, mut i2) = (0, 0);
            let mut seen_diff = false;
            while i1 < w1.len() && i2 < w2.len() {
                if w1[i1] == w2[i2] {
                    i1 += 1;
                    i2 += 1;
                } else if seen_diff {
                    return false;
                } else {
                    i2 += 1;
                    seen_diff = true;
                }
            }
            true
        }

        fn rec(i: usize, ws: &[String], memo: &mut Vec<i32>) -> i32 {
            if ws[i].len() == 1 {
                1
            } else {
                if memo[i] == -1 {
                    memo[i] = 1
                        + (0..ws.len())
                            .filter(|&j| is_predecessor(ws[j].as_bytes(), ws[i].as_bytes()))
                            .map(|j| rec(j, ws, memo))
                            .max()
                            .unwrap_or(0);
                }
                memo[i]
            }
        }
        let n = words.len();
        let mut memo = vec![-1; n];
        (0..n).map(|i| rec(i, &words, &mut memo)).max().unwrap()
    }

    pub fn longest_str_chain_rec(words: Vec<String>) -> i32 {
        fn is_predecessor(w1: &[u8], w2: &[u8]) -> bool {
            if w1.len() + 1 != w2.len() {
                return false;
            }
            let (mut i1, mut i2) = (0, 0);
            let mut seen_diff = false;
            while i1 < w1.len() && i2 < w2.len() {
                if w1[i1] == w2[i2] {
                    i1 += 1;
                    i2 += 1;
                } else if seen_diff {
                    return false;
                } else {
                    i2 += 1;
                    seen_diff = true;
                }
            }
            true
        }

        fn rec(i: usize, ws: &[String]) -> i32 {
            if ws[i].len() == 1 {
                1
            } else {
                1 + (0..ws.len())
                    .filter(|&j| is_predecessor(ws[j].as_bytes(), ws[i].as_bytes()))
                    .map(|j| rec(j, ws))
                    .max()
                    .unwrap_or(0)
            }
        }
        (0..words.len()).map(|i| rec(i, &words)).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($x:expr),*) => {vec![$($x.to_string()),*]}}

    #[test]
    fn a_b_ba_bca_bda_bdca() {
        let w = vs!["a", "b", "ba", "bca", "bda", "bdca"];
        assert_eq!(Solution::longest_str_chain(w), 4);
        // Explanation: One of the longest word chains is ["a","ba","bda","bdca"].
    }
    #[test]
    fn xbc_pcxbcf_xb_cxbc_pcxbc() {
        let w = vs!["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"];
        assert_eq!(Solution::longest_str_chain(w), 5);
        // Explanation: All the words can be put in a word chain ["xb", "xbc", "cxbc", "pcxbc", "pcxbcf"].
    }
    #[test]
    fn abcd_dbqca() {
        let w = vs!["abcd", "dbqca"];
        assert_eq!(Solution::longest_str_chain(w), 1);
        // Explanation: The trivial word chain ["abcd"] is one of the longest word chains.
        // ["abcd","dbqca"] is not a valid word chain because the ordering of the letters is changed.
    }
    #[test]
    fn a_x_16_x_1000() {
        let w = vec!["a".repeat(16); 1000];
        assert_eq!(Solution::longest_str_chain(w), 1);
    }
    #[test]
    fn a_x_1_to_a_x_16_x_1000() {
        let w = (0..1000).map(|n| "a".repeat(1 + (n % 16))).collect();
        assert_eq!(Solution::longest_str_chain(w), 16);
    }
}
