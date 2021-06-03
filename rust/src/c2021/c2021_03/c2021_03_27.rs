#![allow(dead_code)]

/// # Palindromic Substrings
///
/// Given a string, your task is to count how many palindromic substrings in this string.
///
/// The substrings with different start indexes or end indexes are counted as different substrings
/// even if they consist of same characters.
///
/// __Note:__
///
/// - The input string length won't exceed 1000.
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/591/week-4-march-22nd-march-28th/3686/
struct Solution;
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let bytes = s.as_bytes();

        let expand_palindrome = |mut l, mut r| {
            let mut result = 0;
            loop {
                if bytes[l] == bytes[r] {
                    result += 1;
                    if l == 0 || r == bytes.len() - 1 {
                        break;
                    } else {
                        l -= 1;
                        r += 1;
                    }
                } else {
                    break;
                }
            }
            result
        };

        let mut result = 0;
        for c in 0..bytes.len() {
            result += 1;
            if c > 0 {
                result += expand_palindrome(c - 1, c);
                if c < bytes.len() - 1 {
                    result += expand_palindrome(c - 1, c + 1);
                }
            }
        }
        result
    }

    pub fn count_substrings_dp(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut dp = vec![vec![false; bytes.len()]; bytes.len()];

        let mut is_palindrome = |l: usize, r: usize| {
            if l == r || (bytes[l] == bytes[r] && (r == l + 1 || dp[l + 1][r - 1])) {
                dp[l][r] = true;
                true
            } else {
                dp[l][r] = false;
                false
            }
        };

        let mut result = 0;
        for size in 0..bytes.len() {
            for l in 0..(bytes.len() - size) {
                let r = l + size;
                if is_palindrome(l, r) {
                    result += 1;
                }
            }
        }
        result
    }

    pub fn count_substrings_brute_force(s: String) -> i32 {
        fn is_palindrome(s: &[u8]) -> bool {
            let mut l = 0;
            let mut r = s.len() - 1;
            while l < r {
                if s[l] != s[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }
            true
        }
        let bytes = s.as_bytes();
        let mut result = 0;
        for size in 1..=bytes.len() {
            bytes.windows(size).for_each(|w| {
                if is_palindrome(w) {
                    result += 1;
                }
            });
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s = "abc".to_string();
        assert_eq!(Solution::count_substrings(s), 3);
        // Explanation: Three palindromic strings: "a", "b", "c".
    }
    #[test]
    fn example2() {
        let s = "aaa".to_string();
        assert_eq!(Solution::count_substrings(s), 6);
        // Explanation: Six palindromic strings: "a", "a", "a", "aa", "aa", "aaa".
    }

    #[test]
    fn s_aaaa() {
        let s = "aaaa".to_string();
        assert_eq!(Solution::count_substrings(s), 10);
        // "a", "a", "a", "a",
        // "aa", "aa", "aa",
        // "aaa", "aaa"
        // "aaaa"
    }

    mod performance {
        use super::*;

        #[test]
        fn s_1000a() {
            let s = "a".repeat(1000);
            assert_eq!(Solution::count_substrings(s), 500500);
        }
    }
}
