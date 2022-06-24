#![allow(dead_code)]
//! \#1332. Remove Palindromic Subsequences
//! =======================================
//!
//! You are given a string `s` consisting __only__ of letters `'a'` and `'b'`.
//! In a single step you can remove one __palindromic subsequence__ from `s`.
//!
//! Return _the __minimum__ number of steps to make the given string empty_.
//!
//! A string is a __subsequence__ of a given string if it is generated by deleting some characters
//! of a given string without changing its order.
//! Note that a subsequence does __not__ necessarily need to be contiguous.
//!
//! A string is called __palindrome__ if is one that reads the same backward as well as forward.
//!
//! __Constraints:__
//!
//! - `1 <= s.length <= 1000`
//! - `s[i]` is either `'a'` or `'b'`.
//!
//! <https://leetcode.com/problems/remove-palindromic-subsequences>

pub struct Solution;
impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        let bs = s.as_bytes();
        let n = bs.len();
        for l in 0..(n / 2) {
            let r = n - l - 1;
            if bs[l] != bs[r] {
                return 2;
            }
        }
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ababa() {
        assert_eq!(Solution::remove_palindrome_sub("ababa".into()), 1);
        // Explanation: s is already a palindrome, so its entirety can be removed in a single step.
    }
    #[test]
    fn abb() {
        assert_eq!(Solution::remove_palindrome_sub("abb".into()), 2);
        // Explanation: "abb" -> "bb" -> "".
        // Remove palindromic subsequence "a" then "bb".
    }
    #[test]
    fn baabb() {
        assert_eq!(Solution::remove_palindrome_sub("baabb".into()), 2);
        // Explanation: "baabb" -> "b" -> "".
        // Remove palindromic subsequence "baab" then "b".
    }
}
