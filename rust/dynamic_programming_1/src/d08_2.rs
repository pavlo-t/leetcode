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
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let (mut held, mut sold) = (-prices[0], 0);
        for price in prices {
            let p_held = held;
            held = held.max(sold - price);
            sold = sold.max(p_held + price - fee);
        }
        sold
    }
    pub fn max_profit_dp_array(prices: Vec<i32>, fee: i32) -> i32 {
        let n = prices.len();
        let mut dp = vec![(-prices[0], 0); n + 1];
        for i in 0..n {
            let price = prices[i];
            let (p_held, p_sold) = dp[i];
            dp[i + 1] = (p_held.max(p_sold - price), p_sold.max(p_held + price - fee));
        }
        dp[n].1
    }
    pub fn max_profit_rec(prices: Vec<i32>, fee: i32) -> i32 {
        fn rec(i: usize, ps: &[i32], fee: i32) -> (i32, i32) {
            if i == 0 {
                (-ps[0], 0)
            } else {
                let (p_held, p_sold) = rec(i - 1, ps, fee);
                let price = ps[i];
                (p_held.max(p_sold - price), p_sold.max(p_held + price - fee))
            }
        }
        rec(prices.len() - 1, &prices, fee).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_1_3_2_8_4_9_f_2() {
        let p = vec![1, 3, 2, 8, 4, 9];
        //held -inf,-1,-1,-1,-1, 1, 1
        //sold    0, 0, 0, 0, 5, 5, 8
        // held = p_held.max(p_sold - price)
        // sold = p_sold.max(p_held + price - fee)
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
    #[test]
    fn p_1_f_2() {
        assert_eq!(Solution::max_profit(vec![1], 2), 0);
    }
}
