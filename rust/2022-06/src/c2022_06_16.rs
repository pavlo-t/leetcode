#![allow(dead_code)]
//! \#5. Longest Palindromic Substring
//! ==================================
//!
//! Given a string `s`, return _the longest palindromic substring in `s`_.
//!
//! __Constraints:__
//!
//! - `1 <= s.length <= 1000`
//! - `s` consist of only digits and English letters.
//!
//! <https://leetcode.com/problems/longest-palindromic-substring>

pub struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        fn expand(mut l: usize, mut r: usize, s: &[u8]) -> &[u8] {
            let n = s.len();
            let mut result = &s[l..l + 1];
            while l > 0 && r < n && s[l] == s[r] {
                result = &s[l..=r];
                l -= 1;
                r += 1;
            }
            if l == 0 && r < n && s[l] == s[r] {
                result = &s[0..=r];
            }
            result
        }
        fn longest_from_center(center: usize, s: &[u8]) -> &[u8] {
            let mut result = expand(center, center + 1, s);
            if center > 0 {
                let palindrome = expand(center - 1, center + 1, s);
                if palindrome.len() > result.len() {
                    result = palindrome;
                }
            }

            result
        }
        let bs = s.as_bytes();
        let mut result = &bs[0..1];
        for i in 0..bs.len() {
            let curr = longest_from_center(i, bs);
            if curr.len() > result.len() {
                result = curr;
            }
        }
        unsafe { String::from_utf8_unchecked(result.iter().map(|&b| b).collect()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn babad() {
        assert_eq!(Solution::longest_palindrome("babad".into()), "bab");
        // Explanation: "aba" is also a valid answer.
    }
    #[test]
    fn cbbd() {
        assert_eq!(Solution::longest_palindrome("cbbd".into()), "bb");
    }
    #[rustfmt::skip] #[test] fn a() { assert_eq!(Solution::longest_palindrome("a".into()), "a"); }
    #[rustfmt::skip] #[test] fn aa() { assert_eq!(Solution::longest_palindrome("aa".into()), "aa"); }
    #[rustfmt::skip] #[test] fn aaa() { assert_eq!(Solution::longest_palindrome("aaa".into()), "aaa"); }
    #[rustfmt::skip] #[test] fn aaaa() { assert_eq!(Solution::longest_palindrome("aaaa".into()), "aaaa"); }
    #[rustfmt::skip] #[test] fn aaaaa() { assert_eq!(Solution::longest_palindrome("aaaaa".into()), "aaaaa"); }
    #[rustfmt::skip] #[test] fn a_x_1000() { assert_eq!(Solution::longest_palindrome("a".repeat(1000)), "a".repeat(1000)); }
}
