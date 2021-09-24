#![allow(dead_code)]
/// N-th Tribonacci Number
/// ======================
///
/// The Tribonacci sequence Tn is defined as follows:
///
/// T0 = 0, T1 = 1, T2 = 1, and Tn+3 = Tn + Tn+1 + Tn+2 for n >= 0.
///
/// Given `n`, return the value of Tn.
///
/// __Constraints:__
///
/// - `0 <= n <= 37`
/// - The answer is guaranteed to fit within a 32-bit integer, ie. `answer <= 2^31 - 1`.
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/639/week-4-september-22nd-september-28th/3986/
struct Solution;
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut rs = [0, 1, 1, 2];
        if n < 4 {
            rs[3] = rs[n as usize];
        } else {
            for _ in 3..n {
                rs.swap(0, 1);
                rs.swap(1, 2);
                rs.swap(2, 3);
                rs[3] = rs[0] + rs[1] + rs[2];
            }
        }
        rs[3]
    }
    pub fn tribonacci_rec(n: i32) -> i32 {
        fn t_rec(n: i32, r0: i32, r1: i32, r2: i32) -> i32 {
            if n == 0 {
                r0 + r1 + r2
            } else {
                t_rec(n - 1, r1, r2, r0 + r1 + r2)
            }
        }

        match n {
            0 => 0,
            1 => 1,
            2 => 1,
            n => t_rec(n - 3, 0, 1, 1),
        }
    }
    pub fn tribonacci_rec_timeout(n: i32) -> i32 {
        //println!("tribonacci({})", n);
        match n {
            0 => 0,
            1 => 1,
            2 => 1,
            n => Self::tribonacci(n - 1) + Self::tribonacci(n - 2) + Self::tribonacci(n - 3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_0() {
        assert_eq!(Solution::tribonacci(0), 0);
    }
    #[test]
    fn n_1() {
        assert_eq!(Solution::tribonacci(1), 1);
    }
    #[test]
    fn n_2() {
        assert_eq!(Solution::tribonacci(2), 1);
    }
    #[test]
    fn n_3() {
        assert_eq!(Solution::tribonacci(3), 2);
    }
    #[test]
    fn n_4() {
        assert_eq!(Solution::tribonacci(4), 4);
        // Explanation:
        // T_3 = 0 + 1 + 1 = 2
        // T_4 = 1 + 1 + 2 = 4
    }
    #[test]
    fn n_5() {
        assert_eq!(Solution::tribonacci(5), 7);
    }
    #[test]
    fn n_25() {
        assert_eq!(Solution::tribonacci(25), 1_389_537);
    }
    #[test]
    fn n_37() {
        assert_eq!(Solution::tribonacci(37), 2_082_876_103);
    }
}
