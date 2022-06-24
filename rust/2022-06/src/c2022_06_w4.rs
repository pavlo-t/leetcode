#![allow(dead_code)]
//! \#1229. Meeting Scheduler
//! =========================
//!
//! Given the availability time slots arrays `slots1` and `slots2` of two people and a meeting duration `duration`,
//! return the __earliest time slot__ that works for both of them and is of duration `duration`.
//!
//! If there is no common time slot that satisfies the requirements, return an __empty array____.
//!
//! The format of a time slot is an array of two elements `[start, end]`
//! representing an inclusive time range from `start` to `end`.
//!
//! It is guaranteed that no two availability slots of the same person intersect with each other.
//! That is, for any two time slots `[start1, end1]` and `[start2, end2]` of the same person,
//! either `start1 > end2` or `start2 > end1`.
//!
//! __Constraints:__
//!
//! - `1 <= slots1.length, slots2.length <= 10_000`
//! - `slots1[i].length, slots2[i].length == 2`
//! - `slots1[i][0] < slots1[i][1]`
//! - `slots2[i][0] < slots2[i][1]`
//! - `0 <= slots1[i][j], slots2[i][j] <= 1_000_000_000`
//! - `1 <= duration <= 1_000_000`
//!
//! <https://leetcode.com/problems/meeting-scheduler>

pub struct Solution;
impl Solution {
    pub fn min_available_duration(
        mut slots1: Vec<Vec<i32>>,
        mut slots2: Vec<Vec<i32>>,
        duration: i32,
    ) -> Vec<i32> {
        slots1.sort_unstable();
        slots2.sort_unstable();
        let (mut i1, mut i2) = (0, 0);

        while i1 < slots1.len() && i2 < slots2.len() {
            let (s1, e1, s2, e2) = (slots1[i1][0], slots1[i1][1], slots2[i2][0], slots2[i2][1]);
            let start = s1.max(s2);
            if e1.min(e2) - start >= duration {
                return vec![start, start + duration];
            } else if e1 < e2 {
                i1 += 1;
            } else {
                i2 += 1;
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }
    macro_rules! check {
        ($s1:tt, $s2:tt, $d:expr, e = $e:tt) => {
            let s1 = vv!$s1;
            let s2 = vv!$s2;
            let e = vec!$e;
            assert_eq!(Solution::min_available_duration(s1, s2, $d), e);
        };
    }

    #[test]
    fn s10e50s60e120s140e210_s0e15s60e70_8() {
        let s1 = vv![[10, 50], [60, 120], [140, 210]];
        let s2 = vv![[0, 15], [60, 70]];
        assert_eq!(Solution::min_available_duration(s1, s2, 8), [60, 68]);
    }
    #[test]
    fn s10e50s60e120s140e210_s0e15s60e70_12() {
        let s1 = vv![[10, 50], [60, 120], [140, 210]];
        let s2 = vv![[0, 15], [60, 70]];
        assert_eq!(Solution::min_available_duration(s1, s2, 12), []);
    }

    #[rustfmt::skip] #[test] fn s1e2_s1e2_1() { check!([[1, 2]], [[1, 2]], 1, e = [1, 2]); }
    #[rustfmt::skip] #[test] fn s1e2_s1e2_2() { check!([[1, 2]], [[1, 2]], 2, e = []); }
    #[rustfmt::skip] #[test] fn s1e3_s1e2_2() { check!([[1, 3]], [[1, 2]], 2, e = []); }
    #[rustfmt::skip] #[test] fn s1e2_s1e3_2() { check!([[1, 2]], [[1, 3]], 2, e = []); }
}
