#![allow(dead_code)]
/// 8. String to Integer (atoi)
/// ===========================
///
/// Implement the `myAtoi(string s)` function,
/// which converts a string to a 32-bit signed integer (similar to C/C++'s `atoi` function).
///
/// The algorithm for `myAtoi(string s)` is as follows:
///
/// 1. Read in and ignore any leading whitespace.
/// 2. Check if the next character (if not already at the end of the string) is `'-'` or `'+'`.
///    Read this character in if it is either.
///    This determines if the final result is negative or positive respectively.
///    Assume the result is positive if neither is present.
/// 3. Read in next the characters until the next non-digit character or the end of the input is reached.
///    The rest of the string is ignored.
/// 4. Convert these digits into an integer (i.e. `"123" -> 123`, `"0032" -> 32`).
///    If no digits were read, then the integer is `0`.
///    Change the sign as necessary (from step 2).
/// 5. If the integer is out of the 32-bit signed integer range `[-2**31, 2**31 - 1]`,
///    then clamp the integer so that it remains in the range.
///    Specifically, integers less than `-2**31` should be clamped to `-2**31`,
///    and integers greater than `2**31 - 1` should be clamped to `2**31 - 1`.
/// 6. Return the integer as the final result.
///
/// __Note:__
///
/// - Only the space character `' '` is considered a whitespace character.
/// - __Do not ignore__ any characters other than the leading whitespace or the rest of the string after the digits.
///
/// __Constraints:__
///
/// - `0 <= s.length <= 200`
/// - `s` consists of English letters (lower-case and upper-case), digits (`0-9`), `' '`, `'+'`, `'-'`, and `'.'`.
///
/// https://leetcode.com/problems/string-to-integer-atoi/
struct Solution;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut result = 0;

        let bs = s.as_bytes();
        let mut i = 0;

        // skip whitespace
        while i < bs.len() && bs[i] == b' ' {
            i += 1;
        }
        if i == bs.len() {
            return result;
        }

        // read sign, reverse to collect negative number, because i32::MIN.abs() > i32::MAX
        let sign = match bs[i] {
            b'-' => {
                i += 1;
                1
            }
            b'+' => {
                i += 1;
                -1
            }
            _ => -1,
        };

        // read digits
        while i < bs.len() && bs[i].is_ascii_digit() && result > i32::MIN {
            let d = (bs[i] - b'0') as i32;
            result = result.saturating_mul(10).saturating_sub(d);
            i += 1;
        }

        result.saturating_mul(sign)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn s_0() { assert_eq!(Solution::my_atoi("0".into()), 0); }
    #[rustfmt::skip] #[test] fn s_sp42() { assert_eq!(Solution::my_atoi(" +42".into()), 42); }
    #[rustfmt::skip] #[test] fn s_sp() { assert_eq!(Solution::my_atoi(" +".into()), 0); }
    #[rustfmt::skip] #[test] fn s_sm() { assert_eq!(Solution::my_atoi(" -".into()), 0); }

    #[rustfmt::skip] #[test] fn s_p9_999_999_999() { assert_eq!(Solution::my_atoi("+9999999999".into()),  2147483647); }
    #[rustfmt::skip] #[test] fn s_m9_999_999_999() { assert_eq!(Solution::my_atoi("-9999999999".into()), -2147483648); }

    #[rustfmt::skip] #[test] fn s_p2147483647() { assert_eq!(Solution::my_atoi("+2147483647".into()),  2147483647); }
    #[rustfmt::skip] #[test] fn s_m2147483648() { assert_eq!(Solution::my_atoi("-2147483648".into()), -2147483648); }

    #[rustfmt::skip] #[test] fn s_p2147483645() { assert_eq!(Solution::my_atoi("+2147483645".into()),  2147483645); }
    #[rustfmt::skip] #[test] fn s_m2147483646() { assert_eq!(Solution::my_atoi("-2147483646".into()), -2147483646); }

    #[rustfmt::skip] #[test] fn s_p200x9() { assert_eq!(Solution::my_atoi("9".repeat(200)), 2147483647); }
    #[rustfmt::skip] #[test] fn s_m200x9() {
        let mut s = "-".to_string();
        (0..200).for_each(|_| s.push('9'));
        assert_eq!(Solution::my_atoi(s), -2147483648);
    }

    #[test]
    fn s_42() {
        assert_eq!(Solution::my_atoi("42".into()), 42);
        // Explanation: The underlined characters are what is read in, the caret is the current reader position.
        // Step 1: "42" (no characters read because there is no leading whitespace)
        //          ^
        // Step 2: "42" (no characters read because there is neither a '-' nor '+')
        //          ^
        // Step 3: "42" ("42" is read in)
        //            ^
        // The parsed integer is 42.
        // Since 42 is in the range [-231, 231 - 1], the final result is 42.
    }
    #[test]
    fn s_sssm42() {
        assert_eq!(Solution::my_atoi("   -42".into()), -42);
        // Explanation:
        // Step 1: "   -42" (leading whitespace is read and ignored)
        //             ^
        // Step 2: "   -42" ('-' is read, so the result should be negative)
        //              ^
        // Step 3: "   -42" ("42" is read in)
        //                ^
        // The parsed integer is -42.
        // Since -42 is in the range [-231, 231 - 1], the final result is -42.
    }
    #[test]
    fn s_4193_with_words() {
        assert_eq!(Solution::my_atoi("4193 with words".into()), 4193);
        // Explanation:
        // Step 1: "4193 with words" (no characters read because there is no leading whitespace)
        //          ^
        // Step 2: "4193 with words" (no characters read because there is neither a '-' nor '+')
        //          ^
        // Step 3: "4193 with words" ("4193" is read in; reading stops because the next character is a non-digit)
        //              ^
        // The parsed integer is 4193.
        // Since 4193 is in the range [-231, 231 - 1], the final result is 4193.
    }
}
