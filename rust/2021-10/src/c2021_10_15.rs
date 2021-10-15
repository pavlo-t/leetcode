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
/// __Note:__ You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
///
/// __Constraints:__
///
/// - `1 <= prices.length <= 5000`
/// - `0 <= prices[i] <= 1000`
///
/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/
struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        let (mut holding, mut can_buy, mut cooldown) = (0, 0, 0);
        for price in prices.iter().rev() {
            let prev_can_buy = can_buy;
            can_buy = can_buy.max(holding - price);
            holding = holding.max(cooldown + price);
            cooldown = prev_can_buy;
        }
        can_buy
    }
    pub fn max_profit_dp_vec(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        let n = prices.len();
        let mut dp = vec![vec![0; 2]; n + 2];
        for i in (0..n).rev() {
            dp[i][0] = dp[i + 1][0].max(dp[i + 1][1] - prices[i]);
            dp[i][1] = dp[i + 1][1].max(dp[i + 2][0] + prices[i]);
        }
        dp[0][0]
    }
    pub fn max_profit_rec_with_memo(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        fn rec(i: usize, h: bool, ps: &[i32], memo: &mut Vec<Vec<i32>>) -> i32 {
            let hi = h as usize;
            if i >= ps.len() {
                0
            } else if memo[i][hi] >= 0 {
                memo[i][hi]
            } else if h {
                let sell = rec(i + 2, false, ps, memo) + ps[i];
                let keep = rec(i + 1, true, ps, memo);
                memo[i][hi] = sell.max(keep);
                memo[i][hi]
            } else {
                let buy = rec(i + 1, true, ps, memo) - ps[i];
                let skip = rec(i + 1, false, ps, memo);
                memo[i][hi] = buy.max(skip);
                memo[i][hi]
            }
        }
        rec(0, false, &prices, &mut vec![vec![-1; 2]; prices.len()])
    }
    pub fn max_profit_rec(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        fn rec(i: usize, h: bool, ps: &[i32]) -> i32 {
            if i >= ps.len() {
                0
            } else if h {
                let sell = rec(i + 2, false, ps) + ps[i];
                let keep = rec(i + 1, true, ps);
                sell.max(keep)
            } else {
                let buy = rec(i + 1, true, ps) - ps[i];
                let skip = rec(i + 1, false, ps);
                buy.max(skip)
            }
        }
        rec(0, false, &prices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_1_2_3_0_2() {
        let p = vec![1, 2, 3, 0, 2];
        //  buy      3, 3, 2, 2, 0, 0]; nb max (ns - p)
        //  sell     _, 4, 3, 2, 2, 0]; ns max (nc + p)
        //  colldown _, 2, 2, 0, 0, 0]; nb
        assert_eq!(Solution::max_profit(p), 3);
        // Explanation: transactions = [buy, sell, cooldown, buy, sell]
    }
    #[test]
    fn p_2_3_0_2() {
        let p = vec![2, 3, 0, 2];
        assert_eq!(Solution::max_profit(p), 2);
    }
    #[test]
    fn p_1() {
        let p = vec![1];
        assert_eq!(Solution::max_profit(p), 0);
    }

    #[test]
    fn p_1to1000x5() {
        let p = (0..5).map(|_| (1..=1000)).flatten().collect();
        assert_eq!(Solution::max_profit(p), 4991);
    }
}
