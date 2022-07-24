#![allow(dead_code)]
//! \#240. Search a 2D Matrix II
//! ============================
//!
//! <https://leetcode.com/problems/search-a-2d-matrix-ii>
//!
//! Write an efficient algorithm that searches for a value `target` in an `m x n` integer matrix `matrix`.
//! This matrix has the following properties:
//!
//! - Integers in each row are sorted in ascending from left to right.
//! - Integers in each column are sorted in ascending from top to bottom.
//!
//! __Constraints:__
//!
//! - `m == matrix.length`
//! - `n == matrix[i].length`
//! - `1 <= n, m <= 300`
//! - `-1_000_000_000 <= matrix[i][j] <= 1_000_000_000`
//! - All the integers in each row are __sorted__ in ascending order.
//! - All the integers in each column are __sorted__ in ascending order.
//! - `-1_000_000_000 <= target <= 1_000_000_000`

pub struct Solution;
impl Solution {
    /// Binary search each row: `O(m * log(n))`
    pub fn search_matrix_v1(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        for r in (0..m).filter(|&r| matrix[r][0] <= target && matrix[r][n - 1] >= target) {
            match &matrix[r].binary_search(&target) {
                Ok(_) => return true,
                _ => (),
            }
        }
        false
    }

    /// Navigate to result `O(m + n)`
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        use std::cmp::Ordering;

        let (mut r, mut c) = (matrix.len() - 1, 0);
        while c < matrix[0].len() {
            match matrix[r][c].cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Greater if r == 0 => return false,
                Ordering::Greater => r -= 1,
                Ordering::Less => c += 1,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn p1p4p7p11p15_p2p5p8p12p19_p3p6p9p16p22_p10p13p14p17p24_p18p21p23p26p30_t_5() {
        let m = vv![
            [1, 4, 7, 11, 15],
            [2, 5, 8, 12, 19],
            [3, 6, 9, 16, 22],
            [10, 13, 14, 17, 24],
            [18, 21, 23, 26, 30]
        ];
        assert_eq!(Solution::search_matrix(m, 5), true);
    }
    #[test]
    fn p1p4p7p11p15_p2p5p8p12p19_p3p6p9p16p22_p10p13p14p17p24_p18p21p23p26p30_t_20() {
        let m = vv![
            [1, 4, 7, 11, 15],
            [2, 5, 8, 12, 19],
            [3, 6, 9, 16, 22],
            [10, 13, 14, 17, 24],
            [18, 21, 23, 26, 30]
        ];
        assert_eq!(Solution::search_matrix(m, 20), false);
    }
    #[test]
    fn p0_to_p599_by_2_x_300_t_1() {
        let m = (0..300)
            .map(|_| (0..600).step_by(2).collect::<Vec<_>>())
            .collect();
        assert_eq!(Solution::search_matrix(m, 1), false);
    }
}
