#![allow(dead_code)]
/// 221. Maximal Square
/// ===================
///
/// Given an `m x n` binary `matrix` filled with `0`'s and `1`'s,
/// _find the largest square containing only `1`'s and return its area_.
///
/// __Constraints:__
///
/// - `1 <= matrix.length, matrix[i].length <= 300`
/// - `matrix[i][j]` is `'0'` or `'1'`.
///
/// https://leetcode.com/problems/maximal-square/
struct Solution;
impl Solution {
    /// 23:56-23:58
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        println!("maximal_square({:?})", matrix);
        fn rec(r: usize, c: usize, mat: &[Vec<char>], memo: &mut Vec<Vec<i32>>) -> i32 {
            let (m, n) = (mat.len(), mat[0].len());
            if r == m || c == n || mat[r][c] == '0' {
                0
            } else if memo[r][c] >= 0 {
                memo[r][c]
            } else {
                memo[r][c] = 1 + rec(r + 1, c, mat, memo)
                    .min(rec(r, c + 1, mat, memo))
                    .min(rec(r + 1, c + 1, mat, memo));
                memo[r][c]
            }
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut max_side = 0;
        let mut memo = vec![vec![-1; n]; m];
        for r in 0..m {
            for c in 0..n {
                max_side = max_side.max(rec(r, c, &matrix, &mut memo));
            }
        }
        max_side * max_side
    }
    /// 23:47-23:56
    pub fn maximal_square_rec(matrix: Vec<Vec<char>>) -> i32 {
        println!("maximal_square({:?})", matrix);
        fn rec(r: usize, c: usize, mat: &[Vec<char>]) -> i32 {
            let (m, n) = (mat.len(), mat[0].len());
            if r == m || c == n || mat[r][c] == '0' {
                0
            } else {
                1 + rec(r + 1, c, mat)
                    .min(rec(r, c + 1, mat))
                    .min(rec(r + 1, c + 1, mat))
            }
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut max_side = 0;
        for r in 0..m {
            for c in 0..n {
                max_side = max_side.max(rec(r, c, &matrix));
            }
        }
        max_side * max_side
    }
    /// Approach #3 (Better Dynamic Programming) [Accepted]
    /// https://leetcode.com/problems/maximal-square/solution/
    pub fn maximal_square_dp_vec(matrix: Vec<Vec<char>>) -> i32 {
        println!("maximal_square({:?})", matrix);
        let (rows, cols) = (matrix.len(), matrix[0].len());
        let mut dp = vec![0; cols + 1];
        let mut prev = 0;
        let mut max_sq_len = 0;
        for i in 1..=rows {
            for j in 1..=cols {
                let tmp = dp[j];
                if matrix[i - 1][j - 1] == '1' {
                    dp[j] = dp[j - 1].min(dp[j]).min(prev) + 1;
                    max_sq_len = max_sq_len.max(dp[j]);
                }
                prev = tmp;
            }
        }
        max_sq_len * max_sq_len
    }
    /// Approach #2 (Dynamic Programming) [Accepted]
    /// https://leetcode.com/problems/maximal-square/solution/
    pub fn maximal_square_dp_vec_vec(matrix: Vec<Vec<char>>) -> i32 {
        println!("maximal_square({:?})", matrix);
        let (rows, cols) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; cols + 1]; rows + 1];
        let mut max_sq_len = 0;
        for i in 1..=rows {
            for j in 1..=cols {
                if matrix[i - 1][j - 1] == '1' {
                    dp[i][j] = dp[i][j - 1].min(dp[i - 1][j]).min(dp[i - 1][j - 1]) + 1;
                    max_sq_len = max_sq_len.max(dp[i][j]);
                }
            }
        }
        max_sq_len * max_sq_len
    }
    /// 22:53-23:12
    pub fn maximal_square_dp_my_3_dim(matrix: Vec<Vec<char>>) -> i32 {
        println!("maximal_square({:?})", matrix);
        let (m, n) = (matrix.len(), matrix[0].len());
        let max_s = m.min(n);
        let mut dp = vec![vec![vec![-1; max_s + 1]; n]; m];
        for r in 0..m {
            for c in 0..n {
                dp[r][c][1] = (matrix[r][c] == '1') as i32;
            }
        }
        for s in 2..=max_s {
            for r in 0..=m - s {
                for c in 0..=n - s {
                    let ps = s - 1;
                    let ts = (ps * ps) as i32;
                    dp[r][c][s] = if dp[r][c][ps] == ts
                        && dp[r][c + 1][ps] == ts
                        && dp[r + 1][c][ps] == ts
                        && dp[r + 1][c + 1][ps] == ts
                    {
                        (s * s) as i32
                    } else {
                        dp[r][c][ps]
                            .max(dp[r][c + 1][ps])
                            .max(dp[r + 1][c][ps])
                            .max(dp[r + 1][c + 1][ps])
                    };
                }
            }
        }
        let mut result = 0;
        for r in 0..=m - max_s {
            for c in 0..=n - max_s {
                result = result.max(dp[r][c][max_s]);
            }
        }
        result
    }
    /// 22:44-22:53
    pub fn maximal_square_rec_with_memo_3_dim(matrix: Vec<Vec<char>>) -> i32 {
        println!("maximal_square({:?})", matrix);
        fn rec(
            r: usize,
            c: usize,
            s: usize,
            m: &[Vec<char>],
            memo: &mut Vec<Vec<Vec<i32>>>,
        ) -> i32 {
            if s == 1 {
                (m[r][c] == '1') as i32
            } else if memo[r][c][s] >= 0 {
                memo[r][c][s]
            } else {
                let ns = s - 1;
                let tl = rec(r, c, ns, m, memo);
                let tr = rec(r, c + 1, ns, m, memo);
                let bl = rec(r + 1, c, ns, m, memo);
                let br = rec(r + 1, c + 1, ns, m, memo);
                memo[r][c][s] = if (ns * ns) as i32 == tl && tl == tr && tr == bl && bl == br {
                    (s * s) as i32
                } else {
                    tl.max(tr).max(bl).max(br)
                };
                memo[r][c][s]
            }
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let s = m.min(n);
        let mut result = 0;
        let mut memo = vec![vec![vec![-1; s + 1]; n]; m];
        for r in 0..=m - s {
            for c in 0..=n - s {
                result = result.max(rec(r, c, s, &matrix, &mut memo))
            }
        }
        result
    }
    /// 22:15-22:44
    pub fn maximal_square_rec_3_dim(matrix: Vec<Vec<char>>) -> i32 {
        println!("maximal_square({:?})", matrix);
        fn rec(r: usize, c: usize, s: usize, m: &[Vec<char>]) -> i32 {
            if s == 1 {
                (m[r][c] == '1') as i32
            } else {
                let ns = s - 1;
                let tl = rec(r, c, ns, m);
                let tr = rec(r, c + 1, ns, m);
                let bl = rec(r + 1, c, ns, m);
                let br = rec(r + 1, c + 1, ns, m);
                if (ns * ns) as i32 == tl && tl == tr && tr == bl && bl == br {
                    (s * s) as i32
                } else {
                    tl.max(tr).max(bl).max(br)
                }
            }
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let s = m.min(n);
        let mut result = 0;
        for r in 0..=m - s {
            for c in 0..=n - s {
                result = result.max(rec(r, c, s, &matrix))
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
    fn m_01_10() {
        let m = vv![['0', '1'], ['1', '0']];
        assert_eq!(Solution::maximal_square(m), 1);
    }
    #[test]
    fn m_0() {
        let m = vv![['0']];
        assert_eq!(Solution::maximal_square(m), 0);
    }
    #[test]
    fn m_10100_10111_11111_10111_11111() {
        let m = vv![
            ['1', '0', '1', '0', '0'],
            ['1', '0', '1', '1', '1'],
            ['1', '1', '1', '1', '1'],
            ['1', '0', '1', '1', '1'],
            ['1', '1', '1', '1', '1']
        ];
        assert_eq!(Solution::maximal_square(m), 9);
    }

    //#[ignore]
    #[test]
    fn m_300x300x0() {
        let m = vec![vec!['0'; 300]; 300];
        assert_eq!(Solution::maximal_square(m), 0);
    }
    //#[ignore]
    #[test]
    fn m_300x300x1() {
        let m = vec![vec!['1'; 300]; 300];
        assert_eq!(Solution::maximal_square(m), 90_000);
    }
}
