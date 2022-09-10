#![allow(dead_code)]
//! \#188. Best Time to Buy and Sell Stock IV
//! =========================================
//!
//! <https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv>
//!
//! You are given an integer array `prices` where `prices[i]` is the price of a given stock on the `i`th day,
//! and an integer `k`.
//!
//! Find the maximum profit you can achieve. You may complete at most `k` transactions.
//!
//! __Note:__ You may not engage in multiple transactions simultaneously
//! (i.e., you must sell the stock before you buy again).
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_09::c2022_09_10::*;
//! assert_eq!(Solution::max_profit(2, vec![2, 4, 1]), 2);
//! ```
//!
//! __Explanation:__ Buy on day `1` (`price = 2`) and sell on day `2` (`price = 4`), `profit = 4-2 = 2`.
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_09::c2022_09_10::*;
//! assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
//! ```
//!
//! __Explanation:__
//! Buy on day `2` (`price = 2`) and sell on day `3` (`price = 6`), `profit = 6-2 = 4`.
//! Then buy on day `5` (`price = 0`) and sell on day `6` (`price = 3`), `profit = 3-0 = 3`.
//!
//! ###### Constraints
//!
//! - `0 <= k <= 100`
//! - `0 <= prices.length <= 1000`
//! - `0 <= prices[i] <= 1000`

pub struct Solution;
impl Solution {
    /// Recursion
    pub fn max_profit_v1(k: i32, prices: Vec<i32>) -> i32 {
        fn rec(i: usize, k: usize, holding: bool, prices: &[i32]) -> i32 {
            if i == prices.len() || (!holding && k == 0) {
                0
            } else if holding {
                let sell = prices[i] + rec(i + 1, k, false, prices);
                let keep = rec(i + 1, k, true, prices);
                sell.max(keep)
            } else {
                let buy = -prices[i] + rec(i + 1, k - 1, true, prices);
                let skip = rec(i + 1, k, false, prices);
                buy.max(skip)
            }
        }

        rec(0, k as usize, false, &prices)
    }

    /// Recursion with memoization
    pub fn max_profit_v2(k: i32, prices: Vec<i32>) -> i32 {
        fn rec(
            i: usize,
            k: usize,
            holding: bool,
            prices: &[i32],
            memo: &mut Vec<Vec<[i32; 2]>>,
        ) -> i32 {
            if i == prices.len() || (!holding && k == 0) {
                0
            } else if holding {
                if memo[i][k][1] == -1 {
                    let sell = prices[i] + rec(i + 1, k, false, prices, memo);
                    let keep = rec(i + 1, k, true, prices, memo);

                    memo[i][k][1] = sell.max(keep)
                }
                memo[i][k][1]
            } else {
                if memo[i][k][0] == -1 {
                    let buy = -prices[i] + rec(i + 1, k - 1, true, prices, memo);
                    let skip = rec(i + 1, k, false, prices, memo);

                    memo[i][k][0] = buy.max(skip);
                }
                memo[i][k][0]
            }
        }

        let k = k as usize;
        let mut memo = vec![vec![[-1; 2]; k + 1]; prices.len()];

        rec(0, k, false, &prices, &mut memo)
    }

    /// DP 3 dimensions
    pub fn max_profit_v3(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let k = k as usize;
        let mut dp = vec![vec![[0; 2]; k + 1]; n + 1];
        for i in (0..n).rev() {
            dp[i][0][1] = dp[i + 1][0][1].max(prices[i]);
            for j in 1..=k {
                dp[i][j][0] = dp[i + 1][j][0].max(dp[i + 1][j - 1][1] - prices[i]);
                dp[i][j][1] = dp[i + 1][j][1].max(dp[i + 1][j][0] + prices[i]);
            }
        }

        dp[0][k][0]
    }

    /// DP 2 dimensions
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let k = k as usize;
        let mut dp = vec![[0; 2]; k + 1];
        for i in (0..n).rev() {
            dp[0][1] = dp[0][1].max(prices[i]);
            for j in (1..=k).rev() {
                dp[j][1] = dp[j][1].max(dp[j][0] + prices[i]);
                dp[j][0] = dp[j][0].max(dp[j - 1][1] - prices[i]);
            }
        }

        dp[k][0]
    }
}
