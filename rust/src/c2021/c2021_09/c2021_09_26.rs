#![allow(dead_code)]
/// Transform to Chessboard
/// =======================
///
/// You are given an `n x n` binary grid `board`.
/// In each move, you can swap any two rows with each other, or any two columns with each other.
///
/// Return _the minimum number of moves to transform the board into a __chessboard board___.
/// If the task is impossible, return `-1`.
///
/// A __chessboard board__ is a board where no `0`'s and no `1`'s are 4-directionally adjacent.
///
/// __Constraints:__
///
/// - `n == board.length`
/// - `n == board[i].length`
/// - `2 <= n <= 30`
/// - `board[i][j]` is either `0` or `1`.
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/639/week-4-september-22nd-september-28th/3988/
struct Solution;
impl Solution {
    /// https://massivealgorithms.blogspot.com/2019/03/leetcode-782-transform-to-chessboard.html
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        println!("moves_to_chessboard(");
        board.iter().for_each(|r| println!("{:?}", r));

        let n = board.len();
        for r in 1..n {
            for c in 1..n {
                if ((board[0][0] ^ board[r][0]) ^ (board[0][c] ^ board[r][c])) == 1 {
                    return -1;
                }
            }
        }

        let (mut r1s, mut c1s) = (0, 0);
        for i in 0..n {
            r1s += board[0][i] as usize;
            c1s += board[i][0] as usize;
        }
        if r1s < n / 2 || r1s > (n + 1) / 2 || c1s < n / 2 || c1s > (n + 1) / 2 {
            return -1;
        }

        let (mut rmv, mut cmv) = (0, 0);
        for i in 0..n {
            let snb = (i % 2) as i32;
            if board[i][0] == snb {
                rmv += 1;
            }
            if board[0][i] == snb {
                cmv += 1;
            }
        }
        if n % 2 == 1 {
            if rmv % 2 == 1 {
                rmv = n - rmv;
            }
            if cmv % 2 == 1 {
                cmv = n - cmv;
            }
        } else {
            rmv = rmv.min(n - rmv);
            cmv = cmv.min(n - cmv);
        }
        ((rmv + cmv) / 2) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]}}

    #[test]
    fn b_0110_0110_1001_1001() {
        #[rustfmt::skip]
        let b = vv![[0, 1, 1, 0],
                    [0, 1, 1, 0],
                    [1, 0, 0, 1],
                    [1, 0, 0, 1]];
        assert_eq!(Solution::moves_to_chessboard(b), 2);
        // Explanation: One potential sequence of moves is shown.
        // 0110    1010    1010
        // 0110 => 1010 => 0101
        // 1001    0101    1010
        // 1001    0101    0101
        // The first move swaps the first and second column.
        // The second move swaps the second and third row.
    }
    #[test]
    fn b_01_10() {
        #[rustfmt::skip]
        let b = vv![[0, 1],
                    [1, 0]];
        assert_eq!(Solution::moves_to_chessboard(b), 0);
        // Explanation: Also note that the board with 0 in the top left corner, is also a valid chessboard.
    }
    #[test]
    fn b_10_10() {
        #[rustfmt::skip]
        let b = vv![[1, 0],
                    [1, 0]];
        assert_eq!(Solution::moves_to_chessboard(b), -1);
        // Explanation: No matter what sequence of moves you make, you cannot end with a valid chessboard.
    }
    #[test]
    fn b_011_011_100() {
        #[rustfmt::skip]
        let b = vv![[0, 1, 1],
                    [0, 1, 1],
                    [1, 0, 0]];
        assert_eq!(Solution::moves_to_chessboard(b), 2);
    }
    #[test]
    fn b_100_100_011() {
        #[rustfmt::skip]
        let b = vv![[1, 0, 0],
                    [1, 0, 0],
                    [0, 1, 1]];
        assert_eq!(Solution::moves_to_chessboard(b), 2);
    }
    #[test]
    fn b_011_001_110() {
        #[rustfmt::skip]
        let b = vv![[0, 1, 1],
                    [0, 0, 1],
                    [1, 1, 0]];
        assert_eq!(Solution::moves_to_chessboard(b), -1);
    }
}
