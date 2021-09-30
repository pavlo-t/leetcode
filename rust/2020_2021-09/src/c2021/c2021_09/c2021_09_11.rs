#![allow(dead_code)]
/// Basic Calculator
/// ================
///
/// Given a string `s` representing a valid expression,
/// implement a basic calculator to evaluate it,
/// and return _the result of the evaluation_.
///
/// __Note:__ You are __not__ allowed to use any built-in function which evaluates strings as mathematical expressions,
/// such as `eval()`.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 300_000`
/// - `s` consists of digits, `'+'`, `'-'`, `'('`, `')'`, and `' '`.
/// - `s` represents a valid expression.
/// - `'+'` is not used as a unary operation.
/// - `'-'` could be used as a unary operation but it has to be inside parentheses.
/// - There will be no two consecutive operators in the input.
/// - Every number and running calculation will fit in a signed 32-bit integer.
///
///
/// https://leetcode.com/explore/challenge/card/september-leetcoding-challenge-2021/637/week-2-september-8th-september-14th/3971/
struct Solution;
impl Solution {
    pub fn calculate(s: String) -> i32 {
        #[derive(Debug)]
        enum Sym {
            N(i32),
            P, // +
            M, // -
            O, // (
        }
        use Sym::{M, N, O, P};

        fn push_n(n: &mut Option<i32>, stack: &mut Vec<Sym>) {
            if let Some(n) = n.take() {
                match stack.last() {
                    Some(P) => {
                        stack.pop();
                        if let Some(N(p)) = stack.pop() {
                            stack.push(N(p + n));
                        }
                    }
                    Some(M) => {
                        stack.pop();
                        match stack.pop() {
                            Some(N(p)) => stack.push(N(p - n)),
                            Some(p) => {
                                stack.push(p);
                                stack.push(N(-n));
                            }
                            _ => stack.push(N(-n)),
                        }
                    }
                    _ => stack.push(N(n)),
                };
            }
        }
        fn push_c(stack: &mut Vec<Sym>) {
            match stack.pop() {
                Some(O) => (),
                Some(N(n)) => {
                    stack.pop();
                    push_n(&mut Some(n), stack);
                }
                _ => unreachable!(),
            }
        }

        let mut stack = vec![];
        let mut chars = s.chars();
        let mut n = None;
        while let Some(c) = chars.next() {
            if let Some(d) = c.to_digit(10) {
                n = n.map(|n| n * 10 + d as i32).or(Some(d as i32));
            } else {
                push_n(&mut n, &mut stack);
                match c {
                    '+' => stack.push(P),
                    '-' => stack.push(M),
                    '(' => stack.push(O),
                    ')' => push_c(&mut stack),
                    _ => (),
                };
            }
        }
        push_n(&mut n, &mut stack);

        if let Some(N(n)) = stack.pop() {
            n
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // p='+' m='-' o='(' c=')' s=' '
    #[test]
    fn s_1p1_produces_2() {
        assert_eq!(Solution::calculate("1 + 1".to_string()), 2);
    }
    #[test]
    fn s_2m1p2_produces_3() {
        assert_eq!(Solution::calculate(" 2-1 + 2 ".to_string()), 3);
    }
    #[test]
    fn s_o1po4p5p2cm3cpo6p8c_produces_23() {
        assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
    }
    #[test]
    fn s_m12pomom3cc_produces_m9() {
        assert_eq!(Solution::calculate("-12+(-(-3))".to_string()), -9);
    }
    #[test]
    fn s_ooo1ccc_produces_1() {
        assert_eq!(Solution::calculate("(((1)))".to_string()), 1);
    }
    #[test]
    fn s_ooom1ccc_produces_m1() {
        assert_eq!(Solution::calculate("(((-1)))".to_string()), -1);
    }
    #[test]
    fn s_omoo1ccc_produces_m1() {
        assert_eq!(Solution::calculate("(-((1)))".to_string()), -1);
    }
    #[test]
    fn s_ooo1ccoccoc_produces_1() {
        assert_eq!(Solution::calculate("(((1))())()".to_string()), 1);
    }
    #[test]
    fn s_oocc_produces_0() {
        assert_eq!(Solution::calculate("(())".to_string()), 0);
    }
    #[test]
    fn s_2mo5m6c_produces_3() {
        assert_eq!(Solution::calculate("2-(5-6)".to_string()), 3);
    }
}
