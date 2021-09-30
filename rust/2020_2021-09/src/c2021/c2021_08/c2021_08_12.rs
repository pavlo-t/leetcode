#![allow(dead_code)]
/// Group Anagrams
/// ==============
///
/// Given an array of strings `strs`, group __the anagrams__ together.
/// You can return the answer in __any order__.
///
/// An __Anagram__ is a word or phrase formed by rearranging the letters of a different word or phrase,
/// typically using all the original letters exactly once.
///
/// __Constraints:__
///
/// - `1 <= strs.length <= 10_000`
/// - `0 <= strs[i].length <= 100`
/// - `strs[i]` consists of lower-case English letters.
///
/// https://leetcode.com/explore/featured/card/august-leetcoding-challenge-2021/614/week-2-august-8th-august-14th/3887/
struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        fn key(s: &str) -> String {
            let mut k = s.bytes().collect::<Vec<_>>();
            k.sort_unstable();
            String::from_utf8(k).unwrap()
        }

        let mut m: HashMap<String, Vec<String>> = HashMap::new();
        strs.into_iter().for_each(|s| m.entry(key(&s)).or_default().push(s));
        m.into_iter().map(|(_, v)| v).collect()
    }

    pub fn group_anagrams_my_works_with_rust_1_54(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        fn key(s: &str) -> String {
            let mut k = s.bytes().collect::<Vec<_>>();
            k.sort_unstable();
            String::from_utf8(k).unwrap()
        }

        let mut m: HashMap<String, Vec<String>> = HashMap::new();
        strs.into_iter().for_each(|s| m.entry(key(&s)).or_default().push(s));
        m.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($s:tt),*) => {vec![$($s.to_string()),*]};}
    macro_rules! vvs {($($x:tt),*) => {vec![$(vs!$x),*]};}

    fn check(s: Vec<String>, mut e: Vec<Vec<String>>) {
        let mut r = Solution::group_anagrams(s);
        fn sort(vv: &mut Vec<Vec<String>>) {
            for v in vv.iter_mut() {
                v.sort_unstable();
            }
            vv.sort_unstable();
        }
        sort(&mut r);
        sort(&mut e);
        assert_eq!(r, e);
    }

    #[test]
    fn eat_tea_tan_ate_nat_bat() {
        let strs = vs!["eat","tea","tan","ate","nat","bat"];
        let e = vvs![["bat"],["nat","tan"],["ate","eat","tea"]];
        check(strs, e);
    }
    #[test]
    fn empty() {
        let strs = vs![""];
        let e = vvs![[""]];
        check(strs, e);
    }
    #[test]
    fn a() {
        let strs = vs!["a"];
        let e = vvs![["a"]];
        check(strs, e);
    }

    mod performance {
        use super::*;

        #[test]
        fn ax100x10000() {
            let strs = vec!["a".repeat(100); 10000];
            let e = vec![vec!["a".repeat(100); 10000]];
            check(strs, e);
        }
    }
}
