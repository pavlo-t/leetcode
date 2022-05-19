#![allow(dead_code)]
/// \#329. Longest Increasing Path in a Matrix
/// ==========================================
///
/// Given an `m x n` integers `matrix`, return _the length of the longest increasing path in_ `matrix`.
///
/// From each cell, you can either move in four directions: left, right, up, or down.
/// You __may not__ move __diagonally__ or move __outside the boundary__ (i.e., wrap-around is not allowed).
///
/// __Constraints:__
///
/// - `m == matrix.length`
/// - `n == matrix[i].length`
/// - `1 <= m, n <= 200`
/// - `0 <= matrix[i][j] <= 2**31 - 1`
///
/// https://leetcode.com/problems/longest-increasing-path-in-a-matrix/
struct Solution;
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        use std::iter::once;

        fn bfs(r: usize, c: usize, matrix: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>) -> i32 {
            if dp[r][c] == 0 {
                let valid_prev = |&(pr, pc): &(usize, usize)| {
                    pr < matrix.len() && pc < matrix[0].len() && matrix[pr][pc] < matrix[r][c]
                };
                dp[r][c] = 1 + once((r.wrapping_sub(1), c))
                    .chain(once((r + 1, c)))
                    .chain(once((r, c.wrapping_sub(1))))
                    .chain(once((r, c + 1)))
                    .filter(valid_prev)
                    .map(|(r, c)| bfs(r, c, matrix, dp))
                    .max()
                    .unwrap_or(0);
            }
            dp[r][c]
        }

        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; n]; m];

        (0..m)
            .flat_map(|r| (0..n).map(move |c| (r, c)))
            .map(|(r, c)| bfs(r, c, &matrix, &mut dp))
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn r_9_9_4_r_6_6_8_r_2_1_1() {
        let m = vv![[9, 9, 4], [6, 6, 8], [2, 1, 1]];
        // 9 9 4
        // 6 6 8
        // 2 1 1
        // Up to bottom: [r,c] = 1 + (if [r-1,c] < [r,c] {[r-1,c]} else {0}).max(if [r,c-1] < [r,c] {[r,c-1]} else {0})
        // 1 1 1
        // 1 1 2
        // 1 1 1
        // Bottom to up: [r,c] = 1 + (if [r+1,c] < [r,c] {[r+1,c]} else {0}).max(if [r,c+1] < [r,c] {[r,c+1]} else {0})
        // 4 3 1
        // 3 2 2
        // 2 1 1
        assert_eq!(Solution::longest_increasing_path(m), 4);
        // Explanation: The longest increasing path is [1, 2, 6, 9].
    }
    #[test]
    fn r_3_4_5_r_3_2_6_r_2_2_1() {
        let m = vv![[3, 4, 5], [3, 2, 6], [2, 2, 1]];
        // 3 4 5
        // 3 2 6
        // 2 2 1
        // Up to bottom
        // 1 2 3
        // 1 1 4
        // 1 1 1
        // Bottom to up
        // 1 2 1
        // 2 1 2
        // 1 2 1
        assert_eq!(Solution::longest_increasing_path(m), 4);
        // Explanation: The longest increasing path is [3, 4, 5, 6]. Moving diagonally is not allowed.
    }
    #[rustfmt::skip] #[test] fn r_1() { assert_eq!(Solution::longest_increasing_path(vv![[1]]), 1); }
    #[rustfmt::skip] #[test] fn r_1_2_3() { assert_eq!(Solution::longest_increasing_path(vv![[1,2,3]]), 3); }
    #[rustfmt::skip] #[test] fn r_3_2_1() { assert_eq!(Solution::longest_increasing_path(vv![[3,2,1]]), 3); }
    #[rustfmt::skip] #[test] fn r_1_r_2_r_3() { assert_eq!(Solution::longest_increasing_path(vv![[1],[2],[3]]), 3); }
    #[rustfmt::skip] #[test] fn r_3_r_2_r_1() { assert_eq!(Solution::longest_increasing_path(vv![[3],[2],[1]]), 3); }

    #[test]
    fn r_7_8_9_r_9_7_6_r_7_2_3() {
        // 7 8 9   2->3->6->7->8->9
        // 9 7 6
        // 7 2 3
        // Up to bottom
        // 1 2 3
        // 2 1 1
        // 1 1 2
        // Bottom to up
        // 1 5 4
        // 5 4 3
        // 2 1 2
        let m = vv![[7, 8, 9], [9, 7, 6], [7, 2, 3]];
        assert_eq!(Solution::longest_increasing_path(m), 6);
    }
}
