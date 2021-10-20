#![allow(dead_code)]
/// 516. Longest Palindromic Subsequence
/// ====================================
///
/// Given a string `s`, find _the longest palindromic __subsequence__'s length in `s`_.
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
    /// 02:12-02:16
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        println!("longest_palindrome_subseq({})", s);
        let bs = s.as_bytes();
        let n = bs.len();
        let mut prev = vec![1; n];
        let mut curr = vec![1; n];
        for i in 0..n - 1 {
            if bs[i] == bs[i + 1] {
                curr[i] = 2;
            }
        }
        for len in 2..n {
            std::mem::swap(&mut curr, &mut prev);
            for i in 0..n - len {
                curr[i] = if bs[i] == bs[i + len] {
                    2 + curr[i + 1]
                } else {
                    prev[i].max(prev[i + 1])
                };
            }
        }
        curr[0]
    }
    /// 02:05-02:12
    pub fn longest_palindrome_subseq_dp_vec_vec(s: String) -> i32 {
        println!("longest_palindrome_subseq({})", s);
        let bs = s.as_bytes();
        let n = bs.len();
        let mut dp = vec![vec![1; n]; n];
        for l in 0..n - 1 {
            if bs[l] == bs[l + 1] {
                dp[l][l + 1] = 2;
            }
        }
        for len in 2..n {
            for l in 0..n - len {
                let r = l + len;
                dp[l][r] = if bs[l] == bs[r] {
                    2 + dp[l + 1][r - 1]
                } else {
                    dp[l + 1][r].max(dp[l][r - 1])
                };
            }
        }
        dp[0][n - 1]
    }
    /// 02:02-02:05
    pub fn longest_palindrome_subseq_rec_with_memo(s: String) -> i32 {
        println!("longest_palindrome_subseq({})", s);
        fn rec(l: usize, r: usize, s: &[u8], memo: &mut Vec<Vec<i32>>) -> i32 {
            if l >= r {
                1
            } else if l + 1 == r {
                1 + (s[l] == s[r]) as i32
            } else if memo[l][r] > 0 {
                memo[l][r]
            } else {
                memo[l][r] = if s[l] == s[r] {
                    2 + rec(l + 1, r - 1, s, memo)
                } else {
                    rec(l + 1, r, s, memo).max(rec(l, r - 1, s, memo))
                };
                memo[l][r]
            }
        }
        let bs = s.as_bytes();
        let n = bs.len();
        let mut memo = vec![vec![-1; n]; n];
        rec(0, n - 1, bs, &mut memo)
    }
    /// 01:50-02:02
    pub fn longest_palindrome_subseq_rec(s: String) -> i32 {
        println!("longest_palindrome_subseq({})", s);
        fn rec(l: usize, r: usize, s: &[u8]) -> i32 {
            if l == r {
                1
            } else if l + 1 == r {
                1 + (s[l] == s[r]) as i32
            } else if s[l] == s[r] {
                2 + rec(l + 1, r - 1, s)
            } else {
                rec(l + 1, r, s).max(rec(l, r - 1, s))
            }
        }
        let bs = s.as_bytes();
        rec(0, bs.len() - 1, bs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bbbab() {
        let s = "bbbab".to_string();
        assert_eq!(Solution::longest_palindrome_subseq(s), 4);
        // Explanation: One possible longest palindromic subsequence is "bbbb".
    }
    #[test]
    fn cbbd() {
        let s = "cbbd".to_string();
        assert_eq!(Solution::longest_palindrome_subseq(s), 2);
        // Explanation: One possible longest palindromic subsequence is "bb".
    }
    #[rustfmt::skip] #[test] fn a() { assert_eq!(Solution::longest_palindrome_subseq("a".to_string()), 1); }
    #[rustfmt::skip] #[test] fn aa() { assert_eq!(Solution::longest_palindrome_subseq("aa".to_string()), 2); }
    #[rustfmt::skip] #[test] fn aaa() { assert_eq!(Solution::longest_palindrome_subseq("aaa".to_string()), 3); }
    #[rustfmt::skip] #[test] fn baa() { assert_eq!(Solution::longest_palindrome_subseq("baa".to_string()), 2); }
    #[rustfmt::skip] #[test] fn aba() { assert_eq!(Solution::longest_palindrome_subseq("aba".to_string()), 3); }
    #[rustfmt::skip] #[test] fn aab() { assert_eq!(Solution::longest_palindrome_subseq("aab".to_string()), 2); }

    #[test]
    fn ax1000() {
        let s = "a".repeat(1000);
        assert_eq!(Solution::longest_palindrome_subseq(s), 1000);
    }
    #[test]
    fn abcdefghijx100() {
        let s = "abcdefghij".repeat(100);
        assert_eq!(Solution::longest_palindrome_subseq(s), 199);
    }
}
