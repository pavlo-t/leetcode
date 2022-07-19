#![allow(dead_code)]
//! \#1182. Shortest Distance to Target Color
//! =========================================
//!
//! <https://leetcode.com/problems/shortest-distance-to-target-color>
//!
//! You are given an array `colors`, in which there are three colors: `1`, `2` and `3`.
//!
//! You are also given some queries.
//! Each query consists of two integers `i` and `c`,
//! return _the shortest distance between the given index `i` and the target color `c`_.
//! If there is no solution return `-1`.
//!
//! __Constraints:__
//!
//! - `1 <= colors.length <= 50_000`
//! - `1 <= colors[i] <= 3`
//! - `1 <= queries.length <= 50_000`
//! - `queries[i].length == 2`
//! - `0 <= queries[i][0] < colors.length`
//! - `1 <= queries[i][1] <= 3`

pub struct Solution;
impl Solution {
    pub fn shortest_distance_color(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const COLORS: usize = 3;
        let n = colors.len();
        let mut dp = Vec::with_capacity(n);
        let mut distances = vec![i32::MAX; COLORS];
        for i in 0..n {
            for c in 0..COLORS {
                distances[c] = distances[c].saturating_add(1);
            }
            distances[colors[i] as usize - 1] = 0;
            dp.push(distances.clone());
        }
        let mut distances = vec![i32::MAX; COLORS];
        for i in (0..n).rev() {
            let color = colors[i] as usize - 1;
            for c in 0..COLORS {
                if c == color {
                    distances[c] = 0;
                } else {
                    distances[c] = distances[c].saturating_add(1);
                }
                dp[i][c] = dp[i][c].min(distances[c]);
            }
        }

        let mut result = Vec::with_capacity(queries.len());
        for q in queries {
            let (i, c) = (q[0] as usize, q[1] as usize - 1);
            result.push(match dp[i][c] {
                i32::MAX => -1,
                distance => distance,
            });
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn c_1_1_2_1_3_2_2_3_3_q_i1c3_i2c2_i6c1() {
        let c = vec![1, 1, 2, 1, 3, 2, 2, 3, 3];
        let q = vv![[1, 3], [2, 2], [6, 1]];
        let e = vec![3, 0, 3];
        assert_eq!(Solution::shortest_distance_color(c, q), e);
        // Explanation:
        // The nearest 3 from index 1 is at index 4 (3 steps away).
        // The nearest 2 from index 2 is at index 2 itself (0 steps away).
        // The nearest 1 from index 6 is at index 3 (3 steps away).
    }
    #[test]
    fn c_1_2_q_i0c3() {
        let c = vec![1, 2];
        let q = vv![[0, 3]];
        let e = vec![-1];
        assert_eq!(Solution::shortest_distance_color(c, q), e);
        // Explanation: There is no 3 in the array.
    }
}
