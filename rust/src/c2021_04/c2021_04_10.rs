#![allow(dead_code)]
/// Longest Increasing Path in a Matrix
/// ===================================
///
/// Given an `m x n` integers matrix, return _the length of the longest increasing path in_ `matrix`.
///
/// From each cell, you can either move in four directions: left, right, up, or down.
/// You __may not__ move __diagonally__ or move __outside the boundary__
/// (i.e., wrap-around is not allowed).
///
/// __Constraints:__
///
/// - `1 <= matrix.length, matrix[i].length <= 200`
/// - `0 <= matrix[i][j] <= 2^31 - 1`
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/594/week-2-april-8th-april-14th/3703/
struct Solution;
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        fn dfs(r: usize, c: usize, matrix: &[Vec<i32>], cache: &mut [Vec<i32>]) -> i32 {
            if cache[r][c] > 0 {
                cache[r][c]
            } else {
                let curr = matrix[r][c];
                let m = matrix.len() - 1;
                let n = matrix[0].len() - 1;

                let mut longest = 0;
                if r > 0 && curr < matrix[r - 1][c] {
                    longest = longest.max(dfs(r - 1, c, matrix, cache));
                }
                if r < m && curr < matrix[r + 1][c] {
                    longest = longest.max(dfs(r + 1, c, matrix, cache));
                }
                if c > 0 && curr < matrix[r][c - 1] {
                    longest = longest.max(dfs(r, c - 1, matrix, cache));
                }
                if c < n && curr < matrix[r][c + 1] {
                    longest = longest.max(dfs(r, c + 1, matrix, cache));
                }
                cache[r][c] = 1 + longest;

                cache[r][c]
            }
        }

        let mut rsf = 1;
        let mut cache = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                rsf = rsf.max(dfs(i, j, &matrix, &mut cache));
            }
        }
        rsf
    }

    pub fn longest_increasing_path_brute_force(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        fn dfs(r: usize, c: usize, matrix: &[Vec<i32>]) -> i32 {
            let curr = matrix[r][c];
            let m = matrix.len() - 1;
            let n = matrix[0].len() - 1;

            let mut longest = 0;
            if r > 0 && curr < matrix[r - 1][c] {
                longest = longest.max(dfs(r - 1, c, matrix));
            }
            if r < m && curr < matrix[r + 1][c] {
                longest = longest.max(dfs(r + 1, c, matrix));
            }
            if c > 0 && curr < matrix[r][c - 1] {
                longest = longest.max(dfs(r, c - 1, matrix));
            }
            if c < n && curr < matrix[r][c + 1] {
                longest = longest.max(dfs(r, c + 1, matrix));
            }
            1 + longest
        }

        let mut rsf = 1;
        for i in 0..m {
            for j in 0..n {
                rsf = rsf.max(dfs(i, j, &matrix));
            }
        }
        rsf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
        assert_eq!(Solution::longest_increasing_path(matrix), 4);
        // Explanation: The longest increasing path is [1, 2, 6, 9].
        // 9   9   4
        // ^
        // 6   6   8
        // ^
        // 2 < 1   1
    }
    #[test]
    fn example2() {
        let matrix = vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]];
        assert_eq!(Solution::longest_increasing_path(matrix), 4);
        // Explanation: The longest increasing path is [3, 4, 5, 6]. Moving diagonally is not allowed.
        // 3 > 4 > 5
        //         v
        // 3   2   6
        //
        // 2   2   1
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::longest_increasing_path(vec![vec![1]]), 1);
    }

    mod performance {
        use super::*;

        #[test]
        fn matrix_200x200_1_produces_1() {
            let matrix = vec![vec![1; 200]; 200];
            assert_eq!(Solution::longest_increasing_path(matrix), 1);
        }
        #[test]
        fn matrix_200x200_0to39999_produces_399() {
            let m = 200;
            let n = 200;
            let matrix = (0..m)
                .map(|r| (0..n).map(|c| (r * n) + c).collect())
                .collect();
            assert_eq!(Solution::longest_increasing_path(matrix), 399);
        }
        #[test]
        fn matrix_200x200_0to39999_produces_40000() {
            let m = 200;
            let n = 200;
            let matrix = (0..m)
                .map(|r| if r % 2 == 0 {
                    (0..n).map(|c| (r * n) + c).collect()
                } else {
                    (0..n).rev().map(|c| (r * n) + c).collect()
                })
                .collect();
            assert_eq!(Solution::longest_increasing_path(matrix), 40_000);
        }
        #[test]
        fn matrix_200x200_39999to0_produces_40000() {
            let m = 200;
            let n = 200;
            let matrix = (0..m).rev()
                .map(|r| if r % 2 == 0 {
                    (0..n).map(|c| (r * n) + c).collect()
                } else {
                    (0..n).rev().map(|c| (r * n) + c).collect()
                })
                .collect();
            assert_eq!(Solution::longest_increasing_path(matrix), 40_000);
        }
    }
}
