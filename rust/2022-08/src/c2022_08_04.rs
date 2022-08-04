#![allow(dead_code)]
//! \#858. Mirror Reflection
//! ========================
//!
//! <https://leetcode.com/problems/mirror-reflection>
//!
//! There is a special square room with mirrors on each of the four walls.
//! Except for the southwest corner, there are receptors on each of the remaining corners,
//! numbered `0`, `1`, and `2`.
//!
//! The square room has walls of length `p` and a laser ray from the southwest corner
//! first meets the east wall at a distance `q` from the `0`th receptor.
//!
//! Given the two integers `p` and `q`, return _the number of the receptor that the ray meets first_.
//!
//! The test cases are guaranteed so that the ray will meet a receptor eventually.
//!
//!
//! ##### Constraints
//!
//! - `1 <= q <= p <= 1000`
//!
//! ##### Examples
//!
//! ```text
//!  |<-     p     ->|
//! -2---------------1
//! ^| '-.           |
//! ||     '-.       |
//!  |         '-.   |
//! p|              >|-
//!  |         .-'   |^
//! ||     .-'       |q
//! v| .-'           |v
//! -Source----------0-
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_04::Solution;
//! assert_eq!(Solution::mirror_reflection(2, 1), 2);
//! ```
//!
//! Explanation: The ray meets receptor `2` the first time it gets reflected back to the left wall.
//!
//! ```
//! # use c2022_08::c2022_08_04::Solution;
//! assert_eq!(Solution::mirror_reflection(3, 1), 1);
//! ```

pub struct Solution;
impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        enum Side {
            Left,
            Right,
        }
        enum Direction {
            Up,
            Down,
        }
        use Direction::*;
        use Side::*;

        let mut dist = q;
        let mut side = Right;
        let mut direction = Up;

        loop {
            match side {
                Right if dist == 0 => return 0,
                Right if dist == p => return 1,
                Left if dist == p => return 2,
                Right => side = Left,
                Left => side = Right,
            }

            let (new_dist, new_direction) = match direction {
                Up if dist + q > p => (p - (dist + q - p), Down),
                Up => (dist + q, Up),
                Down if q > dist => (q - dist, Up),
                Down => (dist - q, Down),
            };
            dist = new_dist;
            direction = new_direction;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn p_1_q_1() { assert_eq!(Solution::mirror_reflection(1, 1), 1); }
    #[rustfmt::skip] #[test] fn p_2_q_1() { assert_eq!(Solution::mirror_reflection(2, 1), 2); }
    #[rustfmt::skip] #[test] fn p_3_q_1() { assert_eq!(Solution::mirror_reflection(3, 1), 1); }
    #[rustfmt::skip] #[test] fn p_4_q_1() { assert_eq!(Solution::mirror_reflection(4, 1), 2); }
    #[rustfmt::skip] #[test] fn p_5_q_1() { assert_eq!(Solution::mirror_reflection(5, 1), 1); }
    #[rustfmt::skip] #[test] fn p_6_q_1() { assert_eq!(Solution::mirror_reflection(6, 1), 2); }

    #[rustfmt::skip] #[test] fn p_2_q_2() { assert_eq!(Solution::mirror_reflection(2, 2), 1); }
    #[rustfmt::skip] #[test] fn p_3_q_2() { assert_eq!(Solution::mirror_reflection(3, 2), 0); }
    #[rustfmt::skip] #[test] fn p_4_q_2() { assert_eq!(Solution::mirror_reflection(4, 2), 2); }
    #[rustfmt::skip] #[test] fn p_5_q_2() { assert_eq!(Solution::mirror_reflection(5, 2), 0); }
    #[rustfmt::skip] #[test] fn p_6_q_2() { assert_eq!(Solution::mirror_reflection(6, 2), 1); }
    #[rustfmt::skip] #[test] fn p_7_q_2() { assert_eq!(Solution::mirror_reflection(7, 2), 0); }
    #[rustfmt::skip] #[test] fn p_8_q_2() { assert_eq!(Solution::mirror_reflection(8, 2), 2); }

    #[rustfmt::skip] #[test] fn p_3_q_3() { assert_eq!(Solution::mirror_reflection(3, 3), 1); }
    #[rustfmt::skip] #[test] fn p_4_q_3() { assert_eq!(Solution::mirror_reflection(4, 3), 2); }
    #[rustfmt::skip] #[test] fn p_5_q_3() { assert_eq!(Solution::mirror_reflection(5, 3), 1); }
    #[rustfmt::skip] #[test] fn p_6_q_3() { assert_eq!(Solution::mirror_reflection(6, 3), 2); }
    #[rustfmt::skip] #[test] fn p_7_q_3() { assert_eq!(Solution::mirror_reflection(7, 3), 1); }
    #[rustfmt::skip] #[test] fn p_8_q_3() { assert_eq!(Solution::mirror_reflection(8, 3), 2); }
    #[rustfmt::skip] #[test] fn p_9_q_3() { assert_eq!(Solution::mirror_reflection(9, 3), 1); }

    #[rustfmt::skip] #[test] fn  p_4_q_4() { assert_eq!(Solution::mirror_reflection( 4, 4), 1); }
    #[rustfmt::skip] #[test] fn  p_5_q_4() { assert_eq!(Solution::mirror_reflection( 5, 4), 0); }
    #[rustfmt::skip] #[test] fn  p_6_q_4() { assert_eq!(Solution::mirror_reflection( 6, 4), 0); }
    #[rustfmt::skip] #[test] fn  p_7_q_4() { assert_eq!(Solution::mirror_reflection( 7, 4), 0); }
    #[rustfmt::skip] #[test] fn  p_8_q_4() { assert_eq!(Solution::mirror_reflection( 8, 4), 2); }
    #[rustfmt::skip] #[test] fn  p_9_q_4() { assert_eq!(Solution::mirror_reflection( 9, 4), 0); }
    #[rustfmt::skip] #[test] fn p_10_q_4() { assert_eq!(Solution::mirror_reflection(10, 4), 0); }
}
