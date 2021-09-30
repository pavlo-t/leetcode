#![allow(dead_code)]
/// Longest Uncommon Subsequence II
/// ===============================
///
/// Given an array of strings `strs`, return _the length of the __longest uncommon subsequence__ between them_.
/// If the longest uncommon subsequence does not exist, return `-1`.
///
/// An __uncommon subsequence__ between an array of strings is a string
/// that is a __subsequence of one string but not the others__.
///
/// A __subsequence__ of a string `s` is a string that can be obtained after deleting any number of characters from `s`.
///
/// - For example, `"abc"` is a subsequence of `"aebdc"` because you can delete the `'e'` and `'d'` characters in
///   `"aebdc"` to get "abc".
///   Other subsequences of `"aebdc"` include `"aebdc"`, `"aeb"`, and `""` (empty string).
///
/// __Constraints:__
///
/// - `1 <= strs.length <= 50`
/// - `1 <= strs[i].length <= 10`
/// - `strs[i]` consists of lowercase English letters.
///
///
/// https://leetcode.com/explore/challenge/card/august-leetcoding-challenge-2021/616/week-4-august-22nd-august-28th/3921/
struct Solution;
impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        use std::collections::{HashMap, HashSet};

        fn is_subsequence(a: &str, b: &str) -> bool {
            let (a, b) = (a.as_bytes(), b.as_bytes());
            if a == b || a.len() >= b.len() {
                false
            } else {
                let (mut ai, mut bi) = (0, 0);
                while ai < a.len() && bi < b.len() {
                    if a[ai] == b[bi] {
                        ai += 1;
                    }
                    bi += 1;
                }
                ai == a.len()
            }
        }

        let ss = strs.iter().map(|s| s.clone()).collect::<HashSet<_>>();
        let mut counts = HashMap::new();
        for s in strs {
            *counts.entry(s).or_insert(0) += 1;
        }
        let mut strs = counts
            .iter()
            .filter(|(_, &c)| c == 1)
            .filter(|(a, _)| ss.iter().all(|b| !is_subsequence(a, b)))
            .map(|(s, _)| s.len() as i32)
            .collect::<Vec<_>>();
        strs.sort_unstable_by(|a, b| b.cmp(&a));
        strs.first().map(|&i| i).unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($s:tt),*) => {vec![$($s.to_string()),*]};}

    #[test]
    fn aba_cdc_eae_3() {
        let strs = vs!["aba", "cdc", "eae"];
        assert_eq!(Solution::find_lu_slength(strs), 3);
    }
    #[test]
    fn aaa_aaa_aa_m1() {
        let strs = vs!["aaa", "aaa", "aa"];
        assert_eq!(Solution::find_lu_slength(strs), -1);
    }
    #[test]
    fn aaa_aaa_aaaa_4() {
        let strs = vs!["aaa", "aaa", "aaaa"];
        assert_eq!(Solution::find_lu_slength(strs), 4);
    }
    #[test]
    fn aaaa_4() {
        let strs = vs!["aaaa"];
        assert_eq!(Solution::find_lu_slength(strs), 4);
    }
    #[test]
    fn aaaa_bbb_cc_dddd_4() {
        let strs = vs!["aaaa", "bbb", "cc", "dddd"];
        assert_eq!(Solution::find_lu_slength(strs), 4);
    }
    #[test]
    fn abcde_abcde_ace_m1() {
        let strs = vs!["abcde", "abcde", "ace"];
        assert_eq!(Solution::find_lu_slength(strs), -1);
    }
}
