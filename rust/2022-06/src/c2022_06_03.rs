#![allow(dead_code)]
//! \#304. Range Sum Query 2D - Immutable
//! =====================================
//!
//! Given a 2D matrix `matrix`, handle multiple queries of the following type:
//!
//! - Calculate the __sum__ of the elements of `matrix` inside the rectangle
//!   defined by its __upper left corner__ `(row1, col1)` and __lower right corner__ `(row2, col2)`.
//!
//! Implement the `NumMatrix` class:
//!
//! - `NumMatrix(int[][] matrix)` Initializes the object with the integer matrix `matrix`.
//! - `int sumRegion(int row1, int col1, int row2, int col2)` Returns the __sum__ of the elements of `matrix` inside
//!   the rectangle defined by its __upper left corner__ `(row1, col1)` and __lower right corner__ `(row2, col2)`.
//!
//! __Constraints:__
//!
//! - `m == matrix.length`
//! - `n == matrix[i].length`
//! - `1 <= m, n <= 200`
//! - `-100_000 <= matrix[i][j] <= 100_000`
//! - `0 <= row1 <= row2 < m`
//! - `0 <= col1 <= col2 < n`
//! - At most `10_000` calls will be made to `sumRegion`.
//!
//! <https://leetcode.com/problems/range-sum-query-2d-immutable>

pub struct NumMatrix {
    matrix: Vec<Vec<i32>>,
}
impl NumMatrix {
    pub fn new(mut matrix: Vec<Vec<i32>>) -> Self {
        let (m, n) = (matrix.len(), matrix[0].len());
        for r in 1..m {
            matrix[r][0] += matrix[r - 1][0];
        }
        for c in 1..n {
            matrix[0][c] += matrix[0][c - 1];
        }
        for r in 1..m {
            for c in 1..n {
                matrix[r][c] += matrix[r][c - 1] + matrix[r - 1][c] - matrix[r - 1][c - 1];
            }
        }
        Self { matrix }
    }
    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (r1, c1, r2, c2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
        let sum_up = if r1 > 0 { self.matrix[r1 - 1][c2] } else { 0 };
        let sum_left = if c1 > 0 { self.matrix[r2][c1 - 1] } else { 0 };
        let sum_left_up = if r1 > 0 && c1 > 0 {
            self.matrix[r1 - 1][c1 - 1]
        } else {
            0
        };
        self.matrix[r2][c2] - sum_up - sum_left + sum_left_up
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn m_30142_56321_12015_41017_10305() {
        let nm = NumMatrix::new(vv![
            [3, 0, 1, 4, 2],
            [5, 6, 3, 2, 1],
            [1, 2, 0, 1, 5],
            [4, 1, 0, 1, 7],
            [1, 0, 3, 0, 5]
        ]);
        // [ 3,  3,  4,  8, 10],
        // [ 8, 14, 18, 24, 27],
        // [ 9, 17, 21, 28, 36],
        // [13, 22, 26, 34, 49],
        // [14, 23, 30, 38, 58]
        assert_eq!(nm.sum_region(2, 1, 4, 3), 8); //  38 - 24 - 14 + 8
        assert_eq!(nm.sum_region(1, 1, 2, 2), 11); // 21 -  4 -  9 + 3
        assert_eq!(nm.sum_region(1, 2, 2, 4), 12); // 36 - 10 - 17 + 3
    }
    #[test]
    fn m_1() {
        let nm = NumMatrix::new(vv![[1]]);
        assert_eq!(nm.sum_region(0, 0, 0, 0), 1);
    }
}
