#![allow(dead_code)]
/// \#32. Longest Valid Parentheses
/// ===============================
///
/// Given a string containing just the characters `'('` and `')'`,
/// find the length of the longest valid (well-formed) parentheses substring.
///
/// __Constraints:__
///
/// - `0 <= s.length <= 30_000`
/// - `s[i]` is `'('`, or `')'`.
///
/// https://leetcode.com/problems/longest-valid-parentheses/
struct Solution;
impl Solution {
    /// From older solution `rust/2020_2021-09/src/c2021/c2021_04/c2021_04_03.rs`
    pub fn longest_valid_parentheses(s: String) -> i32 {
        fn check<B, I>(bytes: B, invalid: I) -> i32
        where
            B: Iterator<Item = u8>,
            I: Fn(i32, i32) -> bool,
        {
            bytes.fold((0, 0, 0), |(result, open, close), b| {
                let (open, close) = match b {
                    b'(' => (open + 1, close),
                    b')' => (open, close + 1),
                    _ => unreachable!(),
                };
                if open == close {
                    (result.max(open + close), open, close)
                } else if invalid(open, close) {
                    (result, 0, 0)
                } else {
                    (result, open, close)
                }
            }).0
        }

        check(s.bytes(), |o, c| c > o).max(check(s.bytes().rev(), |o, c| o > c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn empty() { assert_eq!(Solution::longest_valid_parentheses("".into()), 0); }
    #[rustfmt::skip] #[test] fn co() { assert_eq!(Solution::longest_valid_parentheses(")(".into()), 0); }
    #[rustfmt::skip] #[test] fn oc() { assert_eq!(Solution::longest_valid_parentheses("()".into()), 2); }

    #[rustfmt::skip] #[test] fn ooo() { assert_eq!(Solution::longest_valid_parentheses("(((".into()), 0); }
    #[rustfmt::skip] #[test] fn ccc() { assert_eq!(Solution::longest_valid_parentheses(")))".into()), 0); }

    #[rustfmt::skip] #[test] fn oocc() { assert_eq!(Solution::longest_valid_parentheses("))((".into()), 0); }

    #[test]
    fn ooc() {
        assert_eq!(Solution::longest_valid_parentheses("(()".into()), 2);
        // Explanation: The longest valid parentheses substring is "()".
    }
    #[test]
    fn cococc() {
        assert_eq!(Solution::longest_valid_parentheses(")()())".into()), 4);
        // Explanation: The longest valid parentheses substring is "()()".
    }
    #[test]
    fn coococc() {
        assert_eq!(Solution::longest_valid_parentheses(")(()())".into()), 6);
    }

    #[rustfmt::skip] #[test] fn cx30000() { assert_eq!(Solution::longest_valid_parentheses("(".repeat(30000)), 0); }
}
