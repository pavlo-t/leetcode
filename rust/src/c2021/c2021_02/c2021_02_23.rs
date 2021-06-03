#![allow(dead_code)]

/// # Search a 2D Matrix II
///
/// Write an efficient algorithm that searches for a `target` value in an `m x n` integer `matrix`.
/// The `matrix` has the following properties:
///
/// - Integers in each row are sorted in ascending from left to right.
/// - Integers in each column are sorted in ascending from top to bottom.
///
/// __Constraints:__
///
/// - `1 <= matrix.length, matrix[i].length <= 300`
/// - `-10^9 <= matix[i][j] <= 10^9`
/// - All the integers in each row are __sorted__ in ascending order.
/// - All the integers in each column are __sorted__ in ascending order.
/// - `-10^9 <= target <= 10^9`
///
/// https://leetcode.com/explore/featured/card/february-leetcoding-challenge-2021/587/week-4-february-22nd-february-28th/3650/
struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut r = matrix.len() - 1;
        let mut c = 0;
        let cols = matrix[0].len();

        while c < cols {
            if matrix[r][c] > target {
                if r == 0 {
                    break;
                }
                r -= 1;
            } else if matrix[r][c] < target {
                c += 1;
            } else {
                return true;
            }
        }

        false
    }

    pub fn search_matrix_my(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix[0].len();
        for r in matrix {
            if r[0] > target || r[n - 1] < target {
                continue;
            } else {
                if r.contains(&target) {
                    return true;
                }
            }
        }

        false
    }

    pub fn search_matrix_brute_force(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        matrix.iter().find(|&r| r.contains(&target)).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //noinspection DuplicatedCode
    fn example1() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        assert!(Solution::search_matrix(matrix, 5));
    }
    #[test]
    //noinspection DuplicatedCode
    fn example2() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        assert!(!Solution::search_matrix(matrix, 20));
    }

    #[test]
    //noinspection DuplicatedCode
    fn m_contains_in_top_left_corner() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        assert!(Solution::search_matrix(matrix, 1));
    }
    #[test]
    //noinspection DuplicatedCode
    fn m_does_not_contain() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        assert!(!Solution::search_matrix(matrix, 0));
    }
}
