#![allow(dead_code)]
/// Add Strings
/// ===========
///
/// Given two non-negative integers, `num1` and `num2` represented as string,
/// return _the sum of `num1` and `num2` as a string_.
///
/// You must solve the problem without using any built-in library for handling large integers (such as `BigInteger`).
/// You must also not convert the inputs to integers directly.
///
/// Constraints:
///
/// - `1 <= num1.length, num2.length <= 10_000`
/// - `num1` and `num2` consist of only digits.
/// - `num1` and `num2` don't have any leading zeros except for the zero itself.
///
/// https://leetcode.com/explore/featured/card/august-leetcoding-challenge-2021/614/week-2-august-8th-august-14th/3875/
struct Solution;
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let a = num1.bytes().rev().collect::<Vec<u8>>();
        let b = num2.bytes().rev().collect::<Vec<u8>>();
        let (a, b) = if a.len() > b.len() { (a, b) } else { (b, a) };
        let mut i = 0;
        let mut c = 0;
        let mut r = vec![];
        while i < a.len() {
            let mut n = a[i] + c;
            if i < b.len() {
                n += b[i] - b'0';
            }
            if n > b'9' {
                n -= 10;
                c = 1;
            } else {
                c = 0;
            }
            r.push(n);
            i += 1;
        }
        if c > 0 {
            r.push(b'1');
        }
        r.reverse();
        String::from_utf8(r).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ns_11_123_produces_134() {
        let num1 = "11".to_string();
        let num2 = "123".to_string();
        let e = "134".to_string();
        assert_eq!(Solution::add_strings(num1, num2), e);
    }
    #[test]
    fn ns_456_77_produces_533() {
        let num1 = "456".to_string();
        let num2 = "77".to_string();
        let e = "533".to_string();
        assert_eq!(Solution::add_strings(num1, num2), e);
    }
    #[test]
    fn ns_0_0_produces_0() {
        let num1 = "0".to_string();
        let num2 = "0".to_string();
        let e = "0".to_string();
        assert_eq!(Solution::add_strings(num1, num2), e);
    }
    #[test]
    fn ns_1_999_produces_1000() {
        let num1 = "1".to_string();
        let num2 = "999".to_string();
        let e = "1000".to_string();
        assert_eq!(Solution::add_strings(num1, num2), e);
    }

    #[test]
    fn ns_1x10000_2x10000_produces_3x10000() {
        let num1 = "1".repeat(10000);
        let num2 = "2".repeat(10000);
        let e = "3".repeat(10000);
        assert_eq!(Solution::add_strings(num1, num2), e);
    }
}
