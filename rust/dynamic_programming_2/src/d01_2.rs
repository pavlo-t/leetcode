#![allow(dead_code)]
/// 70. Climbing Stairs
/// ===================
///
/// You are climbing a staircase.
/// It takes `n` steps to reach the top.
///
/// Each time you can either climb `1` or `2` steps.
/// In how many distinct ways can you climb to the top?
///
/// __Constraints:__
///
/// - `1 <= n <= 45`
///
/// https://leetcode.com/problems/climbing-stairs/
struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        println!("climb_stairs({})", n);
        let (mut penultimate, mut ultimate) = (1, 1);
        for _ in 2..=n {
            std::mem::swap(&mut penultimate, &mut ultimate);
            ultimate += penultimate;
        }
        ultimate
    }
    pub fn climb_stairs_dp_vec_bottom_up(n: i32) -> i32 {
        println!("climb_stairs({})", n);
        let n = n as usize;
        let mut dp = vec![1; n + 1];
        for n in 2..=n {
            dp[n] = dp[n - 1] + dp[n - 2];
        }
        dp[n]
    }
    pub fn climb_stairs_rec_with_memo(n: i32) -> i32 {
        println!("climb_stairs({})", n);
        let n = n as usize;
        static mut MEMO: [i32; 46] = [-1; 46];

        fn rec(n: usize) -> i32 {
            unsafe {
                if n < 2 {
                    1
                } else if MEMO[n] > 0 {
                    MEMO[n]
                } else {
                    MEMO[n] = rec(n - 1) + rec(n - 2);
                    MEMO[n]
                }
            }
        }

        rec(n)
    }
    pub fn climb_stairs_rec(n: i32) -> i32 {
        println!("climb_stairs({})", n);
        fn rec(n: i32) -> i32 {
            if n < 2 {
                1
            } else {
                rec(n - 1) + rec(n - 2)
            }
        }
        rec(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1() { assert_eq!(Solution::climb_stairs(1), 1); }
    #[test]
    fn n_2() {
        assert_eq!(Solution::climb_stairs(2), 2);
        // Explanation: There are two ways to climb to the top.
        // 1. 1 step + 1 step
        // 2. 2 steps
    }
    #[test]
    fn n_3() {
        assert_eq!(Solution::climb_stairs(3), 3);
        // Explanation: There are three ways to climb to the top.
        // 1. 1 step + 1 step + 1 step
        // 2. 1 step + 2 steps
        // 3. 2 steps + 1 step
    }
    // 1111 112 121 211 22
    #[rustfmt::skip] #[test] fn n_4() { assert_eq!(Solution::climb_stairs(4), 5); }
    // 11111 1112 1121 1211 2111 122 212 221
    #[rustfmt::skip] #[test] fn n_5() { assert_eq!(Solution::climb_stairs(5), 8); }
    #[rustfmt::skip] #[test] fn n_45() { assert_eq!(Solution::climb_stairs(45), 1_836_311_903); }
}
