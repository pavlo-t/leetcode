#![allow(dead_code)]
/// Number of Submatrices That Sum to Target
/// ========================================
///
/// Given a `matrix` and a `target`, return the number of non-empty submatrices that sum to target.
///
/// A submatrix `x1, y1, x2, y2` is the set of all cells `matrix[x][y]` with `x1 <= x <= x2` and `y1 <= y <= y2`.
///
/// Two submatrices `(x1, y1, x2, y2)` and `(x1', y1', x2', y2')` are different if they have some
/// coordinate that is different: for example, if `x1 != x1'`.
///
/// __Constraints:__
///
/// - `1 <= matrix.length <= 100`
/// - `1 <= matrix[0].length <= 100`
/// - `-1000 <= matrix[i] <= 1000`
/// - `-10^8 <= target <= 10^8`
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/595/week-3-april-15th-april-21st/3711/
struct Solution;
impl Solution {
    pub fn num_submatrix_sum_target(mut matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        use std::collections::HashMap;

        let x_len = matrix.len();
        let y_len = matrix[0].len();
        for x in 0..x_len {
            for y in 1..y_len {
                matrix[x][y] += matrix[x][y - 1];
            }
        }

        let mut cache = HashMap::new();
        let mut result = 0;
        for y1 in 0..y_len {
            for y2 in y1..y_len {
                cache.clear();
                cache.insert(0, 1);
                let mut sum = 0;
                for x in 0..x_len {
                    sum += matrix[x][y2] - (if y1 > 0 { matrix[x][y1 - 1] } else { 0 });
                    result += cache.get(&(sum - target)).unwrap_or(&0);
                    cache.insert(sum, *cache.get(&sum).unwrap_or(&0) + 1);
                }
            }
        }

        result
    }

    pub fn num_submatrix_sum_target_cache_times_out(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        fn sm_sum(
            x1: usize,
            y1: usize,
            x2: usize,
            y2: usize,
            matrix: &[Vec<i32>],
            cache: &mut Vec<Vec<Vec<Vec<i32>>>>,
        ) -> i32 {
            if cache[x1][y1][x2][y2] > i32::MIN {
                cache[x1][y1][x2][y2]
            } else if x1 == x2 && y1 == y2 {
                matrix[x1][y1]
            } else if x1 == x2 {
                let ym = y1 + (y2 - y1) / 2;
                cache[x1][y1][x2][y2] = sm_sum(x1, y1, x2, ym, matrix, cache);
                cache[x1][y1][x2][y2] += sm_sum(x1, ym + 1, x2, y2, matrix, cache);
                cache[x1][y1][x2][y2]
            } else if y1 == y2 {
                let xm = x1 + (x2 - x1) / 2;
                cache[x1][y1][x2][y2] = sm_sum(x1, y1, xm, y2, matrix, cache);
                cache[x1][y1][x2][y2] += sm_sum(xm + 1, y1, x2, y2, matrix, cache);
                cache[x1][y1][x2][y2]
            } else {
                let xm = x1 + (x2 - x1) / 2;
                let ym = y1 + (y2 - y1) / 2;
                cache[x1][y1][x2][y2] = sm_sum(x1, y1, xm, ym, matrix, cache);
                cache[x1][y1][x2][y2] += sm_sum(xm + 1, y1, x2, ym, matrix, cache);
                cache[x1][y1][x2][y2] += sm_sum(x1, ym + 1, xm, y2, matrix, cache);
                cache[x1][y1][x2][y2] += sm_sum(xm + 1, ym + 1, x2, y2, matrix, cache);
                cache[x1][y1][x2][y2]
            }
        }

        let m = matrix.len();
        let n = matrix[0].len();
        let mut cache = vec![vec![vec![vec![i32::MIN; n]; m]; n]; m];
        let mut result = 0;
        for x1 in 0..m {
            for y1 in 0..n {
                for x2 in x1..m {
                    for y2 in y1..n {
                        if sm_sum(x1, y1, x2, y2, &matrix, &mut cache) == target {
                            result += 1;
                        }
                    }
                }
            }
        }
        result
    }

    pub fn num_submatrix_sum_target_brute_force(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let sm_sum = |x1: usize, y1: usize, x2: usize, y2: usize| {
            let mut result = 0;
            for x in x1..=x2 {
                for y in y1..=y2 {
                    result += matrix[x][y];
                }
            }
            result
        };

        let mut result = 0;
        let m = matrix.len();
        let n = matrix[0].len();
        for x1 in 0..m {
            for y1 in 0..n {
                for x2 in x1..m {
                    for y2 in y1..n {
                        if sm_sum(x1, y1, x2, y2) == target {
                            result += 1;
                        }
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let matrix = vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]];
        assert_eq!(Solution::num_submatrix_sum_target(matrix, 0), 4);
        // 0 1 0
        // 1 1 1
        // 0 1 0
        // Explanation: The four 1x1 submatrices that only contain 0.
    }
    #[test]
    fn example2() {
        let matrix = vec![vec![1, -1], vec![-1, 1]];
        assert_eq!(Solution::num_submatrix_sum_target(matrix, 0), 5);
        // Explanation: The two 1x2 submatrices, plus the two 2x1 submatrices, plus the 2x2 submatrix.
    }
    #[test]
    fn example3() {
        let matrix = vec![vec![904]];
        assert_eq!(Solution::num_submatrix_sum_target(matrix, 0), 0);
    }

    #[test]
    fn test37() {
        let matrix = vec![vec![-84; 81]; 71];
        assert_eq!(Solution::num_submatrix_sum_target(matrix, -12348), 11884);
    }

    mod performance {
        use super::*;

        #[test]
        fn test_matrix_25x25_produces_625() {
            let matrix = vec![vec![1; 25]; 25];
            assert_eq!(Solution::num_submatrix_sum_target(matrix, 1), 625);
        }
        #[test]
        fn test_matrix_25x25_produces_105_625() {
            let matrix = vec![vec![0; 25]; 25];
            assert_eq!(Solution::num_submatrix_sum_target(matrix, 0), 105_625);
        }

        #[test]
        fn test_matrix_70x70_produces_4900() {
            let matrix = vec![vec![1; 70]; 70];
            assert_eq!(Solution::num_submatrix_sum_target(matrix, 1), 4900);
        }
        #[test]
        fn test_matrix_70x70_produces_25_502_500() {
            let matrix = vec![vec![0; 70]; 70];
            assert_eq!(Solution::num_submatrix_sum_target(matrix, 0), 6_175_225);
        }

        #[test]
        fn test_matrix_100x100_produces_10000() {
            let matrix = vec![vec![1; 100]; 100];
            assert_eq!(Solution::num_submatrix_sum_target(matrix, 1), 10_000);
        }
        #[test]
        fn test_matrix_100x100_produces_25_502_500() {
            let matrix = vec![vec![0; 100]; 100];
            assert_eq!(Solution::num_submatrix_sum_target(matrix, 0), 25_502_500);
        }
    }
}
