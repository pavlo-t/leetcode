#![allow(dead_code)]
/// Find Winner on a Tic Tac Toe Game
/// =================================
///
/// Tic-tac-toe is played by two players _A_ and _B_ on a _3 x 3_ grid.
///
/// Here are the rules of Tic-Tac-Toe:
///
/// - Players take turns placing characters into empty squares (" ").
/// - The first player _A_ always places "X" characters, while the second player _B_ always places "O" characters.
/// - "X" and "O" characters are always placed into empty squares, never on filled ones.
/// - The game ends when there are 3 of the same (non-empty) character filling any row, column, or diagonal.
/// - The game also ends if all squares are non-empty.
/// - No more moves can be played if the game is over.
///
/// Given an array `moves` where each element is another array of size 2 corresponding to the row and column
/// of the grid where they mark their respective character in the order in which _A_ and _B_ play.
///
/// Return the winner of the game if it exists (_A_ or _B_),
/// in case the game ends in a draw return "Draw",
/// if there are still movements to play return "Pending".
///
/// You can assume that `moves` is __valid__ (It follows the rules of Tic-Tac-Toe),
/// the grid is initially empty and _A_ will play __first__.
///
/// __Constraints:__
///
/// - `1 <= moves.length <= 9`
/// - `moves[i].length == 2`
/// - `0 <= moves[i][j] <= 2`
/// - There are no repeated elements on `moves`.
/// - `moves` follow the rules of tic tac toe.
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/638/week-3-september-15th-september-21st/3981/
struct Solution;
impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        const S: usize = 3;
        let board = moves.iter().enumerate()
            .map(|(i, v)| (v[0] as usize, v[1] as usize, i % 2))
            .fold([[' '; S]; S], |mut rsf, (r, c, p)| {
                rsf[r][c] = (b'A' + p as u8) as char;
                rsf
            });
        for i in 0..S {
            if board[i][0] != ' ' && board[i].windows(2).all(|v| v[0] == v[1]) {
                return board[i][0].to_string();
            }
            if board[0][i] != ' ' && board.windows(2).all(|v| v[0][i] == v[1][i]) {
                return board[0][i].to_string();
            }
        }
        if board[0][S - 1] != ' ' && board.windows(2).enumerate().all(|(i, v)| v[0][S - i - 1] == v[1][S - i - 2]) {
            board[0][S - 1].to_string()
        } else if board[0][0] != ' ' && board.windows(2).enumerate().all(|(i, v)| v[0][i] == v[1][i + 1]) {
            board[0][0].to_string()
        } else {
            if moves.len() == S * S { "Draw" } else { "Pending" }.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]}}

    #[test]
    fn diagonal_lt_rb_win() {
        let moves = vv![[0, 0], [2, 0], [1, 1], [2, 1], [2, 2]];
        assert_eq!(Solution::tictactoe(moves), "A");
        // Explanation: "A" wins, he always plays first.
        // "X  "    "X  "    "X  "    "X  "    "X  "
        // "   " -> "   " -> " X " -> " X " -> " X "
        // "   "    "O  "    "O  "    "OO "    "OOX"
    }
    #[test]
    fn diagonal_rt_lb_win() {
        let moves = vv![[0, 0], [1, 1], [0, 1], [0, 2], [1, 0], [2, 0]];
        assert_eq!(Solution::tictactoe(moves), "B");
        // Explanation: "B" wins.
        // "X  "    "X  "    "XX "    "XXO"    "XXO"    "XXO"
        // "   " -> " O " -> " O " -> " O " -> "XO " -> "XO "
        // "   "    "   "    "   "    "   "    "   "    "O  "
    }
    #[test]
    fn horizontal_win() {
        let moves = vv![[1, 0], [2, 0], [1, 1], [2, 1], [1, 2]];
        assert_eq!(Solution::tictactoe(moves), "A");
        // "   " -> "   " -> "   " -> "   " -> "   "
        // "X  "    "X  "    "XX "    "XX "    "XXX"
        // "   "    "O  "    "O  "    "OO "    "OO "
    }
    #[test]
    fn vertical_win() {
        let moves = vv![[0, 2], [2, 0], [1, 2], [2, 1], [2, 2]];
        assert_eq!(Solution::tictactoe(moves), "A");
        // "  X" -> "  X" -> "  X" -> "  X" -> "  X"
        // "   "    "   "    "  X"    "  X"    "  X"
        // "   "    "O  "    "O  "    "OO "    "OOX"
    }
    #[test]
    fn draw() {
        let moves = vv![
            [0, 0],
            [1, 1],
            [2, 0],
            [1, 0],
            [1, 2],
            [2, 1],
            [0, 1],
            [0, 2],
            [2, 2]
        ];
        assert_eq!(Solution::tictactoe(moves), "Draw");
        // Explanation: The game ends in a draw since there are no moves to make.
        // "XXO"
        // "OOX"
        // "XOX"
    }
    #[test]
    fn pending() {
        let moves = vv![[0, 0], [1, 1]];
        assert_eq!(Solution::tictactoe(moves), "Pending");
        // Explanation: The game has not finished yet.
        // "X  "
        // " O "
        // "   "
    }
    #[test]
    fn pending_1_move_left() {
        let moves = vv![
            [0, 0],
            [1, 1],
            [2, 0],
            [1, 0],
            [1, 2],
            [2, 1],
            [0, 1],
            [0, 2]
        ];
        assert_eq!(Solution::tictactoe(moves), "Pending");
        // "XXO"
        // "OOX"
        // "XO "
    }
}
