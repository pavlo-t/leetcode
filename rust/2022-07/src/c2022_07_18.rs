#![allow(dead_code)]
//! \#1074. Number of Submatrices That Sum to Target
//! ================================================
//!
//! <https://leetcode.com/problems/number-of-submatrices-that-sum-to-target>
//!
//! Given a `matrix` and a `target`, return the number of non-empty submatrices that sum to target.
//!
//! A submatrix `x1`, `y1`, `x2`, `y2` is the set of all cells `matrix[x][y]` with `x1 <= x <= x2` and `y1 <= y <= y2`.
//!
//! Two submatrices `(x1, y1, x2, y2)` and `(x1', y1', x2', y2')` are different
//! if they have some coordinate that is different: for example, if `x1` != `x1'`.
//!
//! __Constraints:__
//!
//! - `1 <= matrix.length <= 100`
//! - `1 <= matrix[0].length <= 100`
//! - `-1000 <= matrix[i] <= 1000`
//! - `-100_000_000 <= target <= 100_000_000`

pub struct Solution;
impl Solution {
    pub fn num_submatrix_sum_target_v1(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        // prefix_sums
        let ps = {
            let mut ps = vec![vec![0; n + 1]; m + 1];
            for r in 1..=m {
                let pr = r - 1;
                for c in 1..=n {
                    let pc = c - 1;
                    ps[r][c] = matrix[pr][pc] + ps[pr][c] + ps[r][pc] - ps[pr][pc];
                }
            }
            ps
        };

        let mut result = 0;
        for r1 in 0..m {
            for r2 in r1 + 1..=m {
                for c1 in 0..n {
                    for c2 in c1 + 1..=n {
                        let sum = ps[r2][c2] - ps[r1][c2] - ps[r2][c1] + ps[r1][c1];
                        if sum == target {
                            result += 1;
                        }
                    }
                }
            }
        }
        result
    }

    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        use std::collections::HashMap;

        let (m, n) = (matrix.len(), matrix[0].len());

        // prefix_sums
        let ps = {
            let mut ps = vec![vec![0; n + 1]; m + 1];
            for r in 1..=m {
                let pr = r - 1;
                for c in 1..=n {
                    let pc = c - 1;
                    ps[r][c] = matrix[pr][pc] + ps[pr][c] + ps[r][pc] - ps[pr][pc];
                }
            }
            ps
        };

        let mut result = 0;
        for r1 in 0..m {
            for r2 in r1 + 1..=m {
                let mut counts = HashMap::new();
                counts.insert(0, 1);
                for c in 1..=n {
                    let sum = ps[r2][c] - ps[r1][c];
                    result += counts.get(&(sum - target)).unwrap_or(&0);
                    *counts.entry(sum).or_default() += 1;
                }
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
    fn p0p1p0_p1p1p1_p0p1p0_target_0() {
        // 0 1 0
        // 1 1 1
        // 0 1 0
        // Sums:
        // 0 1 1
        // 1 3 4
        // 1 4 5
        let m = vv![[0, 1, 0], [1, 1, 1], [0, 1, 0]];
        assert_eq!(Solution::num_submatrix_sum_target(m, 0), 4);
        // Explanation: The four 1x1 submatrices that only contain 0.
    }
    #[test]
    fn p1m1_m1p1_target_0() {
        let m = vv![[1, -1], [-1, 1]];
        assert_eq!(Solution::num_submatrix_sum_target(m, 0), 5);
        // Explanation: The two 1x2 submatrices, plus the two 2x1 submatrices, plus the 2x2 submatrix.
    }
    #[test]
    fn p904_target_0() {
        let m = vv![[904]];
        assert_eq!(Solution::num_submatrix_sum_target(m, 0), 0);
    }

    #[test]
    fn m_100x100x1_target_1() {
        let m = vec![vec![1; 100]; 100];
        assert_eq!(Solution::num_submatrix_sum_target(m, 1), 10_000);
    }
    #[test]
    fn m_100x100x0_target_0() {
        let m = vec![vec![0; 100]; 100];
        assert_eq!(Solution::num_submatrix_sum_target(m, 0), 25_502_500);
    }
}
