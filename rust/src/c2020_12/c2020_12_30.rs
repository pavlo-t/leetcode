#![allow(dead_code)]

/// ### Game of Life
///
/// https://leetcode.com/explore/featured/card/december-leetcoding-challenge/573/week-5-december-29th-december-31st/3586/
struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len();
        let n = board[0].len();

        for r in 0..m {
            for c in 0..n {
                let fr = r.checked_sub(1).unwrap_or_default();
                let tr = (r + 1).min(m - 1);
                let fc = c.checked_sub(1).unwrap_or_default();
                let tc = (c + 1).min(n - 1);

                let mut neighbors = 0;
                for nr in fr..=tr {
                    for nc in fc..=tc {
                        if !(nr == r && nc == c) {
                            neighbors += board[nr][nc] & 1;
                        }
                    }
                }

                board[r][c] = match (board[r][c], neighbors) {
                    (0, 3) => 2,     // 10, 0 -> 1
                    (0, _) => 0,     // 00, 0 -> 0
                    (1, 2..=3) => 3, // 11, 1 -> 1
                    (_, _) => 1,     // 01, 1 -> 0
                }
            }
        }

        for r in 0..m {
            for c in 0..n {
                board[r][c] >>= 1;
            }
        }
    }

    pub fn game_of_life_use_clone(board: &mut Vec<Vec<i32>>) {
        let copy = board.clone();

        let m = board.len();
        let n = board[0].len();

        for r in 0..m {
            for c in 0..n {
                let fr = r.checked_sub(1).unwrap_or_default();
                let tr = (r + 1).min(m - 1);
                let fc = c.checked_sub(1).unwrap_or_default();
                let tc = (c + 1).min(n - 1);

                let mut neighbors = -copy[r][c];
                for nr in fr..=tr {
                    for nc in fc..=tc {
                        neighbors += copy[nr][nc];
                    }
                }

                if neighbors == 3 {
                    board[r][c] = 1;
                } else if neighbors < 2 || 3 < neighbors {
                    board[r][c] = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_b010_001_111_000_becomes_000_101_011_010() {
        let mut board = vec![
            vec![0, 1, 0],
            vec![0, 0, 1],
            vec![1, 1, 1],
            vec![0, 0, 0]];

        Solution::game_of_life(&mut board);

        assert_eq!(board, [
            [0, 0, 0],
            [1, 0, 1],
            [0, 1, 1],
            [0, 1, 0]]);
    }

    #[test]
    fn example2_b11_10_becomes_11_11() {
        let mut board = vec![
            vec![1, 1],
            vec![1, 0]];

        Solution::game_of_life(&mut board);
        assert_eq!(board, [[1, 1], [1, 1]]);
    }

    #[test]
    fn b1_becomes_0() {
        let mut board = vec![vec![1]];
        Solution::game_of_life(&mut board);
        assert_eq!(board, [[0]]);
    }

    #[test]
    fn b0_becomes_0() {
        let mut board = vec![vec![0]];
        Solution::game_of_life(&mut board);
        assert_eq!(board, [[0]]);
    }

    #[test]
    fn b25x25_0s_becomes_25x25_0s() {
        let mut board = vec![vec![0; 25]; 25];
        Solution::game_of_life(&mut board);
        assert_eq!(board, vec![vec![0; 25]; 25]);
    }

    #[test]
    fn b25x25_1s_becomes_25x25_0s_with_1s_in_corners() {
        let side = 25;

        let mut board = vec![vec![1; side]; side];
        Solution::game_of_life(&mut board);

        let mut expected = vec![vec![0; side]; side];
        expected[0][0] = 1;
        expected[0][side - 1] = 1;
        expected[side - 1][0] = 1;
        expected[side - 1][side - 1] = 1;
        assert_eq!(board, expected);
    }

    #[test]
    fn b1000x1000_0s_becomes_all_0s() {
        let side = 1000;
        let mut board = vec![vec![0; side]; side];
        Solution::game_of_life(&mut board);
        assert_eq!(board, vec![vec![0; side]; side]);
    }

    #[test]
    fn b1000x1000_1s_becomes_all_0s_with_1s_in_corners() {
        let side = 1000;

        let mut board = vec![vec![1; side]; side];
        Solution::game_of_life(&mut board);

        let mut expected = vec![vec![0; side]; side];
        expected[0][0] = 1;
        expected[0][side - 1] = 1;
        expected[side - 1][0] = 1;
        expected[side - 1][side - 1] = 1;
        assert_eq!(board, expected);
    }
}
