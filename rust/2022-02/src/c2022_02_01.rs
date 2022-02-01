#![allow(dead_code)]
/// 121. Best Time to Buy and Sell Stock
/// ====================================
///
/// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut min_so_far = i32::MAX;
        for p in prices {
            min_so_far = min_so_far.min(p);
            result = result.max(p - min_so_far);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_7_1_5_3_6_4() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }
    #[test]
    fn p_7_6_4_3_1() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
