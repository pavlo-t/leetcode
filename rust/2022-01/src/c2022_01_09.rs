#![allow(dead_code)]
/// 1041. Robot Bounded In Circle
/// =============================
///
/// On an infinite plane, a robot initially stands at `(0, 0)` and faces north.
/// The robot can receive one of three instructions:
///
/// - `"G"`: go straight 1 unit;
/// - `"L"`: turn 90 degrees to the left;
/// - `"R"`: turn 90 degrees to the right.
///
/// The robot performs the `instructions` given in order, and repeats them forever.
///
/// Return `true` if and only if there exists a circle in the plane such that the robot never leaves the circle.
///
/// __Constraints:__
///
/// - `1 <= instructions.length <= 100`
/// - `instructions[i]` is `'G'`, `'L'` or, `'R'`.
///
/// https://leetcode.com/problems/robot-bounded-in-circle/
struct Solution;
impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        println!("is_robot_bounded({:?})", instructions);
        let (mut x, mut y) = (0, 0);
        let mut d = 0;
        for c in instructions.chars() {
            match c {
                'L' => d -= 1,
                'R' => d += 1,
                // 'G'
                _ => match d {
                    0 => x += 1,
                    1 => y += 1,
                    2 => x -= 1,
                    _ => y -= 1,
                }
            }
            if d < 0 {
                d = 3;
            } else if d > 3 {
                d = 0;
            }
        }
        (x, y) == (0, 0) || d != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ggllgg() {
        assert_eq!(Solution::is_robot_bounded("GGLLGG".into()), true);
        // Explanation: The robot moves from (0,0) to (0,2), turns 180 degrees, and then returns to (0,0).
        // When repeating these instructions, the robot remains in the circle of radius 2 centered at the origin.
    }
    #[test]
    fn gg() {
        assert_eq!(Solution::is_robot_bounded("GG".into()), false);
        // Explanation: The robot moves north indefinitely.
    }
    #[test]
    fn gl() {
        assert_eq!(Solution::is_robot_bounded("GL".into()), true);
        // Explanation: The robot moves from (0, 0) -> (0, 1) -> (-1, 1) -> (-1, 0) -> (0, 0) -> ...
    }
}
