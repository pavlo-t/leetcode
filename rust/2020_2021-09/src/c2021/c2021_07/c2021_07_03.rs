#![allow(dead_code)]
/// Max Sum of Rectangle No Larger Than K
/// =====================================
///
/// Given an `m x n` matrix `matrix` and an integer `k`,
/// return _the max sum of a rectangle in the matrix such that its sum is no larger than_ `k`.
///
/// It is __guaranteed__ that there will be a rectangle with a sum no larger than `k`.
///
/// __Constraints:__
///
/// - `1 <= matrix.length, matrix[i].length <= 100`
/// - `-100 <= matrix[i][j] <= 100`
/// - `-100_000 <= k <= 100_000`
///
///
/// __Follow up:__ What if the number of rows is much larger than the number of columns?
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/608/week-1-july-1st-july-7th/3801/
struct Solution;
impl Solution {
    /// https://rustgym.com/leetcode/363
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::collections::BTreeSet;

        let n = matrix.len();
        let m = matrix[0].len();
        let mut prefix = vec![vec![0; m + 1]; n];
        for i in 0..n {
            for j in 0..m {
                prefix[i][j + 1] = prefix[i][j] + matrix[i][j];
            }
        }
        let mut res = i32::MIN;
        for start in 0..m {
            for end in start + 1..=m {
                let mut bts: BTreeSet<i32> = BTreeSet::new();
                bts.insert(0);
                let mut sum = 0;
                for i in 0..n {
                    sum += prefix[i][end] - prefix[i][start];
                    if let Some(prev) = bts.range(sum - k..).take(1).next() {
                        res = res.max(sum - prev);
                    }
                    bts.insert(sum);
                }
            }
        }
        res
    }

    pub fn max_sum_submatrix_my_slow(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for r in 0..m {
            for c in 0..n {
                dp[r + 1][c + 1] = matrix[r][c] + dp[r + 1][c] + dp[r][c + 1] - dp[r][c];
            }
        }

        let mut result = i32::MIN;
        for fc in 1..=n {
            for lc in fc..=n {
                for fr in 1..=m {
                    for lr in fr..=m {
                        let (pr, pc) = (fr - 1, fc - 1);
                        let sum = dp[lr][lc] - dp[lr][pc] - dp[pr][lc] + dp[pr][pc];
                        if sum <= k {
                            result = result.max(sum);
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

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn should_work_with_part_of_the_matrix_being_the_answer() {
        let matrix = vv![[1, 0, 1], [0, -2, 3]];
        assert_eq!(Solution::max_sum_submatrix(matrix, 2), 2);
        // Explanation: Because the sum of the rectangle [[0, 1], [-2, 3]] is 2,
        // and 2 is the max number no larger than k (k = 2).
    }
    #[test]
    fn should_work_with_whole_matrix_being_the_answer() {
        let matrix = vv![[2, 2, -1]];
        assert_eq!(Solution::max_sum_submatrix(matrix, 3), 3);
    }
    #[test]
    fn should_work_with_k_being_larger_than_the_result() {
        let matrix = vv![[2, 2, -1]];
        assert_eq!(Solution::max_sum_submatrix(matrix, 5), 4);
    }

    #[test]
    fn test_26() {
        // let dp = [
        //     [0, 0, 0, 0, 0],
        //     [0, 5, 1,-2, 2],
        //     [0, 2,-6,-5, 4],
        //     [0, 7, 0, 6,11]];
        let matrix = vv![[5, -4, -3, 4], [-3, -4, 4, 5], [5, 1, 5, -4]];
        assert_eq!(Solution::max_sum_submatrix(matrix, 3), 2);
    }

    #[test]
    fn should_work_with_k_being_in_the_matrix() {
        let matrix = vv![
            [0, 1, 2, 3],
            [10, 11, 12, 13],
            [20, 21, 22, 23],
            [30, 31, 32, 33]
        ];
        //    0   1   3   6
        //   10  21  33  46
        //   20  41  63  86
        //   30  61  93 126
        //
        //    0   1   3   6
        //   10  22  36  52
        //   30  63  99 138
        //   60 124 192 264
        assert_eq!(Solution::max_sum_submatrix(matrix, 31), 31);
    }

    mod performance {
        use super::*;

        #[ignore]
        #[test]
        fn should_work_with_100x100_matrix() {
            let matrix = vec![vec![1; 100]; 100];
            assert_eq!(Solution::max_sum_submatrix(matrix, 100_000), 10_000);
        }
    }
}
