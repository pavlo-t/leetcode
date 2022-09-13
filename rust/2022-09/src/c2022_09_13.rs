#![allow(dead_code)]
//! \#393. UTF-8 Validation
//! =======================
//!
//! <https://leetcode.com/problems/utf-8-validation>
//!
//! Given an integer array `data` representing the data, return whether it is a valid __UTF-8__ encoding
//! (i.e. it translates to a sequence of valid UTF-8 encoded characters).
//!
//! A character in __UTF8__ can be from __1 to 4 bytes__ long, subjected to the following rules:
//!
//! 1. For a __1-byte__ character, the first bit is a `0`, followed by its Unicode code.
//! 2. For an __n-bytes__ character, the first `n` bits are all one's, the `n + 1` bit is `0`,
//!    followed by `n - 1` bytes with the most significant `2` bits being `10`.
//!
//! This is how the UTF-8 encoding would work:
//!
//! ```text
//!  Number of Bytes  |     UTF-8 Octet Sequence (binary)
//! ------------------+---------------------------------------
//!         1         | 0xxxxxxx
//!         2         | 110xxxxx 10xxxxxx
//!         3         | 1110xxxx 10xxxxxx 10xxxxxx
//!         4         | 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
//! ```
//!
//! `x` denotes a bit in the binary form of a byte that may be either `0` or `1`.
//!
//! __Note:__ The input is an array of integers.
//! Only the __least significant 8 bits__ of each integer is used to store the data.
//! This means each integer represents only `1` byte of data.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_09::c2022_09_13::*;
//! assert_eq!(Solution::valid_utf8(vec![197, 130, 1]), true);
//! ```
//!
//! __Explanation:__ data represents the octet sequence: `11000101` `10000010` `00000001`.
//! It is a valid utf-8 encoding for a `2`-bytes character followed by a `1`-byte character.
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_09::c2022_09_13::*;
//! assert_eq!(Solution::valid_utf8(vec![235, 140, 4]), false);
//! ```
//!
//! __Explanation:__ data represented the octet sequence: `11101011` `10001100` `00000100`.
//! The first `3` bits are all one's and the `4`th bit is `0` means it is a `3`-bytes character.
//! The next byte is a continuation byte which starts with `10` and that's correct.
//! But the second continuation byte does not start with `10`, so it is invalid.
//!
//! ##### Constraints
//!
//! - `1 <= data.length <= 20_000`
//! - `0 <= data[i] <= 255`

pub struct Solution;
impl Solution {
    pub fn valid_utf8_v1(data: Vec<i32>) -> bool {
        const BIT_1: i32 = 1 << 7;
        const BIT_2: i32 = 1 << 6;

        let mut expected_10_bytes = 0;
        for i in data {
            if expected_10_bytes > 0 {
                if i & BIT_1 != 0 && i & BIT_2 == 0 {
                    expected_10_bytes -= 1;
                } else {
                    return false;
                }
            } else {
                let mut curr_bit = BIT_1;
                while i & curr_bit != 0 {
                    expected_10_bytes += 1;
                    if expected_10_bytes > 4 {
                        return false;
                    }
                    curr_bit >>= 1;
                }
                match expected_10_bytes {
                    0 => (),
                    1 => return false,
                    _ => expected_10_bytes -= 1,
                }
            }
        }

        expected_10_bytes == 0
    }

    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut expected_10_bytes = 0;

        for b in data.iter().map(|&i| i as u8) {
            if expected_10_bytes > 0 {
                if b.leading_ones() == 1 {
                    expected_10_bytes -= 1;
                } else {
                    return false;
                }
            } else {
                match b.leading_ones() {
                    0 => (),
                    n @ (2 | 3 | 4) => expected_10_bytes = n - 1,
                    _ => return false,
                }
            }
        }

        expected_10_bytes == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_1() {
        assert_eq!(Solution::valid_utf8(vec![1]), true);
    }
    #[test]
    fn data_1_2() {
        assert_eq!(Solution::valid_utf8(vec![1, 2]), true);
    }
    #[test]
    fn data_128() {
        assert_eq!(Solution::valid_utf8(vec![128]), false);
    }

    #[test]
    fn data_192_128() {
        assert_eq!(Solution::valid_utf8(vec![192, 128]), true);
    }
    #[test]
    fn data_192_128_128() {
        assert_eq!(Solution::valid_utf8(vec![192, 128, 128]), false);
    }
    #[test]
    fn data_192() {
        assert_eq!(Solution::valid_utf8(vec![192]), false);
    }

    #[test]
    fn data_224_128_128() {
        assert_eq!(Solution::valid_utf8(vec![224, 128, 128]), true);
    }
    #[test]
    fn data_224_128_128_128() {
        assert_eq!(Solution::valid_utf8(vec![224, 128, 128, 128]), false);
    }
    #[test]
    fn data_224_128() {
        assert_eq!(Solution::valid_utf8(vec![224, 128]), false);
    }

    #[test]
    fn data_240_128_128_128() {
        assert_eq!(Solution::valid_utf8(vec![240, 128, 128, 128]), true);
    }
    #[test]
    fn data_240_128_128_128_128() {
        assert_eq!(Solution::valid_utf8(vec![240, 128, 128, 128, 128]), false);
    }
    #[test]
    fn data_240_128_128() {
        assert_eq!(Solution::valid_utf8(vec![240, 128, 128]), false);
    }

    #[test]
    fn data_248_128_128_128_128() {
        assert_eq!(Solution::valid_utf8(vec![248, 128, 128, 128, 128]), false);
    }

    #[test]
    fn data_197_130_1() {
        assert_eq!(Solution::valid_utf8(vec![197, 130, 1]), true);
    }

    #[test]
    fn data_224_130_130() {
        assert_eq!(Solution::valid_utf8(vec![197, 130, 1]), true);
    }

    #[test]
    fn data_235_140_4() {
        assert_eq!(Solution::valid_utf8(vec![235, 140, 4]), false);
    }
}
