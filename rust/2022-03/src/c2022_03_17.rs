#![allow(dead_code)]
/// 856. Score of Parentheses
/// =========================
///
/// Given a balanced parentheses string `s`, return _the __score__ of the string_.
///
/// The __score__ of a balanced parentheses string is based on the following rule:
///
/// - `"()"` has score `1`.
/// - `AB` has score `A + B`, where `A` and `B` are balanced parentheses strings.
/// - `(A)` has score `2 * A`, where `A` is a balanced parentheses string.
///
/// __Constraints:__
///
/// - `2 <= s.length <= 50`
/// - `s` consists of only `'('` and `')'`.
/// - `s` is a balanced parentheses string.
///
/// https://leetcode.com/problems/score-of-parentheses/
struct Solution;
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        fn closing_idx(s: &str) -> usize {
            let mut opened = 1;
            for (i, c) in s.chars().enumerate().skip(1) {
                match c {
                    '(' => opened += 1,
                    ')' if opened == 1 => return i,
                    _ => opened -= 1,
                }
            }
            unreachable!()
        }

        fn rec(s: &str) -> i32 {
            if s == "()" {
                1
            } else {
                let ci = closing_idx(s);
                if ci == s.len() - 1 {
                    rec(&s[1..ci]) * 2
                } else {
                    rec(&s[..=ci]) + rec(&s[ci + 1..])
                }
            }
        }

        rec(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oc() {
        assert_eq!(Solution::score_of_parentheses("()".into()), 1);
    }
    #[test]
    fn oocc() {
        assert_eq!(Solution::score_of_parentheses("(())".into()), 2);
    }
    #[test]
    fn ococ() {
        assert_eq!(Solution::score_of_parentheses("()()".into()), 2);
    }
}
