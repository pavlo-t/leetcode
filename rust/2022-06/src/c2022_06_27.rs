#![allow(dead_code)]
//! \#1689. Partitioning Into Minimum Number Of Deci-Binary Numbers
//! ===============================================================
//!
//! A decimal number is called __deci-binary__ if each of its digits is either `0` or `1` without any leading zeros.
//! For example, `101` and `1100` are __deci-binary__, while `112` and `3001` are not.
//!
//! Given a string `n` that represents a positive decimal integer,
//! return _the __minimum__ number of positive __deci-binary__ numbers needed so that they sum up to `n`_.
//!
//! __Constraints:__
//!
//! - `1 <= n.length <= 100_000`
//! - `n` consists of only digits.
//! - `n` does not contain any leading zeros and represents a positive integer.
//!
//! <https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers>

pub struct Solution;
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.bytes().map(|b| (b - b'0') as i32).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_32() {
        assert_eq!(Solution::min_partitions("32".into()), 3);
        // Explanation: 10 + 11 + 11 = 32
    }
    #[test]
    fn n_82734() {
        assert_eq!(Solution::min_partitions("82734".into()), 8);
    }
    #[test]
    fn n_27346209830709182346() {
        assert_eq!(Solution::min_partitions("27346209830709182346".into()), 9);
    }
}
