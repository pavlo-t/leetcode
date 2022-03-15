#![allow(dead_code)]
/// 1249. Minimum Remove to Make Valid Parentheses
/// ==============================================
///
/// Given a string s of `'('`, `')'` and lowercase English characters.
///
/// Your task is to remove the minimum number of parentheses (`'('` or `')'`, in any positions)
/// so that the resulting _parentheses string_ is valid and return __any__ valid string.
///
/// Formally, a _parentheses string_ is valid if and only if:
///
/// - It is the empty string, contains only lowercase characters, or
/// - It can be written as `AB` (`A` concatenated with `B`), where `A` and `B` are valid strings, or
/// - It can be written as `(A)`, where `A` is a valid string.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 100_000`
/// - `s[i]` is either `'('`, `')'`, or lowercase English letter.
///
/// https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/
struct Solution;
impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut closed = 0;
        let s = s
            .chars()
            .rev()
            .filter(|c| match c {
                ')' => {
                    closed += 1;
                    true
                }
                '(' if closed == 0 => false,
                '(' => {
                    closed -= 1;
                    true
                }
                _ => true,
            })
            .collect::<String>();

        let mut open = 0;
        s.chars()
            .rev()
            .filter(|c| match c {
                '(' => {
                    open += 1;
                    true
                }
                ')' if open == 0 => false,
                ')' => {
                    open -= 1;
                    true
                }
                _ => true,
            })
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lee0t0c1o1de1() {
        let s = "lee(t(c)o)de)".to_string();
        assert_eq!(Solution::min_remove_to_make_valid(s), "lee(t(c)o)de");
        // Explanation: "lee(t(co)de)" , "lee(t(c)ode)" would also be accepted.
    }
    #[test]
    fn a1b0c1d() {
        let s = "a)b(c)d".to_string();
        assert_eq!(Solution::min_remove_to_make_valid(s), "ab(c)d");
    }
    #[test]
    fn _1100() {
        let s = "))((".to_string();
        assert_eq!(Solution::min_remove_to_make_valid(s), "");
        // Explanation: An empty string is also valid.
    }
}
