#![allow(dead_code)]

/// # Pacific Atlantic Water Flow
///
/// Given an `m x n` matrix of non-negative integers representing the height of each unit cell in a continent,
/// the "Pacific ocean" touches the left and top edges of the matrix
/// and the "Atlantic ocean" touches the right and bottom edges.
///
/// Water can only flow in four directions (up, down, left, or right) from a cell to another one
/// with height equal or lower.
///
/// Find the list of grid coordinates where water can flow to both the Pacific and Atlantic ocean.
///
/// __Note:__
///
/// - The order of returned grid coordinates does not matter.
/// - Both `m` and `n` are less than 150.
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/591/week-4-march-22nd-march-28th/3684/
struct Solution;
impl Solution {
    pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if matrix.is_empty() {
            return Vec::new();
        }

        let rows = matrix.len();
        let cols = matrix[0].len();
        let pacific_reachable = vec![vec![false; cols]; rows];
        let atlantic_reachable = vec![vec![false; cols]; rows];

        use std::cell::RefCell;

        let pacific = RefCell::new(pacific_reachable);
        let atlantic = RefCell::new(atlantic_reachable);

        let mut stack = Vec::new();
        (0..rows).for_each(|r| stack.push((r, 0, &pacific)));
        (0..cols).for_each(|c| stack.push((0, c, &pacific)));
        (0..rows).for_each(|r| stack.push((r, cols - 1, &atlantic)));
        (0..cols).for_each(|c| stack.push((rows - 1, c, &atlantic)));

        while let Some((r, c, ocean)) = stack.pop() {
            let mut seen = ocean.borrow_mut();
            if !seen[r][c] {
                seen[r][c] = true;
                if r > 0 && matrix[r - 1][c] >= matrix[r][c] {
                    stack.push((r - 1, c, ocean));
                }
                if c > 0 && matrix[r][c - 1] >= matrix[r][c] {
                    stack.push((r, c - 1, ocean));
                }
                if r < rows - 1 && matrix[r + 1][c] >= matrix[r][c] {
                    stack.push((r + 1, c, ocean));
                }
                if c < cols - 1 && matrix[r][c + 1] >= matrix[r][c] {
                    stack.push((r, c + 1, ocean));
                }
            }
        }

        let mut result = Vec::new();
        for r in 0..rows {
            for c in 0..cols {
                if pacific.borrow()[r][c] && atlantic.borrow()[r][c] {
                    result.push(vec![r as i32, c as i32])
                }
            }
        }
        result
    }

    pub fn pacific_atlantic_dfs_rec(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if matrix.is_empty() {
            return Vec::new();
        }

        fn dfs(r: usize, c: usize, m: &[Vec<i32>], seen: &mut Vec<Vec<bool>>) {
            if !seen[r][c] {
                seen[r][c] = true;
                if r > 0 && m[r - 1][c] >= m[r][c] {
                    dfs(r - 1, c, m, seen);
                }
                if c > 0 && m[r][c - 1] >= m[r][c] {
                    dfs(r, c - 1, m, seen);
                }
                if r < m.len() - 1 && m[r + 1][c] >= m[r][c] {
                    dfs(r + 1, c, m, seen);
                }
                if c < m[0].len() - 1 && m[r][c + 1] >= m[r][c] {
                    dfs(r, c + 1, m, seen);
                }
            }
        }

        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut pacific_reachable = vec![vec![false; cols]; rows];
        let mut atlantic_reachable = vec![vec![false; cols]; rows];

        for r in 0..rows {
            dfs(r, 0, &matrix, &mut pacific_reachable);
        }
        for c in 0..cols {
            dfs(0, c, &matrix, &mut pacific_reachable);
        }
        for r in 0..rows {
            dfs(r, cols - 1, &matrix, &mut atlantic_reachable);
        }
        for c in 0..cols {
            dfs(rows - 1, c, &matrix, &mut atlantic_reachable);
        }

        let mut result = Vec::new();
        for r in 0..rows {
            for c in 0..cols {
                if pacific_reachable[r][c] && atlantic_reachable[r][c] {
                    result.push(vec![r as i32, c as i32])
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn example() {
        let matrix = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];
        let expected = vec![
            vec![0, 4],
            vec![1, 3], vec![1, 4],
            vec![2, 2],
            vec![3, 0], vec![3, 1],
            vec![4, 0],
        ].into_iter().collect::<HashSet<_>>();

        let result = Solution::pacific_atlantic(matrix);
        let result_set = result.iter().map(|v| v.clone()).collect::<HashSet<_>>();

        assert_eq!(result_set, expected);
        assert_eq!(result.len(), expected.len());
        //Given the following 5x5 matrix:
        //
        //   Pacific ~   ~   ~   ~   ~
        //        ~  1   2   2   3  (5) *
        //        ~  3   2   3  (4) (4) *
        //        ~  2   4  (5)  3   1  *
        //        ~ (6) (7)  1   4   5  *
        //        ~ (5)  1   1   2   4  *
        //           *   *   *   *   * Atlantic
    }

    #[test]
    fn test51() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![8, 9, 4],
            vec![7, 6, 5]];
        let e = vec![
            vec![0, 2],
            vec![1, 0], vec![1, 1], vec![1, 2],
            vec![2, 0], vec![2, 1], vec![2, 2]];
        assert_eq!(Solution::pacific_atlantic(matrix), e);
    }

    #[test]
    fn m_empty() {
        let e: Vec<Vec<i32>> = Vec::new();
        assert_eq!(Solution::pacific_atlantic(Vec::new()), e);
    }
    #[test]
    fn m111_111_111() {
        let matrix = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        let expected = vec![
            vec![0, 0], vec![0, 1], vec![0, 2],
            vec![1, 0], vec![1, 1], vec![1, 2],
            vec![2, 0], vec![2, 1], vec![2, 2]
        ].into_iter().collect::<HashSet<_>>();

        let result = Solution::pacific_atlantic(matrix);
        let result_set = result.iter().map(|v| v.clone()).collect::<HashSet<_>>();

        assert_eq!(result_set, expected);
        assert_eq!(result.len(), expected.len());
    }
    #[test]
    fn m_spiral() {
        let matrix = vec![
            vec![2, 2, 2, 2, 1],
            vec![1, 1, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 2, 2, 1],
            vec![1, 1, 1, 1, 1],
        ];
        let expected = vec![
            vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4],
            vec![1, 0], vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4],
            vec![2, 0], vec![2, 1], vec![2, 2], vec![2, 3], vec![2, 4],
            vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 3], vec![3, 4],
            vec![4, 0], vec![4, 1], vec![4, 2], vec![4, 3], vec![4, 4],
        ];

        assert_eq!(Solution::pacific_atlantic(matrix), expected);
    }

    /// If getting stack overflow:
    ///
    /// ```sh
    /// thread 'c2021_03::c2021_03_25::tests::performance_m150x150_1s' has overflowed its stack
    /// fatal runtime error: stack overflow
    /// ```
    ///
    /// Add `RUST_MIN_STACK=33554432` to env
    #[test]
    fn performance_m150x150_1s() {
        let m = 150;
        let matrix = vec![vec![1; m]; m];
        let mut expected = Vec::with_capacity(m * m);
        for i in 0..m {
            for j in 0..m {
                expected.push(vec![i as i32, j as i32]);
            }
        }

        assert_eq!(Solution::pacific_atlantic(matrix), expected);
    }
}
