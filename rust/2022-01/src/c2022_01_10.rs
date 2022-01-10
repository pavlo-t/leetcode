#![allow(dead_code)]
/// 67. Add Binary
/// ==============
///
/// Given two binary strings `a` and `b`, return _their sum as a binary string_.
///
/// __Constraints:__
///
/// - `1 <= a.length, b.length <= 10_000`
/// - `a` and `b` consist only of `'0'` or `'1'` characters.
/// - Each string does not contain leading zeros except for the zero itself.
///
/// https://leetcode.com/problems/add-binary/
struct Solution;
impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        if b.len() > a.len() {
            std::mem::swap(&mut a, &mut b);
        }

        let mut carry = 0;
        let mut result = vec![];
        let (mut chars_a, mut chars_b) = (a.chars().rev(), b.chars().rev());

        while let Some(ca) = chars_a.next() {
            carry += ca.to_digit(2).unwrap();
            if let Some(cb) = chars_b.next() {
                carry += cb.to_digit(2).unwrap();
            }
            //result.push(char::from_digit(carry % 2, 2).unwrap());
            //result.push(((carry % 2) as u8 + b'0') as char);
            result.push(if carry % 2 == 1 { '1' } else { '0' });
            carry >>= 1;
        }
        if carry == 1 {
            result.push('1');
        }

        result.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn a0_b0() { assert_eq!(Solution::add_binary("0".into(), "0".into()), "0"); }
    #[rustfmt::skip] #[test] fn a1_b0() { assert_eq!(Solution::add_binary("1".into(), "0".into()), "1"); }
    #[rustfmt::skip] #[test] fn a0_b1() { assert_eq!(Solution::add_binary("0".into(), "1".into()), "1"); }
    #[rustfmt::skip] #[test] fn a1_b1() { assert_eq!(Solution::add_binary("1".into(), "1".into()), "10"); }

    #[rustfmt::skip] #[test] fn a11_b1() { assert_eq!(Solution::add_binary("11".into(), "1".into()), "100"); }
    #[rustfmt::skip] #[test] fn a1_b11() { assert_eq!(Solution::add_binary("1".into(), "11".into()), "100"); }
    #[rustfmt::skip] #[test] fn a1010_b1011() { assert_eq!(Solution::add_binary("1010".into(), "1011".into()), "10101"); }
}
