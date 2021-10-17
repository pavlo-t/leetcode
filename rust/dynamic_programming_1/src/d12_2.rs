#![allow(dead_code)]
/// 119. Pascal's Triangle II
/// =========================
///
/// Given an integer `rowIndex`, return the `rowIndexth` (__0-indexed__) row of the __Pascal's triangle__.
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
/// - `1 <= numRows <= 33`
///
/// https://leetcode.com/problems/pascals-triangle/
struct Solution;
impl Solution {
    /// 03:26-03:41
    pub fn get_row(row_index: i32) -> Vec<i32> {
        println!("get_row({})", row_index);
        let n = row_index as usize;
        let mut result = vec![1; n + 1];
        for r in 1..=n {
            for i in (1..r).rev() {
                result[i] += result[i - 1];
            }
        }
        result
    }
    /// 03:32-03:39
    pub fn get_row_rec(row_index: i32) -> Vec<i32> {
        println!("get_row({})", row_index);
        fn rec(r: usize) -> Vec<i32> {
            if r == 0 {
                vec![1]
            } else {
                let mut result = rec(r - 1);
                result.push(1);
                for i in (1..r).rev() {
                    result[i] += result[i - 1];
                }
                result
            }
        }
        rec(row_index as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r_0() {
        let e = vec![1];
        assert_eq!(Solution::get_row(0), e);
    }
    #[test]
    fn r_1() {
        let e = vec![1, 1];
        assert_eq!(Solution::get_row(1), e);
    }
    #[test]
    fn r_3() {
        let e = vec![1, 3, 3, 1];
        assert_eq!(Solution::get_row(3), e);
    }
    #[test]
    fn r_4() {
        let e = vec![1, 4, 6, 4, 1];
        assert_eq!(Solution::get_row(4), e);
    }

    #[test]
    fn r_33() {
        assert_eq!(Solution::get_row(33).len(), 34);
    }
}
