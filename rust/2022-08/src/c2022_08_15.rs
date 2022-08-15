#![allow(dead_code)]
//! \#13. Roman to Integer
//! ======================
//!
//! <https://leetcode.com/problems/roman-to-integer>
//!
//! Roman numerals are represented by seven different symbols: `I`, `V`, `X`, `L`, `C`, `D` and `M`.
//!
//! |Symbol|Value|
//! |-----:|----:|
//! |    I |    1|
//! |    V |    5|
//! |    X |   10|
//! |    L |   50|
//! |    C |  100|
//! |    D |  500|
//! |    M | 1000|
//!
//! For example, `2` is written as `II` in Roman numeral, just two ones added together.
//! `12` is written as `XII`, which is simply `X + II`.
//! The number `27` is written as `XXVII`, which is `XX + V + II`.
//!
//! Roman numerals are usually written largest to smallest from left to right.
//! However, the numeral for four is not `IIII`.
//! Instead, the number four is written as `IV`.
//! Because the one is before the five we subtract it making four.
//! The same principle applies to the number nine, which is written as `IX`.
//! There are six instances where subtraction is used:
//!
//! - `I` can be placed before `V` (`5`) and `X` (`10`) to make `4` and `9`.
//! - `X` can be placed before `L` (`50`) and `C` (`100`) to make `40` and `90`.
//! - `C` can be placed before `D` (`500`) and `M` (`1000`) to make `400` and `900`.
//!
//! Given a roman numeral, convert it to an integer.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_08::c2022_08_15::*;
//! assert_eq!(Solution::roman_to_int("III".into()), 3);
//! ```
//!
//! __Explanation:__ `III = 3`.
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_15::*;
//! assert_eq!(Solution::roman_to_int("LVIII".into()), 58);
//! ```
//!
//! __Explanation:__ `L = 50`, `V = 5`, `III = 3`.
//!
//! ###### Example 3
//!
//! ```
//! # use c2022_08::c2022_08_15::*;
//! assert_eq!(Solution::roman_to_int("MCMXCIV".into()), 1994);
//! ```
//!
//! __Explanation:__ `M = 1000`, `CM = 900`, `XC = 90` and `IV = 4`.
//!
//! ##### Constraints
//!
//! - `1 <= s.length <= 15`
//! - `s` contains only the characters (`'I'`, `'V'`, `'X'`, `'L'`, `'C'`, `'D'`, `'M'`).
//! - It is __guaranteed__ that `s` is a valid roman numeral in the range `[1, 3999]`.

pub struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let to_dec = |c| match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => unreachable!(),
        };

        let fold = |(total, prev): (i32, i32), curr: i32| {
            (total + if curr < prev { -curr } else { curr }, curr)
        };

        s.chars().rev().map(to_dec).fold((0, 0), fold).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn i() { assert_eq!(Solution::roman_to_int("I".into()),    1); }
    #[rustfmt::skip] #[test] fn v() { assert_eq!(Solution::roman_to_int("V".into()),    5); }
    #[rustfmt::skip] #[test] fn x() { assert_eq!(Solution::roman_to_int("X".into()),   10); }
    #[rustfmt::skip] #[test] fn l() { assert_eq!(Solution::roman_to_int("L".into()),   50); }
    #[rustfmt::skip] #[test] fn c() { assert_eq!(Solution::roman_to_int("C".into()),  100); }
    #[rustfmt::skip] #[test] fn d() { assert_eq!(Solution::roman_to_int("D".into()),  500); }
    #[rustfmt::skip] #[test] fn m() { assert_eq!(Solution::roman_to_int("M".into()), 1000); }

    #[rustfmt::skip] #[test] fn iv() { assert_eq!(Solution::roman_to_int("IV".into()),   4); }
    #[rustfmt::skip] #[test] fn ix() { assert_eq!(Solution::roman_to_int("IX".into()),   9); }
    #[rustfmt::skip] #[test] fn xl() { assert_eq!(Solution::roman_to_int("XL".into()),  40); }
    #[rustfmt::skip] #[test] fn xc() { assert_eq!(Solution::roman_to_int("XC".into()),  90); }
    #[rustfmt::skip] #[test] fn cd() { assert_eq!(Solution::roman_to_int("CD".into()), 400); }
    #[rustfmt::skip] #[test] fn cm() { assert_eq!(Solution::roman_to_int("CM".into()), 900); }

    #[rustfmt::skip] #[test] fn   ii() { assert_eq!(Solution::roman_to_int(  "II".into()),  2); }
    #[rustfmt::skip] #[test] fn  iii() { assert_eq!(Solution::roman_to_int( "III".into()),  3); }
    #[rustfmt::skip] #[test] fn   vi() { assert_eq!(Solution::roman_to_int(  "VI".into()),  6); }
    #[rustfmt::skip] #[test] fn  vii() { assert_eq!(Solution::roman_to_int( "VII".into()),  7); }
    #[rustfmt::skip] #[test] fn viii() { assert_eq!(Solution::roman_to_int("VIII".into()),  8); }
    #[rustfmt::skip] #[test] fn   xi() { assert_eq!(Solution::roman_to_int(  "XI".into()), 11); }
    #[rustfmt::skip] #[test] fn  xii() { assert_eq!(Solution::roman_to_int( "XII".into()), 12); }
    #[rustfmt::skip] #[test] fn xiii() { assert_eq!(Solution::roman_to_int("XIII".into()), 13); }
    #[rustfmt::skip] #[test] fn  xiv() { assert_eq!(Solution::roman_to_int( "XIV".into()), 14); }
    #[rustfmt::skip] #[test] fn   xv() { assert_eq!(Solution::roman_to_int(  "XV".into()), 15); }
    #[rustfmt::skip] #[test] fn  xvi() { assert_eq!(Solution::roman_to_int( "XVI".into()), 16); }

    #[rustfmt::skip] #[test] fn lvii() { assert_eq!(Solution::roman_to_int("LVIII".into()), 58); }

    #[rustfmt::skip] #[test] fn mcmxliv() { assert_eq!(Solution::roman_to_int("MCMLXXXVI".into()), 1986); }
    #[rustfmt::skip] #[test] fn mcmxciv() { assert_eq!(Solution::roman_to_int("MCMXCIV".into()), 1994); }

    #[rustfmt::skip] #[test] fn mmmcmxcix() { assert_eq!(Solution::roman_to_int("MMMCMXCIX".into()), 3999); }
}
