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
struct Solution;
impl Solution {
    /// 20:34-20:35
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        println!("min_cost_climbing_stairs({:?})", cost);
        let (mut a, mut b) = (0, 0);
        for i in (0..cost.len()).rev() {
            std::mem::swap(&mut a, &mut b);
            a = cost[i] + a.min(b);
        }
        a.min(b)
    }
    /// 20:33-20:34
    pub fn min_cost_climbing_stairs_dp_mutate_input(mut cost: Vec<i32>) -> i32 {
        println!("min_cost_climbing_stairs({:?})", cost);
        for i in (0..cost.len() - 2).rev() {
            cost[i] += cost[i + 1].min(cost[i + 2]);
        }
        cost[0].min(cost[1])
    }
    /// 20:27-20:33
    pub fn min_cost_climbing_stairs_dp_vec(cost: Vec<i32>) -> i32 {
        println!("min_cost_climbing_stairs({:?})", cost);
        let mut dp: Vec<i32> = vec![0; cost.len() + 2];
        for i in (0..cost.len()).rev() {
            dp[i] = cost[i] + dp[i + 1].min(dp[i + 2]);
        }
        dp[0].min(dp[1])
    }
    /// 20:24-20:27
    pub fn min_cost_climbing_stairs_rec_with_memo(cost: Vec<i32>) -> i32 {
        println!("min_cost_climbing_stairs({:?})", cost);
        static mut MEMO: Vec<i32> = vec![];
        unsafe {
            MEMO = vec![-1; cost.len()];
        }
        fn rec(i: usize, c: &[i32]) -> i32 {
            unsafe {
                if i >= c.len() {
                    0
                } else if MEMO[i] >= 0 {
                    MEMO[i]
                } else {
                    MEMO[i] = c[i] + rec(i + 1, c).min(rec(i + 2, c));
                    MEMO[i]
                }
            }
        }
        rec(0, &cost).min(rec(1, &cost))
    }
    /// 20:22-20:24
    pub fn min_cost_climbing_stairs_rec(cost: Vec<i32>) -> i32 {
        println!("min_cost_climbing_stairs({:?})", cost);
        fn rec(i: usize, c: &[i32]) -> i32 {
            if i >= c.len() {
                0
            } else {
                c[i] + rec(i + 1, c).min(rec(i + 2, c))
            }
        }
        rec(0, &cost).min(rec(1, &cost))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s_10_15_20() {
        let c = vec![10, 15, 20];
        assert_eq!(Solution::min_cost_climbing_stairs(c), 15);
        // Explanation: You will start at index 1.
        // - Pay 15 and climb two steps to reach the top.
        // The total cost is 15.
    }
    #[test]
    fn s_1_100_1_1_1_100_1_1_1000_1() {
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
    fn s_1_2() {
        let c = vec![1, 2];
        assert_eq!(Solution::min_cost_climbing_stairs(c), 1);
    }
    #[test]
    fn s_0to999() {
        let c = (0..1000).collect();
        assert_eq!(Solution::min_cost_climbing_stairs(c), 249_500);
    }
}
