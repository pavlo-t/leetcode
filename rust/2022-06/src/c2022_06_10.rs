#![allow(dead_code)]
//! \#3. Longest Substring Without Repeating Characters
//! ===================================================
//!
//! Given a string `s`, find the length of the __longest substring__ without repeating characters.
//!
//! __Constraints:__
//!
//! - `0 <= s.length <= 50_000`
//! - `s` consists of English letters, digits, symbols and spaces.
//!
//! <https://leetcode.com/problems/longest-substring-without-repeating-characters>

pub struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashSet;

        let chars = s.chars().collect::<Vec<_>>();
        let mut seen: HashSet<char> = HashSet::new();
        let mut len = 0;
        let mut result = 0;
        let (mut l, mut r) = (0, 0);
        while r < chars.len() {
            if seen.contains(&chars[r]) {
                seen.remove(&chars[l]);
                len -= 1;
                l += 1;
            } else {
                seen.insert(chars[r]);
                len += 1;
                result = result.max(len);
                r += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abcabcbb() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".into()), 3);
        // Explanation: The answer is "abc", with the length of 3.
    }
    #[test]
    fn bbbbb() {
        assert_eq!(Solution::length_of_longest_substring("bbbbb".into()), 1);
        // Explanation: The answer is "b", with the length of 1.
    }
    #[test]
    fn pwwkew() {
        assert_eq!(Solution::length_of_longest_substring("pwwkew".into()), 3);
        // Explanation: The answer is "wke", with the length of 3.
        // Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
    }
}
