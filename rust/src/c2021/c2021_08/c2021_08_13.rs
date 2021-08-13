#![allow(dead_code)]
/// Set Matrix Zeroes
/// =================
///
/// Given an `m x n` integer matrix `matrix`, if an element is `0`, set its entire row and column to `0`'s.
///
/// You must do it in place.
///
/// __Constraints:__
///
/// - `1 <= matrix.length, matrix[0].length <= 200`
/// - `-2^31 <= matrix[i][j] <= 2^31 - 1`
///
/// __Follow up:__
///
/// - A straightforward solution using `O(mn)` space is probably a bad idea.
/// - A simple improvement uses `O(m + n)` space, but still not the best solution.
/// - Could you devise a constant space solution?
///
/// https://leetcode.com/explore/featured/card/august-leetcoding-challenge-2021/614/week-2-august-8th-august-14th/3888/
struct Solution;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        println!("set_zeroes({:?})", matrix);
        let m = matrix.len();
        let n = matrix[0].len();
        let mut zrs = vec![1; m];
        let mut zcs = vec![1; n];
        for r in 0..m {
            for c in 0..n {
                if matrix[r][c] == 0 {
                    zcs[c] = 0;
                    zrs[r] = 0;
                }
            }
        }
        for r in 0..m {
            for c in 0..n {
                if zcs[c] == 0 || zrs[r] == 0 {
                    matrix[r][c] = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn m_p1p1p1_p1p0p1_p1p1p1() {
        let mut matrix = vv![[1, 1, 1], [1, 0, 1], [1, 1, 1]];
        Solution::set_zeroes(&mut matrix);
        let e = vv![[1, 0, 1], [0, 0, 0], [1, 0, 1]];
        assert_eq!(matrix, e);
        // 1 1 1    1 0 1
        // 1 0 1 => 0 0 0
        // 1 1 1    1 0 1
    }
    #[test]
    fn m_p0p1p2p0_p3p4p5p2_p1p3p1p5() {
        let mut matrix = vv![[0, 1, 2, 0], [3, 4, 5, 2], [1, 3, 1, 5]];
        Solution::set_zeroes(&mut matrix);
        let e = vv![[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]];
        assert_eq!(matrix, e);
        // 0 1 2 0    0 0 0 0
        // 3 4 5 2 => 0 4 5 0
        // 1 3 1 5    0 3 1 0
    }
}
