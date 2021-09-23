#![allow(dead_code)]
/// Break a Palindrome
/// ==================
///
/// Given a palindromic string of lowercase English letters `palindrome`,
/// replace __exactly one__ character with any lowercase English letter so that the resulting string
/// is __not__ a palindrome and that it is the __lexicographically__ smallest one possible.
///
/// Return _the resulting string.
/// If there is no way to replace a character to make it not a palindrome,
/// return an __empty string___.
///
/// A string `a` is lexicographically smaller than a string `b` (of the same length)
/// if in the first position where `a` and `b` differ,
/// `a` has a character strictly smaller than the corresponding character in `b`.
/// For example, `"abcc"` is lexicographically smaller than `"abcd"`
/// because the first position they differ is at the fourth character, and `'c'` is smaller than `'d'`.
///
/// __Constraints:__
///
/// - `1 <= palindrome.length <= 1000`
/// - `palindrome` consists of only lowercase English letters.
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/639/week-4-september-22nd-september-28th/3985/
struct Solution;
impl Solution {
    pub fn break_palindrome(mut p: String) -> String {
        println!("break_palindrome({})", p);
        let n = p.len();
        if n == 1 {
            "".to_string()
        } else {
            unsafe {
                let bs = p.as_mut_vec();
                for i in 0..n / 2 {
                    if bs[i] != b'a' {
                        bs[i] = b'a';
                        return p;
                    }
                }
                bs[n - 1] = b'b';
            }
            p
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abccba() {
        let p = "abccba".to_string();
        let e = "aaccba";
        assert_eq!(Solution::break_palindrome(p), e);
        // Explanation: There are many ways to make "abccba" not a palindrome, such as "zbccba", "aaccba", and "abacba".
        // Of all the ways, "aaccba" is the lexicographically smallest.
    }
    #[test]
    fn a() {
        let p = "a".to_string();
        let e = "";
        assert_eq!(Solution::break_palindrome(p), e);
        // Explanation: There is no way to replace a single character to make "a" not a palindrome,
        // so return an empty string.
    }
    #[test]
    fn aa() {
        let p = "aa".to_string();
        let e = "ab";
        assert_eq!(Solution::break_palindrome(p), e);
    }
    #[test]
    fn aba() {
        let p = "aba".to_string();
        let e = "abb";
        assert_eq!(Solution::break_palindrome(p), e);
    }
}
