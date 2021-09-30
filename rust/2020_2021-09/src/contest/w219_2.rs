#![allow(dead_code)]
/// ### 5626. Partitioning Into Minimum Number Of Deci-Binary Numbers
///
/// A decimal number is called __deci-binary__ if each of its digits is either `0` or `1`
/// without any leading zeros.
/// For example, `101` and `1100` are __deci-binary__, while `112` and `3001` are not.
///
/// Given a string `n` that represents a positive decimal integer, return
/// _the __minimum__ number of positive __deci-binary__ numbers needed so that they sum up to_ `n`.
///
/// __Constraints:__
///
/// - `1 <= n.length <= 100_000`
/// - `n` consists of only digits.
/// - `n` does not contain any leading zeros and represents a positive integer.
///
/// https://leetcode.com/contest/weekly-contest-219/problems/partitioning-into-minimum-number-of-deci-binary-numbers/
struct Solution;
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.chars().map(|c| c.to_digit(10).unwrap()).max().unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_partitions("32".to_string()), 3);
        // Example 1:
        //
        // Input: n = "32"
        // Output: 3
        // Explanation: 10 + 11 + 11 = 32
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_partitions("82734".to_string()), 8);
        // Example 2:
        //
        // Input: n = "82734"
        // Output: 8
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::min_partitions("27346209830709182346".to_string()), 9);
        // Example 3:
        //
        // Input: n = "27346209830709182346"
        // Output: 9
    }
}
