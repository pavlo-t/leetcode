#![allow(dead_code)]
/// 476. Number Complement
/// ======================
///
/// The __complement__ of an integer is the integer you get when you flip all the `0`'s to `1`'s
/// and all the `1`'s to `0`'s in its binary representation.
///
/// For example, The integer `5` is `"101"` in binary and its __complement__ is `"010"` which is the integer `2`.
///
/// Given an integer `num`, return _its complement_.
///
/// __Constraints:__
///
/// - `1 <= num < 2**31`
///
/// __Note:__ This question is the same as 1009: https://leetcode.com/problems/complement-of-base-10-integer/
///
/// https://leetcode.com/problems/number-complement/
struct Solution;
impl Solution {
    pub fn find_complement_my_v1(num: i32) -> i32 {
        println!("find_complement({})", num);
        (0..31)
            .map(|s| 1 << s)
            .map(|bit| if bit > num { bit } else { bit & num })
            .fold(i32::MAX, |rsf, bit| rsf ^ bit)
    }
    pub fn find_complement_my_v2(num: i32) -> i32 {
        println!("find_complement({})", num);
        let mut bit = 1 << 30;
        let mut n = num;
        while bit > num {
            n |= bit;
            bit >>= 1;
        }
        i32::MAX ^ n
    }
    pub fn find_complement_my_v3(num: i32) -> i32 {
        println!("find_complement({})", num);
        i32::MAX
            ^ (0..31)
                .rev()
                .map(|shift| 1 << shift)
                .take_while(|&bit| bit > num)
                .fold(num, |rsf, bit| rsf | bit)
    }

    /// from other submissions: https://leetcode.com/submissions/detail/608180994/
    pub fn find_complement_leetcode_submissions(num: i32) -> i32 {
        !num & (1 << 31 - num.leading_zeros()) - 1
    }

    /// Approach 4: highestOneBit OpenJDK algorithm from Hacker's Delight
    /// https://leetcode.com/problems/number-complement/solution/
    pub fn find_complement(num: i32) -> i32 {
        let mut bitmask = num;
        // bitmask has the same length as num and contains only ones 1...1
        bitmask |= bitmask >> 1;
        bitmask |= bitmask >> 2;
        bitmask |= bitmask >> 4;
        bitmask |= bitmask >> 8;
        bitmask |= bitmask >> 16;
        // flip all bits
        bitmask ^ num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1() { assert_eq!(Solution::find_complement(1), 0); } //   1 ->   0
    #[rustfmt::skip] #[test] fn n_2() { assert_eq!(Solution::find_complement(2), 1); } //  10 ->  01
    #[rustfmt::skip] #[test] fn n_3() { assert_eq!(Solution::find_complement(3), 0); } //  11 ->  00
    #[rustfmt::skip] #[test] fn n_4() { assert_eq!(Solution::find_complement(4), 3); } // 100 -> 011
    #[rustfmt::skip] #[test] fn n_5() { assert_eq!(Solution::find_complement(5), 2); } // 101 -> 010

    #[rustfmt::skip] #[test] fn n_2_147_483_647() { assert_eq!(Solution::find_complement(2147483647), 0); }
}
