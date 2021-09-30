#![allow(dead_code)]
/// Decode Ways II
/// ==============
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
/// To __decode__ an encoded message, all the digits must be grouped then mapped back into letters using the reverse
/// of the mapping above (there may be multiple ways).
/// For example, `"11106"` can be mapped into:
///
/// - `"AAJF"` with the grouping `(1 1 10 6)`
/// - `"KJF"` with the grouping `(11 10 6)`
///
/// Note that the grouping `(1 11 06)` is invalid because `"06"` cannot be mapped into `'F'`
/// since `"6"` is different from `"06"`.
///
/// __In addition__ to the mapping above, an encoded message may contain the `'*'` character,
/// which can represent any digit from `'1'` to `'9'` (`'0'` is excluded).
/// For example, the encoded message `"1*"` may represent any of the encoded messages
/// `"11"`, `"12"`, `"13"`, `"14"`, `"15"`, `"16"`, `"17"`, `"18"`, or `"19"`.
/// Decoding `"1*"` is equivalent to decoding __any__ of the encoded messages it can represent.
///
/// Given a string `s` containing digits and the `'*'` character, return _the __number__ of ways to __decode__ it_.
///
/// Since the answer may be very large, return it __modulo__ `10^9 + 7`.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 100_000`
/// - `s[i]` is a digit or `'*'`.
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/609/week-2-july-8th-july-14th/3809/
struct Solution;
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;

        fn mul(a: i64, b: i64) -> i64 {
            (a * b) % MOD
        }
        fn add(a: i64, b: i64) -> i64 {
            (a + b) % MOD
        }

        fn rec(i: usize, memo: &mut Vec<i64>, s: &[u8]) -> i64 {
            if i == s.len() {
                1
            } else if memo[i] >= 0 {
                memo[i]
            } else if i == s.len() - 1 {
                match s[i] {
                    b'*' => 9,
                    b'0' => 0,
                    _ => 1,
                }
            } else {
                let r1 = rec(i + 1, memo, s);
                let r2 = rec(i + 2, memo, s);
                memo[i] = match (s[i], s[i + 1]) {
                    (b'0', _) => 0,
                    (b'*', b'*') => add(mul(9, r1), mul(15, r2)),
                    (b'*', b'7' | b'8' | b'9') => add(mul(9, r1), r2),
                    (b'*', _) => add(mul(9, r1), mul(2, r2)),
                    (b'1', b'*') => add(r1, mul(9, r2)),
                    (b'1', _) => add(r1, r2),
                    (b'2', b'*') => add(r1, mul(6, r2)),
                    (b'2', b'7' | b'8' | b'9') => r1,
                    (b'2', _) => add(r1, r2),
                    _ => r1,
                };
                memo[i]
            }
        }

        let sb = s.as_bytes();
        rec(0, &mut vec![-1; sb.len()], sb) as i32
    }

    pub fn num_decodings_recursion_memo(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;

        fn mul(a: i64, b: i64) -> i64 {
            (a * b) % MOD
        }
        fn add(a: i64, b: i64) -> i64 {
            (a + b) % MOD
        }

        fn rec(i: usize, memo: &mut Vec<i64>, s: &[u8]) -> i64 {
            if i == s.len() {
                1
            } else if memo[i] >= 0 {
                memo[i]
            } else if i == s.len() - 1 {
                match s[i] {
                    b'*' => 9,
                    b'0' => 0,
                    _ => 1,
                }
            } else {
                let r1 = rec(i + 1, memo, s);
                let r2 = rec(i + 2, memo, s);
                let r = match s[i] {
                    b'*' => add(
                        mul(9, r1),
                        match s[i + 1] {
                            b'*' => mul(15, r2),
                            b'7' | b'8' | b'9' => mul(1, r2),
                            _ => mul(2, r2),
                        },
                    ),
                    b'0' => 0,
                    b'1' => add(
                        r1,
                        match s[i + 1] {
                            b'*' => mul(9, r2),
                            _ => r2,
                        },
                    ),
                    b'2' => add(
                        r1,
                        match s[i + 1] {
                            b'*' => mul(6, r2),
                            b'7' | b'8' | b'9' => 0,
                            _ => r2,
                        },
                    ),
                    _ => r1,
                };
                memo[i] = r;
                r
            }
        }

        let sb = s.as_bytes();
        rec(0, &mut vec![-1; sb.len()], sb) as i32
    }

    pub fn num_decodings_recursion_idx(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;

        fn mul(a: i64, b: i64) -> i64 {
            (a * b) % MOD
        }
        fn add(a: i64, b: i64) -> i64 {
            (a + b) % MOD
        }

        fn rec(i: usize, s: &[u8]) -> i64 {
            if i == s.len() {
                1
            } else if i == s.len() - 1 {
                match s[i] {
                    b'*' => 9,
                    b'0' => 0,
                    _ => 1,
                }
            } else {
                let r1 = rec(i + 1, s);
                let r2 = rec(i + 2, s);
                match s[i] {
                    b'*' => add(
                        mul(9, r1),
                        match s[i + 1] {
                            b'*' => mul(15, r2),
                            b'7' | b'8' | b'9' => mul(1, r2),
                            _ => mul(2, r2),
                        },
                    ),
                    b'0' => 0,
                    b'1' => add(
                        r1,
                        match s[i + 1] {
                            b'*' => mul(9, r2),
                            _ => r2,
                        },
                    ),
                    b'2' => add(
                        r1,
                        match s[i + 1] {
                            b'*' => mul(6, r2),
                            b'7' | b'8' | b'9' => 0,
                            _ => r2,
                        },
                    ),
                    _ => r1,
                }
            }
        }

        (rec(0, s.as_bytes()) % MOD) as i32
    }

    pub fn num_decodings_recursion_str_slices(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;

        fn mul(a: i64, b: i64) -> i64 {
            (a * b) % MOD
        }
        fn add(a: i64, b: i64) -> i64 {
            (a + b) % MOD
        }

        fn rec(s: &[u8]) -> i64 {
            if s.is_empty() {
                1
            } else if s.len() == 1 {
                match s[0] {
                    b'*' => 9,
                    b'0' => 0,
                    _ => 1,
                }
            } else {
                let r1 = rec(&s[1..]);
                let r2 = rec(&s[2..]);
                match s[0] {
                    b'*' => add(
                        mul(9, r1),
                        match s[1] {
                            b'*' => mul(15, r2),
                            b'7' | b'8' | b'9' => mul(1, r2),
                            _ => mul(2, r2),
                        },
                    ),
                    b'0' => 0,
                    b'1' => add(
                        r1,
                        match s[1] {
                            b'*' => mul(9, r2),
                            _ => r2,
                        },
                    ),
                    b'2' => add(
                        r1,
                        match s[1] {
                            b'*' => mul(6, r2),
                            b'7' | b'8' | b'9' => 0,
                            _ => r2,
                        },
                    ),
                    _ => r1,
                }
            }
        }

        (rec(s.as_bytes()) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s_x_produces_9() {
        assert_eq!(Solution::num_decodings("*".to_string()), 9);
        // Explanation:
        // The encoded message can represent any of the encoded messages "1", "2", "3", "4", "5", "6", "7", "8", or "9".
        // Each of these can be decoded to the strings "A", "B", "C", "D", "E", "F", "G", "H", and "I" respectively.
        // Hence, there are a total of 9 ways to decode "*".
    }
    #[test]
    fn s_1x_produces_18() {
        assert_eq!(Solution::num_decodings("1*".to_string()), 18);
        // Explanation: The encoded message can represent any of the encoded messages
        // "11", "12", "13", "14", "15", "16", "17", "18", or "19".
        // Each of these encoded messages have 2 ways to be decoded (e.g. "11" can be decoded to "AA" or "K").
        // Hence, there are a total of 9 * 2 = 18 ways to decode "1*".
    }
    #[test]
    fn s_2x_produces_15() {
        assert_eq!(Solution::num_decodings("2*".to_string()), 15);
        // Explanation: The encoded message can represent any of the encoded messages
        // "21", "22", "23", "24", "25", "26", "27", "28", or "29".
        // "21", "22", "23", "24", "25", and "26" have 2 ways of being decoded, but "27", "28", and "29" only have 1 way.
        // Hence, there are a total of (6 * 2) + (3 * 1) = 12 + 3 = 15 ways to decode "2*".
    }
    #[test]
    fn s_999_produces_1() {
        assert_eq!(Solution::num_decodings("999".to_string()), 1);
    }
    #[test]
    fn s_1_produces_1() {
        assert_eq!(Solution::num_decodings("1".to_string()), 1);
    }
    #[test]
    fn s_01_produces_0() {
        assert_eq!(Solution::num_decodings("01".to_string()), 0);
    }
    #[test]
    fn s_10_produces_1() {
        assert_eq!(Solution::num_decodings("10".to_string()), 1);
    }
    #[test]
    fn s_11_produces_2() {
        assert_eq!(Solution::num_decodings("11".to_string()), 2);
    }
    #[test]
    fn s_26_produces_2() {
        assert_eq!(Solution::num_decodings("26".to_string()), 2);
    }
    #[test]
    fn s_27_produces_1() {
        assert_eq!(Solution::num_decodings("27".to_string()), 1);
    }
    #[test]
    fn s_333_produces_1() {
        assert_eq!(Solution::num_decodings("333".to_string()), 1);
    }
    #[test]
    fn s_111_produces_3() {
        assert_eq!(Solution::num_decodings("111".to_string()), 3);
    }
    #[test]
    fn s_1111_produces_5() {
        assert_eq!(Solution::num_decodings("1111".to_string()), 5);
    }
    #[test]
    fn s_11111_produces_8() {
        assert_eq!(Solution::num_decodings("11111".to_string()), 8);
    }

    #[test]
    fn s_3x_produces_9() {
        assert_eq!(Solution::num_decodings("3*".to_string()), 9);
    }
    #[test]
    fn s_x0_produces_2() {
        assert_eq!(Solution::num_decodings("*0".to_string()), 2);
    }
    #[test]
    fn s_x1_produces_11() {
        assert_eq!(Solution::num_decodings("*1".to_string()), 11);
    }
    #[test]
    fn s_x6_produces_11() {
        assert_eq!(Solution::num_decodings("*6".to_string()), 11);
    }
    #[test]
    fn s_x7_produces_10() {
        assert_eq!(Solution::num_decodings("*7".to_string()), 10);
    }

    #[test]
    fn s_1x1_produces_20() {
        assert_eq!(Solution::num_decodings("1*1".to_string()), 20);
    }
    #[test]
    fn s_2x1_produces_17() {
        assert_eq!(Solution::num_decodings("2*1".to_string()), 17);
    }
    #[test]
    fn s_3x1_produces_11() {
        assert_eq!(Solution::num_decodings("3*1".to_string()), 11);
    }
    #[test]
    fn s_3x7_produces_10() {
        assert_eq!(Solution::num_decodings("3*7".to_string()), 10);
    }

    #[test]
    fn s_xx_produces_96() {
        assert_eq!(Solution::num_decodings("**".to_string()), 96);
    }
    #[test]
    fn s_xxx_produces_999() {
        assert_eq!(Solution::num_decodings("***".to_string()), 999);
    }

    #[test]
    fn s_xxxxxxxxxx_produces_483_456_820() {
        assert_eq!(
            Solution::num_decodings("**********".to_string()),
            483_456_820
        );
    }

    /// If getting stack overflow:
    ///
    /// ```sh
    /// thread 'c2021::c2021_07::c2021_07_10::tests::performance::s_7_repeat_100_000_produces_1' has overflowed its stack
    /// fatal runtime error: stack overflow
    /// ```
    ///
    /// Add `RUST_MIN_STACK=67108864` to env
    mod performance {
        use super::*;

        #[test]
        fn s_7_repeat_100_000_produces_1() {
            assert_eq!(Solution::num_decodings("7".repeat(100_000)), 1);
        }
        #[test]
        fn s_x_repeat_100_000_produces_81_890_275() {
            assert_eq!(Solution::num_decodings("*".repeat(100_000)), 81_890_275);
        }
    }
}
