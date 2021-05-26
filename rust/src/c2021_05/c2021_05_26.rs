#![allow(dead_code)]
/// Partitioning Into Minimum Number Of Deci-Binary Numbers
/// =======================================================
///
/// A decimal number is called __deci-binary__ if each of its digits is either `0` or `1`
/// without any leading zeros.
/// For example, `101` and `1100` are __deci-binary__, while `112` and `3001` are not.
///
/// Given a string `n` that represents a positive decimal integer,
/// return _the __minimum__ number of positive __deci-binary__ numbers needed so that they sum up to_ `n`.
///
/// __Constraints:__
///
/// - `1 <= n.length <= 100_000`
/// - `n` consists of only digits.
/// - `n` does not contain any leading zeros and represents a positive integer.
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/601/week-4-may-22nd-may-28th/3756/
struct Solution;
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        (n.bytes().max().unwrap() - b'0') as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_partitions("32".to_string()), 3);
        // Explanation: 10 + 11 + 11 = 32
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::min_partitions("82734".to_string()), 8);
    }
    #[test]
    fn example3() {
        let n = "27346209830709182346".to_string();
        assert_eq!(Solution::min_partitions(n), 9);
    }

    #[test]
    fn n100k_1s_produces_1() {
        let n = "1".repeat(100000);
        assert_eq!(Solution::min_partitions(n), 1);
    }
}
