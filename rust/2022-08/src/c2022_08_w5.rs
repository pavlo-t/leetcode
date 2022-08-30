#![allow(dead_code)]
//! \#505. The Maze II
//! ==================
//!
//! <https://leetcode.com/problems/the-maze-ii>
//!
//! There is a ball in a `maze` with empty spaces (represented as `0`) and walls (represented as `1`).
//! The ball can go through the empty spaces by rolling __up__, __down__, __left__ or __right__,
//! but it won't stop rolling until hitting a wall.
//! When the ball stops, it could choose the next direction.
//!
//! Given the `m x n` maze, the ball's `start` position and the `destination`,
//! where `start = [start_row, start_col]` and `destination = [destination_row, destination_col]`,
//! return _the shortest __distance__ for the ball to stop at the destination_.
//! If the ball cannot stop at `destination`, return `-1`.
//!
//! The __distance__ is the number of __empty spaces__ traveled by the ball from the start position (excluded)
//! to the destination (included).
//!
//! You may assume that __the borders of the maze are all walls__.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```text
//!     0 1 2 3 4
//! _ # # # # # # #
//! 0 # . . # 1 S #
//! 1 # 3 . . 2 . #
//! 2 # 4 . 5 # . #
//! 3 # # # . # # #
//! 4 # . . 6 . D #
//! _ # # # # # # #
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_w5::*;
//! # use c2022_08::vv;
//! let maze = vv![
//!     [0, 0, 1, 0, 0],
//!     [0, 0, 0, 0, 0],
//!     [0, 0, 0, 1, 0],
//!     [1, 1, 0, 1, 1],
//!     [0, 0, 0, 0, 0]
//! ];
//! assert_eq!(Solution::shortest_distance(maze, vec![0, 4], vec![4, 4]), 12);
//! ```
//!
//! Explanation: One possible way is : `left -> down -> left -> down -> right -> down -> right`.
//! The length of the path is `1 + 1 + 3 + 1 + 2 + 2 + 2 = 12`.
//!
//! ###### Example 2
//!
//! ```text
//!     0 1 2 3 4
//! _ # # # # # # #
//! 0 # . . # . S #
//! 1 # . . . . . #
//! 2 # . . . # . #
//! 3 # # # D # # #
//! 4 # . . . . . #
//! _ # # # # # # #
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_w5::*;
//! # use c2022_08::vv;
//! let maze = vv![
//!     [0, 0, 1, 0, 0],
//!     [0, 0, 0, 0, 0],
//!     [0, 0, 0, 1, 0],
//!     [1, 1, 0, 1, 1],
//!     [0, 0, 0, 0, 0]
//! ];
//! assert_eq!(Solution::shortest_distance(maze, vec![0, 4], vec![3, 2]), -1);
//! ```
//!
//! Explanation: There is no way for the ball to stop at the destination.
//! Notice that you can pass through the destination but you cannot stop there.
//!
//! ###### Example 3
//!
//! ```text
//!     0 1 2 3 4
//! _ # # # # # # #
//! 0 # . D . . . #
//! 1 # # # . . # #
//! 2 # . . . . . #
//! 3 # . # . . # #
//! 4 # . # . S . #
//! _ # # # # # # #
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_w5::*;
//! # use c2022_08::vv;
//! let maze = vv![
//!     [0, 0, 0, 0, 0],
//!     [1, 1, 0, 0, 1],
//!     [0, 0, 0, 0, 0],
//!     [0, 1, 0, 0, 1],
//!     [0, 1, 0, 0, 0]
//! ];
//! assert_eq!(Solution::shortest_distance(maze, vec![4, 3], vec![0, 1]), -1);
//! ```
//!
//! ##### Constraints
//!
//! - `m == maze.length`
//! - `n == maze[i].length`
//! - `1 <= m, n <= 100`
//! - `maze[i][j]` is `0` or `1`.
//! - `start.length == 2`
//! - `destination.length == 2`
//! - `0 <= start_row, destination_row <= m`
//! - `0 <= start_col, destination_col <= n`
//! - Both the ball and the destination exist in an empty space, and they will not be in the same position initially.
//! - The maze contains __at least 2 empty spaces__.

pub struct Solution;
impl Solution {
    pub fn shortest_distance(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> i32 {
        use std::collections::VecDeque;
        use std::iter::once;

        let (m, n) = (maze.len(), maze[0].len());
        let (destination_row, destination_col) = (destination[0] as usize, destination[1] as usize);
        let (start_row, start_col) = (start[0] as usize, start[1] as usize);

        let has_not_hit_a_wall = |&(r, c): &(usize, usize)| r < m && c < n && maze[r][c] == 0;

        let next = |mut r: usize, mut c: usize, move_fn: fn((usize, usize)) -> (usize, usize)| {
            let mut dist = 0;
            while let Some((nr, nc)) = Some(move_fn((r, c))).filter(has_not_hit_a_wall) {
                r = nr;
                c = nc;
                dist += 1;
            }
            match dist {
                0 => None,
                _ => Some((dist, r, c)),
            }
        };

        let mut result = i32::MAX;

        let mut queue = VecDeque::new();
        queue.push_back((0, start_row, start_col));
        let mut visited = vec![vec![i32::MAX; n]; m];
        visited[start_row][start_col] = 0;

        while let Some((dist, r, c)) = queue.pop_front() {
            if r == destination_row && c == destination_col {
                result = result.min(dist);
            } else {
                once(next(r, c, |(r, c)| (r.wrapping_sub(1), c)))
                    .chain(once(next(r, c, |(r, c)| (r + 1, c))))
                    .chain(once(next(r, c, |(r, c)| (r, c.wrapping_sub(1)))))
                    .chain(once(next(r, c, |(r, c)| (r, c + 1))))
                    .filter_map(|result| result)
                    .for_each(|(n_dist, nr, nc)| {
                        let n_dist = dist + n_dist;
                        if n_dist < visited[nr][nc] {
                            visited[nr][nc] = n_dist;
                            queue.push_back((n_dist, nr, nc));
                        }
                    });
            }
        }

        match result {
            i32::MAX => -1,
            dist => dist,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vv;

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
    fn test_56() {
        let maze = vv![
            [0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0],
            [1, 1, 0, 1, 1],
            [0, 0, 0, 0, 0]
        ];
        let s = vec![1, 1];
        let d = vec![1, 2];
        assert_eq!(Solution::shortest_distance(maze, s, d), 3);
    }

    #[test]
    fn m_100x100x0_1_1_98_98() {
        let maze = vec![vec![0; 100]; 100];
        let start = vec![1, 1];
        let destination = vec![98, 98];
        assert_eq!(Solution::shortest_distance(maze, start, destination), -1);
    }

    #[test]
    fn m_100x100x0_1_1_99_99() {
        let maze = vec![vec![0; 100]; 100];
        let start = vec![1, 1];
        let destination = vec![99, 99];
        assert_eq!(Solution::shortest_distance(maze, start, destination), 196);
    }

    fn read_data(file: &str) -> (Vec<Vec<i32>>, Vec<i32>, Vec<i32>, i32) {
        let contents = std::fs::read_to_string(file).expect("NO FILE");
        let mut chars = contents.chars().skip(2);

        let mut maze = vec![vec![]];
        let mut row = 0;
        while let Some(c) = chars.next() {
            match c {
                '\n' => break,
                ']' => row += 1,
                ',' => (),
                '[' => maze.push(vec![]),
                '0' => maze[row].push(0),
                '1' => maze[row].push(1),
                c => panic!("Unexpected char: {c}"),
            }
        }

        let mut chars = chars.skip(1);
        let mut start = vec![];
        let mut num = 0;
        while let Some(c) = chars.next() {
            match c {
                '\n' => break,
                '0'..='9' => {
                    num *= 10;
                    num += (c as u8 - b'0') as i32;
                }
                ',' | ']' => {
                    start.push(num);
                    num = 0;
                }
                c => panic!("Unexpected char: {c}"),
            }
        }

        let mut chars = chars.skip(1);
        let mut destination = vec![];
        let mut num = 0;
        while let Some(c) = chars.next() {
            match c {
                '\n' => break,
                '0'..='9' => {
                    num *= 10;
                    num += (c as u8 - b'0') as i32;
                }
                ',' | ']' => {
                    destination.push(num);
                    num = 0;
                }
                c => panic!("Unexpected char: {c}"),
            }
        }

        let mut expected = 0;
        while let Some(c) = chars.next() {
            match c {
                '\n' => break,
                '0'..='9' => {
                    expected *= 10;
                    expected += (c as u8 - b'0') as i32;
                }
                c => panic!("Unexpected char: {c}"),
            }
        }

        (maze, start, destination, expected)
    }

    #[test]
    fn test_70() {
        let (maze, start, destination, e) = read_data("src/c2022_08_w5.test_70.txt");
        assert_eq!(Solution::shortest_distance(maze, start, destination), e);
    }

    #[test]
    fn test_74() {
        let (maze, start, destination, e) = read_data("src/c2022_08_w5.test_74.txt");
        assert_eq!(Solution::shortest_distance(maze, start, destination), e);
    }
}
