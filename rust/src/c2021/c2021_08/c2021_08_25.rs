#![allow(dead_code)]
/// Sum of Square Numbers
/// =====================
///
/// Given a non-negative integer `c`, decide whether there're two integers `a` and `b` such that `a^2 + b^2 = c`.
///
/// __Constraints:__
///
/// - `0 <= c <= 2^31 - 1`
///
/// https://leetcode.com/explore/challenge/card/august-leetcoding-challenge-2021/616/week-4-august-22nd-august-28th/3918/
struct Solution;
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        use std::collections::HashSet;

        let mut sqs = HashSet::new();
        let mut i = 0;
        while i < 46340 {
            let s = i * i;
            sqs.insert(s);
            if s > c {
                break;
            }
            i += 1;
        }
        for &i in &sqs {
            if sqs.contains(&(c - i)) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c0_is_true() {
        assert!(Solution::judge_square_sum(0));
    }
    #[test]
    fn c1_is_true() {
        assert!(Solution::judge_square_sum(1));
    }
    #[test]
    fn c2_is_true() {
        assert!(Solution::judge_square_sum(2));
    }
    #[test]
    fn c3_is_false() {
        assert!(!Solution::judge_square_sum(3));
    }
    #[test]
    fn c4_is_true() {
        assert!(Solution::judge_square_sum(4));
    }
    #[test]
    fn c5_is_true() {
        assert!(Solution::judge_square_sum(5));
        // Explanation: 1 * 1 + 2 * 2 = 5
    }

    #[test]
    fn c2pow31m1_is_false() {
        assert!(!Solution::judge_square_sum(2147483647));
    }
}
