#![allow(dead_code, non_snake_case)]
/// \#191. Number of 1 Bits
/// =======================
///
/// Write a function that takes an unsigned integer and returns the number of '1' bits it has
/// (also known as the Hamming weight).
///
/// __Note:__
///
/// - Note that in some languages, such as Java, there is no unsigned integer type.
///   In this case, the input will be given as a signed integer type.
///   It should not affect your implementation, as the integer's internal binary representation is the same,
///   whether it is signed or unsigned.
/// - In Java, the compiler represents the signed integers using 2's complement notation.
///   Therefore, in Example 3, the input represents the signed integer. -3.
///
/// __Constraints:__
///
/// - The input must be a __binary string__ of length `32`.
///
///
/// __Follow up:__ If this function is called many times, how would you optimize it?
///
/// https://leetcode.com/problems/number-of-1-bits
struct Solution;
impl Solution {
    pub fn hammingWeight_builtin(n: u32) -> i32 {
        n.count_ones() as i32
    }
    pub fn hammingWeight(mut n: u32) -> i32 {
        let mut result = 0;
        while n > 0 {
            result += n & 1;
            n >>= 1;
        }
        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn n00000000000000000000000000001011() {
        let n = u32::from_str_radix("00000000000000000000000000001011", 2).unwrap();
        assert_eq!(Solution::hammingWeight(n), 3);
        // Explanation: The input binary string 00000000000000000000000000001011 has a total of three '1' bits.
    }
    #[test]
    fn n00000000000000000000000010000000() {
        let n = u32::from_str_radix("00000000000000000000000010000000", 2).unwrap();
        assert_eq!(Solution::hammingWeight(n), 1);
        // Explanation: The input binary string 00000000000000000000000010000000 has a total of one '1' bit.
    }
    #[test]
    fn n11111111111111111111111111111101() {
        let n = u32::from_str_radix("11111111111111111111111111111101", 2).unwrap();
        assert_eq!(Solution::hammingWeight(n), 31);
        // Explanation: The input binary string 11111111111111111111111111111101 has a total of thirty one '1' bits.
    }
}
