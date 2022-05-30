#![allow(dead_code)]
/// \#29. Divide Two Integers
/// =========================
///
/// Given two integers `dividend` and `divisor`, divide two integers
/// __without__ using multiplication, division, and mod operator.
///
/// The integer division should truncate toward zero, which means losing its fractional part.
/// For example, `8.345` would be truncated to `8`, and `-2.7335` would be truncated to `-2`.
///
/// Return _the __quotient__ after dividing `dividend` by `divisor`_.
///
/// __Note:__ Assume we are dealing with an environment that could only store integers
/// within the __32-bit__ signed integer range: `[−2**31, 2**31 − 1]`.
/// For this problem, if the quotient is __strictly greater than__ `2**31 - 1`, then return `2**31 - 1`,
/// and if the quotient is __strictly less than__ `-2**31`, then return `-2**31`.
///
/// __Constraints:__
///
/// - `-2**31 <= dividend, divisor <= 2**31 - 1`
/// - `divisor != 0`
///
/// https://leetcode.com/problems/divide-two-integers
struct Solution;
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        fn times_10(n: i32) -> i32 {
            (0..10).fold(0, |acc, _| acc + n)
        }
        match (dividend, divisor) {
            (i32::MIN, -1) => i32::MAX,
            (_, 1) => dividend,
            _ => {
                let s = dividend.to_string();
                let bs = s.as_bytes();
                let divisor_neg = if divisor > 0 { -divisor } else { divisor };
                let mut i = (dividend < 0) as usize;
                let mut remainder = 0;
                let mut result = 0;
                while i < bs.len() {
                    remainder = times_10(remainder);
                    remainder -= (bs[i] - b'0') as i32;
                    let mut curr = 0;
                    while remainder <= divisor_neg {
                        remainder -= divisor_neg;
                        curr += 1;
                    }
                    result = times_10(result);
                    result += curr;
                    i += 1;
                }
                if (dividend < 0) != (divisor < 0) {
                    -result
                } else {
                    result
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d_10_3() {
        assert_eq!(Solution::divide(10, 3), 3);
        // Explanation: 10/3 = 3.33333.. which is truncated to 3.
    }
    #[test]
    fn d_7_m3() {
        assert_eq!(Solution::divide(7, -3), -2);
        // Explanation: 7/-3 = -2.33333.. which is truncated to -2.
    }
    #[test]
    fn d_0_m3() {
        assert_eq!(Solution::divide(0, -3), 0);
    }

    #[test]
    fn d_m1010369383_m2147483648() {
        assert_eq!(Solution::divide(-1010369383, -2147483648), 0);
    }
    #[test]
    fn d_m2147483648_m1017100424() {
        assert_eq!(Solution::divide(-2147483648, -1017100424), 2);
    }

    #[test]
    fn d_2xx31m1_m1() {
        assert_eq!(Solution::divide(i32::MAX, -1), -i32::MAX);
    }
    #[test]
    fn d_m2xx31_m1() {
        assert_eq!(Solution::divide(i32::MIN, -1), i32::MAX);
    }
    #[test]
    fn d_2xx31m1_1() {
        assert_eq!(Solution::divide(i32::MAX, 1), i32::MAX);
    }
    #[test]
    fn d_m2xx31_1() {
        assert_eq!(Solution::divide(i32::MIN, 1), i32::MIN);
    }
}
