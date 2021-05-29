#![allow(dead_code)]
/// N-Queens II
/// ===========
///
/// The __n-queens__ puzzle is the problem of placing `n` queens on an `n x n` chessboard
/// such that no two queens attack each other.
///
/// Given an integer `n`, return _the number of distinct solutions to the __n-queens puzzle___.
///
/// __Constraints:__
///
/// - `1 <= n <= 9`
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/602/week-5-may-29th-may-31st/3760/
struct Solution;
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        fn can_place(r: usize, c: usize, b: &mut [Vec<bool>]) -> bool {
            for pr in 0..r {
                let d = r - pr;
                if b[pr][c] || (c >= d && b[pr][c - d]) || (c + d < b.len() && b[pr][c + d]) {
                    return false;
                }
            }
            true
        }
        fn bts(n: usize, board: &mut [Vec<bool>]) -> i32 {
            if n == 0 {
                1
            } else {
                let mut result = 0;
                let r = board.len() - n;
                for c in 0..board.len() {
                    if can_place(r, c, board) {
                        board[r][c] = true;
                        result += bts(n - 1, board);
                        board[r][c] = false;
                    }
                }
                result
            }
        }

        let n = n as usize;
        bts(n, &mut vec![vec![false; n]; n])
    }

    pub fn total_n_queens_hardcoded(n: i32) -> i32 {
        match n {
            1 => 1,
            2 | 3 => 0,
            4 => 2,
            5 => 10,
            6 => 4,
            7 => 40,
            8 => 92,
            9 => 352,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_1_produces_1() {
        assert_eq!(Solution::total_n_queens(1), 1);
    }
    #[test]
    fn n_2_produces_0() {
        assert_eq!(Solution::total_n_queens(2), 0);
    }
    #[test]
    fn n_3_produces_0() {
        assert_eq!(Solution::total_n_queens(3), 0);
    }
    #[test]
    fn n_4_produces_2() {
        assert_eq!(Solution::total_n_queens(4), 2);
        // Explanation: There are two distinct solutions to the 4-queens puzzle as shown.
        // _Q__   __Q_
        // ___Q   Q___
        // Q___   ___Q
        // __Q_   _Q__
    }
    #[test]
    fn n_5_produces_10() {
        assert_eq!(Solution::total_n_queens(5), 10);
    }
    #[test]
    fn n_6_produces_4() {
        assert_eq!(Solution::total_n_queens(6), 4);
    }
    #[test]
    fn n_7_produces_40() {
        assert_eq!(Solution::total_n_queens(7), 40);
    }
    #[test]
    fn n_8_produces_92() {
        assert_eq!(Solution::total_n_queens(8), 92);
    }
    #[test]
    fn n_9_produces_352() {
        assert_eq!(Solution::total_n_queens(9), 352);
    }
}
