#![allow(dead_code)]
/// Rotate Image
/// ============
///
/// You are given an _n x n_ 2D `matrix` representing an image, rotate the image by 90 degrees (clockwise).
///
/// You have to rotate the image [in-place], which means you have to modify the input 2D matrix directly.
/// __DO NOT__ allocate another 2D matrix and do the rotation.
///
/// [in-place]:https://en.wikipedia.org/wiki/In-place_algorithm
///
/// __Constraints:__
///
/// - `matrix.length == matrix[i].length`
/// - `1 <= matrix.length <= 20`
/// - `-1000 <= matrix[i][j] <= 1000`
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/596/week-4-april-22nd-april-28th/3720/
struct Solution;
impl Solution {
    pub fn rotate_transpose_reflect(matrix: &mut Vec<Vec<i32>>) {
        fn transpose(matrix: &mut Vec<Vec<i32>>) {
            for r in 0..matrix.len() {
                for c in r..matrix.len() {
                    let tmp = matrix[r][c];
                    matrix[r][c] = matrix[c][r];
                    matrix[c][r] = tmp;
                }
            }
        }
        fn reflect(matrix: &mut Vec<Vec<i32>>) {
            let n = matrix.len();
            for r in 0..n {
                for c in 0..n / 2 {
                    let tmp = matrix[r][c];
                    matrix[r][c] = matrix[r][n - c - 1];
                    matrix[r][n - c - 1] = tmp;
                }
            }
        }

        transpose(matrix);
        reflect(matrix);
    }

    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for r in 0..(n + 1) / 2 {
            for c in 0..n / 2 {
                let (rr, rc) = (n - r - 1, n - c - 1);
                let tmp = matrix[rc][r];
                matrix[rc][r] = matrix[rr][rc];
                matrix[rr][rc] = matrix[c][rr];
                matrix[c][rr] = matrix[r][c];
                matrix[r][c] = tmp;
            }
        }
    }

    pub fn rotate_my(matrix: &mut Vec<Vec<i32>>) {
        fn rotate_one(r: usize, c: usize, m: &mut Vec<Vec<i32>>) {
            let n = m.len();
            let (rr, rc) = (n - r - 1, n - c - 1);
            let (v0, v1, v2, v3) = (m[r][c], m[c][rr], m[rr][rc], m[rc][r]);
            m[r][c] = v3;
            m[c][rr] = v0;
            m[rr][rc] = v1;
            m[rc][r] = v2;
        }
        let n = matrix.len();
        for r in 0..n - 1 {
            for c in r..n - r - 1 {
                rotate_one(r, c, matrix);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = [[7, 4, 1], [8, 5, 2], [9, 6, 3]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expected);
    }
    #[test]
    fn example2() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        let expected = [
            [15, 13, 2, 5],
            [14, 3, 4, 1],
            [12, 6, 8, 9],
            [16, 7, 10, 11],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expected);
    }
    #[test]
    fn example3() {
        let mut matrix = vec![vec![1]];
        let expected = [[1]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expected);
    }
    #[test]
    fn example4() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        let expected = [[3, 1], [4, 2]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expected);
    }
}
