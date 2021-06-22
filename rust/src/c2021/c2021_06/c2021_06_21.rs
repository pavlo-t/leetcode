#![allow(dead_code)]
/// Pascal's Triangle
/// =================
///
/// Given an integer `numRows`, return the first numRows of __Pascal's triangle__.
///
/// In __Pascal's triangle__, each number is the sum of the two numbers directly above it as shown:
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
/// https://leetcode.com/explore/challenge/card/june-leetcoding-challenge-2021/605/week-3-june-15th-june-21st/3786/
struct Solution;
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        fn next_row(p_row: &[i32]) -> Vec<i32> {
            let mut r = vec![1; p_row.len() + 1];
            for i in 1..p_row.len() {
                r[i] = p_row[i - 1] + p_row[i];
            }
            r
        }
        let mut r = Vec::with_capacity(num_rows as usize);
        r.push(vec![1]);
        for i in 1..num_rows as usize {
            let p_row = &r[i - 1];
            let new_row = next_row(p_row);
            r.push(new_row)
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn example1() {
        let e = vv![[1], [1, 1], [1, 2, 1], [1, 3, 3, 1], [1, 4, 6, 4, 1]];
        assert_eq!(Solution::generate(5), e);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::generate(1), [[1]]);
    }
}
