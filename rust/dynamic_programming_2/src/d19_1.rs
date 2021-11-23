#![allow(dead_code)]
/// 322. Coin Change
/// ================
///
/// You are given an integer array `coins` representing coins of different denominations
/// and an integer `amount` representing a total amount of money.
///
/// Return _the fewest number of coins that you need to make up that amount_.
/// If that amount of money cannot be made up by any combination of the coins, return `-1`.
///
/// You may assume that you have an infinite number of each kind of coin.
///
/// __Constraints:__
///
/// - `1 <= coins.length <= 12`
/// - `1 <= coins[i] <= 2**31 - 1`
/// - `0 <= amount <= 10_000`
///
/// https://leetcode.com/problems/coin-change/
struct Solution;
impl Solution {
    /// 23:09-23:16
    pub fn coin_change_rec(coins: Vec<i32>, amount: i32) -> i32 {
        println!("coin_change({:?}, {})", coins, amount);
        fn rec(i: usize, a: i32, c: &[i32]) -> i32 {
            if a == 0 {
                0
            } else if a < 0 || i == c.len() {
                i32::MAX
            } else {
                rec(i + 1, a, c).min(rec(i, a - c[i], c).saturating_add(1))
            }
        }
        match rec(0, amount, &coins) {
            i32::MAX => -1,
            r => r,
        }
    }
    /// 23:16-23:20
    pub fn coin_change_rec_with_memo(coins: Vec<i32>, amount: i32) -> i32 {
        println!("coin_change({:?}, {})", coins, amount);
        fn rec(i: usize, a: i32, c: &[i32], memo: &mut Vec<Vec<i32>>) -> i32 {
            if a == 0 {
                0
            } else if a < 0 || i == c.len() {
                i32::MAX
            } else if memo[i][a as usize] != -1 {
                memo[i][a as usize]
            } else {
                memo[i][a as usize] =
                    rec(i + 1, a, c, memo).min(rec(i, a - c[i], c, memo).saturating_add(1));
                memo[i][a as usize]
            }
        }
        let mut memo = vec![vec![-1; amount as usize + 1]; coins.len()];
        match rec(0, amount, &coins, &mut memo) {
            i32::MAX => -1,
            r => r,
        }
    }
    /// 23:20-23:31
    pub fn coin_change_dp_vec_vec(coins: Vec<i32>, amount: i32) -> i32 {
        println!("coin_change({:?}, {})", coins, amount);
        let am = amount as usize + 1;
        let n = coins.len();
        let mut dp = vec![vec![i32::MAX; am]; n + 1];
        for c in 0..n {
            dp[c][am - 1] = 0;
        }
        for c in (0..n).rev() {
            for a in (0..am.saturating_sub(coins[c] as usize)).rev() {
                dp[c][a] = dp[c + 1][a].min(dp[c][a + coins[c] as usize].saturating_add(1));
            }
        }
        match dp[0][0] {
            i32::MAX => -1,
            r => r,
        }
    }
    /// 23:31-23:34
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        println!("coin_change({:?}, {})", coins, amount);
        let am = amount as usize + 1;
        let n = coins.len();
        let mut dp = vec![i32::MAX; am];
        dp[am - 1] = 0;
        for c in (0..n).rev() {
            for a in (0..am.saturating_sub(coins[c] as usize)).rev() {
                dp[a] = dp[a].min(dp[a + coins[c] as usize].saturating_add(1));
            }
        }
        match dp[0] {
            i32::MAX => -1,
            r => r,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c125_a11() {
        let c = vec![1, 2, 5];
        let a = 11;
        assert_eq!(Solution::coin_change(c, a), 3);
        // Explanation: 11 = 5 + 5 + 1
    }
    #[test]
    fn c2_a3() {
        let c = vec![2];
        let a = 3;
        assert_eq!(Solution::coin_change(c, a), -1);
    }
    #[test]
    fn c1_a0() {
        let c = vec![1];
        let a = 0;
        assert_eq!(Solution::coin_change(c, a), 0);
    }
    #[test]
    fn c1_a1() {
        let c = vec![1];
        let a = 1;
        assert_eq!(Solution::coin_change(c, a), 1);
    }
    #[test]
    fn c1_a2() {
        let c = vec![1];
        let a = 2;
        assert_eq!(Solution::coin_change(c, a), 2);
    }

    #[test]
    fn c1to12_a10000() {
        let c = (1..=12).collect();
        let a = 10000;
        assert_eq!(Solution::coin_change(c, a), 834);
    }
}
