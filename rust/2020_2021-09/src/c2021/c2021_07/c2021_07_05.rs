#![allow(dead_code)]
/// Reshape the Matrix
/// ==================
///
/// In MATLAB, there is a handy function called `reshape` which can reshape an `m x n` matrix into a new one
/// with a different size `r x c` keeping its original data.
///
/// You are given an `m x n` matrix mat and two integers `r` and `c` representing the row number and column number
/// of the wanted reshaped matrix.
///
/// The reshaped matrix should be filled with all the elements of the original matrix
/// in the same row-traversing order as they were.
///
/// If the `reshape` operation with given parameters is possible and legal, output the new reshaped matrix;
/// Otherwise, output the original matrix.
///
/// __Constraints:__
///
/// - `1 <= mat.length, mat[i].length <= 100`
/// - `-1000 <= mat[i][j] <= 1000`
/// - `1 <= r, c <= 300`
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/608/week-1-july-1st-july-7th/3803/
struct Solution;
impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let (r, c) = (r as usize, c as usize);
        if r * c != m * n {
            mat
        } else {
            let mut result = vec![vec![0; c]; r];
            let (mut nr, mut nc) = (0,0);
            for or in 0..m {
                for oc in 0..n {
                    result[nr][nc] = mat[or][oc];
                    if nc < c - 1 {
                        nc += 1;
                    } else {
                        nr += 1;
                        nc = 0;
                    }
                }
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn m1_2n3_4_r1_c4_produces_m1_2_3_4() {
        let mat = vv![[1, 2], [3, 4]];
        let e = vv![[1, 2, 3, 4]];
        assert_eq!(Solution::matrix_reshape(mat, 1, 4), e);
    }
    #[test]
    fn m1_2n3_4_r2_c4_produces_m1_2n3_4() {
        let mat = vv![[1, 2], [3, 4]];
        let e = vv![[1, 2], [3, 4]];
        assert_eq!(Solution::matrix_reshape(mat, 2, 4), e);
    }
}
