#![allow(dead_code)]
/// 120. Triangle
/// =============
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
/// __Follow up__: Could you do this using only `O(n)` extra space,
/// where `n` is the total number of rows in the triangle?
///
/// https://leetcode.com/problems/triangle/
struct Solution;
impl Solution {
    /// 19:52-19:54
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        println!("minimum_total({:?})", triangle);
        let n = triangle.len();
        let mut dp = vec![0; n + 1];
        for r in (0..n).rev() {
            for c in 0..=r {
                dp[c] = triangle[r][c] + dp[c].min(dp[c + 1]);
            }
        }
        dp[0]
    }
    /// 19:49-19:52
    pub fn minimum_total_dp_vec_vec(triangle: Vec<Vec<i32>>) -> i32 {
        println!("minimum_total({:?})", triangle);
        let n = triangle.len();
        let mut dp = vec![vec![0; n + 1]; n + 1];
        for r in (0..n).rev() {
            for c in 0..=r {
                dp[r][c] = triangle[r][c] + dp[r + 1][c].min(dp[r + 1][c + 1]);
            }
        }
        dp[0][0]
    }
    /// 19:45-19:49
    pub fn minimum_total_rec_with_memo(triangle: Vec<Vec<i32>>) -> i32 {
        println!("minimum_total({:?})", triangle);
        fn rec(r: usize, c: usize, t: &[Vec<i32>], memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
            if r == t.len() - 1 {
                t[r][c]
            } else if let Some(res) = memo[r][c] {
                res
            } else {
                let res = t[r][c] + rec(r + 1, c, t, memo).min(rec(r + 1, c + 1, t, memo));
                memo[r][c] = Some(res);
                res
            }
        }
        let n = triangle.len();
        let mut memo = vec![vec![None; n]; n];
        rec(0, 0, &triangle, &mut memo)
    }
    /// 19:38-19:45
    pub fn minimum_total_rec(triangle: Vec<Vec<i32>>) -> i32 {
        println!("minimum_total({:?})", triangle);
        fn rec(r: usize, c: usize, t: &[Vec<i32>]) -> i32 {
            if r == t.len() - 1 {
                t[r][c]
            } else {
                t[r][c] + rec(r + 1, c, t).min(rec(r + 1, c + 1, t))
            }
        }
        rec(0, 0, &triangle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn t_2_34_657_4183() {
        let t = vv![[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]];
        assert_eq!(Solution::minimum_total(t), 11);
        // Explanation: The triangle looks like:
        //    2
        //   3 4
        //  6 5 7
        // 4 1 8 3
        // The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11
    }
    #[test]
    fn t_m10() {
        let t = vv![[-10]];
        assert_eq!(Solution::minimum_total(t), -10);
    }

    #[test]
    fn t_200x1() {
        let mut t = vec![];
        for r in 0..200 {
            t.push(vec![1; r + 1]);
        }
        assert_eq!(Solution::minimum_total(t), 200);
    }
}
