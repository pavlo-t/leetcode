#![allow(dead_code)]
/// 714. Best Time to Buy and Sell Stock with Transaction Fee
/// =========================================================
///
/// You are given an array `prices` where `prices[i]` is the price of a given stock on the `i`th day,
/// and an integer `fee` representing a transaction fee.
///
/// Find the maximum profit you can achieve.
/// You may complete as many transactions as you like,
/// but you need to pay the transaction fee for each transaction.
///
/// __Note:__ You may not engage in multiple transactions simultaneously
/// (i.e., you must sell the stock before you buy again).
///
/// __Constraints:__
///
/// - `1 <= prices.length <= 50_000`
/// - `1 <= prices[i] < 50_000`
/// - `0 <= fee < 50_000`
///
/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/
struct Solution;
impl Solution {
    /// 21:34-21:36
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        println!("max_profit({:?}, {})", prices, fee);
        let (mut hold, mut no_hold) = (0, 0);
        for i in (0..prices.len()).rev() {
            let prev_hold = hold;
            hold = hold.max(no_hold + prices[i] - fee);
            no_hold = no_hold.max(prev_hold - prices[i]);
        }
        no_hold
    }
    /// 21:31-21:34
    pub fn max_profit_dp_vec(prices: Vec<i32>, fee: i32) -> i32 {
        println!("max_profit({:?}, {})", prices, fee);
        let mut dp = vec![[0; 2]; prices.len() + 1];
        for i in (0..prices.len()).rev() {
            dp[i][1] = dp[i + 1][1].max(dp[i + 1][0] + prices[i] - fee);
            dp[i][0] = dp[i + 1][0].max(dp[i + 1][1] - prices[i]);
        }
        dp[0][0]
    }
    /// 21:26-21:31
    pub fn max_profit_rec_with_memo(prices: Vec<i32>, fee: i32) -> i32 {
        println!("max_profit({:?}, {})", prices, fee);
        fn rec(i: usize, h: usize, f: i32, ps: &[i32], memo: &mut Vec<[i32; 2]>) -> i32 {
            if i == ps.len() {
                0
            } else if memo[i][h] != -1 {
                memo[i][h]
            } else {
                memo[i][h] = if h == 1 {
                    rec(i + 1, h, f, ps, memo).max(rec(i + 1, 0, f, ps, memo) + ps[i] - f)
                } else {
                    rec(i + 1, h, f, ps, memo).max(rec(i + 1, 1, f, ps, memo) - ps[i])
                };
                memo[i][h]
            }
        }
        let mut memo = vec![[-1; 2]; prices.len()];
        rec(0, 0, fee, &prices, &mut memo)
    }
    /// 21:22-21:26
    pub fn max_profit_rec(prices: Vec<i32>, fee: i32) -> i32 {
        println!("max_profit({:?}, {})", prices, fee);
        fn rec(i: usize, f: i32, h: bool, ps: &[i32]) -> i32 {
            if i == ps.len() {
                0
            } else if h {
                rec(i + 1, f, h, ps).max(rec(i + 1, f, false, ps) + ps[i] - f)
            } else {
                rec(i + 1, f, h, ps).max(rec(i + 1, f, true, ps) - ps[i])
            }
        }
        rec(0, fee, false, &prices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_1_f_1() {
        assert_eq!(Solution::max_profit(vec![1], 1), 0);
    }
    #[test]
    fn p_132849_f_2() {
        let p = vec![1, 3, 2, 8, 4, 9];
        assert_eq!(Solution::max_profit(p, 2), 8);
        // Explanation: The maximum profit can be achieved by:
        // - Buying at prices[0] = 1
        // - Selling at prices[3] = 8
        // - Buying at prices[4] = 4
        // - Selling at prices[5] = 9
        // The total profit is ((8 - 1) - 2) + ((9 - 4) - 2) = 8.
    }
    #[test]
    fn p_1_3_7_5_10_3_f_3() {
        let p = vec![1, 3, 7, 5, 10, 3];
        assert_eq!(Solution::max_profit(p, 3), 6);
    }

    /// If getting stack overflow:
    /// Add `RUST_MIN_STACK=67108864` to env:
    /// RUST_MIN_STACK=67108864 cargo test --lib d04_1
    #[test]
    fn p_1to50000_f_10() {
        let p = (1..=50000).collect();
        assert_eq!(Solution::max_profit(p, 10), 49989);
    }
}
