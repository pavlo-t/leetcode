#![allow(dead_code)]

/// ### Decode Ways
///
/// https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3581/
struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let bytes = s.as_bytes();

        let mut pp = 1;
        let mut p = if bytes[0] == b'0' { 0 } else { 1 };

        for i in 1..s.len() {
            let mut c = 0;
            if bytes[i] != b'0' {
                c += p;
            }
            if bytes[i - 1] == b'1' || (bytes[i - 1] == b'2' && bytes[i] < b'7') {
                c += pp;
            }
            pp = p;
            p = c;
        }

        p
    }

    pub fn num_decodings_dp_optimized(s: String) -> i32 {
        let mut pp = 1;
        let mut p = if &s[0..1] != "0" { 1 } else { 0 };

        for i in 1..s.len() {
            let mut c = 0;
            if &s[i..i + 1] != "0" {
                c += p;
            }
            if let Ok(n) = s[i - 1..=i].parse::<i32>() {
                if 9 < n && n < 27 {
                    c += pp;
                }
            }
            pp = p;
            p = c;
        }

        p
    }

    /// https://leetcode.com/problems/decode-ways/solution/
    pub fn num_decodings_dp(s: String) -> i32 {
        let mut dp = vec![0; s.len() + 1];
        dp[0] = 1;
        dp[1] = if &s[0..1] != "0" { 1 } else { 0 };

        for i in 2..dp.len() {
            if &s[i - 1..i] != "0" {
                dp[i] += dp[i - 1];
            }

            if let Ok(n) = s[i - 2..i].parse::<i32>() {
                if n >= 10 && n <= 26 {
                    dp[i] += dp[i - 2]
                }
            }
        }

        *dp.last().unwrap()
    }

    pub fn num_decodings_rec_cache_vec_i32(s: String) -> i32 {
        fn rec(i: usize, bytes: &[u8], cache: &mut Vec<i32>) -> i32 {
            if i == bytes.len() {
                1
            } else if cache[i] != -1 {
                cache[i]
            } else {
                let result =
                    if bytes[i] == b'0' {
                        0
                    } else if i == bytes.len() - 1 {
                        1
                    } else if bytes[i] > b'2' || (bytes[i] == b'2' && bytes[i + 1] > b'6') {
                        rec(i + 1, bytes, cache)
                    } else {
                        rec(i + 1, bytes, cache) + rec(i + 2, bytes, cache)
                    };
                cache[i] = result;
                cache[i]
            }
        }

        rec(0, s.as_bytes(), &mut vec![-1; s.len()])
    }
    pub fn num_decodings_rec_cache_map_string_i32(s: String) -> i32 {
        use std::collections::HashMap;

        fn rec(s: &str, cache: &mut HashMap<String, i32>) -> i32 {
            if cache.contains_key(s) {
                cache[s]
            } else {
                let mut chars = s.chars();
                let result = match (chars.next(), chars.next()) {
                    (None, _) => 1,
                    (Some('0'), _) => 0,
                    (_, None) => 1,
                    (Some(a), Some(b)) if a > '2' || (a == '2' && b > '6') => rec(&s[1..], cache),
                    _ => rec(&s[1..], cache) + rec(&s[2..], cache),
                };
                cache.insert(s.into(), result);
                cache[s]
            }
        }

        let mut cache = HashMap::new();
        rec(&s, &mut cache)
    }
    pub fn num_decodings_brute_force(s: String) -> i32 {
        fn rec(s: &str) -> i32 {
            let mut chars = s.chars();
            match (chars.next(), chars.next()) {
                (None, _) => 1,
                (Some('0'), _) => 0,
                (_, None) => 1,
                (Some(a), Some(b)) if a > '2' || (a == '2' && b > '6') => rec(&s[1..]),
                _ => rec(&s[1..]) + rec(&s[2..]),
            }
        }

        rec(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_s12_is_2() {
        assert_eq!(Solution::num_decodings("12".into()), 2);
        // Explanation: It could be decoded as "AB" (1 2) or "L" (12).
    }

    #[test]
    fn example2_s226_is_3() {
        assert_eq!(Solution::num_decodings("226".into()), 3);
        // Explanation: It could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
    }

    #[test]
    fn example3_s0_is_0() {
        assert_eq!(Solution::num_decodings("0".into()), 0);
        // Explanation: There is no character that is mapped to a number starting with '0'.
        // We cannot ignore a zero when we face it while decoding.
        // So, each '0' should be part of "10" --> 'J' or "20" --> 'T'.
    }

    #[test]
    fn example4_s1_is_1() {
        assert_eq!(Solution::num_decodings("1".into()), 1);
    }

    #[test]
    fn s01_is_0() {
        assert_eq!(Solution::num_decodings("01".into()), 0);
    }

    #[test]
    fn s11_is_2() {
        assert_eq!(Solution::num_decodings("11".into()), 2);
    }

    #[test]
    fn s111_is_3() {
        assert_eq!(Solution::num_decodings("111".into()), 3);
    }

    #[test]
    fn s1111_is_5() {
        assert_eq!(Solution::num_decodings("1111".into()), 5);
    }

    #[test]
    fn s2222_is_5() {
        assert_eq!(Solution::num_decodings("2222".into()), 5);
    }

    #[test]
    fn s2626_is_4() {
        assert_eq!(Solution::num_decodings("2626".into()), 4);
    }

    #[test]
    fn s2727_is_1() {
        assert_eq!(Solution::num_decodings("2727".into()), 1);
    }

    #[test]
    fn s11111_is_8() {
        assert_eq!(Solution::num_decodings("11111".into()), 8);
    }

    #[test]
    fn s111111_is_13() {
        assert_eq!(Solution::num_decodings("111111".into()), 13);
    }

    #[test]
    fn s222222_is_13() {
        assert_eq!(Solution::num_decodings("222222".into()), 13);
    }

    #[test]
    fn s333333_is_1() {
        assert_eq!(Solution::num_decodings("333333".into()), 1);
    }

    #[test]
    fn s100_3s_is_1() {
        assert_eq!(Solution::num_decodings("3".repeat(100)), 1);
    }

    #[test]
    fn s45_1s_is_1836311903() {
        assert_eq!(Solution::num_decodings("1".repeat(45)), 1836311903);
    }
}
