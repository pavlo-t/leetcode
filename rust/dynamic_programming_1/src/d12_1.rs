#![allow(dead_code)]
/// 118. Pascal's Triangle
/// ======================
///
/// Given an integer numRows, return the first numRows of Pascal's triangle.
///
/// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
///
/// ```text
///     1
///    1 1
///   1 2 1
///  1 3 3 1
/// 1 4 6 4 1
/// ```
///
/// __Constraints:__
///
/// - `1 <= numRows <= 30`
///
/// https://leetcode.com/problems/pascals-triangle/
struct Solution;
impl Solution {
    /// 02:27-02:32
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        println!("generate({})", num_rows);
        let n = num_rows as usize;
        let mut result = Vec::with_capacity(n);
        result.push(vec![1]);
        for r in 1..n {
            let mut row = vec![1; r + 1];
            for i in 1..r {
                row[i] = result[r - 1][i - 1] + result[r - 1][i];
            }
            result.push(row);
        }
        result
    }
    /// 02:20-02:26
    pub fn generate_rec(num_rows: i32) -> Vec<Vec<i32>> {
        println!("generate({})", num_rows);
        fn rec(n: usize) -> Vec<Vec<i32>> {
            if n == 1 {
                vec![vec![1]]
            } else {
                let mut result = rec(n - 1);
                let mut row = vec![1; n];
                for i in 1..n - 1 {
                    row[i] = result[n - 2][i - 1] + result[n - 2][i];
                }
                result.push(row);
                result
            }
        }
        rec(num_rows as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn n_5() {
        let e = vv![[1], [1, 1], [1, 2, 1], [1, 3, 3, 1], [1, 4, 6, 4, 1]];
        assert_eq!(Solution::generate(5), e);
    }
    #[test]
    fn n_1() {
        let e = vv![[1]];
        assert_eq!(Solution::generate(1), e);
    }

    #[test]
    fn n_30() {
        assert_eq!(Solution::generate(30).len(), 30);
    }
}
