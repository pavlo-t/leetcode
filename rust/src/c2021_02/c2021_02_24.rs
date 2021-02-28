#![allow(dead_code)]

/// # Score of Parentheses
///
/// Given a balanced parentheses string `S`,
/// compute the score of the string based on the following rule:
///
/// - `()` has score 1
/// - `AB` has score `A + B`, where `A` and `B` are balanced parentheses strings.
/// - `(A)` has score `2 * A`, where `A` is a balanced parentheses string.
///
/// __Note:__
///
/// 1. `S` is a balanced parentheses string, containing only `(` and `)`.
/// 2. `2 <= S.length <= 50`
///
/// https://leetcode.com/explore/featured/card/february-leetcoding-challenge-2021/587/week-4-february-22nd-february-28th/3651/
struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut score = 0;
        let mut parens = 0;
        let bytes = s.as_bytes();

        for (i, &c) in bytes.iter().enumerate() {
            if c == b'(' {
                parens += 1;
            } else {
                parens -= 1;
                if bytes[i - 1] == b'(' {
                    score += 1 << parens;
                }
            }
        }

        score
    }

    pub fn score_of_parentheses_divide_and_conquer(s: String) -> i32 {
        fn score_rec(s: &str) -> i32 {
            let mut parens = 0;
            let left = s
                .chars()
                .take_while(|c| {
                    match c {
                        '(' => parens += 1,
                        ')' => parens -= 1,
                        _ => (),
                    }
                    parens > 0
                })
                .collect::<String>();

            let left_score = if left == "(" {
                1
            } else {
                2 * score_rec(&left[1..])
            };

            if left.len() + 1 < s.len() {
                left_score + score_rec(&s[left.len() + 1..])
            } else {
                left_score
            }
        }

        score_rec(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_p_oc_should_be_1() {
        assert_eq!(Solution::score_of_parentheses("()".to_string()), 1);
    }
    #[test]
    fn example2_p_oocc_should_be_2() {
        assert_eq!(Solution::score_of_parentheses("(())".to_string()), 2);
    }
    #[test]
    fn example3_p_ococ_should_be_2() {
        assert_eq!(Solution::score_of_parentheses("()()".to_string()), 2);
    }
    #[test]
    fn example4_p_oocooccc_should_be_6() {
        assert_eq!(Solution::score_of_parentheses("(()(()))".to_string()), 6);
    }

    #[test]
    fn p_oocoocccoc_should_be_7() {
        assert_eq!(Solution::score_of_parentheses("(()(()))()".to_string()), 7);
    }
    #[test]
    fn p_25o25c_should_be_16777216() {
        let mut s = "(".repeat(25);
        s.push_str(&")".repeat(25));
        assert_eq!(Solution::score_of_parentheses(s), 16777216);
    }
}
