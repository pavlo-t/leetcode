#![allow(dead_code)]
/// Decode Ways
/// ===========
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
/// To __decode__ an encoded message, all the digits must be grouped then mapped back into letters
/// using the reverse of the mapping above (there may be multiple ways).
/// For example, `"11106"` can be mapped into:
///
/// - `"AAJF"` with the grouping `(1 1 10 6)`
/// - `"KJF"` with the grouping `(11 10 6)`
///
/// Note that the grouping `(1 11 06)` is invalid because `"06"` cannot be mapped into `'F'`
/// since `"6"` is different from `"06"`.
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
/// https://leetcode.com/explore/challenge/card/august-leetcoding-challenge-2021/615/week-3-august-15th-august-21st/3902/
struct Solution;
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        fn rec(i: usize, s: &[u8], memo: &mut Vec<i32>) -> i32 {
            if memo[i] >= 0 {
                memo[i]
            } else if i == s.len() - 1 {
                memo[i] = match s[i] {
                    b'0' => 0,
                    _ => 1,
                };
                memo[i]
            } else {
                memo[i] = match (s[i], s[i + 1]) {
                    (b'0', _) => 0,
                    (b'1', _) | (b'2', b'0'..=b'6') => rec(i + 1, s, memo) + rec(i + 2, s, memo),
                    _ => rec(i + 1, s, memo),
                };
                memo[i]
            }
        }
        let mut memo = vec![-1; s.len()];
        memo.push(1);
        rec(0, s.as_bytes(), &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s12_produces_2() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
        // Explanation: "12" could be decoded as "AB" (1 2) or "L" (12).
    }
    #[test]
    fn s226_produces_3() {
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
        // Explanation: "226" could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
    }
    #[test]
    fn s0_produces_0() {
        assert_eq!(Solution::num_decodings("0".to_string()), 0);
        // Explanation: There is no character that is mapped to a number starting with 0.
        // The only valid mappings with 0 are 'J' -> "10" and 'T' -> "20", neither of which start with 0.
        // Hence, there are no valid ways to decode this since all digits need to be mapped.
    }
    #[test]
    fn s06_produces_0() {
        assert_eq!(Solution::num_decodings("06".to_string()), 0);
        // Explanation: "06" cannot be mapped to "F" because of the leading zero ("6" is different from "06").
    }

    #[test]
    fn s1() {
        assert_eq!(Solution::num_decodings("1".to_string()), 1);
    }
    #[test]
    fn s10() {
        assert_eq!(Solution::num_decodings("10".to_string()), 1);
    }
    #[test]
    fn s11() {
        assert_eq!(Solution::num_decodings("11".to_string()), 2);
    }
    #[test]
    fn s20() {
        assert_eq!(Solution::num_decodings("20".to_string()), 1);
    }
    #[test]
    fn s26() {
        assert_eq!(Solution::num_decodings("26".to_string()), 2);
    }
    #[test]
    fn s27() {
        assert_eq!(Solution::num_decodings("27".to_string()), 1);
    }
    #[test]
    fn s30() {
        assert_eq!(Solution::num_decodings("30".to_string()), 0);
    }
    #[test]
    fn s101() {
        assert_eq!(Solution::num_decodings("101".to_string()), 1);
    }
    #[test]
    fn s110() {
        assert_eq!(Solution::num_decodings("110".to_string()), 1);
    }
    #[test]
    fn s111() {
        assert_eq!(Solution::num_decodings("111".to_string()), 3);
    }
    #[test]
    fn s202() {
        assert_eq!(Solution::num_decodings("202".to_string()), 1);
    }
    #[test]
    fn s222() {
        assert_eq!(Solution::num_decodings("222".to_string()), 3);
    }
    #[test]
    fn s227() {
        assert_eq!(Solution::num_decodings("227".to_string()), 2);
    }
    #[test]
    fn s333() {
        assert_eq!(Solution::num_decodings("333".to_string()), 1);
    }
    // 1,1,1,1; 1,1,11; 1,11,1; 11,1,1; 11,11
    // f(111) + f(11) => (f(11) + f(1)) + (f(1) + f("")) => ((f(1) + f("")) + 1) + (1 + 1) => 5
    #[test]
    fn s1111() {
        assert_eq!(Solution::num_decodings("1111".to_string()), 5);
    }
    // 2,2,2,2; 2,2,22; 2,22,2; 22,2,2; 22,22
    #[test]
    fn s2222() {
        assert_eq!(Solution::num_decodings("2222".to_string()), 5);
    }
    // f(2627) => f(627) + f(27) => f(27) + 1 => 2
    #[test]
    fn s2627() {
        assert_eq!(Solution::num_decodings("2627".to_string()), 2);
    }
    // f(3333) => f(333) => f(33) => f(3) => f("") => 1
    #[test]
    fn s3333() {
        assert_eq!(Solution::num_decodings("3333".to_string()), 1);
    }
    #[test]
    fn s11111() {
        assert_eq!(Solution::num_decodings("11111".to_string()), 8);
    }
    #[test]
    fn s11100() {
        assert_eq!(Solution::num_decodings("11100".to_string()), 0);
    }
    #[test]
    fn s3x100() {
        assert_eq!(Solution::num_decodings("3".repeat(100)), 1);
    }
    #[test]
    fn s1x45() {
        assert_eq!(Solution::num_decodings("1".repeat(45)), 1836311903);
    }
}
