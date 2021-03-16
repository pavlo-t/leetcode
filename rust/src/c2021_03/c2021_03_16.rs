/// # Best Time to Buy and Sell Stock with Transaction Fee
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
/// - `1 < prices.length <= 50_000`
/// - `0 < prices[i], fee < 50_000`
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/590/week-3-march-15th-march-21st/3674/
struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut cash = 0;
        let mut hold = -prices[0];
        for &price in &prices[1..] {
            cash = cash.max(hold + price - fee);
            hold = hold.max(cash - price);
        }
        cash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_p1_3_2_8_4_9_f2_produces_8() {
        assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
        // Explanation: The maximum profit can be achieved by:
        // - Buying at prices[0] = 1
        // - Selling at prices[3] = 8
        // - Buying at prices[4] = 4
        // - Selling at prices[5] = 9
        // The total profit is ((8 - 1) - 2) + ((9 - 4) - 2) = 8.
    }
    #[test]
    fn example2_p1_3_7_5_10_3_f3_produces_6() {
        assert_eq!(Solution::max_profit(vec![1, 3, 7, 5, 10, 3], 3), 6);
    }

    #[test]
    fn p1_f0_produces_0() {
        assert_eq!(Solution::max_profit(vec![1], 0), 0);
    }
    #[test]
    fn p1_2_f0_produces_1() {
        assert_eq!(Solution::max_profit(vec![1, 2], 0), 1);
    }
    #[test]
    fn p1_2_f2_produces_0() {
        assert_eq!(Solution::max_profit(vec![1, 2], 2), 0);
    }
    #[test]
    fn p1_2_3_4_5_f0_produces_4() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5], 0), 4);
    }
    #[test]
    fn p1_2_3_4_5_f3_produces_1() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5], 3), 1);
    }
    #[test]
    fn p1_9_1_9_1_9_f3_produces_15() {
        assert_eq!(Solution::max_profit(vec![1, 9, 1, 9, 1, 9], 3), 15);
    }

    #[test]
    // #[ignore]
    fn performance_p1to50000_f0_produces_49999() {
        let prices = (1..=50000).collect();
        let fee = 0;
        assert_eq!(Solution::max_profit(prices, fee), 49999);
    }
    #[test]
    // #[ignore]
    fn performance_p1to50000_f50000_produces_0() {
        let prices = (1..=50000).collect();
        let fee = 50000;
        assert_eq!(Solution::max_profit(prices, fee), 0);
    }
}
