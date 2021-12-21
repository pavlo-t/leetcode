#![allow(dead_code)]
/// 231. Power of Two
/// =================
///
/// Given an integer `n`, return _`true` if it is a power of two. Otherwise, return `false`_.
///
/// An integer `n` is a power of two, if there exists an integer `x` such that `n == 2**x`.
///
/// __Constraints:__
///
/// - `-2**31 <= n <= 2**31 - 1`
///
/// __Follow up:__ Could you solve it without loops/recursion?
///
/// https://leetcode.com/problems/power-of-two/
struct Solution;
impl Solution {
    pub fn is_power_of_two_rec(n: i32) -> bool {
        n == 1 || n % 2 == 0 && Self::is_power_of_two(n / 2)
    }

    pub fn is_power_of_two_loop(mut n: i32) -> bool {
        if n < 0 {
            return false;
        }
        while n > 2 {
            if n % 2 == 1 {
                return false;
            }
            n /= 2;
        }
        true
    }

    pub fn is_power_of_two_count_ones(n: i32) -> bool {
        n > 0 && n.count_ones() == 1
    }

    /// Approach 1: Bitwise Operators : Get the Rightmost 1-bit
    /// https://leetcode.com/problems/power-of-two/solution/
    pub fn is_power_of_two_leet_code_1(n: i32) -> bool {
        n > 0 && (n & (-n)) == n
    }
    /// Approach 2: Bitwise operators : Turn off the Rightmost 1-bit
    /// https://leetcode.com/problems/power-of-two/solution/
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n1() {
        assert!(Solution::is_power_of_two(1));
        // Explanation: 2**0 = 1
    }
    #[test]
    fn n16() {
        assert!(Solution::is_power_of_two(16));
        // Explanation: 2**4 = 16
    }
    #[test]
    fn n3() {
        assert!(!Solution::is_power_of_two(3));
    }

    #[test]
    fn test_1108_n_minus_2_147_483_648() {
        assert!(!Solution::is_power_of_two(-2147483648))
    }
}
