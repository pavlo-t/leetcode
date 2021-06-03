#![allow(dead_code)]

/// # Letter Case Permutation
///
/// Given a string S, we can transform every letter individually to be lowercase or uppercase
/// to create another string.
///
/// Return _a list of all possible strings we could create_.
/// You can return the output in __any order__.
///
/// __Constraints:__
///
/// - `S` will be a string with length between `1` and `12`.
/// - `S` will consist only of letters or digits.
///
/// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/586/week-3-february-15th-february-21st/3642/
struct Solution;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let letters = s.chars().filter(|c| c.is_alphabetic()).count();
        let max = (1 << letters) - 1;
        let mut result = Vec::new();

        fn apply_mask(mut m: i32, s: &str) -> String {
            m <<= 1;
            s.chars().map(|c| {
                if c.is_digit(10) { c } else {
                    m >>= 1;
                    if (m & 1) == 0 {
                        c.to_ascii_lowercase()
                    } else {
                        c.to_ascii_uppercase()
                    }
                }
            }).collect()
        }

        for m in 0..=max {
            result.push(apply_mask(m, &s));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_a1b2() {
        let e = ["a1b2", "A1b2", "a1B2", "A1B2"];
        assert_eq!(Solution::letter_case_permutation("a1b2".to_string()), e);
        assert_eq!(Solution::letter_case_permutation("A1B2".to_string()), e);
    }

    #[test]
    fn example2_3z4() {
        let e = ["3z4", "3Z4"];
        assert_eq!(Solution::letter_case_permutation("3z4".to_string()), e);
    }

    #[test]
    fn example3_12345() {
        assert_eq!(Solution::letter_case_permutation("12345".to_string()), ["12345"]);
    }

    #[test]
    fn example4_0() {
        assert_eq!(Solution::letter_case_permutation("0".to_string()), ["0"]);
    }

    #[test]
    fn s_a_repeat_12() {
        let result = Solution::letter_case_permutation("a".repeat(12));
        assert_eq!(result.len(), 4096);
    }
}
