#![allow(dead_code)]
/// Valid Number
/// ============
///
/// A __valid number__ can be split up into these components (in order):
///
/// 1. A __decimal number__ or an __integer__.
/// 2. (Optional) An `'e'` or `'E'`, followed by an __integer__.
///
/// A __decimal number__ can be split up into these components (in order):
///
/// 1. (Optional) A sign character (either `'+'` or `'-'`).
/// 2. One of the following formats:
///    1. At least one digit, followed by a dot `'.'`.
///    2. At least one digit, followed by a dot `'.'`, followed by at least one digit.
///    3. A dot `'.'`, followed by at least one digit.
///
/// An __integer__ can be split up into these components (in order):
///
/// 1. (Optional) A sign character (either `'+'` or `'-'`).
/// 2. At least one digit.
///
/// For example, all the following are valid numbers:
/// `["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"]`,
/// while the following are not valid numbers:
/// `["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"]`.
///
/// Given a string `s`, return `true` _if `s` is a __valid number___.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 20`
/// - `s` consists of only English letters (both uppercase and lowercase), digits (`0-9`),
///   plus `'+'`, minus `'-'`, or dot `'.'`.
///
/// https://leetcode.com/explore/featured/card/may-leetcoding-challenge-2021/600/week-3-may-15th-may-21st/3744/
struct Solution;
impl Solution {
    // Approach 2: Deterministic Finite Automaton (DFA):
    // O(n) time, O(1) space
    //
    // https://leetcode.com/problems/valid-number/solution/
    // pub fn is_number(s: String) -> bool { false }

    /// Approach 1: Follow The Rules!:  
    /// O(n) time, O(1) space
    pub fn is_number(s: String) -> bool {
        let mut seen_digit = false;
        let mut seen_exponent = false;
        let mut seen_dot = false;

        let mut prev = 'e';
        for c in s.chars() {
            if c.is_ascii_digit() {
                seen_digit = true;
            } else if c == '+' || c == '-' {
                if prev.to_ascii_lowercase() != 'e' {
                    return false;
                }
            } else if c.to_ascii_lowercase() == 'e' {
                if seen_exponent || !seen_digit {
                    return false;
                }
                seen_exponent = true;
                seen_digit = false;
            } else if c == '.' {
                if seen_dot || seen_exponent {
                    return false;
                }
                seen_dot = true;
            } else {
                return false;
            }
            prev = c;
        }

        seen_digit
    }

    pub fn is_number_my(s: String) -> bool {
        fn remove_sign(s: &str) -> &str {
            if s.starts_with(&['-', '+'][..]) {
                &s[1..]
            } else {
                s
            }
        }
        fn is_integer(s: &str) -> bool {
            let s = remove_sign(s);
            s.len() > 0 && s.bytes().all(|b| b.is_ascii_digit())
        }
        fn is_decimal(s: &str) -> bool {
            let s = remove_sign(s);
            let dot_split = s.split('.').collect::<Vec<_>>();
            if dot_split.len() == 2 {
                let (l, r) = (dot_split[0], dot_split[1]);
                (l.len() > 0 || r.len() > 0)
                    && l.bytes().all(|b| b.is_ascii_digit())
                    && r.bytes().all(|b| b.is_ascii_digit())
            } else {
                false
            }
        }

        let e_split = s.split(&['e', 'E'][..]).collect::<Vec<_>>();
        if e_split.len() == 2 {
            let (n, e) = (e_split[0], e_split[1]);
            (is_integer(n) || is_decimal(n)) && is_integer(e)
        } else if e_split.len() == 1 {
            let n = e_split[0];
            is_integer(n) || is_decimal(n)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs { ($($x:tt),*) => { vec![$($x.to_string()),*] }; }

    #[test]
    fn example1_s_0_is_valid() {
        assert!(Solution::is_number("0".to_string()));
    }
    #[test]
    fn example2_s_e_is_not_valid() {
        assert!(!Solution::is_number("e".to_string()));
    }
    #[test]
    fn example3_s_dot_is_not_valid() {
        assert!(!Solution::is_number(".".to_string()));
    }
    #[test]
    fn example4_s_dot1_is_valid() {
        assert!(Solution::is_number(".1".to_string()));
    }

    #[test]
    fn test_from_doc() {
        vs![
            "2",
            "0089",
            "-0.1",
            "+3.14",
            "4.",
            "-.9",
            "2e10",
            "-90E3",
            "3e+7",
            "+6e-1",
            "53.5e93",
            "-123.456e789"
        ]
        .into_iter()
        .for_each(|s| assert!(Solution::is_number(s.clone()), "{} should be valid", s));

        vs!["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"]
            .into_iter()
            .for_each(|s| assert!(!Solution::is_number(s.clone()), "{} should not be valid", s));
    }
}
