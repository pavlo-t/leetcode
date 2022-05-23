#![allow(dead_code)]
/// \#647. Palindromic Substrings
/// =============================
///
/// Given a string `s`, return _the number of __palindromic substrings__ in it_.
///
/// A string is a __palindrome__ when it reads the same backward as forward.
///
/// A __substring__ is a contiguous sequence of characters within the string.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 1000`
/// - `s` consists of lowercase English letters.
///
/// https://leetcode.com/problems/palindromic-substrings/
struct Solution;
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut curr = vec![true; n];
        let mut prev = vec![true; n];
        let mut result = s.len() as i32;
        for len in 1..n {
            for l in 0..n - len {
                let r = l + len;
                curr[l] = curr[l + 1] && s[l] == s[r];
                result += curr[l] as i32;
            }
            std::mem::swap(&mut curr, &mut prev);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abc() {
        assert_eq!(Solution::count_substrings("abc".into()), 3);
        // Explanation: Three palindromic strings: "a", "b", "c".
    }
    #[test]
    fn aaa() {
        assert_eq!(Solution::count_substrings("aaa".into()), 6);
        // Explanation: Six palindromic strings: "a", "a", "a", "aa", "aa", "aaa".
    }
    #[test]
    fn aaaa() {
        assert_eq!(Solution::count_substrings("aaaa".into()), 10);
        // a,a,a,a,aa,aa,aa,aaa,aaa,aaaa
    }
    #[test]
    fn a_repeat_1000() {
        assert_eq!(Solution::count_substrings("a".repeat(1000)), 500500);
    }
}
