#![allow(dead_code)]
/// 122. Best Time to Buy and Sell Stock II
/// =======================================
///
/// You are given an integer array `prices` where `prices[i]` is the price of a given stock on the `i`th day.
///
/// On each day, you may decide to buy and/or sell the stock.
/// You can only hold __at most one share__ of the stock at any time.
/// However, you can buy it then immediately sell it on the __same day__.
///
/// Find and return _the __maximum__ profit you can achieve_.
///
/// __Constraints:__
///
/// - `1 <= prices.length <= 30_000`
/// - `0 <= prices[i] <= 10_000`
///
/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        prices
            .windows(2)
            .map(|w| w[1] - w[0])
            .filter(|&d| d > 0)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_1() {
        let p = vec![1];
        assert_eq!(Solution::max_profit(p), 0);
    }
    #[test]
    fn p_715364() {
        let p = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(p), 7);
        // Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
        // Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
        // Total profit is 4 + 3 = 7.
    }
    #[test]
    fn p_12345() {
        let p = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::max_profit(p), 4);
        // Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
        // Total profit is 4.
    }
    #[test]
    fn p_76431() {
        let p = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(p), 0);
        // Explanation: There is no way to make a positive profit,
        // so we never buy the stock to achieve the maximum profit of 0.
    }

    #[test]
    fn p_0to9999_repeat_3() {
        let p = (0..30000).map(|i| i % 10000).collect();
        assert_eq!(Solution::max_profit(p), 29_997);
    }
}
