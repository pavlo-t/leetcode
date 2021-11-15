#![allow(dead_code)]
/// 647. Palindromic Substrings
/// ===========================
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
    /// 22:04-22:15
    pub fn count_substrings_rec(s: String) -> i32 {
        println!("count_substrings({})", s);
        fn is_palindrome(s: &[u8]) -> bool {
            (0..s.len() / 2).all(|l| s[l] == s[s.len() - l - 1])
        }
        fn rec(l: usize, i: usize, s: &[u8]) -> i32 {
            if l == 0 {
                0
            } else if i + l > s.len() {
                rec(l - 1, 0, s)
            } else {
                is_palindrome(&s[i..i + l]) as i32 + rec(l, i + 1, s)
            }
        }
        rec(s.len(), 0, s.as_bytes())
    }
    /// 22:15-22:43 - this one is slower that straight recursion
    pub fn count_substrings_rec_with_memo(s: String) -> i32 {
        println!("count_substrings({})", s);
        fn is_palindrome(l: usize, i: usize, s: &[u8], memo: &mut Vec<Vec<Option<bool>>>) -> bool {
            memo[l][i].unwrap_or({
                let res = l < 2 || (s[i] == s[i + l - 1] && is_palindrome(l - 2, i + 1, s, memo));
                memo[l][i] = Some(res);
                res
            })
        }
        fn rec(l: usize, i: usize, s: &[u8], memo: &mut Vec<Vec<Option<bool>>>) -> i32 {
            if l == 0 {
                0
            } else if i + l > s.len() {
                rec(l - 1, 0, s, memo)
            } else {
                is_palindrome(l, i, s, memo) as i32 + rec(l, i + 1, s, memo)
            }
        }
        let bs = s.as_bytes();
        let n = s.len();
        let mut memo = vec![vec![None; n + 1]; n + 1];
        rec(n, 0, bs, &mut memo)
    }
    /// 22:43-23:00
    pub fn count_substrings(s: String) -> i32 {
        println!("count_substrings({})", s);
        let bs = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![true; n]; n + 1];
        let mut result = n as i32;
        for len in 2..=n {
            for l in 0..=n - len {
                if bs[l] == bs[l + len - 1] && dp[len - 2][l + 1] {
                    result += 1;
                } else {
                    dp[len][l] = false;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn a()    { assert_eq!(Solution::count_substrings("a".into()   ),  1); }
    #[rustfmt::skip] #[test] fn aa()   { assert_eq!(Solution::count_substrings("aa".into()  ),  3); }
    #[rustfmt::skip] #[test] fn aaa()  { assert_eq!(Solution::count_substrings("aaa".into() ),  6); }
    #[rustfmt::skip] #[test] fn aaaa() { assert_eq!(Solution::count_substrings("aaaa".into()), 10); }
    #[test]
    fn abc() {
        assert_eq!(Solution::count_substrings("abc".into()), 3);
        // Explanation: Three palindromic strings: "a", "b", "c".
    }

    /// If getting stack overflow: (2 ** 27)
    /// RUST_MIN_STACK=134217728 cargo test --lib d11_1
    #[test]
    fn a_repeat_1000() {
        assert_eq!(Solution::count_substrings("a".repeat(1000)), 500500);
    }
}
