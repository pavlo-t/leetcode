#![allow(dead_code)]
/// 309. Best Time to Buy and Sell Stock with Cooldown
/// ==================================================
///
/// You are given an array `prices` where `prices[i]` is the price of a given stock on the `i`th day.
///
/// Find the maximum profit you can achieve.
/// You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times)
/// with the following restrictions:
///
/// - After you sell your stock, you cannot buy stock on the next day (i.e., cooldown one day).
///
/// __Note:__ You may not engage in multiple transactions simultaneously
/// (i.e., you must sell the stock before you buy again).
///
/// __Constraints:__
///
/// - `1 <= prices.length <= 5000`
/// - `0 <= prices[i] <= 1000`
///
/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/
struct Solution;
impl Solution {
    /// 21:57-22:00
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        let (mut holding, mut not_holding, mut cooldown) = (0, 0, 0);
        for i in (0..prices.len()).rev() {
            let prev_cooldown = cooldown;
            cooldown = not_holding;
            not_holding = not_holding.max(holding - prices[i]);
            holding = holding.max(prev_cooldown + prices[i]);
        }
        not_holding
    }
    /// 21:57-22:00
    pub fn max_profit_dp_vec(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        // s: state - 0 = holding, 1 = not_holding, 2 = cooldown
        let mut dp = vec![[0; 3]; prices.len() + 1];
        for i in (0..prices.len()).rev() {
            dp[i][0] = dp[i + 1][0].max(dp[i + 1][2] + prices[i]);
            dp[i][1] = dp[i + 1][1].max(dp[i + 1][0] - prices[i]);
            dp[i][2] = dp[i + 1][1];
        }
        dp[0][1]
    }
    /// 21:55-21:57
    pub fn max_profit_rec_with_memo(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        /// s: state - 0 = holding, 1 = not_holding, 2 = cooldown
        fn rec(i: usize, s: usize, ps: &[i32], memo: &mut Vec<[i32; 3]>) -> i32 {
            if i == ps.len() {
                0
            } else if memo[i][s] >= 0 {
                memo[i][s]
            } else {
                memo[i][s] = match s {
                    0 => rec(i + 1, 0, ps, memo).max(rec(i + 1, 2, ps, memo) + ps[i]),
                    1 => rec(i + 1, 1, ps, memo).max(rec(i + 1, 0, ps, memo) - ps[i]),
                    _ => rec(i + 1, s - 1, ps, memo),
                };
                memo[i][s]
            }
        }
        let mut memo = vec![[-1; 3]; prices.len()];
        rec(0, 1, &prices, &mut memo)
    }
    pub fn max_profit_rec(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        /// s: state - 0 = holding, 1 = not_holding, 2 = cooldown
        fn rec(i: usize, s: usize, ps: &[i32]) -> i32 {
            if i == ps.len() {
                0
            } else {
                match s {
                    0 => rec(i + 1, 0, ps).max(rec(i + 1, 2, ps) + ps[i]),
                    1 => rec(i + 1, 1, ps).max(rec(i + 1, 0, ps) - ps[i]),
                    _ => rec(i + 1, s - 1, ps),
                }
            }
        }
        rec(0, 1, &prices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_1() {
        assert_eq!(Solution::max_profit(vec![0]), 0);
    }
    #[test]
    fn p_12302() {
        let p = vec![1, 2, 3, 0, 2];
        assert_eq!(Solution::max_profit(p), 3);
        // Explanation: transactions = [buy, sell, cooldown, buy, sell]
    }
    #[test]
    fn p_1to1000_repeat_5() {
        let p = (0..5).flat_map(|_| 1..=1000).collect();
        assert_eq!(Solution::max_profit(p), 4991);
    }
}
