#![allow(dead_code)]
/// \#1197. Minimum Knight Moves
/// ============================
///
/// In an __infinite__ chess board with coordinates from `-infinity` to `+infinity`,
/// you have a __knight__ at square `[0, 0]`.
///
/// A knight has 8 possible moves it can make, as illustrated below.
/// Each move is two squares in a cardinal direction, then one square in an orthogonal direction.
///
/// Return _the minimum number of steps needed to move the knight to the square `[x, y]`_.
/// It is guaranteed the answer exists.
///
/// __Constraints:__
///
/// - `-300 <= x, y <= 300`
/// - `0 <= |x| + |y| <= 300`
///
/// https://leetcode.com/problems/minimum-knight-moves
struct Solution;
impl Solution {
    /// adopted after seeing previous solution `rust/2020_2021-09/src/c2021/c2021_05/c2021_05_w3.rs`
    pub fn min_knight_moves(x: i32, y: i32) -> i32 {
        const MOVES: [(i32, i32); 8] = [
            (-1, 2),
            (1, 2),
            (2, 1),
            (2, -1),
            (1, -2),
            (-1, -2),
            (-2, -1),
            (-2, 1),
        ];
        const OFFSET: i32 = 302;
        const N: usize = (OFFSET * 2 + 1) as usize;

        let mut visited = vec![vec![false; N]; N];
        visited[OFFSET as usize][OFFSET as usize] = true;
        let mut steps = 0;
        let mut curr = vec![(0, 0)];
        let mut next = vec![];
        while !curr.is_empty() {
            while let Some((cx, cy)) = curr.pop() {
                if cx == x && cy == y {
                    return steps;
                }
                for (x_move, y_move) in MOVES {
                    let n @ (nx, ny) = (cx + x_move, cy + y_move);
                    let (nxo, nyo) = ((nx + OFFSET) as usize, (ny + OFFSET) as usize);
                    if nxo < N && nyo < N && !visited[nxo][nyo] {
                        visited[nxo][nyo] = true;
                        next.push(n);
                    }
                }
            }
            steps += 1;
            std::mem::swap(&mut curr, &mut next);
        }
        unreachable!()
    }

    /// 18:25‥18:48
    pub fn min_knight_moves_hash_set(x: i32, y: i32) -> i32 {
        use std::collections::HashSet;

        const MOVES: [(i32, i32); 8] = [
            (-1, 2),
            (1, 2),
            (2, 1),
            (2, -1),
            (1, -2),
            (-1, -2),
            (-2, -1),
            (-2, 1),
        ];

        let mut visited = HashSet::new();
        visited.insert((0, 0));
        let mut steps = 0;
        let mut curr = vec![(0, 0)];
        let mut next = vec![];
        while !curr.is_empty() {
            while let Some((cx, cy)) = curr.pop() {
                if cx == x && cy == y {
                    return steps;
                }
                for (x_move, y_move) in MOVES {
                    let n = (cx + x_move, cy + y_move);
                    if !visited.contains(&n) {
                        next.push(n);
                        visited.insert(n);
                    }
                }
            }
            steps += 1;
            std::mem::swap(&mut curr, &mut next);
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_0_y_0() {
        assert_eq!(Solution::min_knight_moves(0, 0), 0);
    }
    #[test]
    fn x_2_y_1() {
        assert_eq!(Solution::min_knight_moves(2, 1), 1);
        // Explanation: [0, 0] → [2, 1]
    }
    #[test]
    fn x_5_y_5() {
        assert_eq!(Solution::min_knight_moves(5, 5), 4);
        // Explanation: [0, 0] → [2, 1] → [4, 2] → [3, 4] → [5, 5]
    }

    #[test]
    fn x_10_y_10() {
        assert_eq!(Solution::min_knight_moves(10, 10), 8);
    }
    //#[ignore]
    #[test]
    fn x_30_y_30() {
        assert_eq!(Solution::min_knight_moves(30, 30), 20);
    }
    //#[ignore]
    #[test]
    fn x_300_y_300() {
        assert_eq!(Solution::min_knight_moves(300, 300), 200);
    }
}
