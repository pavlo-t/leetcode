#![allow(dead_code)]
/// 20. Valid Parentheses
/// =====================
///
/// Given a string `s` containing just the characters `'('`, `')'`, `'{'`, `'}'`, `'['` and `']'`,
/// determine if the input string is valid.
///
/// An input string is valid if:
///
/// - Open brackets must be closed by the same type of brackets.
/// - Open brackets must be closed in the correct order.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 10_000`
/// - `s` consists of parentheses only `'()[]{}'`.
///
/// https://leetcode.com/problems/valid-parentheses/
struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' if stack.pop() == Some('(') => (),
                ']' if stack.pop() == Some('[') => (),
                '}' if stack.pop() == Some('{') => (),
                _ => {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ro() {
        let s = "(".to_string();
        assert_eq!(Solution::is_valid(s), false);
    }
    #[test]
    fn ro_rc() {
        let s = "()".to_string();
        assert_eq!(Solution::is_valid(s), true);
    }
    #[test]
    fn ro_rc_so_sc_co_cc() {
        let s = "()[]{}".to_string();
        assert_eq!(Solution::is_valid(s), true);
    }
    #[test]
    fn ro_sc() {
        let s = "(]".to_string();
        assert_eq!(Solution::is_valid(s), false);
    }
    #[test]
    fn ro_so_rc_sc() {
        let s = "([)]".to_string();
        assert_eq!(Solution::is_valid(s), false);
    }
}
