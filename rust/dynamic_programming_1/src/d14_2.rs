#![allow(dead_code)]
/// 304. Range Sum Query 2D - Immutable
/// ===================================
///
/// Given a 2D matrix `matrix`, handle multiple queries of the following type:
///
/// Calculate the __sum__ of the elements of `matrix` inside the rectangle
/// defined by its __upper left corner__ `(row1, col1)` and __lower right corner__ `(row2, col2)`.
///
/// Implement the NumMatrix class:
///
/// - `NumMatrix(int[][] matrix)`  
///   Initializes the object with the integer matrix `matrix`.
/// - `int sumRegion(int row1, int col1, int row2, int col2)`  
///   Returns the __sum__ of the elements of matrix inside the rectangle
///   defined by its __upper left corner__ `(row1, col1)` and __lower right corner__ `(row2, col2)`.
///
/// __Constraints:__
///
/// - `m == matrix.length`
/// - `n == matrix[i].length`
/// - `1 <= m, n <= 200`
/// - `-100_000 <= matrix[i][j] <= 100_000`
/// - `0 <= row1 <= row2 < m`
/// - `0 <= col1 <= col2 < n`
/// - At most `10_000` calls will be made to `sumRegion`.
///
/// https://leetcode.com/problems/range-sum-query-2d-immutable/
struct NumMatrix {
    ps: Vec<Vec<i32>>,
}
impl NumMatrix {
    fn new(mut matrix: Vec<Vec<i32>>) -> Self {
        let (m, n) = (matrix.len(), matrix[0].len());
        for r in 1..m {
            matrix[r][0] += matrix[r - 1][0];
        }
        for c in 1..n {
            matrix[0][c] += matrix[0][c - 1];
        }
        for r in 1..m {
            for c in 1..n {
                matrix[r][c] += matrix[r - 1][c] + matrix[r][c - 1] - matrix[r - 1][c - 1];
            }
        }
        Self { ps: matrix }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let r0opt = (row1 as usize).checked_sub(1);
        let c0opt = (col1 as usize).checked_sub(1);
        let r2 = row2 as usize;
        let c2 = col2 as usize;
        let ps = &self.ps;

        let rs_out = r0opt.map(|r| ps[r][c2]).unwrap_or(0);
        let cs_out = c0opt.map(|c| ps[r2][c]).unwrap_or(0);
        let double_subbed = r0opt.and_then(|r| c0opt.map(|c| ps[r][c])).unwrap_or(0);
        ps[r2][c2] - rs_out - cs_out + double_subbed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn example_1() {
        let num_matrix = NumMatrix::new(vv![
            [3, 0, 1, 4, 2],
            [5, 6, 3, 2, 1],
            [1, 2, 0, 1, 5],
            [4, 1, 0, 1, 7],
            [1, 0, 3, 0, 5]
        ]);
        assert_eq!(num_matrix.sum_region(2, 1, 4, 3), 8);
        assert_eq!(num_matrix.sum_region(1, 1, 2, 2), 11);
        assert_eq!(num_matrix.sum_region(1, 2, 2, 4), 12);
    }
    #[test]
    fn m_200x200x1_10000_calls() {
        let num_matrix = NumMatrix::new(vec![vec![1; 200]; 200]);
        for r in 0..100 {
            for c in 0..100 {
                let (r2, c2) = (r + 100, c + 100);
                assert_eq!(num_matrix.sum_region(r, c, r2, c2), 10201);
            }
        }
    }
}
