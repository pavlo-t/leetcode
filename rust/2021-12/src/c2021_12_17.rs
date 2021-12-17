#![allow(dead_code)]
/// 221. Maximal Square
/// ===================
///
/// Given an `m x n` binary matrix filled with `0`'s and `1`'s,
/// find _the largest square containing only `1`'s and return its area_.
///
/// __Constraints:__
///
/// - `m == matrix.length`
/// - `n == matrix[i].length`
/// - `1 <= m, n <= 300`
/// - `matrix[i][j]` is `'0'` or `'1'`.
///
/// https://leetcode.com/problems/maximal-square/
struct Solution;
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        println!("maximal_square({:?})", matrix);
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![0; n + 1];
        let mut prev_rc = 0;

        let mut max_side = 0;
        for r in (0..m).rev() {
            for c in (0..n).rev() {
                if matrix[r][c] == '1' {
                    let curr = 1 + dp[c + 1].min(dp[c]).min(prev_rc);
                    max_side = max_side.max(curr);
                    prev_rc = dp[c];
                    dp[c] = curr;
                } else {
                    dp[c] = 0;
                }
            }
        }

        max_side * max_side
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn m_0() {
        let m = vv![['0']];
        assert_eq!(Solution::maximal_square(m), 0);
    }
    #[test]
    fn m_01_10() {
        let m = vv![['0', '1'], ['1', '0']];
        assert_eq!(Solution::maximal_square(m), 1);
    }
    #[test]
    fn m_10100_10111_11111_10010() {
        let m = vv![
            ['1', '0', '1', '0', '0'],
            ['1', '0', '1', '1', '1'],
            ['1', '1', '1', '1', '1'],
            ['1', '0', '0', '1', '0']
        ];
        assert_eq!(Solution::maximal_square(m), 4);
    }
    #[test]
    fn m_11111_11111_11111_11111() {
        let m = vv![
            ['1', '1', '1', '1', '1'],
            ['1', '1', '1', '1', '1'],
            ['1', '1', '1', '1', '1'],
            ['1', '1', '1', '1', '1']
        ];
        assert_eq!(Solution::maximal_square(m), 16);
    }
    #[test]
    fn m() {
        let m = vv![
            ['1', '1', '1', '1', '1'],
            ['1', '1', '1', '1', '1'],
            ['0', '0', '0', '0', '0'],
            ['1', '1', '1', '1', '1'],
            ['1', '1', '1', '1', '1']
        ];
        assert_eq!(Solution::maximal_square(m), 4);
    }
}
