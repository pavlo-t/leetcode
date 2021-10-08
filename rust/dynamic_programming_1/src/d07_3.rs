#![allow(dead_code)]
/// 122. Best Time to Buy and Sell Stock II
/// =======================================
///
/// You are given an integer array `prices` where `prices[i]` is the price of a given stock on the `i`th day.
///
/// On each day, you may decide to buy and/or sell the stock.
/// You can only hold __at most one__ share of the stock at any time.
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
        let mut result = 0;
        for i in 1..prices.len() {
            if prices[i - 1] < prices[i] {
                result += prices[i] - prices[i - 1];
            }
        }
        result
    }

    /// 0.12 sec
    pub fn max_profit_dp_optimized(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp = vec![0; 10002];
        let sell_price = prices[n - 1] + 1;
        for buy_price in 1..10001 {
            dp[buy_price] = sell_price - buy_price as i32;
        }
        for i in (0..prices.len() - 1).rev() {
            let price = prices[i] + 1;
            let next_no_hold = dp[0];
            dp[0] = next_no_hold.max(dp[price as usize]);
            for bp in 1..10001 {
                let buy_price = bp as i32;
                dp[bp] = dp[bp].max(price - buy_price + next_no_hold);
            }
        }
        dp[0]
    }

    /// 0.90 sec
    pub fn max_profit_dp_vec(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp = vec![vec![0; 10002]; n];
        let last = n - 1;
        let price = prices[last];
        for bp in 1..10001 {
            dp[last][bp] = price - bp as i32 + 1;
        }
        for i in (0..prices.len() - 1).rev() {
            let price = prices[i];
            let pk = price as usize + 1;
            dp[i][0] = dp[i + 1][0].max(dp[i + 1][pk]);
            for bp in 1..10001 {
                dp[i][bp] = dp[i + 1][bp].max(price - bp as i32 + 1 + dp[i + 1][0]);
            }
        }
        dp[0][0]
    }

    /// 5.41 sec
    pub fn max_profit_recursion_with_memo(prices: Vec<i32>) -> i32 {
        fn rec(i: usize, bp: Option<i32>, memo: &mut Vec<Vec<i32>>, ps: &[i32]) -> i32 {
            if i == ps.len() - 1 {
                bp.map(|bp| 0.max(ps[i] - bp)).unwrap_or(0)
            } else {
                let p = bp.map(|p| p as usize + 1).unwrap_or(0);
                if memo[i][p] >= 0 {
                    memo[i][p]
                } else if let Some(bp) = bp {
                    let sell = ps[i] - bp + rec(i + 1, None, memo, ps);
                    let hold = rec(i + 1, Some(bp), memo, ps);
                    memo[i][p] = sell.max(hold);
                    memo[i][p]
                } else {
                    let buy = rec(i + 1, Some(ps[i]), memo, ps);
                    let do_not_buy = rec(i + 1, None, memo, ps);
                    memo[i][p] = buy.max(do_not_buy);
                    memo[i][p]
                }
            }
        }
        rec(0, None, &mut vec![vec![-1; 10002]; prices.len()], &prices)
    }
    pub fn max_profit_recursion_brute_force(prices: Vec<i32>) -> i32 {
        fn rec(i: usize, bp: Option<i32>, ps: &[i32]) -> i32 {
            if i == ps.len() - 1 {
                bp.map(|bp| 0.max(ps[i] - bp)).unwrap_or(0)
            } else if let Some(bp) = bp {
                let sell = ps[i] - bp + rec(i + 1, None, ps);
                let hold = rec(i + 1, Some(bp), ps);
                sell.max(hold)
            } else {
                rec(i + 1, Some(ps[i]), ps).max(rec(i + 1, None, ps))
            }
        }
        rec(0, None, &prices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_7_1_5_3_6_4() {
        let p = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(p), 7);
        // Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
        // Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
        // Total profit is 4 + 3 = 7.
    }
    #[test]
    fn p_1_2_3_4_5() {
        let p = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::max_profit(p), 4);
        // Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
        // Total profit is 4.
    }
    #[test]
    fn p_7_6_4_3_1() {
        let p = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(p), 0);
        // Explanation: There is no way to make a positive profit,
        // so we never buy the stock to achieve the maximum profit of 0.
    }
    #[rustfmt::skip] #[test] fn p_1() { assert_eq!(Solution::max_profit(vec![1]), 0); }
    #[rustfmt::skip] #[test] fn p_1_1() { assert_eq!(Solution::max_profit(vec![1, 1]), 0); }
    #[rustfmt::skip] #[test] fn p_2_1() { assert_eq!(Solution::max_profit(vec![2, 1]), 0); }
    #[rustfmt::skip] #[test] fn p_1_2() { assert_eq!(Solution::max_profit(vec![1, 2]), 1); }

    #[test]
    fn p_2_1_2_0_1() {
        let p = vec![2, 1, 2, 0, 1];
        assert_eq!(Solution::max_profit(p), 2);
    }

    /// If getting stack overflow:
    /// Add `RUST_MIN_STACK=67108864` to env:
    /// RUST_MIN_STACK=67108864 cargo test --lib d07_3
    #[test]
    fn p_30000x1() {
        let p = vec![1; 30000];
        assert_eq!(Solution::max_profit(p), 0);
    }
    #[test]
    fn p_1to10000x3() {
        let p = (1..=10000).chain(1..=10000).chain(1..=10000).collect();
        assert_eq!(Solution::max_profit(p), 29997);
    }
}
