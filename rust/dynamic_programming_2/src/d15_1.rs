#![allow(dead_code)]
/// 139. Word Break
/// ===============
///
/// Given a string `s` and a dictionary of strings `wordDict`,
/// return `true` if `s` can be segmented into a space-separated sequence of one or more dictionary words.
///
/// __Note__ that the same word in the dictionary may be reused multiple times in the segmentation.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 300`
/// - `1 <= wordDict.length <= 1000`
/// - `1 <= wordDict[i].length <= 20`
/// - `s` and `wordDict[i]` consist of only lowercase English letters.
/// - All the strings of `wordDict` are __unique__.
///
/// https://leetcode.com/problems/word-break/
struct Solution;
impl Solution {
    /// 22:33-22:51
    pub fn word_break_rec(s: String, word_dict: Vec<String>) -> bool {
        println!("word_break({}, {:?})", s, word_dict);
        fn rec(i: usize, j: usize, k: usize, s: &[u8], w: &[&[u8]]) -> bool {
            if i == s.len() {
                k == w[j].len()
            } else if k == w[j].len() {
                (0..w.len()).any(|j| rec(i, j, 0, s, w))
            } else if s[i] == w[j][k] {
                rec(i + 1, j, k + 1, s, w)
            } else {
                false
            }
        }
        let s = s.as_bytes();
        let w = word_dict.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
        (0..w.len()).any(|j| rec(0, j, 0, s, &w))
    }
    /// 22:52-23:06
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        println!("word_break({}, {:?})", s, word_dict);
        #[rustfmt::skip]
        fn rec(i: usize, j: usize, k: usize, s: &[u8], w: &[&[u8]], memo: &mut Vec<Vec<Vec<bool>>>) -> bool {
            if memo[i][j][k] {
                false
            } else {
                memo[i][j][k] = true;
                if i == s.len() {
                    k == w[j].len()
                } else if k == w[j].len() {
                    (0..w.len()).any(|j| rec(i, j, 0, s, w, memo))
                } else if s[i] == w[j][k] {
                    rec(i + 1, j, k + 1, s, w, memo)
                } else {
                    false
                }
            }
        }
        let s = s.as_bytes();
        let w = word_dict.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
        let max_word_len = w.iter().map(|s| s.len()).max().unwrap();
        let mut memo = vec![vec![vec![false; max_word_len + 1]; w.len()]; s.len() + 1];
        (0..w.len()).any(|j| rec(0, j, 0, s, &w, &mut memo))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($x:expr),*) => {vec![$($x.to_string()),*]};}

    #[test]
    fn leetcode_leet_code() {
        let s = "leetcode".into();
        let w = vs!["leet", "code"];
        assert!(Solution::word_break(s, w));
        // Explanation: Return true because "leetcode" can be segmented as "leet code".
    }
    #[test]
    fn applepenapple_apple_pen() {
        let s = "applepenapple".into();
        let w = vs!["apple", "pen"];
        assert!(Solution::word_break(s, w));
        // Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
        // Note that you are allowed to reuse a dictionary word.
    }
    #[test]
    fn catsandog_cats_dog_sand_and_cat() {
        let s = "catsandog".into();
        let w = vs!["cats", "dog", "sand", "and", "cat"];
        assert!(!Solution::word_break(s, w));
    }

    #[test]
    fn s_300a_ws_19a1b_repeat_999_chain_20a() {
        let s = "a".repeat(300);
        let mut w = vec!["aaaaaaaaaaaaaaaaaaab".to_string(); 999];
        w.push("aaaaaaaaaaaaaaaaaaaa".to_string());
        assert!(Solution::word_break(s, w));
    }
    //#[ignore]
    #[test]
    fn s_299a1b_ws_20a_repeat_1000() {
        let mut s = "a".repeat(299);
        s.push('b');
        let w = vec!["a".repeat(20); 1000];
        assert!(!Solution::word_break(s, w));
    }
}
