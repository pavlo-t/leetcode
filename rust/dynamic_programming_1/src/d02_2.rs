#![allow(dead_code)]
/// 746. Min Cost Climbing Stairs
/// =============================
///
/// You are given an integer array `cost` where `cost[i]` is the cost of `i`th step on a staircase.
/// Once you pay the cost, you can either climb one or two steps.
///
/// You can either start from the step with index `0`, or the step with index `1`.
///
/// Return _the minimum cost to reach the top of the floor_.
///
/// __Constraints:__
///
/// - `2 <= cost.length <= 1000`
/// - `0 <= cost[i] <= 999`
///
/// https://leetcode.com/problems/min-cost-climbing-stairs/
struct Solution;
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        println!("min_cost_climbing_stairs({:?})", cost);
        let n = cost.len();
        let mut p = cost[n - 2];
        let mut pp = cost[n - 1];
        for i in (0..n - 2).rev() {
            std::mem::swap(&mut p, &mut pp);
            p = cost[i] + p.min(pp);
        }
        p.min(pp)
    }
    pub fn min_cost_climbing_stairs_dp(mut cost: Vec<i32>) -> i32 {
        println!("min_cost_climbing_stairs({:?})", cost);
        let n = cost.len();
        for i in (0..n - 2).rev() {
            cost[i] += cost[i + 1].min(cost[i + 2]);
        }
        cost[0].min(cost[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_10_20() {
        let c = vec![10, 20];
        assert_eq!(Solution::min_cost_climbing_stairs(c), 10);
    }
    #[test]
    fn c_10_15_20() {
        let c = vec![10, 15, 20];
        assert_eq!(Solution::min_cost_climbing_stairs(c), 15);
        // Explanation: Cheapest is: start on cost[1], pay that cost, and go to the top.
    }
    #[test]
    fn c_1_100_1_1_1_100_1_1_100_1() {
        let c = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        assert_eq!(Solution::min_cost_climbing_stairs(c), 6);
        // Explanation: Cheapest is: start on cost[0], and only step on 1s, skipping cost[3].
    }
}
