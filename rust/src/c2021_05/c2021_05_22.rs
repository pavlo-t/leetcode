#![allow(dead_code)]
/// N-Queens
/// ========
///
/// The __n-queens__ puzzle is the problem of placing `n` queens on an `n x n` chessboard
/// such that no two queens attack each other.
///
/// Given an integer `n`, return _all distinct solutions to the __n-queens puzzle___.
///
/// Each solution contains a distinct board configuration of the n-queens' placement,
/// where `'Q'` and `'.'` both indicate a queen and an empty space, respectively.
///
/// __Constraints:__
///
/// - `1 <= n <= 9`
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/601/week-4-may-22nd-may-28th/3752/
struct Solution;
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        fn can_place(r: usize, c: usize, b: &mut [Vec<bool>]) -> bool {
            let n = b.len();
            for cr in 0..r {
                let d = r - cr;
                if b[cr][c] || (d <= c && b[cr][c - d]) || (c + d < n && b[cr][c + d]) {
                    return false;
                }
            }
            true
        }
        fn to_solution(b: &[Vec<bool>]) -> Vec<String> {
            b.iter()
                .map(|r| {
                    String::from_utf8(r.iter().map(|&p| if p { b'Q' } else { b'.' }).collect())
                        .unwrap()
                })
                .collect()
        }
        fn bts(n: usize, board: &mut [Vec<bool>], result: &mut Vec<Vec<String>>) {
            if n == 0 {
                result.push(to_solution(board));
            } else {
                let r = board.len() - n;
                for c in 0..board.len() {
                    if can_place(r, c, board) {
                        board[r][c] = true;
                        bts(n - 1, board, result);
                        board[r][c] = false;
                    }
                }
            }
        }

        let n = n as usize;
        let mut result = vec![];
        bts(n, &mut vec![vec![false; n]; n], &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_1() {
        assert_eq!(Solution::solve_n_queens(1), [["Q"]]);
    }
    #[test]
    fn n_2() {
        let e: Vec<Vec<String>> = vec![];
        assert_eq!(Solution::solve_n_queens(2), e);
    }
    #[test]
    fn n_3() {
        let e: Vec<Vec<String>> = vec![];
        assert_eq!(Solution::solve_n_queens(3), e);
    }
    #[test]
    fn n_4() {
        assert_eq!(
            Solution::solve_n_queens(4),
            [
                [".Q..", "...Q", "Q...", "..Q."],
                ["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );
        // .Q..   ..Q.
        // ...Q   Q...
        // Q...   ...Q
        // ..Q.   .Q..
        // Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above
    }
    #[test]
    fn n_5() {
        assert_eq!(Solution::solve_n_queens(5).len(), 10);
    }
    #[test]
    fn n_6() {
        assert_eq!(Solution::solve_n_queens(6).len(), 4);
    }
    #[test]
    fn n_7() {
        assert_eq!(Solution::solve_n_queens(7).len(), 40);
    }
    #[test]
    fn n_8() {
        assert_eq!(Solution::solve_n_queens(8).len(), 92);
    }
    #[test]
    fn n_9() {
        assert_eq!(Solution::solve_n_queens(9).len(), 352);
    }
}
