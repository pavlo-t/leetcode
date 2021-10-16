#![allow(dead_code)]
/// 123. Best Time to Buy and Sell Stock III
/// ========================================
///
/// You are given an array `prices` where `prices[i]` is the price of a given stock on the `i`th day.
///
/// Find the maximum profit you can achieve.
/// You may complete __at most two transactions__.
///
/// __Note:__ You may not engage in multiple transactions simultaneously
/// (i.e., you must sell the stock before you buy again).
///
/// __Constraints:__
///
/// - `1 <= prices.length <= 100_000`
/// - `0 <= prices[i] <= 100_000`
///
/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/
struct Solution;
impl Solution {
    /// Approach 2: One-pass Simulation
    /// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/solution/
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        let (mut t1cost, mut t2cost, mut t1profit, mut t2profit) = (i32::MAX, i32::MAX, 0, 0);
        for price in prices {
            // the maximum profit if only one transaction is allowed
            t1cost = t1cost.min(price);
            t1profit = t1profit.max(price - t1cost);
            // reinvest the gained profit in the second transaction
            t2cost = t2cost.min(price - t1profit);
            t2profit = t2profit.max(price - t2cost);
        }
        t2profit
    }
    /// 18:22-18:33
    pub fn max_profit_dp(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        let (mut no_hold_ts_0, mut no_hold_ts_1, mut hold_ts_0, mut hold_ts_1) = (0, 0, 0, 0);
        for price in prices.into_iter().rev() {
            let nht1 = no_hold_ts_1;
            no_hold_ts_0 = no_hold_ts_0.max(hold_ts_0 - price);
            no_hold_ts_1 = no_hold_ts_1.max(hold_ts_1 - price);
            hold_ts_0 = hold_ts_0.max(nht1 + price);
            hold_ts_1 = hold_ts_1.max(price);
        }
        no_hold_ts_0
    }
    /// 18:05-18:22
    pub fn max_profit_dp_vec_vec_vec(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        let mut dp = vec![vec![vec![0; 2]; 2]; prices.len() + 1];
        for i in (0..prices.len()).rev() {
            dp[i][0][0] = dp[i + 1][0][0].max(dp[i + 1][1][0] - prices[i]);
            dp[i][0][1] = dp[i + 1][0][1].max(dp[i + 1][1][1] - prices[i]);
            dp[i][1][0] = dp[i + 1][1][0].max(dp[i + 1][0][1] + prices[i]);
            dp[i][1][1] = dp[i + 1][1][1].max(prices[i]);
        }
        dp[0][0][0]
    }
    /// 17:59-18:05
    pub fn max_profit_rev_with_memo(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        fn rec(i: usize, h: bool, ts: usize, ps: &[i32], memo: &mut Vec<Vec<Vec<i32>>>) -> i32 {
            let hi = h as usize;
            if i >= ps.len() || ts == 0 {
                0
            } else if memo[i][hi][ts] >= 0 {
                memo[i][hi][ts]
            } else {
                memo[i][hi][ts] = if h {
                    let sell = rec(i + 1, false, ts - 1, ps, memo) + ps[i];
                    let hold = rec(i + 1, true, ts, ps, memo);
                    sell.max(hold)
                } else {
                    let buy = rec(i + 1, true, ts, ps, memo) - ps[i];
                    let skip = rec(i + 1, false, ts, ps, memo);
                    buy.max(skip)
                };
                memo[i][hi][ts]
            }
        }
        let mut memo = vec![vec![vec![-1; 3]; 2]; prices.len()];
        rec(0, false, 2, &prices, &mut memo)
    }
    /// 17:36-17:59
    pub fn max_profit_rec(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        fn rec(i: usize, h: bool, ts: usize, ps: &[i32]) -> i32 {
            if i >= ps.len() || ts == 0 {
                0
            } else if h {
                let sell = rec(i + 1, false, ts - 1, ps) + ps[i];
                let hold = rec(i + 1, true, ts, ps);
                sell.max(hold)
            } else {
                let buy = rec(i + 1, true, ts, ps) - ps[i];
                let skip = rec(i + 1, false, ts, ps);
                buy.max(skip)
            }
        }
        rec(0, false, 2, &prices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ps_3_3_5_0_0_3_1_4() {
        let p = vec![3, 3, 5, 0, 0, 3, 1, 4];
        assert_eq!(Solution::max_profit(p), 6);
        // Explanation: Buy on day 4 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
        // Then buy on day 7 (price = 1) and sell on day 8 (price = 4), profit = 4-1 = 3.
    }
    #[test]
    fn ps_1_2_3_4_5() {
        let p = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::max_profit(p), 4);
        // Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
        // Note that you cannot buy on day 1, buy on day 2 and sell them later,
        // as you are engaging multiple transactions at the same time.  You must sell before buying again.
    }
    #[test]
    fn ps_7_6_4_3_1() {
        let p = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(p), 0);
        // Explanation: In this case, no transaction is done, i.e. max profit = 0.
    }
    #[test]
    fn ps_1() {
        let p = vec![1];
        assert_eq!(Solution::max_profit(p), 0);
    }

    #[test]
    fn ps_0_9_0_8_0_7_0_6_0_5() {
        let p = vec![0, 9, 0, 8, 0, 7, 0, 6, 0, 5];
        assert_eq!(Solution::max_profit(p), 17);
    }

    #[rustfmt::skip] #[test] fn ps_1_1() { assert_eq!(Solution::max_profit(vec![1,1]), 0); }
    #[rustfmt::skip] #[test] fn ps_1_2() { assert_eq!(Solution::max_profit(vec![1,2]), 1); }
    #[rustfmt::skip] #[test] fn ps_2_1() { assert_eq!(Solution::max_profit(vec![2,1]), 0); }

    /// ```sh
    /// RUST_MIN_STACK=67108864 cargo test --lib c2021_10_16
    /// ```
    //#[ignore]
    #[test]
    fn ps_100000x1() {
        let p = vec![1; 100_000];
        assert_eq!(Solution::max_profit(p), 0);
    }
    //#[ignore]
    #[test]
    fn ps_1to100000() {
        let p = (1..=100_000).collect();
        assert_eq!(Solution::max_profit(p), 99_999);
    }
}
