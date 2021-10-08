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
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut min = i32::MAX;
        for p in prices {
            min = min.min(p);
            result = result.max(p - min);
        }
        result
    }
    pub fn max_profit_brute_force(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        for b in 0..prices.len() - 1 {
            for s in b + 1..prices.len() {
                result = result.max(prices[s] - prices[b]);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_7_1_5_3_6_4() {
        let p = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(p), 5);
        // Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
        // Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
    }
    #[test]
    fn p_7_6_4_3_1() {
        let p = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(p), 0);
        // Explanation: In this case, no transactions are done and the max profit = 0.
    }

    #[test]
    fn p_100000x1() {
        let p = vec![1; 100_000];
        assert_eq!(Solution::max_profit(p), 0);
    }
    #[test]
    fn p_1to100000() {
        let p = (1..=100_000).collect();
        assert_eq!(Solution::max_profit(p), 99_999);
    }
}
