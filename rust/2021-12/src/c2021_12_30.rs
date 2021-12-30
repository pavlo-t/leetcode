#![allow(dead_code)]
/// 1015. Smallest Integer Divisible by K
/// =====================================
///
/// Given a positive integer `k`, you need to find the __length__ of the __smallest__ positive integer `n`
/// such that `n` is divisible by `k`, and `n` only contains the digit `1`.
///
/// Return _the __length__ of `n`_. If there is no such `n`, return `-1`.
///
/// __Note:__ `n` may not fit in a 64-bit signed integer.
///
/// __Constraints:__
///
/// - `1 <= k <= 100_000`
///
/// https://leetcode.com/problems/smallest-integer-divisible-by-k/
struct Solution;
impl Solution {
    /// https://www.geeksforgeeks.org/length-of-the-smallest-number-which-is-divisible-by-k-and-formed-by-using-1s-only/
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if k % 2 == 0 || k % 5 == 0 {
            -1
        } else {
            let mut n = 0;
            for l in 1..=k {
                n = (n * 10 + 1) % k;
                if n == 0 {
                    return l;
                }
            }
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn k_1() {
        assert_eq!(Solution::smallest_repunit_div_by_k(1), 1);
        // Explanation: The smallest answer is n = 1, which has length 1.
    }
    #[test]
    fn k_2() {
        assert_eq!(Solution::smallest_repunit_div_by_k(2), -1);
        // Explanation: There is no such positive integer n divisible by 2.
    }
    #[test]
    fn k_3() {
        assert_eq!(Solution::smallest_repunit_div_by_k(3), 3);
        // Explanation: The smallest answer is n = 111, which has length 3.
    }
}
