#![allow(dead_code)]
/// 121. Best Time to Buy and Sell Stock
/// ====================================
///
/// You are given an array `prices` where `prices[i]` is the price of a given stock on the `i`th day.
///
/// You want to maximize your profit by choosing a __single day__ to buy one stock
/// and choosing a __different day in the future__ to sell that stock.
///
/// Return _the maximum profit you can achieve from this transaction_.
/// If you cannot achieve any profit, return `0`.
///
/// __Constraints:__
///
/// - `1 <= prices.length <= 100_000`
/// - `0 <= prices[i] <= 10_000`
///
/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
struct Solution;
impl Solution {
    /// 03:15-03:18
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        let (mut hold, mut no_hold) = (0, 0);
        for i in (0..prices.len()).rev() {
            no_hold = no_hold.max(hold - prices[i]);
            hold = prices[i].max(hold);
        }
        no_hold
    }
    /// 03:10-03:15
    pub fn max_profit_dp_vec(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        let n = prices.len();
        let mut dp = vec![[0, 0]; n + 1];
        for i in (0..n).rev() {
            dp[i][0] = dp[i + 1][0].max(dp[i + 1][1] - prices[i]);
            dp[i][1] = prices[i].max(dp[i + 1][1]);
        }
        dp[0][0]
    }
    /// 03:04-03:10
    pub fn max_profit_rec_with_memo(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        fn rec(i: usize, holding: usize, ps: &[i32], memo: &mut Vec<[i32; 2]>) -> i32 {
            if i == ps.len() {
                0
            } else if memo[i][holding] >= 0 {
                memo[i][holding]
            } else {
                memo[i][holding] = if holding == 1 {
                    ps[i].max(rec(i + 1, holding, ps, memo))
                } else {
                    rec(i + 1, holding, ps, memo).max(rec(i + 1, 1, ps, memo) - ps[i])
                };
                memo[i][holding]
            }
        }
        let n = prices.len();
        let mut memo = vec![[-1, -1]; n];
        rec(0, 0, &prices, &mut memo)
    }
    /// 02:58-03:04
    pub fn max_profit_rec(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        fn rec(i: usize, holding: bool, ps: &[i32]) -> i32 {
            if i == ps.len() {
                0
            } else if holding {
                ps[i].max(rec(i + 1, holding, ps))
            } else {
                rec(i + 1, holding, ps).max(rec(i + 1, true, ps) - ps[i])
            }
        }
        rec(0, false, &prices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn p_7() { assert_eq!(Solution::max_profit(vec![7]), 0); }
    #[test]
    fn p_715364() {
        let p = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(p), 5);
        // Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
        // Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
    }
    #[test]
    fn p_76431() {
        let p = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(p), 0);
        // Explanation: In this case, no transactions are done and the max profit = 0.
    }

    /// If getting stack overflow:
    /// Add `RUST_MIN_STACK=67108864` to env:
    /// RUST_MIN_STACK=67108864 cargo test --lib d03_3
    #[rustfmt::skip] #[test] fn p_1to100000() { assert_eq!(Solution::max_profit((1..=100000).collect()), 99999); }
}
