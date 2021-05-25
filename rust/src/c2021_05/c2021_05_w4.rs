#![allow(dead_code)]

use std::cell::RefCell;

/// Design Tic-Tac-Toe
/// ==================
///
/// Assume the following rules are for the tic-tac-toe game on an `n x n` board between two players:
///
/// 1. A move is guaranteed to be valid and is placed on an empty block.
/// 2. Once a winning condition is reached, no more moves are allowed.
/// 3. A player who succeeds in placing `n` of their marks in a horizontal, vertical, or diagonal row wins the game.
///
/// Implement the `TicTacToe` class:
///
/// - `TicTacToe(int n)`
///   Initializes the object the size of the board `n`.
/// - `int move(int row, int col, int player)`
///   Indicates that player with id `player` plays at the cell `(row, col)` of the board.
///   The move is guaranteed to be a valid move.
///
/// __Follow up:__
///
/// - Could you do better than `O(n^2)` per `move()` operation?
///
/// __Constraints:__
///
/// - `2 <= n <= 100`
/// - player is `1` or `2`.
/// - `0 <= row, col < n`
/// - `(row, col)` are __unique__ for each different call to `move`.
/// - At most `n^2` calls will be made to `move`.
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/601/week-4-may-22nd-may-28th/3751/
struct TicTacToe {
    n: usize,
    verticals: RefCell<Vec<Vec<usize>>>,
    horizontals: RefCell<Vec<Vec<usize>>>,
    main_diagonals: RefCell<Vec<usize>>,
    anti_diagonals: RefCell<Vec<usize>>,
}
impl TicTacToe {
    fn new(n: i32) -> Self {
        let n = n as usize;
        let players = 2;
        Self {
            n,
            verticals: RefCell::new(vec![vec![0; n]; players]),
            horizontals: RefCell::new(vec![vec![0; n]; players]),
            main_diagonals: RefCell::new(vec![0; players]),
            anti_diagonals: RefCell::new(vec![0; players]),
        }
    }

    fn make_a_move(&self, row: i32, col: i32, player: i32) -> i32 {
        let (r, c, p) = (row as usize, col as usize, player as usize - 1);
        let vs = &mut self.verticals.borrow_mut()[p];
        let hs = &mut self.horizontals.borrow_mut()[p];
        vs[c] += 1;
        hs[r] += 1;
        if vs[c] == self.n || hs[r] == self.n {
            return player;
        }

        if r == c {
            let mut ds = self.main_diagonals.borrow_mut();
            ds[p] += 1;
            if ds[p] == self.n {
                return player;
            }
        }

        if r == self.n - c - 1 {
            let mut ds = self.anti_diagonals.borrow_mut();
            ds[p] += 1;
            if ds[p] == self.n {
                return player;
            }
        }

        0
    }
}

/// O(N) time; O(N^2) space
struct TicTacToeV1 {
    board: RefCell<Vec<Vec<i32>>>,
}
impl TicTacToeV1 {
    fn new(n: i32) -> Self {
        let n = n as usize;
        let board = RefCell::new(vec![vec![0; n]; n]);
        Self { board }
    }

    /** Player {player} makes a move at ({row}, {col}).

    - @param row The row of the board.
    - @param col The column of the board.
    - @param player The player, can be either 1 or 2.

    @return The current winning condition, can be either:
            - 0: No one wins.
            - 1: Player 1 wins.
            - 2: Player 2 wins. */
    fn make_a_move(&self, row: i32, col: i32, p: i32) -> i32 {
        let (r, c) = (row as usize, col as usize);
        let mut b = self.board.borrow_mut();
        b[r][c] = p;
        let n = b.len();

        if b[r].iter().all(|v| v == &p) {
            return p;
        }
        let mut col_victory = true;
        let mut diag_victory_1 = r == c;
        let mut diag_victory_2 = r == n - c - 1;

        for (r, row) in b.iter().enumerate() {
            if col_victory && row[c] != p {
                col_victory = false;
            }
            if diag_victory_1 && row[r] != p {
                diag_victory_1 = false;
            }
            if diag_victory_2 && row[n - r - 1] != p {
                diag_victory_2 = false;
            }

            if !col_victory && !diag_victory_1 && !diag_victory_2 {
                break;
            }
        }

        if col_victory || diag_victory_1 || diag_victory_2 {
            return p;
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let ttt = TicTacToe::new(3);
        assert_eq!(ttt.make_a_move(0, 0, 1), 0);
        assert_eq!(ttt.make_a_move(0, 2, 2), 0);
        assert_eq!(ttt.make_a_move(2, 2, 1), 0);
        assert_eq!(ttt.make_a_move(1, 1, 2), 0);
        assert_eq!(ttt.make_a_move(2, 0, 1), 0);
        assert_eq!(ttt.make_a_move(1, 0, 2), 0);
        assert_eq!(ttt.make_a_move(2, 1, 1), 1);
        // Explanation
        // TicTacToe ticTacToe = new TicTacToe(3);
        // Assume that player 1 is "X" and player 2 is "O" in the board.
        // ticTacToe.move(0, 0, 1); // return 0 (no one wins)
        // |X| | |
        // | | | |    // Player 1 makes a move at (0, 0).
        // | | | |
        //
        // ticTacToe.move(0, 2, 2); // return 0 (no one wins)
        // |X| |O|
        // | | | |    // Player 2 makes a move at (0, 2).
        // | | | |
        //
        // ticTacToe.move(2, 2, 1); // return 0 (no one wins)
        // |X| |O|
        // | | | |    // Player 1 makes a move at (2, 2).
        // | | |X|
        //
        // ticTacToe.move(1, 1, 2); // return 0 (no one wins)
        // |X| |O|
        // | |O| |    // Player 2 makes a move at (1, 1).
        // | | |X|
        //
        // ticTacToe.move(2, 0, 1); // return 0 (no one wins)
        // |X| |O|
        // | |O| |    // Player 1 makes a move at (2, 0).
        // |X| |X|
        //
        // ticTacToe.move(1, 0, 2); // return 0 (no one wins)
        // |X| |O|
        // |O|O| |    // Player 2 makes a move at (1, 0).
        // |X| |X|
        //
        // ticTacToe.move(2, 1, 1); // return 1 (player 1 wins)
        // |X| |O|
        // |O|O| |    // Player 1 makes a move at (2, 1).
        // |X|X|X|
    }

    #[test]
    fn test_diagonal_3x3() {
        let ttt = TicTacToe::new(3);
        assert_eq!(ttt.make_a_move(0, 0, 1), 0);
        assert_eq!(ttt.make_a_move(0, 2, 2), 0);
        assert_eq!(ttt.make_a_move(2, 2, 1), 0);
        assert_eq!(ttt.make_a_move(2, 0, 2), 0);
        assert_eq!(ttt.make_a_move(1, 1, 1), 1);
        // |X| | | => |X| |O| => |X| |O| => |X| |O| => |X| |O|
        // | | | | => | | | | => | | | | => | | | | => | |X| |
        // | | | | => | | | | => | | |X| => |O| |X| => | | |X|
    }
    #[test]
    fn test_horizontal_3x3() {
        let ttt = TicTacToe::new(3);
        assert_eq!(ttt.make_a_move(0, 0, 1), 0);
        assert_eq!(ttt.make_a_move(0, 1, 1), 0);
        assert_eq!(ttt.make_a_move(0, 2, 1), 1);
        // |X|X|X|
        // | | | |
        // | | | |
    }
    #[test]
    fn test_vertical_3x3() {
        let ttt = TicTacToe::new(3);
        assert_eq!(ttt.make_a_move(0, 0, 1), 0);
        assert_eq!(ttt.make_a_move(1, 0, 1), 0);
        assert_eq!(ttt.make_a_move(2, 0, 1), 1);
        // |X| | |
        // |X| | |
        // |X| | |
    }
    #[test]
    fn test_vertical_3x3_player_2_wins() {
        let ttt = TicTacToe::new(3);
        assert_eq!(ttt.make_a_move(0, 0, 2), 0);
        assert_eq!(ttt.make_a_move(1, 0, 2), 0);
        assert_eq!(ttt.make_a_move(2, 0, 2), 2);
        // |X| | |
        // |X| | |
        // |X| | |
    }
    #[test]
    fn test_diagonal_4x4() {
        let ttt = TicTacToe::new(4);
        assert_eq!(ttt.make_a_move(0, 1, 1), 0);
        assert_eq!(ttt.make_a_move(1, 2, 1), 0);
        assert_eq!(ttt.make_a_move(2, 3, 1), 0);
        assert_eq!(ttt.make_a_move(3, 0, 1), 0);
        // | |X| | |
        // | | |X| |
        // | | | |X|
        // |X| | | |
        assert_eq!(ttt.make_a_move(2, 1, 1), 0);
        assert_eq!(ttt.make_a_move(0, 3, 1), 1);
        // | |X| |X|
        // | | |X| |
        // | |X| |X|
        // |X| | | |
    }
}
