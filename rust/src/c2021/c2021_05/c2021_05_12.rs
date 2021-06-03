#![allow(dead_code)]
/// Range Sum Query 2D - Immutable
/// ==============================
///
/// Given a 2D matrix `matrix`, handle multiple queries of the following type:
///
/// 1. Calculate the __sum__ of the elements of `matrix` inside the rectangle defined by
///    its __upper left corner__ `(row1, col1)` and __lower right corner__ `(row2, col2)`.
///
/// Implement the NumMatrix class:
///
/// - `NumMatrix(int[][] matrix)` Initializes the object with the integer matrix `matrix`.
/// - `int sumRegion(int row1, int col1, int row2, int col2)` Returns the __sum__ of the elements
///   of `matrix` inside the rectangle defined by its __upper left corner__ `(row1, col1)` and
///   __lower right corner__ `(row2, col2)`.
///
/// __Constraints:__
///
/// - `m == matrix.length`
/// - `n == matrix[i].length`
/// - `1 <= m, n <= 200`
/// - `-105 <= matrix[i][j] <= 105`
/// - `0 <= row1 <= row2 < m`
/// - `0 <= col1 <= col2 < n`
/// - At most `10_000` calls will be made to `sumRegion`.
///
/// https://leetcode.com/explore/featured/card/may-leetcoding-challenge-2021/599/week-2-may-8th-may-14th/3740/
struct NumMatrix {
    sums: Vec<Vec<i32>>,
}
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let sums = matrix
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .fold((vec![], 0), |(mut acc, rsf), i| {
                        let next = rsf + i;
                        acc.push(next);
                        (acc, next)
                    })
                    .0
            })
            .collect();
        Self { sums }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (r1, c1, r2, c2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
        if c1 == 0 {
            (r1..=r2).fold(0, |rsf, r| rsf + self.sums[r][c2])
        } else {
            (r1..=r2).fold(0, |rsf, r| rsf + self.sums[r][c2] - self.sums[r][c1 - 1])
        }
    }
}

struct NumMatrixMy0Padded {
    sums: Vec<Vec<i32>>,
}
impl NumMatrixMy0Padded {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let sums = matrix
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .fold((vec![0], 0), |(mut acc, rsf), i| {
                        let next = rsf + i;
                        acc.push(next);
                        (acc, next)
                    })
                    .0
            })
            .collect();
        Self { sums }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (r1, c1, r2, c2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
        (r1..=r2).fold(0, |rsf, r| rsf + self.sums[r][c2 + 1] - self.sums[r][c1])
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {
        ($x:tt; $n:tt) => { vec![vec!$x; $n] };
        ($($x:tt),*) => { vec![$(vec!$x),*] };
    }

    #[test]
    fn example1() {
        let m = vv![
            [3, 0, 1, 4, 2],
            [5, 6, 3, 2, 1],
            [1, 2, 0, 1, 5],
            [4, 1, 0, 1, 7],
            [1, 0, 3, 0, 5]
        ];
        let m = NumMatrix::new(m);
        assert_eq!(m.sum_region(2, 1, 4, 3), 8);
        assert_eq!(m.sum_region(1, 1, 2, 2), 11);
        assert_eq!(m.sum_region(1, 2, 2, 4), 12);
        // Explanation
        // NumMatrix numMatrix = new NumMatrix(
        //   [[3, 0, 1, 4, 2], [5, 6, 3, 2, 1], [1, 2, 0, 1, 5], [4, 1, 0, 1, 7], [1, 0, 3, 0, 5]]);
        // numMatrix.sumRegion(2, 1, 4, 3); // return 8
        // numMatrix.sumRegion(1, 1, 2, 2); // return 11
        // numMatrix.sumRegion(1, 2, 2, 4); // return 12
    }

    mod performance {
        use super::*;

        #[test]
        fn m200x200_10k_assertions() {
            let m = NumMatrix::new(vec![vec![1; 200]; 200]);
            for r in 0..100 {
                for c in 0..100 {
                    assert_eq!(m.sum_region(0, 0, r, c), (r + 1) * (c + 1));
                }
            }
        }
    }
}
