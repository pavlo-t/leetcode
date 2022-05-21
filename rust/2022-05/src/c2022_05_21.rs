#![allow(dead_code)]
/// \#322. Coin Change
/// ==================
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
    pub fn coin_change_rec(coins: Vec<i32>, amount: i32) -> i32 {
        fn rec(i: usize, a: usize, coins: &[usize]) -> i32 {
            if a == 0 {
                0
            } else if i >= coins.len() || coins[i] > a {
                i32::MAX
            } else if coins[i] == a {
                1
            } else {
                let curr = rec(i, a - coins[i], coins);
                let next = rec(i + 1, a, coins);
                if curr == i32::MAX {
                    next
                } else {
                    next.min(curr + 1)
                }
            }
        }

        let coins = coins.into_iter().map(|c| c as usize).collect::<Vec<_>>();
        let amount = amount as usize;
        match rec(0, amount, &coins) {
            i32::MAX => -1,
            r => r,
        }
    }

    pub fn coin_change_rec_with_memo(coins: Vec<i32>, amount: i32) -> i32 {
        fn rec(i: usize, a: usize, coins: &[usize], memo: &mut Vec<Vec<i32>>) -> i32 {
            if a == 0 {
                0
            } else if i >= coins.len() {
                i32::MAX
            } else {
                if memo[i][a] == -1 {
                    memo[i][a] = if coins[i] > a {
                        rec(i + 1, a, coins, memo)
                    } else {
                        let take = rec(0, a - coins[i], coins, memo);
                        let skip = rec(i + 1, a, coins, memo);
                        if take == i32::MAX {
                            skip
                        } else {
                            skip.min(take + 1)
                        }
                    }
                }
                memo[i][a]
            }
        }

        let coins = coins.into_iter().map(|c| c as usize).collect::<Vec<_>>();
        let amount = amount as usize;
        let mut memo = vec![vec![-1; amount + 1]; coins.len() + 1];

        match rec(0, amount, &coins, &mut memo) {
            i32::MAX => -1,
            r => r,
        }
    }

    /// from the last time I solved it
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
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
    fn c_1_2_5_a_11() {
        let c = vec![1, 2, 5];
        assert_eq!(Solution::coin_change(c, 11), 3);
        // Explanation: 11 = 5 + 5 + 1
    }
    #[test]
    fn c_2_a_3() {
        let c = vec![2];
        assert_eq!(Solution::coin_change(c, 3), -1);
    }
    #[test]
    fn c_1_a_0() {
        let c = vec![1];
        assert_eq!(Solution::coin_change(c, 0), 0);
    }

    #[test]
    fn c_195_265_404_396_a_3239() {
        let c = vec![195, 265, 404, 396];
        assert_eq!(Solution::coin_change(c, 3239), 12);
    }
    #[test]
    fn c_474_83_404_3_a_264() {
        let c = vec![474, 83, 404, 3];
        assert_eq!(Solution::coin_change(c, 264), 8);
    }

    #[test]
    fn c_1to12_a_10000() {
        let c = (1..=12).collect();
        assert_eq!(Solution::coin_change(c, 10000), 834);
    }
    #[test]
    fn c_12to1_a_10000() {
        let c = (1..=12).rev().collect();
        assert_eq!(Solution::coin_change(c, 10000), 834);
    }
    #[test]
    fn c_2to24by2_a_9999() {
        let c = (2..=24).filter(|&c| c % 2 == 0).collect();
        assert_eq!(Solution::coin_change(c, 9999), -1);
    }
}
