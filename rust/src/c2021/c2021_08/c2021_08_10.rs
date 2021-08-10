#![allow(dead_code)]
/// Flip String to Monotone Increasing
/// ==================================
///
/// A binary string is monotone increasing if it consists of some number of `0`'s (possibly none),
/// followed by some number of `1`'s (also possibly none).
///
/// You are given a binary string `s`.
/// You can flip `s[i]` changing it from `0` to `1` or from `1` to `0`.
///
/// Return _the minimum number of flips to make `s` monotone increasing_.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 100_000`
/// - `s[i]` is either `'0'` or `'1'`.
///
/// https://leetcode.com/explore/featured/card/august-leetcoding-challenge-2021/614/week-2-august-8th-august-14th/3876/
struct Solution;
impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        s.chars() .fold((0, 0), |(f, o), c| match c {
            '0' => (o.min(f + 1), o),
            '1' => (f, o + 1),
            _ => unreachable!(),
        }).0
    }
    pub fn min_flips_mono_incr_1(s: String) -> i32 {
        let (f0, f1) = s.chars().fold((0, 0), |(f0, f1), c| match c {
            '0' => (f0, f0.min(f1) + 1),
            '1' => (f0 + 1, f0.min(f1)),
            _ => unreachable!(),
        });
        f0.min(f1)
    }

    pub fn min_flips_mono_incr_my(s: String) -> i32 {
        let n = s.len();

        let mut l1 = Vec::with_capacity(n + 1);
        l1.push(0);
        let mut ones = 0;
        for c in s.chars() {
            if c == '1' {
                ones += 1;
            }
            l1.push(ones);
        }

        let mut r0 = Vec::with_capacity(n + 1);
        r0.push(0);
        let mut zeros = 0;
        for c in s.chars().rev() {
            if c == '0' {
                zeros += 1;
            }
            r0.push(zeros);
        }
        r0.reverse();

        l1.iter().zip(r0.iter()).map(|(l, r)| l + r).min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s_00110_produces_1() {
        let s = "00110".to_string();
        assert_eq!(Solution::min_flips_mono_incr(s), 1);
        // Explanation: We flip the last digit to get 00111.
    }
    #[test]
    fn s_010110_produces_2() {
        let s = "010110".to_string();
        assert_eq!(Solution::min_flips_mono_incr(s), 2);
        // Explanation: We flip to get 011111, or alternatively 000111.
    }
    #[test]
    fn s_00011000_produces_2() {
        let s = "00011000".to_string();
        assert_eq!(Solution::min_flips_mono_incr(s), 2);
        // Explanation: We flip to get 00000000.
    }

    #[test]
    fn s_0_produces_0() {
        let s = "0".to_string();
        assert_eq!(Solution::min_flips_mono_incr(s), 0);
    }
    #[test]
    fn s_1_produces_0() {
        let s = "1".to_string();
        assert_eq!(Solution::min_flips_mono_incr(s), 0);
    }

    #[test]
    fn s_10011111110010111011_produces_5() {
        let s = "10011111110010111011".to_string();
        assert_eq!(Solution::min_flips_mono_incr(s), 5);
    }
}
