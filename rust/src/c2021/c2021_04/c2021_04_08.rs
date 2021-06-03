#![allow(dead_code)]
/// Letter Combinations of a Phone Number
/// =====================================
///
/// Given a string containing digits from `2-9` inclusive,
/// return all possible letter combinations that the number could represent.
/// Return the answer in __any order__.
///
/// A mapping of digit to letters (just like on the telephone buttons) is given below.
/// Note that 1 does not map to any letters.
///
/// ```text
/// 2: abc
/// 3: def
/// 4: ghi
/// 5: jkl
/// 6: mno
/// 7: pqrs
/// 8: tuv
/// 9: wxyz
/// ```
///
/// __Constraints:__
///
/// - `0 <= digits.length <= 4`
/// - `digits[i]` is a digit in the range `['2', '9']`.
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/594/week-2-april-8th-april-14th/3701/
struct Solution;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }

        fn digit_to_letters(digit: char) -> &'static str {
            match digit {
                '2' => "abc",
                '3' => "def",
                '4' => "ghi",
                '5' => "jkl",
                '6' => "mno",
                '7' => "pqrs",
                '8' => "tuv",
                '9' => "wxyz",
                _ => unreachable!(),
            }
        }

        digits.chars().fold(vec!["".to_string()], |rsf, digit| {
            rsf.into_iter()
                .flat_map(|prev_s| {
                    digit_to_letters(digit)
                        .chars()
                        .map(|l| {
                            let mut s = prev_s.clone();
                            s.push(l);
                            s
                        })
                        .collect::<Vec<_>>()
                })
                .collect()
        })
    }

    pub fn letter_combinations_iteration(digits: String) -> Vec<String> {
        fn digit_to_letters(digit: u8) -> Vec<u8> {
            match digit {
                b'2' => vec![b'a', b'b', b'c'],
                b'3' => vec![b'd', b'e', b'f'],
                b'4' => vec![b'g', b'h', b'i'],
                b'5' => vec![b'j', b'k', b'l'],
                b'6' => vec![b'm', b'n', b'o'],
                b'7' => vec![b'p', b'q', b'r', b's'],
                b'8' => vec![b't', b'u', b'v'],
                b'9' => vec![b'w', b'x', b'y', b'z'],
                _ => unreachable!(),
            }
        }

        let mut result: Vec<Vec<u8>> = Vec::new();
        for digit in digits.bytes() {
            if result.is_empty() {
                for letter in digit_to_letters(digit) {
                    result.push(vec![letter])
                }
            } else {
                let rsf: Vec<Vec<u8>> = result.drain(..).collect();
                for prev_s in rsf {
                    for letter in digit_to_letters(digit) {
                        let mut s = prev_s.clone();
                        s.push(letter);
                        result.push(s);
                    }
                }
            }
        }

        result
            .into_iter()
            .map(|bs| String::from_utf8(bs).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_d_23() {
        let e = ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        assert_eq!(Solution::letter_combinations("23".to_string()), e);
    }
    #[test]
    fn example2_d_empty() {
        let e: Vec<String> = Vec::new();
        assert_eq!(Solution::letter_combinations("".to_string()), e);
    }
    #[test]
    fn example3_d_2() {
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            ["a", "b", "c"]
        );
    }

    #[test]
    fn example3_d_7777() {
        assert_eq!(Solution::letter_combinations("7777".to_string()).len(), 256);
    }
}
