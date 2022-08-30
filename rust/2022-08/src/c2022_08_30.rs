#![allow(dead_code)]
//! \#48. Rotate Image
//! ==================
//!
//! <https://leetcode.com/problems/rotate-image>
//!
//! You are given an `n x n` 2D `matrix` representing an image, rotate the image by __90__ degrees (clockwise).
//!
//! You have to rotate the image [in-place], which means you have to modify the input 2D matrix directly.
//! __DO NOT__ allocate another 2D matrix and do the rotation.
//!
//! [in-place]:https://en.wikipedia.org/wiki/In-place_algorithm
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```text
//! 1, 2, 3      7, 4, 1
//! 4, 5, 6  =>  8, 5, 2
//! 7, 8, 9      9, 6, 3
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_30::*;
//! # use c2022_08::vv;
//! let mut matrix = vv![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
//! Solution::rotate(&mut matrix);
//! assert_eq!(matrix, [[7, 4, 1], [8, 5, 2], [9, 6, 3]]);
//! ```
//!
//! ###### Example 2
//!
//! ```text
//!  5,  1,  9, 11        15, 13,  2,  5
//!  2,  4,  8, 10   =>   14,  3,  4,  1
//! 13,  3,  6,  7        12,  6,  8,  9
//! 15, 14, 12, 16        16,  7, 10, 11
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_30::*;
//! # use c2022_08::vv;
//! let mut matrix = vv![[5, 1, 9, 11], [2, 4, 8, 10], [13, 3, 6, 7], [15, 14, 12, 16]];
//! Solution::rotate(&mut matrix);
//! assert_eq!(matrix, [[15, 13, 2, 5], [14, 3, 4, 1], [12, 6, 8, 9], [16, 7, 10, 11]]);
//! ```
//!
//! ##### Constraints
//!
//! - `n == matrix.length == matrix[i].length`
//! - `1 <= n <= 20`
//! - `-1000 <= matrix[i][j] <= 1000`

pub struct Solution;
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for r in 0..n / 2 {
            for c in 0..if n % 2 == 0 { n / 2 } else { n / 2 + 1 } {
                let top_left = matrix[r][c];
                matrix[r][c] = matrix[n - c - 1][r];
                matrix[n - c - 1][r] = matrix[n - r - 1][n - c - 1];
                matrix[n - r - 1][n - c - 1] = matrix[c][n - r - 1];
                matrix[c][n - r - 1] = top_left;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vv;
}
