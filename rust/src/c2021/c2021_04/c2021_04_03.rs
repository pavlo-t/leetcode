#![allow(dead_code)]
/// Longest Valid Parentheses
/// =========================
///
/// Given a string containing just the characters `'('` and `')'`,
/// find the length of the longest valid (well-formed) parentheses substring.
///
/// __Constraints:__
///
/// - `0 <= s.length <= 30_000`
/// - `s[i]` is `'('`, or `')'`.
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/593/week-1-april-1st-april-7th/3695/
struct Solution;
impl Solution {
    fn longest_valid_parentheses(s: String) -> i32 {
        fn check<B, I>(bytes: B, invalid: I) -> i32
        where
            B: Iterator<Item = u8>,
            I: Fn(i32, i32) -> bool,
        {
            let mut open = 0;
            let mut close = 0;
            let mut result = 0;
            for b in bytes {
                match b {
                    b'(' => open += 1,
                    b')' => close += 1,
                    _ => unreachable!(),
                }
                if open == close {
                    result = result.max(open + close);
                } else if invalid(open, close) {
                    open = 0;
                    close = 0;
                }
            }
            result
        }

        check(s.bytes(), |o, c| c > o).max(check(s.bytes().rev(), |o, c| o > c))
    }

    fn longest_valid_parentheses_stack(s: String) -> i32 {
        let mut ops = vec![-1];
        let mut result = 0;
        for (r, b) in s.bytes().enumerate() {
            match b {
                b'(' => ops.push(r as i32),
                b')' => {
                    ops.pop();
                    if let Some(l) = ops.last() {
                        result = result.max(r as i32 - l);
                    } else {
                        ops.push(r as i32)
                    }
                }
                _ => unreachable!(),
            }
        }
        result
    }

    fn longest_valid_parentheses_better_brute_force(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        fn is_valid(s: &[u8]) -> bool {
            let mut open_parens = 0;
            for b in s {
                match b {
                    b'(' => open_parens += 1,
                    b')' if open_parens == 0 => return false,
                    b')' => open_parens -= 1,
                    _ => unreachable!(),
                }
            }
            open_parens == 0
        }
        let mut result = 0;
        let bytes = s.as_bytes();
        for size in (2..=bytes.len()).step_by(2) {
            for w in bytes.windows(size) {
                if is_valid(w) {
                    result = size;
                }
            }
        }
        result as i32
    }
    fn longest_valid_parentheses_brute_force(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        fn is_valid(s: &[u8]) -> bool {
            let mut open_parens = 0;
            for b in s {
                match b {
                    b'(' => open_parens += 1,
                    b')' if open_parens == 0 => return false,
                    b')' => open_parens -= 1,
                    _ => unreachable!(),
                }
            }
            open_parens == 0
        }

        let bytes = s.as_bytes();
        for size in (1..=bytes.len()).rev() {
            for w in bytes.windows(size) {
                if is_valid(w) {
                    return size as i32;
                }
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s = "(()".to_string();
        assert_eq!(Solution::longest_valid_parentheses(s), 2);
        // Explanation: The longest valid parentheses substring is "()".
    }
    #[test]
    fn example2() {
        let s = ")()())".to_string();
        assert_eq!(Solution::longest_valid_parentheses(s), 4);
        // Explanation: The longest valid parentheses substring is "()()".
    }
    #[test]
    fn example3() {
        let s = "".to_string();
        assert_eq!(Solution::longest_valid_parentheses(s), 0);
    }

    #[test]
    fn s_occococ_is_4() {
        let s = "())()()".to_string();
        assert_eq!(Solution::longest_valid_parentheses(s), 4);
    }
    #[test]
    fn s_ococcoc_is_4() {
        let s = "()())()".to_string();
        assert_eq!(Solution::longest_valid_parentheses(s), 4);
    }
    #[test]
    fn s_oooccc_is_6() {
        let s = "((()))".to_string();
        assert_eq!(Solution::longest_valid_parentheses(s), 6);
    }

    mod performance {
        use super::*;

        #[test]
        fn s_30k_produces_30k() {
            let s = "(".repeat(15_000) + &")".repeat(15_000);
            assert_eq!(Solution::longest_valid_parentheses(s), 30_000);
        }
        #[test]
        fn s_30k_produces_0() {
            let s = "(".repeat(30_000);
            assert_eq!(Solution::longest_valid_parentheses(s), 0);
        }
    }
}
