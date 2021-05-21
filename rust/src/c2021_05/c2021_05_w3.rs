#![allow(dead_code)]
/// Minimum Knight Moves
/// ====================
///
/// In an __infinite__ chess board with coordinates from `-infinity` to `+infinity`,
/// you have a __knight__ at square `[0, 0]`.
///
/// A knight has 8 possible moves it can make.
/// Each move is two squares in a cardinal direction, then one square in an orthogonal direction.
///
/// Return the minimum number of steps needed to move the knight to the square `[x, y]`.
/// It is guaranteed the answer exists.
///
/// __Constraints:__
///
/// - `|x| + |y| <= 300`
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/600/week-3-may-15th-may-21st/3743/
struct Solution;
impl Solution {
    pub fn min_knight_moves(x: i32, y: i32) -> i32 {
        use std::collections::VecDeque;

        const OFFSET: i32 = 302;

        let mut q = VecDeque::new();
        q.push_back((0, 0, 0));
        let mut seen = vec![vec![false; 605]; 605];
        seen[OFFSET as usize][OFFSET as usize] = true;
        while let Some((cx, cy, ms)) = q.pop_front() {
            if cx == x && cy == y {
                return ms;
            } else {
                let nms = ms + 1;
                [
                    (cx + 1, cy + 2),
                    (cx + 1, cy - 2),
                    (cx - 1, cy + 2),
                    (cx - 1, cy - 2),
                    (cx + 2, cy + 1),
                    (cx + 2, cy - 1),
                    (cx - 2, cy + 1),
                    (cx - 2, cy - 1),
                ]
                .iter()
                .for_each(|&(x, y)| {
                    if !seen[(x + OFFSET) as usize][(y + OFFSET) as usize] {
                        seen[(x + OFFSET) as usize][(y + OFFSET) as usize] = true;
                        q.push_back((x, y, nms))
                    }
                })
            }
        }
        unreachable!()
    }

    pub fn min_knight_moves_seen_hash_set(x: i32, y: i32) -> i32 {
        use std::collections::{HashSet, VecDeque};

        let mut q = VecDeque::new();
        q.push_back((0, 0, 0));
        let mut seen = HashSet::new();
        seen.insert((0, 0));
        while let Some((cx, cy, ms)) = q.pop_front() {
            if cx == x && cy == y {
                return ms;
            } else {
                let nms = ms + 1;
                [
                    (cx + 1, cy + 2),
                    (cx + 1, cy - 2),
                    (cx - 1, cy + 2),
                    (cx - 1, cy - 2),
                    (cx + 2, cy + 1),
                    (cx + 2, cy - 1),
                    (cx - 2, cy + 1),
                    (cx - 2, cy - 1),
                ]
                .iter()
                .for_each(|&(x, y)| {
                    if !seen.contains(&(x, y)) {
                        seen.insert((x, y));
                        q.push_back((x, y, nms))
                    }
                })
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_knight_moves(2, 1), 1);
        // Explanation: [0, 0] → [2, 1]
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::min_knight_moves(5, 5), 4);
        // Explanation: [0, 0] → [2, 1] → [4, 2] → [3, 4] → [5, 5]
    }

    #[test]
    fn x0y0_produces_0() {
        assert_eq!(Solution::min_knight_moves(0, 0), 0);
    }
    #[test]
    fn x150y150_produces_100() {
        assert_eq!(Solution::min_knight_moves(150, 150), 100);
    }
}
