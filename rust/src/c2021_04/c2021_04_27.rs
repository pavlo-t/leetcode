#![allow(dead_code)]
/// Power of Three
/// ==============
///
/// Given an integer `n`, return _`true` if it is a power of three. Otherwise, return `false`_.
///
/// An integer `n` is a power of three, if there exists an integer `x` such that `n == 3^x`.
///
/// __Constraints:__
///
/// - `-2^31 <= n <= 2^31 - 1`
///
/// __Follow up:__ Could you solve it without loops/recursion?
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/596/week-4-april-22nd-april-28th/3722/
struct Solution;
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        n > 0 && 1_162_261_467 % n == 0
    }

    pub fn is_power_of_three_iter(mut n: i32) -> bool {
        if n == 0 {
            false
        } else {
            while n > 1 {
                if n % 3 != 0 {
                    return false;
                } else {
                    n /= 3;
                }
            }
            true
        }
    }

    pub fn is_power_of_three_rec(n: i32) -> bool {
        if n == 1 {
            true
        } else if n == 0 || n % 3 != 0 {
            false
        } else {
            Self::is_power_of_three(n / 3)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_n0_is_not() {
        assert!(!Solution::is_power_of_three(0));
    }
    #[test]
    fn example2_n9_is() {
        assert!(Solution::is_power_of_three(9));
    }
    #[test]
    fn example3_n27_is() {
        assert!(Solution::is_power_of_three(27));
    }
    #[test]
    fn example4_n45_is_not() {
        assert!(!Solution::is_power_of_three(45))
    }

    #[test]
    fn n1_is() {
        assert!(Solution::is_power_of_three(1));
    }
    #[test]
    fn n3_is() {
        assert!(Solution::is_power_of_three(3));
    }

    #[test]
    fn n_1_162_261_467_is() {
        assert!(Solution::is_power_of_three(1_162_261_467));
    }
    #[test]
    fn n_i32_max_is_not() {
        assert!(!Solution::is_power_of_three(i32::MAX));
    }
}
