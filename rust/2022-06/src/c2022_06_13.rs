#![allow(dead_code)]
//! \#120. Triangle
//! ===============
//!
//! Given a `triangle` array, return _the minimum path sum from top to bottom_.
//!
//! For each step, you may move to an adjacent number of the row below.
//! More formally, if you are on index `i` on the current row,
//! you may move to either index `i` or index `i + 1` on the next row.
//!
//! __Constraints:__
//!
//! - `1 <= triangle.length <= 200`
//! - `triangle[0].length == 1`
//! - `triangle[i].length == triangle[i - 1].length + 1`
//! - `-10_000 <= triangle[i][j] <= 10_000`
//!
//! __Follow up:__ Could you do this using only `O(n)` extra space,
//! where `n` is the total number of rows in the triangle?
//!
//! <https://leetcode.com/problems/triangle>

pub struct Solution;
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; triangle.len() + 1];
        for r in (0..triangle.len()).rev() {
            for c in 0..=r {
                dp[c] = triangle[r][c] + dp[c].min(dp[c + 1]);
            }
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn r_2_r_3_4_r_6_5_7_r_4_1_8_3() {
        let t = vv![[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]];
        assert_eq!(Solution::minimum_total(t), 11);
        // Explanation: The triangle looks like:
        //    2
        //   3 4
        //  6 5 7
        // 4 1 8 3
        // The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11 (underlined above).
    }
    #[test]
    fn r_m10() {
        let t = vv![[-10]];
        assert_eq!(Solution::minimum_total(t), -10);
    }
}
