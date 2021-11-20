#![allow(dead_code)]
/// 329. Longest Increasing Path in a Matrix
/// ========================================
///
/// Given an `m x n` integers `matrix`, return _the length of the longest increasing path in `matrix`_.
///
/// From each cell, you can either move in four directions: left, right, up, or down.
/// You __may not__ move __diagonally__ or move __outside the boundary__ (i.e., wrap-around is not allowed).
///
/// __Constraints:__
///
/// - `1 <= matrix.length, matrix[i].length <= 200`
/// - `0 <= matrix[i][j] <= 2**31 - 1`
///
/// https://leetcode.com/problems/longest-increasing-path-in-a-matrix/
struct Solution;
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        println!("longest_increasing_path({:?})", matrix);
        use std::iter::once;
        fn dfs(r: usize, c: usize, mat: &[Vec<i32>], memo: &mut Vec<Vec<i32>>) -> i32 {
            if memo[r][c] != -1 {
                memo[r][c]
            } else {
                let (m, n) = (mat.len(), mat[0].len());
                let mut max = 0;
                let curr = mat[r][c];
                once((r.wrapping_sub(1), c))
                    .chain(once((r + 1, c)))
                    .chain(once((r, c.wrapping_sub(1))))
                    .chain(once((r, c + 1)))
                    .filter(|&(r, c)| r < m && c < n && mat[r][c] > curr)
                    .for_each(|(r, c)| max = max.max(dfs(r, c, mat, memo)));
                memo[r][c] = max + 1;
                memo[r][c]
            }
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut memo = vec![vec![-1; n]; m];
        let mut max = 0;
        for r in 0..m {
            for c in 0..n {
                max = max.max(dfs(r, c, &matrix, &mut memo));
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn m_994_668_211() {
        let m = vv![[9, 9, 4], [6, 6, 8], [2, 1, 1]];
        assert_eq!(Solution::longest_increasing_path(m), 4);
        // Explanation: The longest increasing path is [1, 2, 6, 9].
    }
    #[test]
    fn m_345_326_221() {
        let m = vv![[3, 4, 5], [3, 2, 6], [2, 2, 1]];
        assert_eq!(Solution::longest_increasing_path(m), 4);
        // Explanation: The longest increasing path is [3, 4, 5, 6]. Moving diagonally is not allowed.
    }
    #[test]
    fn m_1() {
        let m = vv![[1]];
        assert_eq!(Solution::longest_increasing_path(m), 1);
    }

    #[test]
    fn m_1x200x200() {
        let m = vec![vec![1; 200]; 200];
        assert_eq!(Solution::longest_increasing_path(m), 1);
    }
    #[test]
    fn m_1_to_40000_in_rows() {
        let m = (0..200)
            .map(|r| (0..200).map(|c| r * 200 + c).collect())
            .collect();
        assert_eq!(Solution::longest_increasing_path(m), 399);
    }
    #[test]
    fn m_1_to_40000() {
        let m = (0..200)
            .map(|r| {
                if r % 2 == 0 {
                    (0..200).map(|c| r * 200 + c).collect()
                } else {
                    (0..200).rev().map(|c| r * 200 + c).collect()
                }
            })
            .collect();
        assert_eq!(Solution::longest_increasing_path(m), 40000);
    }
}
