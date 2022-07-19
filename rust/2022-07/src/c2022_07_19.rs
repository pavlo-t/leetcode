#![allow(dead_code)]
//! \#118. Pascal's Triangle
//! ========================
//!
//! <https://leetcode.com/problems/pascals-triangle>
//!
//! Given an integer `numRows`, return the first numRows of __Pascal's triangle__.
//!
//! In __Pascal's triangle__, each number is the sum of the two numbers directly above it as shown:
//!
//! ```text
//!     1
//!    1 1
//!   1 2 1
//!  1 3 3 1
//! 1 4 6 4 1
//! ```
//!
//! __Constraints:__
//!
//! - `1 <= numRows <= 30`

pub struct Solution;
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![1]];
        for n in 1..num_rows as usize {
            let mut row = vec![1; n + 1];
            for i in 1..n {
                row[i] = result[n - 1][i - 1] + result[n - 1][i];
            }
            result.push(row);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] } }

    #[rustfmt::skip] #[test] fn n_1() { assert_eq!(Solution::generate(1), [[1]]); }
    #[rustfmt::skip] #[test] fn n_2() { assert_eq!(Solution::generate(2), vv![[1], [1, 1]]); }
    #[rustfmt::skip] #[test] fn n_3() { assert_eq!(Solution::generate(3), vv![[1], [1, 1], [1, 2, 1]]); }
    #[rustfmt::skip] #[test] fn n_4() { assert_eq!(Solution::generate(4), vv![[1], [1, 1], [1, 2, 1], [1, 3, 3, 1]]); }
    #[rustfmt::skip] #[test] fn n_5() { assert_eq!(Solution::generate(5), vv![[1], [1, 1], [1, 2, 1], [1, 3, 3, 1], [1, 4, 6, 4, 1]]); }
}
