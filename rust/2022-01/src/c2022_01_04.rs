#![allow(dead_code)]
/// 1009. Complement of Base 10 Integer
/// ===================================
///
/// The __complement__ of an integer is the integer you get when you flip all the `0`'s to `1`'s and
/// all the `1`'s to `0`'s in its binary representation.
///
/// For example, The integer `5` is `"101"` in binary and its __complement__ is `"010"` which is the integer `2`.
///
/// Given an integer `n`, return _its complement_.
///
/// __Constraints:__
///
/// - `0 <= n < 1_000_000_000`
///
/// __Note:__ This question is the same as 476: https://leetcode.com/problems/number-complement/
///
/// https://leetcode.com/problems/complement-of-base-10-integer/
struct Solution;
impl Solution {
    pub fn bitwise_complement_my_0(n: i32) -> i32 {
        if n == 0 {
            1
        } else {
            let pref = {
                let mut result = -1;
                while (result & n) != 0 {
                    result <<= 1;
                }
                result
            };
            (n ^ -1) ^ pref
        }
    }

    pub fn bitwise_complement(n: i32) -> i32 {
        let next_power_of_2 = {
            let mut result = 2;
            while result <= n {
                result <<= 1;
            }
            result
        };
        next_power_of_2 - 1 - n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_0() { assert_eq!(Solution::bitwise_complement(0), 1); } // 0 -> 1 = 2-1-0
    #[rustfmt::skip] #[test] fn n_1() { assert_eq!(Solution::bitwise_complement(1), 0); } // 1 -> 0 = 2-1-1
    #[rustfmt::skip] #[test] fn n_2() { assert_eq!(Solution::bitwise_complement(2), 1); } // 10 -> 01 = 4-1-2
    #[rustfmt::skip] #[test] fn n_3() { assert_eq!(Solution::bitwise_complement(3), 0); } // 11 -> 00 = 4-1-3
    #[rustfmt::skip] #[test] fn n_4() { assert_eq!(Solution::bitwise_complement(4), 3); } // 100 -> 011 = 8-1-4
    #[rustfmt::skip] #[test] fn n_5() { assert_eq!(Solution::bitwise_complement(5), 2); } // 101 -> 010 = 8-1-5
    #[rustfmt::skip] #[test] fn n_6() { assert_eq!(Solution::bitwise_complement(6), 1); } // 110 -> 001 = 8-1-6
    #[rustfmt::skip] #[test] fn n_7() { assert_eq!(Solution::bitwise_complement(7), 0); } // 111 -> 000 = 8-1-7
    #[rustfmt::skip] #[test] fn n_8()  { assert_eq!(Solution::bitwise_complement(8),  7); } // 1000 -> 0111 = 16-1-8
    #[rustfmt::skip] #[test] fn n_9()  { assert_eq!(Solution::bitwise_complement(9),  6); } // 1001 -> 0110 = 16-1-9
    #[rustfmt::skip] #[test] fn n_10() { assert_eq!(Solution::bitwise_complement(10), 5); } // 1010 -> 0101 = 16-1-10

    #[rustfmt::skip] #[test] fn n_1_000_000_000() { assert_eq!(Solution::bitwise_complement(1_000_000_000), 73_741_823); }
}
