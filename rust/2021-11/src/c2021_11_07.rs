#![allow(dead_code)]
/// 43. Multiply Strings
/// ====================
///
/// Given two non-negative integers `num1` and `num2` represented as strings,
/// return the product of `num1` and `num2`, also represented as a string.
///
/// __Note:__ You must not use any built-in BigInteger library or convert the inputs to integer directly.
///
/// Constraints:
///
/// - `1 <= num1.length, num2.length <= 200`
/// - `num1` and `num2` consist of digits only.
/// - Both `num1` and `num2` do not contain any leading zero, except the number `0` itself.
///
/// https://leetcode.com/problems/multiply-strings/
struct Solution;
impl Solution {
    /// 16:51-18:28
    pub fn multiply(num1: String, num2: String) -> String {
        println!("multiply({}, {})", num1, num2);
        if num1 == "0" || num2 == "0" {
            "0".into()
        } else {
            let (xs, ys) = (num1.as_bytes(), num2.as_bytes());
            let mut result: Vec<u8> = Vec::with_capacity(xs.len() + ys.len());
            let mut carry = 0;
            for xi in (0..xs.len()).rev() {
                let x = xs[xi] - b'0';
                for yi in (0..ys.len()).rev() {
                    let y = ys[yi] - b'0';
                    let ri = xs.len() - xi - 1 + ys.len() - yi - 1;
                    while ri >= result.len() {
                        result.push(0);
                    }
                    result[ri] = {
                        let r = x * y + carry + result[ri];
                        if r < 10 {
                            carry = 0;
                            r
                        } else {
                            carry = r / 10;
                            r % 10
                        }
                    };
                }
                if carry > 0 {
                    result.push(carry);
                    carry = 0;
                }
            }
            result.reverse();
            String::from_utf8(result.into_iter().map(|i| i + b'0').collect()).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n1_2_n2_3()     { assert_eq!(Solution::multiply("2".into(), "3".into()), "6"); }
    #[rustfmt::skip] #[test] fn n1_3_n2_7()     { assert_eq!(Solution::multiply("3".into(), "7".into()), "21"); }
    #[rustfmt::skip] #[test] fn n1_4_n2_7()     { assert_eq!(Solution::multiply("4".into(), "7".into()), "28"); }
    #[rustfmt::skip] #[test] fn n1_123_n2_456() { assert_eq!(Solution::multiply("123".into(), "456".into()), "56088"); }
    #[rustfmt::skip] #[test] fn n1_0_n2_7()     { assert_eq!(Solution::multiply("0".into(), "7".into()), "0"); }
    #[rustfmt::skip] #[test] fn n1_0_n2_16()    { assert_eq!(Solution::multiply("0".into(), "16".into()), "0"); }
    #[rustfmt::skip] #[test] fn n1_16_n2_0()    { assert_eq!(Solution::multiply("16".into(), "0".into()), "0"); }
    #[rustfmt::skip] #[test] fn n1_10_n2_15()   { assert_eq!(Solution::multiply("10".into(), "15".into()), "150"); }
    #[rustfmt::skip] #[test] fn n1_11_n2_15()   { assert_eq!(Solution::multiply("11".into(), "15".into()), "165"); }
    #[rustfmt::skip] #[test] fn n1_21_n2_5()    { assert_eq!(Solution::multiply("21".into(), "5".into()), "105"); }
    #[rustfmt::skip] #[test] fn n1_5_n2_21()    { assert_eq!(Solution::multiply("5".into(), "21".into()), "105"); }
    #[rustfmt::skip] #[test] fn n1_9_n2_99()    { assert_eq!(Solution::multiply("9".into(), "99".into()), "891"); }
    #[rustfmt::skip] #[test] fn n1_99_n2_9()    { assert_eq!(Solution::multiply("99".into(), "9".into()), "891"); }
    #[rustfmt::skip] #[test] fn n1_99_n2_99()   { assert_eq!(Solution::multiply("99".into(), "99".into()), "9801"); }
    #[rustfmt::skip] #[test] fn n1_101_n2_101() { assert_eq!(Solution::multiply("101".into(), "101".into()), "10201"); }
    #[rustfmt::skip] #[test] fn n1_999_n2_999() { assert_eq!(Solution::multiply("999".into(), "999".into()), "998001"); }

    #[test]
    fn n1_1_199x0_n2_1_199x0() {
        let mut n1: String = "1".into();
        for _ in 0..199 {
            n1.push('0');
        }
        let n2 = n1.clone();
        let mut e = n1.clone();
        for _ in 0..199 {
            e.push('0');
        }
        assert_eq!(Solution::multiply(n1, n2), e);
    }
}
