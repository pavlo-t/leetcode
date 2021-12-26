#![allow(dead_code)]
/// 227. Basic Calculator II
/// ========================
///
/// Given a string `s` which represents an expression, _evaluate this expression and return its value_.
///
/// The integer division should truncate toward zero.
///
/// You may assume that the given expression is always valid.
/// All intermediate results will be in the range of `[-2**31, 2**31 - 1]`.
///
/// __Note:__ You are not allowed to use any built-in function which evaluates strings as mathematical expressions,
/// such as `eval()`.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 300_000`
/// - `s` consists of integers and operators `('+', '-', '*', '/')` separated by some number of spaces.
/// - `s` represents a valid expression.
/// - All the integers in the expression are non-negative integers in the range `[0, 2**31 - 1]`.
/// - The answer is __guaranteed__ to fit in a __32-bit integer__.
///
/// https://leetcode.com/problems/basic-calculator-ii/
struct Solution;
impl Solution {
    pub fn calculate_my_rust_1_57_0(s: String) -> i32 {
        let mut result = 0;
        let mut prev = 0;
        let mut curr = 0;
        let mut op_sum: Option<char> = None;
        let mut op_mul: Option<char> = None;
        for c in s.chars() {
            match c {
                op @ ('+' | '-') => {
                    match op_mul {
                        Some('*') => curr *= prev,
                        Some('/') => curr = prev / curr,
                        _ => (),
                    }
                    op_mul = None;

                    match op_sum {
                        Some('-') => result -= curr,
                        _ => result += curr,
                    }
                    curr = 0;
                    op_sum = Some(op);
                }
                op @ ('*' | '/') => {
                    match op_mul {
                        Some('*') => prev *= curr,
                        Some('/') => prev /= curr,
                        _ => prev = curr,
                    }
                    curr = 0;
                    op_mul = Some(op);
                }
                ' ' => (),
                d => curr = curr * 10 + d.to_digit(10).unwrap() as i32,
            }
        }

        match op_mul {
            Some('*') => curr *= prev,
            Some('/') => curr = prev / curr,
            _ => (),
        }
        match op_sum {
            Some('-') => result -= curr,
            _ => result += curr,
        }

        result
    }

    /// for leetcode as or-patters syntax is still experimental in their rust compiler
    pub fn calculate_my_2(s: String) -> i32 {
        let mut result = 0;
        let mut prev = 0;
        let mut curr = 0;
        let mut op_sum: Option<char> = None;
        let mut op_mul: Option<char> = None;
        for c in s.chars() {
            if c == '+' || c == '-' {
                match op_mul {
                    Some('*') => curr *= prev,
                    Some('/') => curr = prev / curr,
                    _ => (),
                }
                op_mul = None;

                match op_sum {
                    Some('-') => result -= curr,
                    _ => result += curr,
                }
                curr = 0;
                op_sum = Some(c);
            } else if c == '*' || c == '/' {
                match op_mul {
                    Some('*') => prev *= curr,
                    Some('/') => prev /= curr,
                    _ => prev = curr,
                }
                curr = 0;
                op_mul = Some(c);
            } else if c == ' ' {
            } else {
                curr = curr * 10 + c.to_digit(10).unwrap() as i32;
            }
        }

        match op_mul {
            Some('*') => curr *= prev,
            Some('/') => curr = prev / curr,
            _ => (),
        }
        match op_sum {
            Some('-') => result -= curr,
            _ => result += curr,
        }

        result
    }

    /// from year ago
    pub fn calculate(s: String) -> i32 {
        let (rsf, a, op, b) = s.chars().fold((0, 0, '+', 0), |(rsf, a, op, b), ch| {
            if let Some(d) = ch.to_digit(10) {
                (rsf, a, op, b * 10 + d as i32)
            } else if ch != ' ' {
                match op {
                    '+' => (rsf + a, b, ch, 0),
                    '-' => (rsf + a, -b, ch, 0),
                    '*' => (rsf, a * b, ch, 0),
                    '/' => (rsf, a / b, ch, 0),
                    _ => panic!("Unexpected char: {}", ch),
                }
            } else {
                (rsf, a, op, b)
            }
        });

        match op {
            '+' => rsf + a + b,
            '-' => rsf + a - b,
            '*' => rsf + a * b,
            '/' => rsf + a / b,
            ch => panic!("Unexpected char: {}", ch),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn s_7() { assert_eq!(Solution::calculate("7".into()), 7); }
    #[rustfmt::skip] #[test] fn s_1p2() { assert_eq!(Solution::calculate("1+2".into()), 3); }
    #[rustfmt::skip] #[test] fn s_1m2() { assert_eq!(Solution::calculate("1-2".into()), -1); }
    #[rustfmt::skip] #[test] fn s_2t3() { assert_eq!(Solution::calculate("2*3".into()), 6); }
    #[rustfmt::skip] #[test] fn s_2d3() { assert_eq!(Solution::calculate("2/3".into()), 0); }

    #[test]
    fn s_3p2t2() {
        let s = "3+2*2".to_string();
        assert_eq!(Solution::calculate(s), 7);
    }
    #[test]
    fn s_s3d2s() {
        let s = " 3/2 ".to_string();
        assert_eq!(Solution::calculate(s), 1);
    }
    #[test]
    fn s_s3p5sds2s() {
        let s = " 3+5 / 2 ".to_string();
        assert_eq!(Solution::calculate(s), 5);
    }

    #[test]
    fn s_1p1t2t3t4d3p7m19() {
        let s = "1+1*2*3*4/3+7-19".to_string();
        assert_eq!(Solution::calculate(s), -3);
    }
}
