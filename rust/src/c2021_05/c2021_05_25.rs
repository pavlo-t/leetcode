#![allow(dead_code)]
/// Evaluate Reverse Polish Notation
/// ================================
///
/// Evaluate the value of an arithmetic expression in [Reverse Polish Notation].
///
/// Valid operators are `+`, `-`, `*`, and `/`.
/// Each operand may be an integer or another expression.
///
/// __Note__ that division between two integers should truncate toward zero.
///
/// It is guaranteed that the given RPN expression is always valid.
/// That means the expression would always evaluate to a result,
/// and there will not be any division by zero operation.
///
/// __Constraints:__
///
/// - `1 <= tokens.length <= 10_000`
/// - `tokens[i]` is either an operator: `"+"`, `"-"`, `"*"`, or `"/"`, or an integer in the range `[-200, 200]`.
///
/// [Reverse Polish Notation]:http://en.wikipedia.org/wiki/Reverse_Polish_notation
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/601/week-4-may-22nd-may-28th/3755/
struct Solution;
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        tokens.into_iter().for_each(|s| match s.parse::<i32>() {
            Ok(n) => stack.push(n),
            _ => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                match s.as_bytes()[0] {
                    b'+' => stack.push(a + b),
                    b'-' => stack.push(a - b),
                    b'*' => stack.push(a * b),
                    b'/' => stack.push(a / b),
                    _ => unreachable!(),
                }
            }
        });
        stack[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($s:tt),*) => {vec![$($s.to_string()),*]};}

    #[test]
    fn example1_tokens_2_1_plus_3_mul_produce_9() {
        let tokens = vs!["2", "1", "+", "3", "*"];
        assert_eq!(Solution::eval_rpn(tokens), 9);
        // Explanation: ((2 + 1) * 3) = 9
    }
    #[test]
    fn example2_tokens_4_13_5_div_plus_produce_6() {
        let tokens = vs!["4", "13", "5", "/", "+"];
        assert_eq!(Solution::eval_rpn(tokens), 6);
        // Explanation: (4 + (13 / 5)) = 6
    }
    #[test]
    fn example3_tokens_10_6_9_3_plus_m11_mul_div_mul_17_plus_5_plus_produce_22() {
        let tokens = vs!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"];
        assert_eq!(Solution::eval_rpn(tokens), 22);
        // Explanation:
        //   ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
        // = ((10 * (6 / (12      * -11))) + 17) + 5
        // = ((10 * (6 / -132           )) + 17) + 5
        // = ((10 * 0                    ) + 17) + 5
        // = (0 +                            17) + 5
        // = 17 +                                  5
        // = 22
    }

    #[test]
    fn tokens_42_produce_42() {
        let tokens = vs!["42"];
        assert_eq!(Solution::eval_rpn(tokens), 42);
    }
}
