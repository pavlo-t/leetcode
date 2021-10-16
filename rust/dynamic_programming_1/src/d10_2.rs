#![allow(dead_code)]
/// 91. Decode Ways
/// ===============
///
/// A message containing letters from `A-Z` can be __encoded__ into numbers using the following mapping:
///
/// ```text
/// 'A' -> "1"
/// 'B' -> "2"
/// ...
/// 'Z' -> "26"
/// ```
///
/// To __decode__ an encoded message,
/// all the digits must be grouped then mapped back into letters using the reverse of the mapping above
/// (there may be multiple ways).
/// For example, `"11106"` can be mapped into:
///
/// - `"AAJF"` with the grouping `(1 1 10 6)`
/// - `"KJF"` with the grouping `(11 10 6)`
///
/// Note that the grouping `(1 11 06)` is invalid
/// because `"06"` cannot be mapped into `'F'` since `"6"` is different from `"06"`.
///
/// Given a string `s` containing only digits, return _the __number__ of ways to __decode__ it_.
///
/// The answer is guaranteed to fit in a __32-bit__ integer.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 100`
/// - `s` contains only digits and may contain leading zero(s).
///
/// https://leetcode.com/problems/decode-ways/
struct Solution;
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        println!("num_decodings({})", s);
        let n = s.len();
        let bs = s.as_bytes();
        let mut penult = 1;
        let mut ult = (bs[n - 1] != b'0') as i32;
        for i in (0..n - 1).rev() {
            std::mem::swap(&mut ult, &mut penult);
            ult = match (bs[i], bs[i + 1]) {
                (b'0', _) => 0,
                (b'1', _) | (b'2', b'0'..=b'6') => ult + penult,
                _ => penult,
            };
        }
        ult
    }
    /// 15:50
    pub fn num_decodings_dp_const_mem(s: String) -> i32 {
        println!("num_decodings({})", s);
        let n = s.len();
        let bs = s.as_bytes();
        let mut penult = 1;
        let mut ult = (bs[n - 1] != b'0') as i32;
        for i in (0..n - 1).rev() {
            let prev_ult = ult;
            ult = match (bs[i], bs[i + 1]) {
                (b'0', _) => 0,
                (b'1', _) | (b'2', b'0'..=b'6') => ult + penult,
                _ => ult,
            };
            penult = prev_ult;
        }
        ult
    }
    /// 15:32
    pub fn num_decodings_dp_vec(s: String) -> i32 {
        println!("num_decodings({})", s);
        let n = s.len();
        let mut dp = vec![0; n + 2];
        let bs = s.as_bytes();
        dp[n] = 1;
        dp[n - 1] = if bs[n - 1] == b'0' { 0 } else { 1 };
        for i in (0..n - 1).rev() {
            dp[i] = match (bs[i], bs[i + 1]) {
                (b'0', _) => 0,
                (b'1', _) | (b'2', b'0'..=b'6') => dp[i + 1] + dp[i + 2],
                _ => dp[i + 1],
            };
        }
        dp[0]
    }
    /// 15:23
    pub fn num_decodings_rec_with_memo(s: String) -> i32 {
        println!("num_decodings({})", s);
        fn rec(i: usize, s: &[u8], memo: &mut Vec<i32>) -> i32 {
            if i >= s.len() {
                1
            } else if memo[i] >= 0 {
                memo[i]
            } else {
                memo[i] = if s[i] == b'0' {
                    0
                } else if i >= s.len() - 1 {
                    1
                } else if s[i] == b'1' || (s[i] == b'2' && s[i + 1] < b'7') {
                    rec(i + 1, s, memo) + rec(i + 2, s, memo)
                } else {
                    rec(i + 1, s, memo)
                };
                memo[i]
            }
        }
        rec(0, s.as_bytes(), &mut vec![-1; s.len()])
    }
    /// 14:39
    pub fn num_decodings_rec(s: String) -> i32 {
        println!("num_decodings({})", s);
        fn rec(i: usize, s: &[u8]) -> i32 {
            if i >= s.len() {
                1
            } else if s[i] == b'0' {
                0
            } else if i >= s.len() - 1 {
                1
            } else if s[i] == b'1' || (s[i] == b'2' && s[i + 1] < b'7') {
                rec(i + 1, s) + rec(i + 2, s)
            } else {
                rec(i + 1, s)
            }
        }
        rec(0, s.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s_12() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
        // Explanation: "12" could be decoded as "AB" (1 2) or "L" (12).
    }
    #[test]
    fn s_226() {
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
        // Explanation: "226" could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
    }
    #[test]
    fn s_0() {
        assert_eq!(Solution::num_decodings("0".to_string()), 0);
        // Explanation: There is no character that is mapped to a number starting with 0.
        // The only valid mappings with 0 are 'J' -> "10" and 'T' -> "20", neither of which start with 0.
        // Hence, there are no valid ways to decode this since all digits need to be mapped.
    }
    #[test]
    fn s_06() {
        assert_eq!(Solution::num_decodings("06".to_string()), 0);
        // Explanation: "06" cannot be mapped to "F" because of the leading zero ("6" is different from "06").
    }

    // (10 10 1)
    #[rustfmt::skip] #[test] fn s_10101() { assert_eq!(Solution::num_decodings("10101".to_string()), 1); }

    #[rustfmt::skip] #[test] fn s_1() { assert_eq!(Solution::num_decodings("1".to_string()), 1); }
    #[rustfmt::skip] #[test] fn s_11() { assert_eq!(Solution::num_decodings("11".to_string()), 2); }
    // (1 1 1) (1 11) (11 1)
    #[rustfmt::skip] #[test] fn s_111() { assert_eq!(Solution::num_decodings("111".to_string()), 3); }
    // (1 1 1 1) (1 1 11) (1 11 1) (11 1 1) (11 11)
    #[rustfmt::skip] #[test] fn s_1111() { assert_eq!(Solution::num_decodings("1111".to_string()), 5); }
    // (1 1 1 1 1) (1 1 1 11) (1 1 11 1) (1 11 1 1) (11 1 1 1) (1 11 11) (11 1 11) (11 11 1)
    #[rustfmt::skip] #[test] fn s_11111() { assert_eq!(Solution::num_decodings("11111".to_string()), 8); }

    #[test]
    fn s_100x3() {
        assert_eq!(Solution::num_decodings("3".repeat(100)), 1);
    }
    #[test]
    fn s_50x10() {
        assert_eq!(Solution::num_decodings("10".repeat(50)), 1);
    }
    #[test]
    fn s_98x1_2x0() {
        let mut s = "1".repeat(98);
        s.push_str("00");
        assert_eq!(Solution::num_decodings(s), 0);
    }
    #[test]
    fn s_45x1() {
        assert_eq!(Solution::num_decodings("1".repeat(45)), 1_836_311_903);
    }
    #[test]
    fn s_55x3_45x1() {
        let mut s = "3".repeat(55);
        s.push_str(&"1".repeat(45));
        assert_eq!(Solution::num_decodings(s), 1_836_311_903);
    }
}
