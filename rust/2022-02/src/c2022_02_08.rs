#![allow(dead_code)]
/// 258. Add Digits
/// ===============
///
/// Given an integer `num`, repeatedly add all its digits until the result has only one digit, and return it.
///
/// __Constraints:__
///
/// - `0 <= num <= 2**31 - 1`
///
/// __Follow up:__ Could you do it without any loop/recursion in `O(1)` runtime?
///
/// https://leetcode.com/problems/add-digits/
struct Solution;
impl Solution {
    pub fn add_digits_my(mut num: i32) -> i32 {
        fn digit_sum(mut n: i32) -> i32 {
            let mut result = 0;
            while n > 0 {
                result += n % 10;
                n /= 10;
            }
            result
        }

        while num > 9 {
            num = digit_sum(num);
        }

        num
    }

    /// from other submissions
    /// https://leetcode.com/submissions/detail/637428960/
    pub fn add_digits_from_other_solutions(mut num: i32) -> i32 {
        while num > 9 {
            num = num / 10 + num % 10;
        }
        num
    }

    /// Approach 1: Mathematical: Digital Root
    /// https://leetcode.com/problems/add-digits/solution/
    pub fn add_digits(num: i32) -> i32 {
        if num == 0 {
            0
        } else if num % 9 == 0 {
            9
        } else {
            num % 9
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_38() {
        assert_eq!(Solution::add_digits(38), 2);
        // Explanation: The process is
        // 38 --> 3 + 8 --> 11
        // 11 --> 1 + 1 --> 2
        // Since 2 has only one digit, return it.
    }
    #[rustfmt::skip] #[test] fn n_0() { assert_eq!(Solution::add_digits(0), 0); }
    #[rustfmt::skip] #[test] fn n_1() { assert_eq!(Solution::add_digits(1), 1); }
    #[rustfmt::skip] #[test] fn n_2() { assert_eq!(Solution::add_digits(2), 2); }
    #[rustfmt::skip] #[test] fn n_3() { assert_eq!(Solution::add_digits(3), 3); }
    #[rustfmt::skip] #[test] fn n_4() { assert_eq!(Solution::add_digits(4), 4); }
    #[rustfmt::skip] #[test] fn n_5() { assert_eq!(Solution::add_digits(5), 5); }
    #[rustfmt::skip] #[test] fn n_6() { assert_eq!(Solution::add_digits(6), 6); }
    #[rustfmt::skip] #[test] fn n_7() { assert_eq!(Solution::add_digits(7), 7); }
    #[rustfmt::skip] #[test] fn n_8() { assert_eq!(Solution::add_digits(8), 8); }
    #[rustfmt::skip] #[test] fn n_9() { assert_eq!(Solution::add_digits(9), 9); }
    #[rustfmt::skip] #[test] fn n_10() { assert_eq!(Solution::add_digits(10), 1); }
    #[rustfmt::skip] #[test] fn n_i32_max() { assert_eq!(Solution::add_digits(i32::MAX), 1); }
}
