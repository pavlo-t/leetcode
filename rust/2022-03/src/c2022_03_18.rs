#![allow(dead_code)]
/// 316. Remove Duplicate Letters
/// =============================
///
/// Given a string `s`, remove duplicate letters so that every letter appears once and only once.
/// You must make sure your result is __the smallest in lexicographical order__ among all possible results.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 10_000`
/// - `s` consists of lowercase English letters.
///
/// __Note:__ This question is the same as 1081:
/// <https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/>
///
/// https://leetcode.com/problems/remove-duplicate-letters/
struct Solution;
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut indexes = vec![vec![]; 26];
        for (i, b) in s.bytes().enumerate() {
            indexes[(b - b'a') as usize].push(i);
        }

        let mut added = [false; 26];
        for i in 0..26 {
            if indexes[i].len() == 0 {
                added[i] = true;
            }
        }

        let mut result = String::new();
        let mut idx = 0;
        while !added.iter().all(|&a| a) {
            for c in (0..26).filter(|&c| !added[c]) {
                let i = indexes[c].iter().find(|&&i| i >= idx).unwrap();
                if (0..26)
                    .filter(|&n| n != c && !added[n])
                    .all(|o| indexes[o].iter().any(|j| j > i))
                {
                    result.push((c as u8 + b'a') as char);
                    idx = i + 1;
                    added[c] = true;
                    break;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn a() { assert_eq!(Solution::remove_duplicate_letters("a".into()), "a"); }
    #[rustfmt::skip] #[test] fn aaa() { assert_eq!(Solution::remove_duplicate_letters("aaa".into()), "a"); }
    #[rustfmt::skip] #[test] fn babaa() { assert_eq!(Solution::remove_duplicate_letters("babaa".into()), "ab"); }

    #[rustfmt::skip] #[test] fn bcabc() { assert_eq!(Solution::remove_duplicate_letters("bcabc".into()), "abc"); }
    #[rustfmt::skip] #[test] fn cbacdcbc() { assert_eq!(Solution::remove_duplicate_letters("cbacdcbc".into()), "acdb"); }
}
