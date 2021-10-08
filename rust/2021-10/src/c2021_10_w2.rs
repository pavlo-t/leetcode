#![allow(dead_code)]
/// 1216. Valid Palindrome III
/// ==========================
///
/// Given a string `s` and an integer `k`, return `true` if `s` is a `k`__-palindrome__.
///
/// A string is `k`__-palindrome__ if it can be transformed into a palindrome
/// by removing at most `k` characters from it.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 1000`
/// - `s` consists of only lowercase English letters.
/// - `1 <= k <= s.length`
///
/// https://leetcode.com/problems/valid-palindrome-iii/
struct Solution;
impl Solution {
    /// Approach 3: Bottom-Up DP (1D)
    /// https://leetcode.com/problems/valid-palindrome-iii/solution/
    pub fn is_valid_palindrome(s: String, k: i32) -> bool {
        println!("is_valid_palindrome({}, {})", s, k);
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![0; n];

        let mut temp;
        let mut prev;

        for l in (0..n - 1).rev() {
            prev = 0;
            for r in l + 1..n {
                temp = dp[r];
                dp[r] = if s[l] == s[r] { prev } else { 1 + dp[r].min(dp[r - 1]) };
                prev = temp;
            }
        }

        dp[n - 1] <= k
    }

    pub fn is_valid_palindrome_dp(s: String, k: i32) -> bool {
        println!("is_valid_palindrome({}, {})", s, k);
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0; n]; n];

        for len in 1..n {
            for l in 0..n - len {
                let r = l + len;
                dp[l][r] = if s[l] == s[r] {
                    dp[l + 1][r - 1]
                } else {
                    1 + dp[l + 1][r].min(dp[l][r - 1])
                };
            }
        }

        dp[0][n - 1] <= k
    }

    pub fn is_valid_palindrome_recursion_with_memo(s: String, k: i32) -> bool {
        println!("is_valid_palindrome({}, {})", s, k);
        fn rec(l: usize, r: usize, memo: &mut Vec<Vec<i32>>, s: &[u8]) -> i32 {
            println!(" rec({},{})", s[l] as char, s[r] as char);
            if l >= r {
                0
            } else if memo[l][r] >= 0 {
                memo[l][r]
            } else if s[l] == s[r] {
                memo[l][r] = rec(l + 1, r - 1, memo, s);
                memo[l][r]
            } else {
                memo[l][r] = 1 + rec(l + 1, r, memo, s).min(rec(l, r - 1, memo, s));
                memo[l][r]
            }
        }
        let bs = s.as_bytes();
        let n = bs.len();
        rec(0, bs.len() - 1, &mut vec![vec![-1; n]; n], bs) <= k
    }

    /// Got idea to remo `k` from arguments from https://leetcode.com/problems/valid-palindrome-iii/solution/
    pub fn is_valid_palindrome_recursion(s: String, k: i32) -> bool {
        println!("is_valid_palindrome({}, {})", s, k);
        fn rec(l: usize, r: usize, s: &[u8]) -> i32 {
            println!(" rec({},{})", s[l] as char, s[r] as char);
            if l >= r {
                0
            } else if s[l] == s[r] {
                rec(l + 1, r - 1, s)
            } else {
                1 + rec(l + 1, r, s).min(rec(l, r - 1, s))
            }
        }
        let bs = s.as_bytes();
        rec(0, bs.len() - 1, bs) <= k
    }

    pub fn is_valid_palindrome_my_recursion_with_memo(s: String, k: i32) -> bool {
        println!("is_valid_palindrome({}, {})", s, k);
        fn rec(l: usize, r: usize, k: usize, seen: &mut Vec<Vec<Vec<bool>>>, s: &[u8]) -> bool {
            println!(" rec({},{},{})", s[l] as char, s[r] as char, k);
            if l >= r || r - l <= k {
                true
            } else if seen[l][r][k] {
                false
            } else if s[l] == s[r] {
                seen[l][r][k] = !rec(l + 1, r - 1, k, seen, s);
                !seen[l][r][k]
            } else if k < 1 {
                false
            } else {
                seen[l][r][k] = !(rec(l + 1, r, k - 1, seen, s) || rec(l, r - 1, k - 1, seen, s));
                !seen[l][r][k]
            }
        }
        let bs = s.as_bytes();
        let mut seen = vec![vec![vec![false; k as usize + 1]; bs.len()]; bs.len()];
        rec(0, bs.len() - 1, k as usize, &mut seen, bs)
    }
    pub fn is_valid_palindrome_my_recursion_brute_force(s: String, k: i32) -> bool {
        println!("is_valid_palindrome({}, {})", s, k);
        fn rec(l: usize, r: usize, k: usize, s: &[u8]) -> bool {
            println!(" rec({},{},{})", s[l] as char, s[r] as char, k);
            if r - l <= k {
                true
            } else if s[l] == s[r] {
                rec(l + 1, r - 1, k, s)
            } else if k < 1 {
                false
            } else {
                rec(l + 1, r, k - 1, s) || rec(l, r - 1, k - 1, s)
            }
        }
        let bs = s.as_bytes();
        rec(0, bs.len() - 1, k as usize, bs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abcdeca_k_2() {
        let s = "abcdeca".to_string();
        let k = 2;
        assert!(Solution::is_valid_palindrome(s, k));
        // Explanation: Remove 'b' and 'e' characters.
    }
    #[test]
    fn abbababa_k_1() {
        let s = "abbababa".to_string();
        let k = 1;
        assert!(Solution::is_valid_palindrome(s, k));
    }
    #[test]
    fn abcde_k_1() {
        let s = "abcde".to_string();
        let k = 1;
        assert!(!Solution::is_valid_palindrome(s, k));
    }
    #[test]
    fn abcde_k_3() {
        let s = "abcde".to_string();
        let k = 3;
        assert!(!Solution::is_valid_palindrome(s, k));
    }
    #[test]
    fn abcde_k_4() {
        let s = "abcde".to_string();
        let k = 4;
        assert!(Solution::is_valid_palindrome(s, k));
    }

    #[test]
    fn aaabaabaa_k_1() {
        let s = "aaabaabaa".to_string();
        let k = 1;
        assert!(Solution::is_valid_palindrome(s, k));
    }
    #[test]
    fn test49() {
        let mut s = "fcgihcgeadfehgiabegbiahbeadbiafgcfchbcacedbificicihibaeehbffeidia".to_string();
        s.push_str("iighceegbfdggggcfaiibefbgeegbcgeadcfdfegfghebcfceiabiagehhibiheddbcgdebdcfega");
        s.push_str("iahibcfhheggbheebfdahgcfcahafecfehgcgdabbghddeadecidicchfgicbdbecibddfcgbiadi");
        s.push_str("ffcifiggigdeedbiiihfgehhdegcaffaggiidiifgfigfiaiicadceefbhicfhbcachacaeiefdcc");
        s.push_str("hegfbifhaeafdehicfgbecahidgdagigbhiffhcccdhfdbd");

        let k = 216;
        assert!(Solution::is_valid_palindrome(s, k));
    }

    //#[ignore]
    #[test]
    fn ax1000_k_1() {
        let s = "a".repeat(1000);
        let k = 1;
        assert!(Solution::is_valid_palindrome(s, k));
    }
    //#[ignore]
    #[test]
    fn a_to_y_repeat_40_k_1() {
        let s = ('a'..='y').collect::<String>().repeat(40);
        let k = 1;
        assert!(!Solution::is_valid_palindrome(s, k));
    }
    //#[ignore]
    #[test]
    fn a_to_y_repeat_40_k_1000() {
        let s = ('a'..='y').collect::<String>().repeat(40);
        let k = 900;
        assert!(!Solution::is_valid_palindrome(s, k));
    }
}
