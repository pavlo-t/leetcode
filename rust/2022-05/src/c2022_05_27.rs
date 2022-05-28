#![allow(dead_code)]
/// \#1342. Number of Steps to Reduce a Number to Zero
/// ==================================================
///
/// Given an integer `num`, return _the number of steps to reduce it to zero_.
///
/// In one step, if the current number is even, you have to divide it by `2`,
/// otherwise, you have to subtract `1` from it.
///
/// __Constraints:__
///
/// - `0 <= num <= 1_000_000`
///
/// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
struct Solution;
impl Solution {
    pub fn number_of_steps_iter_if_mod(mut num: i32) -> i32 {
        let mut steps = 0;
        while num > 0 {
            if num % 2 == 0 {
                num /= 2;
            } else {
                num -= 1;
            }
            steps += 1;
        }
        steps
    }
    pub fn number_of_steps_rec(num: i32) -> i32 {
        match num {
            0 => 0,
            n => Self::number_of_steps(if n % 2 == 0 { n / 2 } else { n - 1 }) + 1,
        }
    }
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut steps = 0;
        while num > 0 {
            num = if num & 1 == 0 { num / 2 } else { num - 1 };
            steps += 1;
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn  n0() { assert_eq!(Solution::number_of_steps( 0), 0); }
    #[rustfmt::skip] #[test] fn  n1() { assert_eq!(Solution::number_of_steps( 1), 1); }
    #[rustfmt::skip] #[test] fn  n2() { assert_eq!(Solution::number_of_steps( 2), 2); }
    #[rustfmt::skip] #[test] fn  n3() { assert_eq!(Solution::number_of_steps( 3), 3); }
    #[rustfmt::skip] #[test] fn  n4() { assert_eq!(Solution::number_of_steps( 4), 3); }
    #[rustfmt::skip] #[test] fn  n5() { assert_eq!(Solution::number_of_steps( 5), 4); }
    #[rustfmt::skip] #[test] fn  n6() { assert_eq!(Solution::number_of_steps( 6), 4); }
    #[rustfmt::skip] #[test] fn  n7() { assert_eq!(Solution::number_of_steps( 7), 5); }
    #[rustfmt::skip] #[test] fn  n8() { assert_eq!(Solution::number_of_steps( 8), 4); }
    #[rustfmt::skip] #[test] fn  n9() { assert_eq!(Solution::number_of_steps( 9), 5); }
    #[rustfmt::skip] #[test] fn n10() { assert_eq!(Solution::number_of_steps(10), 5); }
    #[rustfmt::skip] #[test] fn n11() { assert_eq!(Solution::number_of_steps(11), 6); }
    #[rustfmt::skip] #[test] fn n12() { assert_eq!(Solution::number_of_steps(12), 5); }
    #[rustfmt::skip] #[test] fn n13() { assert_eq!(Solution::number_of_steps(13), 6); }
    /// Explanation:
    /// Step 1) 14 is even; divide by 2 and obtain 7.
    /// Step 2) 7 is odd; subtract 1 and obtain 6.
    /// Step 3) 6 is even; divide by 2 and obtain 3.
    /// Step 4) 3 is odd; subtract 1 and obtain 2.
    /// Step 5) 2 is even; divide by 2 and obtain 1.
    /// Step 6) 1 is odd; subtract 1 and obtain 0.
    #[rustfmt::skip] #[test] fn n14() { assert_eq!(Solution::number_of_steps(14), 6); }

    #[test]
    fn n123() {
        assert_eq!(Solution::number_of_steps(123), 12);
    }

    #[test]
    fn n_100_to_1_000_000() {
        for n in 100..=1_000_000 {
            assert!(Solution::number_of_steps(n) > 0);
        }
    }
}
