#![allow(dead_code)]
/// 931. Minimum Falling Path Sum
/// =============================
///
/// Given an `n x n` array of integers `matrix`, return _the __minimum sum__ of any __falling path__ through `matrix`_.
///
/// A __falling path__ starts at any element in the first row
/// and chooses the element in the next row that is either directly below or diagonally left/right.
/// Specifically, the next element from position `(row, col)`
/// will be `(row + 1, col - 1)`, `(row + 1, col)`, or `(row + 1, col + 1)`.
///
/// __Constraints:__
///
/// - `n == matrix.length`
/// - `n == matrix[i].length`
/// - `1 <= n <= 100`
/// - `-100 <= matrix[i][j] <= 100`
///
/// https://leetcode.com/problems/minimum-falling-path-sum/
struct Solution;
impl Solution {
    /// 19:00-19:03
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        println!("min_falling_path_sum({:?})", matrix);
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut curr = vec![0; n + 2];
        curr[0] = i32::MAX;
        curr[n + 1] = i32::MAX;
        let mut prev = curr.clone();
        for r in (0..m).rev() {
            std::mem::swap(&mut curr, &mut prev);
            for c in 1..=n {
                curr[c] = matrix[r][c - 1] + prev[c - 1].min(prev[c]).min(prev[c + 1]);
            }
        }
        curr.into_iter().min().unwrap()
    }
    /// 18:54-19:00
    pub fn min_falling_path_sum_dp_vec_vec(matrix: Vec<Vec<i32>>) -> i32 {
        println!("min_falling_path_sum({:?})", matrix);
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![i32::MAX; n + 2]; m + 1];
        for c in 1..=n {
            dp[m][c] = 0;
        }
        for r in (0..m).rev() {
            for c in 1..=n {
                dp[r][c] =
                    matrix[r][c - 1] + dp[r + 1][c - 1].min(dp[r + 1][c]).min(dp[r + 1][c + 1]);
            }
        }
        dp[0].iter().min().unwrap().to_owned()
    }
    /// 18:51-18:54
    pub fn min_falling_path_sum_rec_with_memo(matrix: Vec<Vec<i32>>) -> i32 {
        println!("min_falling_path_sum({:?})", matrix);
        fn rec(r: usize, c: usize, m: &[Vec<i32>], memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
            if r == m.len() - 1 {
                m[r][c]
            } else if let Some(res) = memo[r][c] {
                res
            } else {
                let res = [c, c + 1, c.wrapping_sub(1)]
                    .iter()
                    .filter(|&&c| c < m[0].len())
                    .map(|&c| rec(r + 1, c, m, memo))
                    .min()
                    .unwrap()
                    + m[r][c];
                memo[r][c] = Some(res);
                res
            }
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut memo = vec![vec![None; n]; m];
        (0..matrix.len())
            .map(|c| rec(0, c, &matrix, &mut memo))
            .min()
            .unwrap()
    }
    /// 18:42-18:50
    pub fn min_falling_path_sum_rec(matrix: Vec<Vec<i32>>) -> i32 {
        println!("min_falling_path_sum({:?})", matrix);
        fn rec(r: usize, c: usize, m: &[Vec<i32>]) -> i32 {
            if r == m.len() - 1 {
                m[r][c]
            } else {
                [c, c + 1, c.wrapping_sub(1)]
                    .iter()
                    .filter(|&&c| c < m[0].len())
                    .map(|&c| rec(r + 1, c, m))
                    .min()
                    .unwrap()
                    + m[r][c]
            }
        }
        (0..matrix.len()).map(|c| rec(0, c, &matrix)).min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn m_213_654_789() {
        let m = vv![[2, 1, 3], [6, 5, 4], [7, 8, 9]];
        assert_eq!(Solution::min_falling_path_sum(m), 13);
        // Explanation: There are two falling paths with a minimum sum:
        // [[_,1,_],      [[_,1,_],
        //  [_,5,_],       [_,_,4],
        //  [7,_,_]]       [_,8,_]]
    }
    #[test]
    fn m_m19p57_m40m5() {
        let m = vv![[-19, 57], [-40, -5]];
        assert_eq!(Solution::min_falling_path_sum(m), -59);
        // Explanation: The falling path with a minimum sum:
        // [[-19,_],
        //  [-40,_]]
    }
    #[test]
    fn m_m48() {
        let m = vv![[-48]];
        assert_eq!(Solution::min_falling_path_sum(m), -48);
    }

    #[test]
    fn m_100x100x1() {
        let m = vec![vec![1; 100]; 100];
        assert_eq!(Solution::min_falling_path_sum(m), 100);
    }
}
