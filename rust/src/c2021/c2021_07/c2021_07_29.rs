#![allow(dead_code)]
/// 01 Matrix
/// =========
///
/// Given an `m x n` binary matrix `mat`, return _the distance of the nearest_ `0` _for each cell_.
///
/// The distance between two adjacent cells is `1`.
///
/// Constraints:
///
/// - `m == mat.length`
/// - `n == mat[i].length`
/// - `1 <= m, n <= 10_000`
/// - `1 <= m * n <= 10_000`
/// - `mat[i][j]` is either `0` or `1`.
/// - There is at least one `0` in `mat`.
///
/// https://leetcode.com/explore/challenge/card/july-leetcoding-challenge-2021/612/week-5-july-29th-july-31st/3831/
struct Solution;
impl Solution {
    /// Approach 3: Dynamic Programming
    ///
    /// https://leetcode.com/problems/01-matrix/solution/
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const MD: i32 = 10_001;
        let rows = mat.len();
        let cols = mat[0].len();
        let mut dist = vec![vec![MD; cols]; rows];
        for r in 0..rows {
            for c in 0..cols {
                if mat[r][c] == 0 {
                    dist[r][c] = 0;
                } else {
                    if let Some(pr) = r.checked_sub(1) {
                        dist[r][c] = dist[r][c].min(dist[pr][c] + 1);
                    }
                    if let Some(pc) = c.checked_sub(1) {
                        dist[r][c] = dist[r][c].min(dist[r][pc] + 1);
                    }
                }
            }
        }
        for r in (0..rows).rev() {
            for c in (0..rows).rev() {
                if r < rows - 1 {
                    dist[r][c] = dist[r][c].min(dist[r + 1][c] + 1);
                }
                if c < cols - 1 {
                    dist[r][c] = dist[r][c].min(dist[r][c + 1] + 1);
                }
            }
        }
        dist
    }

    pub fn update_matrix_my_imperfect(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const MD: i32 = 10_001;
        let m = mat.len();
        let n = mat[0].len();
        let next_d = |r: usize, c: usize, d: i32| if mat[r][c] == 0 { 0 } else { d + 1 };
        let mut lr = mat.clone();
        for r in 0..m {
            let mut d = MD;
            for c in 0..n {
                d = next_d(r, c, d);
                lr[r][c] = d;
            }
        }

        let mut rl = mat.clone();
        for r in 0..m {
            let mut d = MD;
            for c in (0..n).rev() {
                d = next_d(r, c, d);
                rl[r][c] = d;
            }
        }

        let mut td = mat.clone();
        for c in 0..n {
            let mut d = MD;
            for r in 0..m {
                d = next_d(r, c, d);
                td[r][c] = d;
            }
        }

        let mut dt = mat.clone();
        for c in 0..n {
            let mut d = MD;
            for r in (0..m).rev() {
                d = next_d(r, c, d);
                dt[r][c] = d;
            }
        }

        let mut result = mat.clone();
        for r in 0..m {
            for c in 0..n {
                result[r][c] = lr[r][c].min(rl[r][c].min(td[r][c].min(dt[r][c])));
            }
        }
        for r in 1..m {
            for c in 0..n {
                result[r][c] = result[r][c].min(result[r - 1][c] + 1);
            }
        }
        for r in (0..m - 1).rev() {
            for c in 0..n {
                result[r][c] = result[r][c].min(result[r + 1][c] + 1);
            }
        }
        for r in 0..m {
            for c in 1..n {
                result[r][c] = result[r][c].min(result[r][c - 1] + 1);
            }
        }
        for r in 0..m {
            for c in (0..n - 1).rev() {
                result[r][c] = result[r][c].min(result[r][c + 1] + 1);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn m_000_010_000_produces_000_010_000() {
        let mat = vv![[0, 0, 0], [0, 1, 0], [0, 0, 0]];
        let e = vv![[0, 0, 0], [0, 1, 0], [0, 0, 0]];
        assert_eq!(Solution::update_matrix(mat), e);
    }
    #[test]
    fn m_000_010_111_produces_000_010_111() {
        let mat = vv![[0, 0, 0], [0, 1, 0], [1, 1, 1]];
        let e = vv![[0, 0, 0], [0, 1, 0], [1, 2, 1]];
        assert_eq!(Solution::update_matrix(mat), e);
    }
    #[test]
    fn m_011_111_111_produces_012_123_234() {
        let mat = vv![[0, 1, 1], [1, 1, 1], [1, 1, 1]];
        let e = vv![[0, 1, 2], [1, 2, 3], [2, 3, 4]];
        assert_eq!(Solution::update_matrix(mat), e);
    }
    #[test]
    fn m_111_111_110_produces_432_321_210() {
        let mat = vv![[1, 1, 1], [1, 1, 1], [1, 1, 0]];
        let e = vv![[4, 3, 2], [3, 2, 1], [2, 1, 0]];
        assert_eq!(Solution::update_matrix(mat), e);
    }
}
