#![allow(dead_code)]

use std::cell::RefCell;

// This is the BinaryMatrix's API interface.
// You should not implement it, or speculate about its implementation
#[derive(PartialEq, Eq, Clone, Debug)]
struct BinaryMatrix {
    m: Vec<Vec<i32>>,
    calls: RefCell<usize>,
}
impl BinaryMatrix {
    fn new(m: Vec<Vec<i32>>) -> Self {
        let calls = RefCell::new(0);
        BinaryMatrix { m, calls }
    }
    fn get(&self, row: i32, col: i32) -> i32 {
        let mut calls = self.calls.borrow_mut();
        *calls += 1;
        if *calls > 1000 {
            panic!("More then 1000 calls to BinaryMatrix.get(?, ?)");
        }
        self.m[row as usize][col as usize]
    }
    fn dimensions(&self) -> Vec<i32> {
        vec![self.m.len() as i32, self.m[0].len() as i32]
    }
}

/// 1428. Leftmost Column with at Least a One
/// =========================================
///
/// (This problem is an __interactive problem__.)
///
/// A __row-sorted binary matrix__ means that all elements are `0` or `1`
/// and each row of the matrix is sorted in non-decreasing order.
///
/// Given a __row-sorted binary matrix__ `binary_matrix`,
/// return _the index (0-indexed) of the __leftmost column__ with a 1 in it_.
/// If such an index does not exist, return `-1`.
///
/// __You can't access the Binary Matrix directly__.
/// You may only access the matrix using a `BinaryMatrix` interface:
///
/// - `BinaryMatrix.get(row, col)` returns the element of the matrix at index `(row, col)` (0-indexed).
/// - `BinaryMatrix.dimensions()` returns the dimensions of the matrix as a list of 2 elements `[rows, cols]`,
///   which means the matrix is `rows x cols`.
///
/// Submissions making more than `1000` calls to `BinaryMatrix.get` will be judged _Wrong Answer_.
/// Also, any solutions that attempt to circumvent the judge will result in disqualification.
///
/// For custom testing purposes, the input will be the entire binary matrix `mat`.
/// You will not have access to the binary matrix directly.
///
/// __Constraints:__
///
/// - `1 <= mat.length, mat[i].length <= 100`
/// - `mat[i][j]` is either `0` or `1`.
/// - `mat[i]` is sorted in non-decreasing order.
///
/// https://leetcode.com/problems/leftmost-column-with-at-least-a-one/
struct Solution;
impl Solution {
    /// Approach 3: Start at Top Right, Move Only Left and Down
    /// https://leetcode.com/problems/leftmost-column-with-at-least-a-one/solution/
    pub fn left_most_column_with_one(bm: &BinaryMatrix) -> i32 {
        let (rs, cs) = {
            let dims = bm.dimensions();
            (dims[0], dims[1])
        };
        let (mut r, mut c) = (0, cs - 1);
        while r < rs && c >= 0 {
            if bm.get(r, c) == 0 {
                r += 1;
            } else {
                c -= 1;
            }
        }
        if c == cs - 1 {
            -1
        } else {
            c + 1
        }
    }

    pub fn left_most_column_with_one_binary_search(bm: &BinaryMatrix) -> i32 {
        fn get_col(row: i32, cols: i32, bm: &BinaryMatrix) -> i32 {
            let (mut l, mut r) = (0, cols - 1);
            while l < r {
                let m = l + (r - l) / 2;
                if bm.get(row, m) == 0 {
                    l = m + 1;
                } else {
                    r = m;
                }
            }
            match bm.get(row, l) {
                0 => i32::MAX,
                _ => l,
            }
        }
        let (rs, cs) = {
            let dims = bm.dimensions();
            (dims[0], dims[1])
        };
        let mut res = i32::MAX;
        for r in 0..rs {
            res = res.min(get_col(r, cs, bm));
        }
        match res {
            i32::MAX => -1,
            col => col,
        }
    }

    pub fn left_most_column_with_one_brute_force(bm: &BinaryMatrix) -> i32 {
        let (rs, cs) = {
            let dims = bm.dimensions();
            (dims[0], dims[1])
        };
        let mut res = i32::MAX;
        for r in 0..rs {
            for c in 0..cs {
                if bm.get(r, c) == 1 {
                    res = res.min(c);
                    break;
                }
            }
        }
        match res {
            i32::MAX => -1,
            col => col,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn m(vv: Vec<Vec<i32>>) -> BinaryMatrix { BinaryMatrix::new(vv) }
    macro_rules! m {($($x:tt),*) => {BinaryMatrix::new(vec![$(vec!$x),*])};}

    #[test]
    fn m_00_11() {
        let bm = m![[0, 0], [1, 1]];
        assert_eq!(Solution::left_most_column_with_one(&bm), 0);
    }
    #[test]
    fn m_00_01() {
        let bm = m![[0, 0], [0, 1]];
        assert_eq!(Solution::left_most_column_with_one(&bm), 1);
    }
    #[test]
    fn m_00_00() {
        let bm = m![[0, 0], [0, 0]];
        assert_eq!(Solution::left_most_column_with_one(&bm), -1);
    }
    #[test]
    fn m_0001_0011_0111_0000() {
        let bm = m![[0, 0, 0, 1], [0, 0, 1, 1], [0, 1, 1, 1], [0, 0, 0, 0]];
        assert_eq!(Solution::left_most_column_with_one(&bm), 1);
    }
    #[test]
    fn m_100x100x0() {
        let bm = m(vec![vec![0; 100]; 100]);
        assert_eq!(Solution::left_most_column_with_one(&bm), -1);
    }
}
