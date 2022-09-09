#![allow(dead_code)]
//! \#1329. Sort the Matrix Diagonally
//! ==================================
//!
//! <https://leetcode.com/problems/sort-the-matrix-diagonally>
//!
//! A __matrix diagonal__ is a diagonal line of cells starting from some cell in either the topmost
//! row or leftmost column and going in the bottom-right direction until reaching the matrix's end.
//! For example, the __matrix diagonal__ starting from `mat[2][0]`, where `mat` is a `6 x 3` matrix,
//! includes cells `mat[2][0]`, `mat[3][1]`, and `mat[4][2]`.
//!
//! Given an `m x n` matrix `mat` of integers, sort each __matrix diagonal__ in ascending order
//! and return the _resulting matrix_.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_08::vv;
//! # use c2022_08::c2022_08_28::*;
//! let mat = vv![
//!     [3, 3, 1, 1],
//!     [2, 2, 1, 2],
//!     [1, 1, 1, 2]];
//! let expected = vv![
//!     [1, 1, 1, 1],
//!     [1, 2, 2, 2],
//!     [1, 2, 3, 3]];
//!
//! assert_eq!(Solution::diagonal_sort(mat), expected);
//! ```
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::vv;
//! # use c2022_08::c2022_08_28::*;
//! let mat = vv![
//!     [11, 25, 66,  1, 69,  7],
//!     [23, 55, 17, 45, 15, 52],
//!     [75, 31, 36, 44, 58,  8],
//!     [22, 27, 33, 25, 68,  4],
//!     [84, 28, 14, 11,  5, 50]];
//! let expected = vv![
//!     [ 5, 17,  4,  1, 52,  7],
//!     [11, 11, 25, 45,  8, 69],
//!     [14, 23, 25, 44, 58, 15],
//!     [22, 27, 31, 36, 50, 66],
//!     [84, 28, 75, 33, 55, 68]];
//!
//! assert_eq!(Solution::diagonal_sort(mat), expected);
//! ```
//!
//! ##### Constraints
//!
//! - `m == mat.length`
//! - `n == mat[i].length`
//! - `1 <= m, n <= 100`
//! - `1 <= mat[i][j] <= 100`

pub struct Solution;
impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());

        let mut result = vec![vec![0; n]; m];

        let mut sort_and_add_diagonal_to_result = |mut r: usize, mut c: usize| {
            let mut diagonal = vec![];
            while r < m && c < n {
                diagonal.push(mat[r][c]);
                r += 1;
                c += 1;
            }

            diagonal.sort_unstable();

            while let Some(val) = diagonal.pop() {
                r -= 1;
                c -= 1;
                result[r][c] = val;
            }
        };

        for r in 0..m {
            sort_and_add_diagonal_to_result(r, 0);
        }

        for c in 1..n {
            sort_and_add_diagonal_to_result(0, c);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::vv;

    #[test]
    fn r_1() {
        assert_eq!(Solution::diagonal_sort(vv![[1]]), vv![[1]]);
    }

    #[test]
    fn example_1() {
        let mat = vv![[3, 3, 1, 1], [2, 2, 1, 2], [1, 1, 1, 2]];
        let expected = vv![[1, 1, 1, 1], [1, 2, 2, 2], [1, 2, 3, 3]];

        assert_eq!(Solution::diagonal_sort(mat), expected);
    }

    #[test]
    fn example_2() {
        let mat = vv![
            [11, 25, 66, 1, 69, 7],
            [23, 55, 17, 45, 15, 52],
            [75, 31, 36, 44, 58, 8],
            [22, 27, 33, 25, 68, 4],
            [84, 28, 14, 11, 5, 50]
        ];
        let expected = vv![
            [5, 17, 4, 1, 52, 7],
            [11, 11, 25, 45, 8, 69],
            [14, 23, 25, 44, 58, 15],
            [22, 27, 31, 36, 50, 66],
            [84, 28, 75, 33, 55, 68]
        ];

        assert_eq!(Solution::diagonal_sort(mat), expected);
    }
}
