#![allow(dead_code)]
/// 1314. Matrix Block Sum
/// ======================
///
/// Given a `m x n` matrix `mat` and an integer `k`,
/// return _a matrix `answer` where each `answer[i][j]` is the sum of all elements `mat[r][c]` for_:
///
/// - `i - k <= r <= i + k`,
/// - `j - k <= c <= j + k`, and
/// - `(r, c)` is a valid position in the matrix.
///
/// __Constraints:__
///
/// - `1 <= mat.length, mat[i].length, k <= 100`
/// - `1 <= mat[i][j] <= 100`
///
/// https://leetcode.com/problems/matrix-block-sum/
struct Solution;
impl Solution {
    /// 22:03-22:48
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        println!("matrix_block_sum({:?}, {})", mat, k);
        let (m, n) = (mat.len(), mat[0].len());
        let k = k as usize;
        let mut ps = mat.clone();
        for r in 1..m {
            ps[r][0] += ps[r - 1][0];
        }
        for c in 1..n {
            ps[0][c] += ps[0][c - 1];
        }
        for r in 1..m {
            for c in 1..n {
                ps[r][c] += ps[r - 1][c] + ps[r][c - 1] - ps[r - 1][c - 1];
            }
        }
        let mut result = vec![vec![0; n]; m];
        for r in 0..m {
            for c in 0..n {
                let (psr_max, psc_max) = ((r + k).min(m - 1), (c + k).min(n - 1));
                let (psr_out, psc_out) = (r.checked_sub(k + 1), c.checked_sub(k + 1));
                let total = ps[psr_max][psc_max];
                let rs_out = psr_out.map(|r_min| ps[r_min][psc_max]).unwrap_or(0);
                let cs_out = psc_out.map(|c_min| ps[psr_max][c_min]).unwrap_or(0);
                let double_subbed = psr_out.zip(psc_out).map(|(r, c)| ps[r][c]).unwrap_or(0);
                // In LeetCode's rust version `Option.zip` is still `unstable`
                //let double_subbed = psr_out.and_then(|r| psc_out.map(|c| ps[r][c])).unwrap_or(0);
                result[r][c] = total - rs_out - cs_out + double_subbed;
            }
        }
        result
    }
    /// 21:53-22:01; actually good enough with `--release` flag :)
    pub fn matrix_block_sum_brute_force(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        println!("matrix_block_sum({:?}, {})", mat, k);
        let (m, n) = (mat.len(), mat[0].len());
        let k = k as usize;
        let mut result = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                for r in i.saturating_sub(k)..=(i + k).min(m - 1) {
                    for c in j.saturating_sub(k)..=(j + k).min(n - 1) {
                        result[i][j] += mat[r][c];
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
    fn m_123_456_789_k_1() {
        #[rustfmt::skip]
        let m = vv![[1, 2, 3],
                    [4, 5, 6],
                    [7, 8, 9]];
        #[rustfmt::skip]
        // for (r <- (i-k) to (i+k); c <- (j-k) to (j+k)) ???
        // (1+2+4+5    ) (1+2+3+4+5+6      ) (2+3+5+6    )
        // (1+2+4+5+7+8) (1+2+3+4+5+6+7+8+9) (2+3+5+6+8+9)
        // (    4+5+7+8) (      4+5+6+7+8+9) (    5+6+8+9)
        //
        // __Prefixes__: ps[i][j] = mat[i][j] + ps[i-1][j] + ps[i][j-1] - ps[i-1][j-1]
        // | 1  3  6|  <=  (1 + 0 + 0 - 0) (2 +  0 +  1 - 0) (3 +  0 +  3 -  0)
        // | 5 12 21|  <=  (4 + 1 + 0 - 0) (5 +  3 +  5 - 1) (6 +  6 + 12 -  3)
        // |12 27 45|  <=  (7 + 5 + 0 - 0) (8 + 12 + 12 - 5) (9 + 21 + 27 - 12)
        //
        // res[i][j] = ps[i+k][j+k] - ps[i-k-1][j+k] - ps[i+k][j-k-1] + ps[i-k-1][j-k-1]
        // (12-0-0+0) (21-0-0+0) (21-0-5+0)
        // (27-0-0+0) (45-0-0+0) (45-0-12+0)
        // (27-3-0+0) (45-6-0+0) (45-6-12+1)
        let e = vv![[12, 21, 16],
                    [27, 45, 33],
                    [24, 39, 28]];
        assert_eq!(Solution::matrix_block_sum(m, 1), e);
    }
    #[test]
    fn m_123_456_789_k_2() {
        #[rustfmt::skip]
        let m = vv![[1, 2, 3],
                    [4, 5, 6],
                    [7, 8, 9]];
        #[rustfmt::skip]
        let e = vv![[45, 45, 45],
                    [45, 45, 45],
                    [45, 45, 45]];
        assert_eq!(Solution::matrix_block_sum(m, 2), e);
    }

    #[test]
    fn m_100x100x1_k_100() {
        let m = vec![vec![1; 100]; 100];
        let e = vec![vec![10_000; 100]; 100];
        assert_eq!(Solution::matrix_block_sum(m, 100), e);
    }
}
