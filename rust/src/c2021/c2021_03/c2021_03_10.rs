#![allow(dead_code)]

/// # Integer to Roman
///
/// Roman numerals are represented by seven different symbols: `I`, `V`, `X`, `L`, `C`, `D` and `M`.
///
/// ```text
/// Symbol       Value
/// I             1
/// V             5
/// X             10
/// L             50
/// C             100
/// D             500
/// M             1000
/// ```
///
/// For example, `2` is written as `II` in Roman numeral, just two one's added together.
/// `12` is written as `XII`, which is simply `X + II`.
/// The number `27` is written as `XXVII`, which is `XX + V + II`.
///
/// Roman numerals are usually written largest to smallest from left to right.
/// However, the numeral for four is not `IIII`.
/// Instead, the number four is written as `IV`.
/// Because the one is before the five we subtract it making four.
/// The same principle applies to the number nine, which is written as `IX`.
/// There are six instances where subtraction is used:
///
/// - `I` can be placed before `V` (5) and `X` (10) to make 4 and 9.
/// - `X` can be placed before `L` (50) and `C` (100) to make 40 and 90.
/// - `C` can be placed before `D` (500) and `M` (1000) to make 400 and 900.
///
/// Given an integer, convert it to a roman numeral.
///
/// __Constraints:__
///
/// - `1 <= num <= 3999`
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/589/week-2-march-8th-march-14th/3667/
struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut result = String::new();
        while num > 0 {
            match num {
                m if m >= 1000 => {
                    result.push('M');
                    num -= 1000;
                }
                cm if cm >= 900 => {
                    result.push_str("CM");
                    num -= 900;
                }
                d if d >= 500 => {
                    result.push('D');
                    num -= 500;
                }
                cd if cd >= 400 => {
                    result.push_str("CD");
                    num -= 400;
                }

                c if c >= 100 => {
                    result.push('C');
                    num -= 100;
                }
                xc if xc >= 90 => {
                    result.push_str("XC");
                    num -= 90;
                }
                l if l >= 50 => {
                    result.push('L');
                    num -= 50;
                }
                xl if xl >= 40 => {
                    result.push_str("XL");
                    num -= 40;
                }

                x if x >= 10 => {
                    result.push('X');
                    num -= 10;
                }
                ix if ix >= 9 => {
                    result.push_str("IX");
                    num -= 9;
                }
                v if v >= 5 => {
                    result.push('V');
                    num -= 5;
                }
                iv if iv >= 4 => {
                    result.push_str("IV");
                    num -= 4;
                }

                _ => {
                    result.push('I');
                    num -= 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
//noinspection RsFunctionNaming
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::int_to_roman(3), "III");
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::int_to_roman(4), "IV");
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::int_to_roman(9), "IX");
    }
    #[test]
    fn example4() {
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        // Explanation: L = 50, V = 5, III = 3.
    }
    #[test]
    fn example5() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
        // Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
    }

    #[test]
    fn i3999_produces_mmmcmxcix() {
        assert_eq!(Solution::int_to_roman(3999), "MMMCMXCIX");
    }
    #[test]
    fn i555_produces_dlv() {
        assert_eq!(Solution::int_to_roman(555), "DLV");
    }
}
