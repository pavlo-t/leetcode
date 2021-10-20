#![allow(dead_code)]
/// 5. Longest Palindromic Substring
/// ================================
///
/// Given a string `s`, return _the longest palindromic substring_ in `s`.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 1000`
/// - `s` consist of only digits and English letters.
///
/// https://leetcode.com/problems/longest-palindromic-substring/
struct Solution;
impl Solution {
    /// 00:25-01:00
    pub fn longest_palindrome(s: String) -> String {
        println!("longest_palindrome({})", s);
        let bs = s.as_bytes();
        let n = bs.len();
        let mut prev = vec![(0, 0); n];
        let mut curr = vec![(0, 0); n];
        for i in 0..n - 1 {
            if bs[i] == bs[i + 1] {
                curr[i] = (1, i);
            }
        }
        for len in 2..n {
            std::mem::swap(&mut curr, &mut prev);
            for i in 0..n - len {
                curr[i] = prev[i].max(prev[i + 1]);
                if bs[i] == bs[i + len] {
                    let (max_len, _) = curr[i + 1];
                    if max_len == len - 2 {
                        curr[i] = curr[i].max((max_len + 2, i));
                    }
                }
            }
        }

        let (len, i) = curr[0];
        s[i..=i + len].to_string()
    }
    /// 00:25-00:32
    pub fn longest_palindrome_dp_vec_vec_2(s: String) -> String {
        println!("longest_palindrome({})", s);
        let bs = s.as_bytes();
        let n = bs.len();
        let mut dp = vec![vec![(0, 0); n]; n];
        for i in 0..n - 1 {
            if bs[i] == bs[i + 1] {
                dp[1][i] = (1, i);
            }
        }
        for len in 2..n {
            for l in 0..n - len {
                let r = l + len;
                dp[len][l] = dp[len - 1][l].max(dp[len - 1][l + 1]);
                if bs[l] == bs[r] {
                    let (max_len, _) = dp[len - 2][l + 1];
                    if max_len == len - 2 {
                        dp[len][l] = dp[len][l].max((max_len + 2, l));
                    }
                }
            }
        }

        let (len, i) = dp[n - 1][0];
        s[i..=i + len].to_string()
    }
    /// 00:13-00:25
    pub fn longest_palindrome_dp_vec_vec(s: String) -> String {
        println!("longest_palindrome({})", s);
        let bs = s.as_bytes();
        let n = bs.len();
        let mut dp = vec![vec![(0, 0); n]; n];
        for i in 0..n - 1 {
            if bs[i] == bs[i + 1] {
                dp[i][i + 1] = (1, i);
            }
        }
        for len in 2..n {
            for l in 0..n - len {
                let r = l + len;
                dp[l][r] = dp[l + 1][r].max(dp[l][r - 1]);
                if bs[l] == bs[r] {
                    let (len, _) = dp[l + 1][r - 1];
                    if len == r - l - 2 {
                        dp[l][r] = dp[l][r].max((len + 2, l));
                    }
                }
            }
        }

        let (len, i) = dp[0][n - 1];
        s[i..=i + len].to_string()
    }
    /// 00:09-00:13
    pub fn longest_palindrome_rec_with_memo(s: String) -> String {
        println!("longest_palindrome({})", s);
        fn rec(
            l: usize,
            r: usize,
            s: &[u8],
            memo: &mut Vec<Vec<(usize, usize)>>,
        ) -> (usize, usize) {
            if l == r {
                (0, l)
            } else if memo[l][r].0 > 0 {
                memo[l][r]
            } else if l + 1 == r && s[l] == s[r] {
                (1, l)
            } else {
                let res1 = rec(l + 1, r, s, memo);
                let res2 = rec(l, r - 1, s, memo);
                let res3 = if s[l] == s[r] {
                    let (len, i) = rec(l + 1, r - 1, s, memo);
                    if i == l + 1 && len == r - l - 2 {
                        (len + 2, l)
                    } else {
                        (len, i)
                    }
                } else {
                    (0, l)
                };
                memo[l][r] = res1.max(res2).max(res3);
                memo[l][r]
            }
        }
        let bs = s.as_bytes();
        let n = bs.len();
        let mut memo = vec![vec![(0, 0); n]; n];
        let (len, i) = rec(0, n - 1, bs, &mut memo);
        s[i..=i + len].to_string()
    }
    /// 00:04-00:09
    pub fn longest_palindrome_my_rec_2(s: String) -> String {
        println!("longest_palindrome({})", s);
        fn rec(l: usize, r: usize, s: &[u8]) -> (usize, usize) {
            if l == r {
                (0, l)
            } else if l + 1 == r && s[l] == s[r] {
                (1, l)
            } else {
                let res1 = rec(l + 1, r, s);
                let res2 = rec(l, r - 1, s);
                let res3 = if s[l] == s[r] {
                    let (len, i) = rec(l + 1, r - 1, s);
                    if i == l + 1 && len == r - l - 2 {
                        (len + 2, l)
                    } else {
                        (len, i)
                    }
                } else {
                    (0, l)
                };
                res1.max(res2).max(res3)
            }
        }
        let bs = s.as_bytes();
        let (len, i) = rec(0, bs.len() - 1, bs);
        s[i..=i + len].to_string()
    }
    /// 23:41-00:04
    pub fn longest_palindrome_my_rec_1(s: String) -> String {
        println!("longest_palindrome({})", s);
        fn rec(l: usize, r: usize, s: &[u8]) -> (usize, usize) {
            if l == r {
                (l, l)
            } else if l + 1 == r && s[l] == s[r] {
                (l, r)
            } else {
                let (l1, r1) = rec(l + 1, r, s);
                let (l2, r2) = rec(l, r - 1, s);
                let (l3, r3) = if s[l] == s[r] {
                    let (ll, rr) = rec(l + 1, r - 1, s);
                    if ll == l + 1 && rr == r - 1 {
                        (l, r)
                    } else {
                        (ll, rr)
                    }
                } else {
                    (l, l)
                };
                let (d1, d2, d3) = (r1 - l1, r2 - l2, r3 - l3);
                [(d1, (l1, r1)), (d2, (l2, r2)), (d3, (l3, r3))]
                    .iter()
                    .max()
                    .map(|&(_, lr)| lr.to_owned())
                    .unwrap()
            }
        }
        let bs = s.as_bytes();
        let (l, r) = rec(0, bs.len() - 1, bs);
        s[l..=r].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn babad() {
        let s = "babad".to_string();
        let result = Solution::longest_palindrome(s);
        assert!(
            result == "bab" || result == "aba",
            "expected: \"bab\" || \"aba\"; got: \"{}\"",
            result
        );
        //Note: "aba" is also a valid answer.
    }
    #[test]
    fn cbbd() {
        let s = "cbbd".to_string();
        let e = "bb";
        assert_eq!(Solution::longest_palindrome(s), e);
    }
    #[test]
    fn a() {
        assert_eq!(Solution::longest_palindrome("a".to_string()), "a");
    }
    #[test]
    fn ac() {
        let result = Solution::longest_palindrome("ac".to_string());
        assert!(
            result == "a" || result == "c",
            "expected: \"a\" || \"c\"; got: \"{}\"",
            result
        );
    }

    #[test]
    fn abacda() {
        let s = "abacda".to_string();
        let e = "aba";
        assert_eq!(Solution::longest_palindrome(s), e);
    }

    #[test]
    fn ax1000() {
        let s = "a".repeat(1000);
        let e = "a".repeat(1000);
        assert_eq!(Solution::longest_palindrome(s), e);
    }
}
