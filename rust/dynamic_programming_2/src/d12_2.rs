#![allow(dead_code)]
/// 516. Longest Palindromic Subsequence
/// ====================================
///
/// Given a string `s`, find _the longest palindromic __subsequence's__ length in `s`_.
///
/// A __subsequence__ is a sequence that can be derived from another sequence
/// by deleting some or no elements without changing the order of the remaining elements.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 1000`
/// - `s` consists only of lowercase English letters.
///
/// https://leetcode.com/problems/longest-palindromic-subsequence/
struct Solution;
impl Solution {
    pub fn longest_palindrome_subseq_rec(s: String) -> i32 {
        println!("longest_palindrome_subseq({})", s);
        fn expand_around_center(l: usize, r: usize, s: &[u8]) -> i32 {
            if l > s.len() || r == s.len() {
                0
            } else if s[l] == s[r] {
                expand_around_center(l.wrapping_sub(1), r + 1, s) + if l == r { 1 } else { 2 }
            } else {
                expand_around_center(l.wrapping_sub(1), r, s).max(expand_around_center(l, r + 1, s))
            }
        }
        let mut max = 1;
        let bs = s.as_bytes();
        for i in 1..bs.len() {
            max = max
                .max(expand_around_center(i, i, bs))
                .max(expand_around_center(i - 1, i, bs));
        }
        max
    }
    pub fn longest_palindrome_subseq_rec_with_memo(s: String) -> i32 {
        println!("longest_palindrome_subseq({})", s);
        fn expand_around_center(l: usize, r: usize, s: &[u8], memo: &mut Vec<Vec<i32>>) -> i32 {
            if l > s.len() || r == s.len() {
                0
            } else if memo[l][r] != -1 {
                memo[l][r]
            } else {
                memo[l][r] = if s[l] == s[r] {
                    (l != r) as i32 + 1 + expand_around_center(l.wrapping_sub(1), r + 1, s, memo)
                } else {
                    let skip_l = expand_around_center(l.wrapping_sub(1), r, s, memo);
                    let skip_r = expand_around_center(l, r + 1, s, memo);
                    skip_l.max(skip_r)
                };
                memo[l][r]
            }
        }

        let bs = s.as_bytes();
        let n = bs.len();
        let mut memo = vec![vec![-1; n]; n];
        let mut max = 1;
        for i in 1..n {
            max = max
                .max(expand_around_center(i, i, bs, &mut memo))
                .max(expand_around_center(i - 1, i, bs, &mut memo));
        }
        //println!(" memo: {:?}", memo);
        max
    }
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        println!("longest_palindrome_subseq({})", s);
        let bs = s.as_bytes();
        let n = bs.len();
        let mut dp = vec![vec![0; n + 2]; n + 2];
        let mut max = 1;
        for len in (2..=n).rev() {
            for l in 1..=n - len + 1 {
                let r = l + len - 1;
                dp[l][r] = if bs[l - 1] == bs[r - 1] {
                    2 + dp[l - 1][r + 1]
                } else {
                    dp[l - 1][r].max(dp[l][r + 1])
                };
                max = max.max(dp[l][r]);
            }
        }
        for i in 1..=n {
            dp[i][i] = 1 + dp[i - 1][i + 1];
            max = max.max(dp[i][i]);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(s: &str, e: i32) {
        assert_eq!(Solution::longest_palindrome_subseq(s.into()), e);
    }

    #[rustfmt::skip] #[test] fn a() { check("a", 1); }
    #[rustfmt::skip] #[test] fn ab() { check("ab", 1); }
    #[rustfmt::skip] #[test] fn aa() { check("aa", 2); }
    #[rustfmt::skip] #[test] fn aaa() { check("aaa", 3); }
    #[rustfmt::skip] #[test] fn aaaa() { check("aaaa", 4); }
    #[test]
    fn bbbab() {
        check("bbbab", 4);
        // Explanation: One possible longest palindromic subsequence is "bbbb".
    }
    #[test]
    fn cbbd() {
        check("cbbd", 2);
        // Explanation: One possible longest palindromic subsequence is "bb".
    }

    #[rustfmt::skip] #[test] fn a_repeat_1000() { check(&"a".repeat(1000), 1000); }
    #[rustfmt::skip] #[test] fn a_to_y_repeat_400() { check(&"abcdefghijklmnopqrstuvwxy".repeat(40), 79); }
}
