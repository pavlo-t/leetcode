#![allow(dead_code)]
//! \#473. Matchsticks to Square
//! ============================
//!
//! You are given an integer array `matchsticks` where `matchsticks[i]` is the length of the `i`th matchstick.
//! You want to use __all the matchsticks__ to make one square.
//! You __should not break__ any stick, but you can link them up,
//! and each matchstick must be used __exactly one time__.
//!
//! Return `true` if you can make this square and `false` otherwise.
//!
//! __Constraints:__
//!
//! - `1 <= matchsticks.length <= 15`
//! - `1 <= matchsticks[i] <= 100_000_000`
//!
//! <https://leetcode.com/problems/matchsticks-to-square>

pub struct Solution;
impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        use std::collections::HashSet;

        fn next(i: usize, head: i32, sq: [i32; 4]) -> [i32; 4] {
            let mut next_sq = [
                sq[i] + head,
                sq[(i + 1) % 4],
                sq[(i + 2) % 4],
                sq[(i + 3) % 4],
            ];
            next_sq.sort_unstable();
            next_sq
        }

        fn rec(
            sq: [i32; 4],
            target: i32,
            matchsticks: &[i32],
            seen: &mut HashSet<[i32; 4]>,
        ) -> bool {
            !seen.contains(&sq) && {
                let result = if matchsticks.is_empty() {
                    sq.iter().all(|&s| s == target)
                } else {
                    let (head, tail) = (matchsticks[0], &matchsticks[1..]);

                    (0..4)
                        .filter(|&i| sq[i] + head <= target)
                        .map(|i| next(i, head, sq))
                        .any(|sq| rec(sq, target, tail, seen))
                };
                seen.insert(sq);
                result
            }
        }

        let total = matchsticks.iter().sum::<i32>();
        total % 4 == 0 && {
            let target = total / 4;
            let mut seen = HashSet::new();
            rec([0; 4], target, &matchsticks, &mut seen)
        }
    }

    pub fn makesquare_v2(matchsticks: Vec<i32>) -> bool {
        fn rec(sq: [i32; 4], target: i32, matchsticks: &[i32]) -> bool {
            if matchsticks.is_empty() {
                sq.into_iter().all(|s| s == target)
            } else {
                let (head, tail) = (matchsticks[0], &matchsticks[1..]);

                for i in (0..4).filter(|&i| sq[i] + head <= target) {
                    let mut next = [
                        sq[i] + head,
                        sq[(i + 1) % 4],
                        sq[(i + 2) % 4],
                        sq[(i + 3) % 4],
                    ];
                    next.sort_unstable();
                    if rec(next, target, tail) {
                        return true;
                    }
                }
                false
            }
        }

        let total = matchsticks.iter().sum::<i32>();
        total % 4 == 0 && {
            let target = total / 4;
            rec([0; 4], target, &matchsticks)
        }
    }

    pub fn makesquare_v1(mut matchsticks: Vec<i32>) -> bool {
        fn rec(a: i32, b: i32, c: i32, d: i32, matchsticks: &[i32]) -> bool {
            if matchsticks.is_empty() {
                a == b && b == c && c == d
            } else {
                let head = matchsticks[0];
                let tail = &matchsticks[1..];

                rec(a + head, b, c, d, tail)
                    || rec(a, b + head, c, d, tail)
                    || rec(a, b, c + head, d, tail)
                    || rec(a, b, c, d + head, tail)
            }
        }

        matchsticks.len() >= 4 && {
            matchsticks.sort_unstable();
            matchsticks.reverse();
            rec(
                matchsticks[0],
                matchsticks[1],
                matchsticks[2],
                matchsticks[3],
                &matchsticks[4..],
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn m_1() { assert!(!Solution::makesquare(vec![1])); }
    #[rustfmt::skip] #[test] fn m_1_1() { assert!(!Solution::makesquare(vec![1,1])); }
    #[rustfmt::skip] #[test] fn m_1_1_1() { assert!(!Solution::makesquare(vec![1,1,1])); }
    #[rustfmt::skip] #[test] fn m_1_1_1_1() { assert!(Solution::makesquare(vec![1,1,1,1])); }
    #[rustfmt::skip] #[test] fn m_1_1_1_2() { assert!(!Solution::makesquare(vec![1,1,1,2])); }

    #[test]
    fn m_1_1_2_2_2() {
        let m = vec![1, 1, 2, 2, 2];
        assert_eq!(Solution::makesquare(m), true);
        // Explanation: You can form a square with length 2, one side of the square came two sticks with length 1.
    }
    #[test]
    fn m_3_3_3_3_4() {
        let m = vec![3, 3, 3, 3, 4];
        assert_eq!(Solution::makesquare(m), false);
        // Explanation: You cannot find a way to form a square with all the matchsticks.
    }

    #[test]
    fn m_1_x_15() {
        assert!(!Solution::makesquare(vec![1; 15]));
    }
    #[test]
    fn m_1x14_appended_2() {
        let m = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2];
        assert!(Solution::makesquare(m));
    }
}
