#![allow(dead_code)]
/// Triangle
/// ========
///
/// Given a `triangle` array, return _the minimum path sum from top to bottom_.
///
/// For each step, you may move to an adjacent number of the row below.
/// More formally, if you are on index `i` on the current row,
/// you may move to either index `i` or index `i + 1` on the next row.
///
/// __Constraints:__
///
/// - `1 <= triangle.length <= 200`
/// - `triangle[0].length == 1`
/// - `triangle[i].length == triangle[i - 1].length + 1`
/// - `-10_000 <= triangle[i][j] <= 10_000`
///
/// __Follow up:__
/// Could you do this using only `O(n)` extra space,
/// where `n` is the total number of rows in the triangle?
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/595/week-3-april-15th-april-21st/3715/
struct Solution;
impl Solution {
    // O(1) memory, mutate input
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len() - 1;
        let (triangle, last_r) = triangle.split_at_mut(n);
        triangle.iter().rev().fold(&mut last_r[0], |dp, r| {
            r.iter().enumerate().for_each(|(i, &v)| dp[i] = v + dp[i].min(dp[i + 1]));
            dp
        })[0]
    }

    // O(n) memory, n == triangle.len()
    pub fn minimum_total_iterative(triangle: Vec<Vec<i32>>) -> i32 {
        let mut dp = triangle.last().unwrap().clone();
        for r in (0..triangle.len() - 1).rev() {
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

    #[test]
    fn example1_t2_34_657_4183_produces_11() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(Solution::minimum_total(triangle), 11);
        // Explanation: The triangle looks like:
        //    2
        //   3 4
        //  6 5 7
        // 4 1 8 3
        // The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11.
    }
    #[test]
    fn example2() {
        let triangle = vec![vec![-10]];
        assert_eq!(Solution::minimum_total(triangle), -10);
    }
}
