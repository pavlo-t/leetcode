#![allow(dead_code)]
/// Min Cost Climbing Stairs
/// ========================
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
/// https://leetcode.com/explore/featured/card/june-leetcoding-challenge-2021/603/week-1-june-1st-june-7th/3770/
struct Solution;
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut a = cost[cost.len() - 2];
        let mut b = cost[cost.len() - 1];

        for c in cost.into_iter().rev().skip(2) {
            let tmp = a;
            a = c + a.min(b);
            b = tmp;
        }

        a.min(b)
    }

    pub fn min_cost_climbing_stairs_dp(cost: Vec<i32>) -> i32 {
        let mut dp = cost.clone();

        for i in (0..cost.len() - 2).rev() {
            dp[i] = dp[i] + dp[i + 1].min(dp[i + 2]);
        }

        dp[0].min(dp[1])
    }

    pub fn min_cost_climbing_stairs_cache(cost: Vec<i32>) -> i32 {
        fn dfs(i: usize, cost: &[i32], cache: &mut Vec<i32>) -> i32 {
            if i >= cost.len() {
                0
            } else if cache[i] >= 0 {
                cache[i]
            } else {
                let r = cost[i] + dfs(i + 1, cost, cache).min(dfs(i + 2, cost, cache));
                cache[i] = r;
                r
            }
        }

        let mut cache = vec![-1; cost.len()];
        dfs(0, &cost, &mut cache).min(dfs(1, &cost, &mut cache))
    }

    pub fn min_cost_climbing_stairs_brute_force(cost: Vec<i32>) -> i32 {
        fn dfs(i: usize, cost: &[i32]) -> i32 {
            if i >= cost.len() {
                0
            } else {
                cost[i] + dfs(i + 1, cost).min(dfs(i + 2, cost))
            }
        }

        dfs(0, &cost).min(dfs(1, &cost))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_10_15_20_produces_15() {
        let cost = vec![10, 15, 20];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 15);
        // Explanation: Cheapest is: start on cost[1], pay that cost, and go to the top.
    }
    #[test]
    fn c_1_100_1_1_1_100_1_1_100_1_produces_6() {
        let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 6);
        // Explanation: Cheapest is: start on cost[0], and only step on 1s, skipping cost[3].
    }
    #[test]
    fn c_1_2_produces_1() {
        let cost = vec![1, 2];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 1);
    }

    mod performance {
        use super::*;

        #[test]
        fn c_1000x999_produces_499_500() {
            let cost = vec![999; 1000];
            assert_eq!(Solution::min_cost_climbing_stairs(cost), 499_500);
        }
    }
}
