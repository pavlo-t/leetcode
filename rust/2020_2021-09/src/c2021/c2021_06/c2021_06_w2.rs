#![allow(dead_code)]
/// The Maze II
/// ===========
///
/// There is a ball in a `maze` with empty spaces (represented as `0`) and walls (represented as `1`).
/// The ball can go through the empty spaces by rolling __up__, __down__, __left__ or __right__,
/// but it won't stop rolling until hitting a wall.
/// When the ball stops, it could choose the next direction.
///
/// Given the `m x n maze`, the ball's `start` position and the `destination`,
/// where `start = [start_row, start_col]` and `destination = [destination_row, destination_col]`,
/// return _the shortest __distance__ for the ball to stop at the destination_.
/// If the ball cannot stop at `destination`, return `-1`.
///
/// The __distance__ is the number of __empty spaces__ traveled by the ball
/// from the start position (excluded) to the destination (included).
///
/// You may assume that __the borders of the maze are all walls__.
///
/// __Constraints:__
///
/// - `m == maze.length`
/// - `n == maze[i].length`
/// - `1 <= m, n <= 100`
/// - `maze[i][j]` is `0` or `1`.
/// - `start.length == 2`
/// - `destination.length == 2`
/// - `0 <= start_row, destination_row <= m`
/// - `0 <= start_col, destination_col <= n
/// - Both the ball and the destination exist in an empty space, and they will not be in the same position initially.
/// - The maze contains __at least 2 empty spaces__.
///
/// https://leetcode.com/explore/featured/card/june-leetcoding-challenge-2021/604/week-2-june-8th-june-14th/3771/
struct Solution;
impl Solution {
    pub fn shortest_distance(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> i32 {
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        enum Direction {
            L,
            R,
            U,
            D,
        }
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        struct Ball {
            r: usize,
            c: usize,
            d: Direction,
        }
        impl Ball {
            fn new(r: usize, c: usize, d: Direction) -> Self {
                Self { r, c, d }
            }
        }

        fn can_move(r: usize, c: usize, d: Direction, maze: &[Vec<i32>]) -> bool {
            let m = maze.len();
            let n = maze[0].len();

            match d {
                L => c > 0 && maze[r][c - 1] == 0,
                R => c + 1 < n && maze[r][c + 1] == 0,
                U => r > 0 && maze[r - 1][c] == 0,
                D => r + 1 < m && maze[r + 1][c] == 0,
            }
        }

        fn change_direction(r: usize, c: usize, maze: &[Vec<i32>]) -> Vec<Ball> {
            let mut result = vec![];
            if can_move(r, c, L, maze) {
                result.push(Ball::new(r, c - 1, L));
            }
            if can_move(r, c, R, maze) {
                result.push(Ball::new(r, c + 1, R));
            }
            if can_move(r, c, U, maze) {
                result.push(Ball::new(r - 1, c, U));
            }
            if can_move(r, c, D, maze) {
                result.push(Ball::new(r + 1, c, D));
            }
            result
        }

        fn next(mut ball: Ball, maze: &[Vec<i32>]) -> Vec<Ball> {
            if can_move(ball.r, ball.c, ball.d, maze) {
                match ball.d {
                    L => ball.c -= 1,
                    R => ball.c += 1,
                    U => ball.r -= 1,
                    D => ball.r += 1,
                }
                vec![ball]
            } else {
                change_direction(ball.r, ball.c, maze)
            }
        }

        use std::collections::HashSet;
        use std::mem::swap;
        use Direction::*;

        let (dest_r, dest_c) = (destination[0] as usize, destination[1] as usize);
        let mut q1 = change_direction(start[0] as usize, start[1] as usize, &maze);
        let mut q2 = Vec::new();
        let mut seen = HashSet::new();

        let mut moves = 1;

        while !q1.is_empty() {
            while let Some(b) = q1.pop() {
                let Ball { r, c, d: prev_d } = b;
                for b in next(b, &maze) {
                    if b.d != prev_d && r == dest_r && c == dest_c {
                        return moves;
                    }
                    if !seen.contains(&b) {
                        seen.insert(b);
                        q2.push(b);
                    }
                }
            }
            swap(&mut q1, &mut q2);
            moves += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {
        ($x:tt;$n:tt) => {vec![vec!$x;n]};
        ($($x:tt),*) => {vec![$(vec!$x),*]};
    }

    #[test]
    fn example1() {
        let maze = vv![
            [0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0],
            [1, 1, 0, 1, 1],
            [0, 0, 0, 0, 0]
        ];
        let start = vec![0, 4];
        let destination = vec![4, 4];
        assert_eq!(Solution::shortest_distance(maze, start, destination), 12);
        // Explanation:
        // One possible way is : left -> down -> left -> down -> right -> down -> right.
        // The length of the path is 1 + 1 + 3 + 1 + 2 + 2 + 2 = 12.
    }
    #[test]
    fn example2() {
        let maze = vv![
            [0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0],
            [1, 1, 0, 1, 1],
            [0, 0, 0, 0, 0]
        ];
        let start = vec![0, 4];
        let destination = vec![3, 2];
        assert_eq!(Solution::shortest_distance(maze, start, destination), -1);
        // Explanation:
        // There is no way for the ball to stop at the destination.
        // Notice that you can pass through the destination but you cannot stop there.
    }
    #[test]
    fn example3() {
        let maze = vv![
            [0, 0, 0, 0, 0],
            [1, 1, 0, 0, 1],
            [0, 0, 0, 0, 0],
            [0, 1, 0, 0, 1],
            [0, 1, 0, 0, 0]
        ];
        let start = vec![4, 3];
        let destination = vec![0, 1];
        assert_eq!(Solution::shortest_distance(maze, start, destination), -1);
    }

    #[test]
    fn one_move() {
        let maze = vv![[0, 0]];
        let start = vec![0, 0];
        let destination = vec![0, 1];
        assert_eq!(Solution::shortest_distance(maze, start, destination), 1);
    }
    #[test]
    fn no_moves() {
        let maze = vv![[0, 1, 0]];
        let start = vec![0, 0];
        let destination = vec![0, 2];
        assert_eq!(Solution::shortest_distance(maze, start, destination), -1);
    }
    #[test]
    fn need_more_moves_to_stop() {
        let maze = vv![
            [0, 0, 0, 0, 0],
            [0, 0, 1, 0, 1],
            [1, 1, 1, 0, 0],
            [0, 0, 0, 0, 1],
            [0, 1, 0, 0, 0]
        ];
        let start = vec![3, 0];
        let destination = vec![0, 1];
        assert_eq!(Solution::shortest_distance(maze, start, destination), 12);
    }
    #[test]
    fn maze_100x100_all_empty() {
        let maze = vec![vec![0; 100]; 100];
        let start = vec![0, 0];
        let destination = vec![99, 99];
        assert_eq!(Solution::shortest_distance(maze, start, destination), 198);
    }
}
