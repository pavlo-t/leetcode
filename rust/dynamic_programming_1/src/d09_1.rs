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
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        println!("word_break({}, {:?})", s, word_dict);
        use std::collections::HashSet;

        let n = s.len();
        let max_len = word_dict.iter().map(|w| w.len()).max().unwrap();
        let wd: HashSet<String> = word_dict.into_iter().collect();
        let mut dp: HashSet<usize> = HashSet::with_capacity(max_len);
        dp.insert(n);
        let mut l = n - 1;

        while l < usize::MAX {
            dp.remove(&(l + max_len + 1));
            if dp.is_empty() {
                return false;
            }
            let mut found = false;
            for &r in dp.iter() {
                if wd.contains(&s[l..r]) {
                    found = true;
                    break;
                }
            }
            if found {
                dp.insert(l);
            }
            l = l.wrapping_sub(1);
        }
        dp.contains(&0)
    }

    pub fn word_break_rec_with_memo(s: String, word_dict: Vec<String>) -> bool {
        println!("word_break({}, {:?})", s, word_dict);
        use std::collections::HashSet;

        fn rec(l: usize, r: usize, s: &str, wd: &HashSet<String>, seen: &mut Vec<bool>) -> bool {
            if l >= s.len() {
                true
            } else if r >= s.len() || seen[l] {
                false
            } else {
                let result = if wd.contains(&s[l..=r]) {
                    rec(r + 1, r + 1, s, wd, seen) || rec(l, r + 1, s, wd, seen)
                } else {
                    rec(l, r + 1, s, wd, seen)
                };
                seen[l] = true;
                result
            }
        }
        let wd = word_dict.into_iter().collect::<HashSet<_>>();
        rec(0, 0, &s, &wd, &mut vec![false; s.len()])
    }
    pub fn word_break_rec(s: String, word_dict: Vec<String>) -> bool {
        println!("word_break({}, {:?})", s, word_dict);
        use std::collections::HashSet;

        fn rec(l: usize, r: usize, s: &str, wd: &HashSet<String>) -> bool {
            if l >= s.len() {
                true
            } else if r >= s.len() {
                false
            } else if wd.contains(&s[l..=r]) {
                rec(r + 1, r + 1, s, wd) || rec(l, r + 1, s, wd)
            } else {
                rec(l, r + 1, s, wd)
            }
        }
        let wd = word_dict.into_iter().collect::<HashSet<_>>();
        rec(0, 0, &s, &wd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($s:tt),*) => {vec![$($s.to_string()),*]};}

    #[test]
    fn s_leetcode_wd_leet_code() {
        let s = "leetcode".to_string();
        let wd = vs!["leet", "code"];
        assert!(Solution::word_break(s, wd));
        // Explanation: Return true because "leetcode" can be segmented as "leet code".
    }
    #[test]
    fn s_applepenapple_wd_apple_pen() {
        let s = "applepenapple".to_string();
        let wd = vs!["apple", "pen"];
        assert!(Solution::word_break(s, wd));
        // Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
        // Note that you are allowed to reuse a dictionary word.
    }
    #[test]
    fn s_catsandog_wd_cats_dog_sand_and_cat() {
        let s = "catsandog".to_string();
        let wd = vs!["cats", "dog", "sand", "and", "cat"];
        assert!(!Solution::word_break(s, wd));
    }

    #[test]
    fn s_300xa_wd_a_aa_aaa_and_so_on_up_to_20xa() {
        let s = "a".repeat(300);
        let wd = (1..=20).map(|i| "a".repeat(i)).collect();
        assert!(Solution::word_break(s, wd));
    }
    #[test]
    fn s_299xa_wd_aa() {
        let s = "a".repeat(299);
        let wd = vs!["aa"];
        assert!(!Solution::word_break(s, wd));
    }
}
