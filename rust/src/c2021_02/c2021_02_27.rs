#![allow(dead_code)]

/// # Divide Two Integers
///
/// Given two integers `dividend` and `divisor`,
/// divide two integers without using multiplication, division, and mod operator.
///
/// Return the quotient after dividing `dividend` by `divisor`.
///
/// The integer division should truncate toward zero, which means losing its fractional part.
/// For example, `truncate(8.345) = 8` and `truncate(-2.7335) = -2`.
///
/// __Note:__
///
/// - Assume we are dealing with an environment that could only store integers within the 32-bit
///   signed integer range: `[−2^31,  2^31 − 1]`.
///   For this problem, assume that your function
///   __returns `2^31 − 1` when the division result overflows__.
///
/// __Constraints:__
///
/// - `-2^31 <= dividend, divisor <= 2^31 - 1`
/// - `divisor != 0`
///
/// https://leetcode.com/explore/featured/card/february-leetcoding-challenge-2021/587/week-4-february-22nd-february-28th/3654/
struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == -2147483648 && divisor == -1 {
            2147483647
        } else {
            let mut positives = 0;
            let mut dividend = if dividend > 0 {
                positives += 1;
                -dividend
            } else {
                dividend
            };
            let divisor = if divisor > 0 {
                positives += 1;
                -divisor
            } else {
                divisor
            };

            let mut quotient = 0;

            while dividend <= divisor {
                let mut po2 = -1;
                let mut val = divisor;
                while val >= -1073741824 && val + val >= dividend {
                    val += val;
                    po2 += po2;
                }
                quotient += po2;
                dividend -= val;
            }

            if positives == 1 {
                quotient
            } else {
                -quotient
            }
        }
    }

    pub fn divide_brute_force(dividend: i32, divisor: i32) -> i32 {
        if dividend == -2147483648 && divisor == -1 {
            2147483647
        } else {
            let mut positives = 0;
            let mut dividend = if dividend > 0 {
                positives += 1;
                -dividend
            } else {
                dividend
            };
            let divisor = if divisor > 0 {
                positives += 1;
                -divisor
            } else {
                divisor
            };

            let mut quotient = 0;

            while dividend <= divisor {
                quotient += 1;
                dividend -= divisor;
            }

            if positives == 1 {
                -quotient
            } else {
                quotient
            }
        }
    }

    /// https://www.geeksforgeeks.org/divide-two-integers-without-using-multiplication-division-mod-operator/
    pub fn divide_binary_i64(dividend: i32, divisor: i32) -> i32 {
        const MAX: i64 = 2147483647;

        let dividend_abs = (dividend as i64).abs();
        let divisor_abs = (divisor as i64).abs();

        let mut quotient = 0;
        let mut temp = 0;

        for i in (0..32).rev() {
            if temp + (divisor_abs << i) <= dividend_abs {
                temp += divisor_abs << i;
                quotient |= 1 << i;
            }
        }

        (if (dividend > 0) == (divisor > 0) {
            if quotient > MAX {
                MAX
            } else {
                quotient
            }
        } else {
            -quotient
        }) as i32
    }

    pub fn divide_brute_force_i64(dividend: i32, divisor: i32) -> i32 {
        const MAX: i64 = 2147483647;

        let dividend_abs = (dividend as i64).abs();
        let divisor_abs = (divisor as i64).abs();

        let mut quotient = 0;
        let mut sum = 0;

        while sum < dividend_abs {
            quotient += 1;
            sum += divisor_abs;
        }
        if sum > dividend_abs {
            quotient -= 1;
        }

        (if dividend == 0 || (dividend > 0 && divisor > 0) || (dividend < 0 && divisor < 0) {
            if quotient > MAX {
                MAX
            } else {
                quotient
            }
        } else {
            -quotient
        }) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_divide_10_by_3_should_produce_3() {
        assert_eq!(Solution::divide(10, 3), 3);
        // Explanation: 10/3 = truncate(3.33333..) = 3.
    }
    #[test]
    fn example2_divide_7_by_m3_should_produce_m2() {
        assert_eq!(Solution::divide(7, -3), -2);
        // Explanation: 7/-3 = truncate(-2.33333..) = -2.
    }
    #[test]
    fn example3_divide_0_by_1_should_produce_0() {
        assert_eq!(Solution::divide(0, 1), 0);
    }
    #[test]
    fn example4_divide_1_by_1_should_produce_1() {
        assert_eq!(Solution::divide(1, 1), 1);
    }

    #[test]
    fn test12_divide_m2147483648_by_m1_should_produce_1() {
        assert_eq!(Solution::divide(-2147483648, -1), 2147483647);
    }
    #[test]
    fn test15_divide_m2147483648_by_2_should_produce_m1073741824() {
        assert_eq!(Solution::divide(-2147483648, 2), -1073741824);
    }

    #[test]
    fn divide_2147483647_by_2147483647_should_produce_1() {
        assert_eq!(Solution::divide(2147483647, 2147483647), 1);
    }
    #[test]
    fn divide_2147483647_by_1_should_produce_2147483647() {
        assert_eq!(Solution::divide(2147483647, 1), 2147483647);
    }
}
