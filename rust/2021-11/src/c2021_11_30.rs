#![allow(dead_code)]
/// 85. Maximal Rectangle
/// =====================
///
/// Given a `rows x cols` binary matrix filled with `0`'s and `1`'s,
/// find the largest rectangle containing only `1`'s and return _its area_.
///
/// __Constraints:__
///
/// - `rows == matrix.length`
/// - `cols == matrix[i].length`
/// - `0 <= row, cols <= 200`
/// - `matrix[i][j]` is `'0'` or `'1'`.
///
/// https://leetcode.com/problems/maximal-rectangle/
struct Solution;
impl Solution {
    /// https://www.geeksforgeeks.org/maximum-size-rectangle-binary-sub-matrix-1s/
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        println!("maximal_rectangle({:?})", matrix);
        if matrix.is_empty() {
            0
        } else {
            let (rs, cs) = (matrix.len(), matrix[0].len());
            let mut dp: Vec<i32> = vec![0; cs];
            let mut result = 0;
            let mut curr = 0;
            for c in 0..cs {
                if matrix[0][c] == '0' {
                    dp[c] = 0;
                    curr = 0;
                } else {
                    dp[c] = 1;
                    curr += 1;
                    result = result.max(curr);
                }
            }
            for r in 1..rs {
                let mut stack: Vec<usize> = vec![];
                for c in 0..cs {
                    dp[c] = if matrix[r][c] == '0' { 0 } else { dp[c] + 1 };
                    while let Some(height) = stack.last().map(|&i| dp[i]) {
                        if height <= dp[c] {
                            break;
                        } else {
                            stack.pop();
                            result = result
                                .max(height * stack.last().map(|&p| c - p - 1).unwrap_or(c) as i32);
                        }
                    }
                    stack.push(c);
                }
                while let Some(height) = stack.pop().map(|i| dp[i]) {
                    result =
                        result.max(height * stack.last().map(|&i| cs - i - 1).unwrap_or(cs) as i32);
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
    fn m_empty() {
        assert_eq!(Solution::maximal_rectangle(vec![]), 0);
    }
    #[test]
    fn m_0() {
        assert_eq!(Solution::maximal_rectangle(vv![['0']]), 0);
    }
    #[test]
    fn m_1() {
        assert_eq!(Solution::maximal_rectangle(vv![['1']]), 1);
    }
    #[test]
    fn m_00() {
        assert_eq!(Solution::maximal_rectangle(vv![['0', '0']]), 0);
    }
    #[test]
    fn m_0_0() {
        assert_eq!(Solution::maximal_rectangle(vv![['0'], ['0']]), 0);
    }
    #[test]
    fn m_11() {
        assert_eq!(Solution::maximal_rectangle(vv![['1', '1']]), 2);
    }
    #[test]
    fn m_1_1() {
        assert_eq!(Solution::maximal_rectangle(vv![['1'], ['1']]), 2);
    }
    #[test]
    fn m_10100_10111_11111_10010() {
        let m = vv![
            ['1', '0', '1', '0', '0'],
            ['1', '0', '1', '1', '1'],
            ['1', '1', '1', '1', '1'],
            ['1', '0', '0', '1', '0']
        ];
        assert_eq!(Solution::maximal_rectangle(m), 6);
        // Explanation: The maximal rectangle's left top is [1,2] and bottom right is [2,4]
    }
}
