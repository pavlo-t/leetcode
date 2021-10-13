#![allow(dead_code)]
/// 309. Best Time to Buy and Sell Stock with Cooldown
/// ==================================================
///
/// You are given an array `prices` where `prices[i]` is the price of a given stock on the `i`th day.
///
/// Find the maximum profit you can achieve.
/// You may complete as many transactions as you like
/// (i.e., buy one and sell one share of the stock multiple times)
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
    /// Approach 1: Dynamic Programming with State Machine
    /// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/solution/
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut sold, mut held, mut reset) = (i32::MIN, i32::MIN, 0);
        for price in prices {
            let prev_sold = sold;

            sold = held + price;
            held = held.max(reset - price);
            reset = reset.max(prev_sold);
        }
        sold.max(reset)
    }
    pub fn max_profit_rec_with_memo(prices: Vec<i32>) -> i32 {
        println!("max_profit({:?})", prices);
        fn rec(i: usize, bp: Option<i32>, ps: &[i32], memo: &mut Vec<Vec<i32>>) -> i32 {
            if i >= ps.len() {
                0
            } else {
                let p = bp.map(|p| p + 1).unwrap_or(0) as usize;
                if memo[i][p] >= 0 {
                    memo[i][p]
                } else if let Some(bp) = bp {
                    let sell = ps[i] - bp + rec(i + 2, None, ps, memo);
                    let hold = rec(i + 1, Some(bp), ps, memo);
                    memo[i][p] = sell.max(hold);
                    memo[i][p]
                } else {
                    let buy = rec(i + 1, Some(ps[i]), ps, memo);
                    let skip = rec(i + 1, None, ps, memo);
                    memo[i][p] = buy.max(skip);
                    memo[i][p]
                }
            }
        }
        let mut memo = vec![vec![-1; 1002]; prices.len()];
        rec(0, None, &prices, &mut memo)
    }
    pub fn max_profit_recursion_brute_force(prices: Vec<i32>) -> i32 {
        // cooldown
        const C: usize = 1;
        println!("max_profit({:?})", prices);
        fn rec(i: usize, bp: Option<i32>, ps: &[i32]) -> i32 {
            if i >= ps.len() {
                0
            } else if let Some(bp) = bp {
                let sell = ps[i] - bp + rec(i + 1 + C, None, ps);
                let hold = rec(i + 1, Some(bp), ps);
                sell.max(hold)
            } else {
                let buy = rec(i + 1, Some(ps[i]), ps);
                let skip = rec(i + 1, None, ps);
                buy.max(skip)
            }
        }
        rec(0, None, &prices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_1_2_3_0_2() {
        let p = vec![1, 2, 3, 0, 2];
        assert_eq!(Solution::max_profit(p), 3);
        // Explanation: transactions = [buy, sell, cooldown, buy, sell]
    }
    #[test]
    fn p_1() {
        let p = vec![1];
        assert_eq!(Solution::max_profit(p), 0);
    }
    #[test]
    fn p_1_2() {
        let p = vec![1, 2];
        assert_eq!(Solution::max_profit(p), 1);
    }

    #[test]
    fn p_1to1000x5() {
        let mut p = Vec::with_capacity(5000);
        for _ in 0..5 {
            (1..=1000).for_each(|i| p.push(i));
        }
        assert_eq!(Solution::max_profit(p), 4991);
    }
}
