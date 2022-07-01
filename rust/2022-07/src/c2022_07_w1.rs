#![allow(dead_code)]
//! \#256. Paint House
//! ==================
//!
//! There is a row of `n` houses, where each house can be painted one of three colors: red, blue, or green.
//! The cost of painting each house with a certain color is different.
//! You have to paint all the houses such that no two adjacent houses have the same color.
//!
//! The cost of painting each house with a certain color is represented by an `n x 3` cost matrix `costs`.
//!
//! - For example, `costs[0][0]` is the cost of painting house `0` with the color red;
//!   `costs[1][2]` is the cost of painting house `1` with color green, and so on...
//!
//! Return _the minimum cost to paint all houses_.
//!
//! __Constraints:__
//!
//! - `costs[i].length == 3`
//! - `1 <= costs.length <= 100`
//! - `1 <= costs[i][j] <= 20`
//!
//! <https://leetcode.com/problems/paint-house>

pub struct Solution;
impl Solution {
    pub fn min_cost(mut costs: Vec<Vec<i32>>) -> i32 {
        let mut dp = costs.pop().unwrap();
        while let Some(mut curr) = costs.pop() {
            curr[0] += dp[1].min(dp[2]);
            curr[1] += dp[0].min(dp[2]);
            curr[2] += dp[0].min(dp[1]);
            std::mem::swap(&mut curr, &mut dp);
        }
        dp.into_iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]}}

    #[test]
    fn c17c2c17_c16c16c5_c14c3c19() {
        let c = vv![[17, 2, 17], [16, 16, 5], [14, 3, 19]];
        assert_eq!(Solution::min_cost(c), 10);
        // Explanation: Paint house 0 into blue, paint house 1 into green, paint house 2 into blue.
        // Minimum cost: 2 + 5 + 3 = 10.
    }
    #[test]
    fn c7c6c2() {
        let c = vv![[7, 6, 2]];
        assert_eq!(Solution::min_cost(c), 2);
    }
}
