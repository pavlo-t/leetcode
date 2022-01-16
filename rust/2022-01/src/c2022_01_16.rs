#![allow(dead_code)]
/// 849. Maximize Distance to Closest Person
/// ========================================
///
/// You are given an array representing a row of `seats`
/// where `seats[i] = 1` represents a person sitting in the `i`th seat,
/// and `seats[i] = 0` represents that the `i`th seat is empty __(0-indexed)__.
///
/// There is at least one empty seat, and at least one person sitting.
///
/// Alex wants to sit in the seat such that the distance between him and the closest person to him is maximized.
///
/// Return that maximum distance to the closest person.
///
/// __Constraints:__
///
/// - `2 <= seats.length <= 20_000`
/// - `seats[i]` is `0` or `1`.
/// - At least one seat is __empty__.
/// - At least one seat is __occupied__.
///
/// https://leetcode.com/problems/maximize-distance-to-closest-person/
struct Solution;
impl Solution {
    pub fn max_dist_to_closest(mut seats: Vec<i32>) -> i32 {
        let n = seats.len();
        seats[0] = if seats[0] == 0 { i32::MAX } else { 0 };
        for i in 1..n {
            seats[i] = if seats[i] == 0 { seats[i - 1].saturating_add(1) } else { 0 };
        }
        let mut result = seats[n - 1];
        for i in (0..n - 1).rev() {
            seats[i] = seats[i].min(seats[i + 1].saturating_add(1));
            result = result.max(seats[i]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s_1000101() {
        let s = vec![1, 0, 0, 0, 1, 0, 1];
        assert_eq!(Solution::max_dist_to_closest(s), 2);
        // Explanation:
        // If Alex sits in the second open seat (i.e. seats[2]), then the closest person has distance 2.
        // If Alex sits in any other open seat, the closest person has distance 1.
        // Thus, the maximum distance to the closest person is 2.
    }
    #[test]
    fn s_1000() {
        let s = vec![1, 0, 0, 0];
        assert_eq!(Solution::max_dist_to_closest(s), 3);
        // Explanation:
        // If Alex sits in the last seat (i.e. seats[3]), the closest person is 3 seats away.
        // This is the maximum distance possible, so the answer is 3.
    }
    #[test]
    fn s_01() {
        let s = vec![0, 1];
        assert_eq!(Solution::max_dist_to_closest(s), 1);
    }

    #[test]
    fn s_001() {
        let s = vec![0, 0, 1];
        assert_eq!(Solution::max_dist_to_closest(s), 2);
    }
}
