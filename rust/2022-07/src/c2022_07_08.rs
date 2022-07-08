#![allow(dead_code)]
//! \#1473. Paint House III
//! =======================
//!
//! There is a row of `m` houses in a small city,
//! each house must be painted with one of the `n` colors (labeled from `1` to `n`),
//! some houses that have been painted last summer should not be painted again.
//!
//! A neighborhood is a maximal group of continuous houses that are painted with the same color.
//!
//! - For example: `houses = [1,2,2,3,3,2,1,1]` contains `5` neighborhoods `[{1}, {2,2}, {3,3}, {2}, {1,1}]`.
//!
//! Given an array `houses`, an `m x n` matrix `cost` and an integer `target` where:
//!
//! - `houses[i]`: is the color of the house `i`, and `0` if the house is not painted yet.
//! - `cost[i][j]`: is the cost of paint the house `i` with the color `j + 1`.
//!
//! Return _the minimum cost of painting all the remaining houses in such a way
//! that there are exactly `target` neighborhoods_.
//! If it is not possible, return `-1`.
//!
//! __Constraints:__
//!
//! - `m == houses.length == cost.length`
//! - `n == cost[i].length`
//! - `1 <= m <= 100`
//! - `1 <= n <= 20`
//! - `1 <= target <= m`
//! - `0 <= houses[i] <= n`
//! - `1 <= cost[i][j] <= 10_000`
//!
//! <https://leetcode.com/problems/paint-house-iii>

struct Solution;
impl Solution {
    pub fn min_cost_rec_brute_force(houses: Vec<i32>, cost: Vec<Vec<i32>>, _: i32, n: i32, target: i32) -> i32 {
        fn rec(house: usize, color: usize, target: i32, houses: &[i32], costs: &[Vec<i32>]) -> i32 {
            if house == houses.len() {
                if target == 0 {
                    0
                } else {
                    i32::MAX
                }
            } else if target < 1 {
                i32::MAX
            } else {
                if houses[house] == 0 {
                    let next = (0..costs[0].len())
                        .map(|next_color| (next_color, target - (next_color != color) as i32))
                        .map(|(color, target)| rec(house + 1, color, target, houses, costs))
                        .min()
                        .unwrap();
                    costs[house][color].saturating_add(next)
                } else if houses[house] == color as i32 + 1 {
                    (0..costs[0].len())
                        .map(|next_color| (next_color, target - (next_color != color) as i32))
                        .map(|(color, target)| rec(house + 1, color, target, houses, costs))
                        .min()
                        .unwrap()
                } else {
                    i32::MAX
                }
            }
        }

        match (0..n as usize).map(|color| rec(0, color, target, &houses, &cost)).min().unwrap() {
            i32::MAX => -1,
            cost => cost,
        }
    }

    pub fn min_cost_rec_with_memo(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        fn rec(
            house: usize,
            color: usize,
            target: i32,
            houses: &[i32],
            costs: &[Vec<i32>],
            memo: &mut Vec<Vec<Vec<i32>>>,
        ) -> i32 {
            let t = target as usize;
            if memo[house][color][t] == -1 {
                memo[house][color][t] = if house == houses.len() {
                    if target == 0 {
                        0
                    } else {
                        i32::MAX
                    }
                } else if target < 1 {
                    i32::MAX
                } else {
                    if houses[house] == 0 {
                        let next = (0..costs[0].len())
                            .map(|next_color| (next_color, target - (next_color != color) as i32))
                            .map(|(color, target)| {
                                rec(house + 1, color, target, houses, costs, memo)
                            })
                            .min()
                            .unwrap();
                        costs[house][color].saturating_add(next)
                    } else if houses[house] == color as i32 + 1 {
                        (0..costs[0].len())
                            .map(|next_color| (next_color, target - (next_color != color) as i32))
                            .map(|(color, target)| {
                                rec(house + 1, color, target, houses, costs, memo)
                            })
                            .min()
                            .unwrap()
                    } else {
                        i32::MAX
                    }
                }
            };
            memo[house][color][t]
        }

        let mut memo = vec![vec![vec![-1; target as usize + 1]; n as usize]; m as usize + 1];
        match (0..n as usize).map(|color| rec(0, color, target, &houses, &cost, &mut memo)).min().unwrap() {
            i32::MAX => -1,
            cost => cost,
        }
    }

    pub fn min_cost_dp_3_dimensions(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let (m, n, target) = (m as usize, n as usize, target as usize);
        let mut dp = vec![vec![vec![i32::MAX; target + 1]; n]; m + 1];
        for color in 0..n {
            dp[m][color][0] = 0;
        }
        for house in (0..m).rev() {
            for color in 0..n {
                for target in 1..=target {
                    dp[house][color][target] =
                        if houses[house] != 0 && houses[house] != color as i32 + 1 {
                            i32::MAX
                        } else {
                            let mut next = i32::MAX;
                            for next_color in 0..n {
                                let next_target = target - (next_color != color) as usize;
                                next = next.min(dp[house + 1][next_color][next_target]);
                            }
                            if houses[house] == color as i32 + 1 {
                                next
                            } else {
                                cost[house][color].saturating_add(next)
                            }
                        };
                }
            }
        }

        let mut result = i32::MAX;
        for color in 0..n {
            result = result.min(dp[0][color][target]);
        }
        match result {
            i32::MAX => -1,
            cost => cost,
        }
    }

    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let (m, n, target) = (m as usize, n as usize, target as usize);
        let mut curr = vec![vec![i32::MAX; target + 1]; n];
        let mut prev = vec![vec![i32::MAX; target + 1]; n];
        for color in 0..n {
            prev[color][0] = 0;
        }
        for house in (0..m).rev() {
            for color in 0..n {
                curr[color][0] = i32::MAX;
                for target in 1..=target {
                    curr[color][target] = if houses[house] != 0 && houses[house] != color as i32 + 1
                    {
                        i32::MAX
                    } else {
                        let mut next = i32::MAX;
                        for next_color in 0..n {
                            let next_target = target - (next_color != color) as usize;
                            next = next.min(prev[next_color][next_target]);
                        }
                        if houses[house] == color as i32 + 1 {
                            next
                        } else {
                            cost[house][color].saturating_add(next)
                        }
                    };
                }
            }
            std::mem::swap(&mut curr, &mut prev);
        }

        match prev.into_iter().map(|v| v[target]).min().unwrap() {
            i32::MAX => -1,
            cost => cost,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[rustfmt::skip] #[test] fn h_0_c1c2_m1_n2_t1() { assert_eq!(Solution::min_cost(vec![0], vv![[1, 2]], 1, 2, 1), 1); }
    #[rustfmt::skip] #[test] fn h_1_c1c2_m1_n2_t1() { assert_eq!(Solution::min_cost(vec![1], vv![[1, 2]], 1, 2, 1), 0); }

    #[rustfmt::skip] #[test] fn h_1_1_c1c2_c2c1_m2_n2_t2() { assert_eq!(Solution::min_cost(vec![1,1], vv![[1,2],[2,1]], 2, 2, 2), -1); }
    #[rustfmt::skip] #[test] fn h_2_0_c1c2_c2c1_m2_n2_t2() { assert_eq!(Solution::min_cost(vec![2,0], vv![[1,2],[2,1]], 2, 2, 2), 2); }

    #[test]
    fn h_0_0_0_0_0_c1c10_c10c1_c10c1_c1c10_c5c1_m5_n2_t3() {
        let h = vec![0, 0, 0, 0, 0];
        let c = vv![[1, 10], [10, 1], [10, 1], [1, 10], [5, 1]];
        assert_eq!(Solution::min_cost(h, c, 5, 2, 3), 9);
        // Explanation: Paint houses of this way [1,2,2,1,1]
        // This array contains target = 3 neighborhoods, [{1}, {2,2}, {1,1}].
        // Cost of paint all houses (1 + 1 + 1 + 1 + 5) = 9.
    }
    #[test]
    fn h_0_2_1_2_0_c1c10_c10c1_c10c1_c1c10_c5c1_m5_n2_t3() {
        let h = vec![0, 2, 1, 2, 0];
        let c = vv![[1, 10], [10, 1], [10, 1], [1, 10], [5, 1]];
        assert_eq!(Solution::min_cost(h, c, 5, 2, 3), 11);
        // Explanation: Some houses are already painted, Paint the houses of this way [2,2,1,2,2]
        // This array contains target = 3 neighborhoods, [{2,2}, {1}, {2,2}].
        // Cost of paint the first and last house (10 + 1) = 11.
    }
    #[test]
    fn h_3_1_2_3_c1c1c1_c1c1c1_c1c1c1_c1c1c1_m4_n3_t3() {
        let h = vec![3, 1, 2, 3];
        let c = vv![[1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1]];
        assert_eq!(Solution::min_cost(h, c, 4, 3, 3), -1);
        // Explanation:
        // Houses are already painted with a total of 4 neighborhoods [{3},{1},{2},{3}] different of target = 3.
    }
}
