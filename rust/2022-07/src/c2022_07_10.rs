#![allow(dead_code)]
//! \#746. Min Cost Climbing Stairs
//! ===============================
//!
//! <https://leetcode.com/problems/min-cost-climbing-stairs>
//!
//! You are given an integer array `cost` where `cost[i]` is the cost of `i`th step on a staircase.
//! Once you pay the cost, you can either climb one or two steps.
//!
//! You can either start from the step with index `0`, or the step with index `1`.
//!
//! Return _the minimum cost to reach the top of the floor_.
//!
//! __Constraints:__
//!
//! - `2 <= cost.length <= 1000`
//! - `0 <= cost[i] <= 999`

pub struct Solution;
impl Solution {
    pub fn min_cost_climbing_stairs_rec(cost: Vec<i32>) -> i32 {
        fn rec(i: usize, cost: &[i32]) -> i32 {
            if i >= cost.len() {
                0
            } else {
                cost[i] + rec(i + 1, cost).min(rec(i + 2, cost))
            }
        }
        rec(0, &cost).min(rec(1, &cost))
    }

    pub fn min_cost_climbing_stairs_rec_with_memo(cost: Vec<i32>) -> i32 {
        fn rec(i: usize, cost: &[i32], memo: &mut Vec<i32>) -> i32 {
            if memo[i] == -1 {
                memo[i] = if i >= cost.len() {
                    0
                } else {
                    cost[i] + rec(i + 1, cost, memo).min(rec(i + 2, cost, memo))
                };
            }
            memo[i]
        }

        let mut memo = vec![-1; cost.len() + 2];
        rec(0, &cost, &mut memo).min(rec(1, &cost, &mut memo))
    }

    pub fn min_cost_climbing_stairs_dp_2_dim(cost: Vec<i32>) -> i32 {
        let mut dp = vec![0; cost.len() + 2];
        for i in (0..cost.len()).rev() {
            dp[i] = cost[i] + dp[i + 1].min(dp[i + 2]);
        }
        dp[0].min(dp[1])
    }

    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let (mut climb1, mut climb2) = (0, 0);
        for cost in cost {
            climb2 = cost + climb1.min(climb2);
            std::mem::swap(&mut climb1, &mut climb2);
        }
        climb1.min(climb2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_1_2() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![1, 2]), 1);
    }
    #[test]
    fn c_10_15_20() {
        let c = vec![10, 15, 20];
        assert_eq!(Solution::min_cost_climbing_stairs(c), 15);
        // Explanation: You will start at index 1.
        // - Pay 15 and climb two steps to reach the top.
        // The total cost is 15.
    }
    #[test]
    fn c_1_100_1_1_1_100_1_1_100_1() {
        let c = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        assert_eq!(Solution::min_cost_climbing_stairs(c), 6);
        // Explanation: You will start at index 0.
        // - Pay 1 and climb two steps to reach index 2.
        // - Pay 1 and climb two steps to reach index 4.
        // - Pay 1 and climb two steps to reach index 6.
        // - Pay 1 and climb one step to reach index 7.
        // - Pay 1 and climb two steps to reach index 9.
        // - Pay 1 and climb one step to reach the top.
        // The total cost is 6.
    }

    #[test]
    fn c_1_repeat_1000() {
        let c = vec![1; 1000];
        assert_eq!(Solution::min_cost_climbing_stairs(c), 500);
    }
    #[test]
    fn c_1_to_1000() {
        let c = (1..=1000).collect();
        assert_eq!(Solution::min_cost_climbing_stairs(c), 250_000);
    }
}
