#![allow(dead_code)]
//! \#867. Transpose Matrix
//! =======================
//!
//! Given a 2D integer array `matrix`, return _the __transpose__ of `matrix`_.
//!
//! The __transpose__ of a matrix is the matrix flipped over its main diagonal,
//! switching the matrix's row and column indices.
//!
//! Constraints:
//!
//! - `m == matrix.length`
//! - `n == matrix[i].length`
//! - `1 <= m, n <= 1000`
//! - `1 <= m * n <= 100_000`
//! - `-1_000_000_000 <= matrix[i][j] <= 1_000_000_000`
//!
//! <https://leetcode.com/problems/transpose-matrix>

pub struct Solution;
impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut result = vec![vec![0; m]; n];
        for r in 0..m {
            for c in 0..n {
                result[c][r] = matrix[r][c];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn m_1() {
        let m = vv![[1]];
        let e = vv![[1]];
        assert_eq!(Solution::transpose(m), e);
    }
    #[test]
    fn m_123_456_789() {
        let m = vv![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let e = vv![[1, 4, 7], [2, 5, 8], [3, 6, 9]];
        assert_eq!(Solution::transpose(m), e);
    }
    #[test]
    fn m_123_456() {
        let m = vv![[1, 2, 3], [4, 5, 6]];
        let e = vv![[1, 4], [2, 5], [3, 6]];
        assert_eq!(Solution::transpose(m), e);
    }
}
