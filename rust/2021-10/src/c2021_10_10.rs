#![allow(dead_code)]
/// 201. Bitwise AND of Numbers Range
/// =================================
///
/// Given two integers `left` and `right` that represent the range `[left, right]`,
/// return _the bitwise AND of all numbers in this range, inclusive_.
///
/// __Constraints:__
///
/// - `0 <= left <= right <= 2^31 - 1`
///
/// https://leetcode.com/problems/bitwise-and-of-numbers-range/
struct Solution;
impl Solution {
    /// Approach 2: Brian Kernighan's Algorithm
    /// https://leetcode.com/problems/bitwise-and-of-numbers-range/solution/
    pub fn range_bitwise_and(l: i32, mut r: i32) -> i32 {
        while l < r {
            // turn off rightmost 1-bit
            r = r & (r - 1);
        }
        l & r
    }
    /// Approach 1: Bit Shift (common bit-prefix)
    /// https://leetcode.com/problems/bitwise-and-of-numbers-range/solution/
    pub fn range_bitwise_and_bit_shift(mut l: i32, mut r: i32) -> i32 {
        let mut shift = 0;
        while l != r {
            l >>= 1;
            r >>= 1;
            shift += 1;
        }
        l << shift
    }

    pub fn range_bitwise_and_iteration(left: i32, right: i32) -> i32 {
        if left == 0 || right == 0 {
            0
        } else {
            let mut result = 0;
            for i in (0..32).rev() {
                let l = (left >> i) & 1;
                let r = (right >> i) & 1;

                if l != r {
                    break;
                }
                if l == 1 {
                    result |= 1 << i;
                }
            }
            result
        }
    }
    pub fn range_bitwise_and_recursive(left: i32, right: i32) -> i32 {
        if left == 0 || right == 0 {
            0
        } else {
            fn most_significant_bit(mut i: i32) -> i32 {
                if i == 0 {
                    0
                } else {
                    let mut msb = 1;
                    i >>= 1;
                    while i > 0 {
                        i >>= 1;
                        msb <<= 1;
                    }
                    msb
                }
            }
            let msb = most_significant_bit(left);
            if msb != most_significant_bit(right) {
                0
            } else {
                msb | Self::range_bitwise_and(msb ^ left, msb ^ right)
            }
        }
    }

    pub fn range_bitwise_and_brute_force(left: i32, right: i32) -> i32 {
        println!("range_bitwise_and({}, {})", left, right);
        let mut result = left;
        for i in left..=right {
            result &= i;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l_5_r_7() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
    }
    #[test]
    fn l_0_r_0() {
        assert_eq!(Solution::range_bitwise_and(0, 0), 0);
    }
    #[test]
    fn l_1_r_0() {
        assert_eq!(Solution::range_bitwise_and(1, 0), 0);
    }
    #[test]
    fn l_0_r_1() {
        assert_eq!(Solution::range_bitwise_and(0, 1), 0);
    }
    #[test]
    fn l_1_r_1() {
        assert_eq!(Solution::range_bitwise_and(1, 1), 1);
    }

    //#[ignore]
    #[test]
    fn l_1_r_2147483647() {
        assert_eq!(Solution::range_bitwise_and(1, 2147483647), 0);
    }
    //#[ignore]
    #[test]
    fn l_1073741824_r_2147483647() {
        assert_eq!(
            Solution::range_bitwise_and(1073741824, 2147483647),
            1073741824
        );
    }
}
