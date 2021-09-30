#![allow(dead_code)]
/// Meeting Scheduler
/// =================
///
/// Given the availability time slots arrays `slots1` and `slots2` of two people
/// and a meeting duration `duration`,
/// return the __earliest time slot__ that works for both of them and is of duration `duration`.
///
/// If there is no common time slot that satisfies the requirements, return an __empty array__.
///
/// The format of a time slot is an array of two elements `[start, end]` representing
/// an inclusive time range from `start` to `end`.
///
/// It is guaranteed that no two availability slots of the same person intersect with each other.
/// That is, for any two time slots `[start1, end1]` and `[start2, end2]` of the same person,
/// either `start1 > end2` or `start2 > end1`.
///
/// __Constraints:__
///
/// - `1 <= slots1.length, slots2.length <= 10_000`
/// - `slots1[i].length, slots2[i].length == 2`
/// - `slots1[i][0] < slots1[i][1]`
/// - `slots2[i][0] < slots2[i][1]`
/// - `0 <= slots1[i][j], slots2[i][j] <= 10^9`
/// - `1 <= duration <= 1_000_000`
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/597/week-5-april-29th-april-30th/3724/
struct Solution;
impl Solution {
    pub fn min_available_duration_min_heap(
        slots1: Vec<Vec<i32>>,
        slots2: Vec<Vec<i32>>,
        duration: i32,
    ) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::new();
        for s in slots1.into_iter().chain(slots2.into_iter()) {
            let (s, e) = (s[0], s[1]);
            heap.push(Reverse((s, e)));
        }
        while heap.len() > 1 {
            let Reverse((_, e1)) = heap.pop().unwrap();
            let &Reverse((s2, _)) = heap.peek().unwrap();
            if e1 >= s2 + duration {
                return vec![s2, s2 + duration];
            }
        }
        Vec::new()
    }

    pub fn min_available_duration(
        mut slots1: Vec<Vec<i32>>,
        mut slots2: Vec<Vec<i32>>,
        duration: i32,
    ) -> Vec<i32> {
        slots1.sort_unstable();
        slots2.sort_unstable();

        let mut i1 = 0;
        let mut i2 = 0;

        while i1 < slots1.len() && i2 < slots2.len() {
            let (s1, e1) = (slots1[i1][0], slots1[i1][1]);
            let (s2, e2) = (slots2[i2][0], slots2[i2][1]);

            let s_max = s1.max(s2);
            let e_min = e1.min(e2);
            if e_min - s_max >= duration {
                return vec![s_max, s_max + duration];
            }

            if e_min == e1 {
                i1 += 1;
            } else {
                i2 += 1;
            }
        }

        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn example1_s10t50_60t120_140t210_s0t15_60t70_d8_produces_60t68() {
        let s1 = vv![[10, 50], [60, 120], [140, 210]];
        let s2 = vv![[0, 15], [60, 70]];
        let d = 8;
        assert_eq!(Solution::min_available_duration(s1, s2, d), [60, 68]);
    }
    #[test]
    fn example2_s10t50_60t120_140t210_s0t15_60t70_d12_produces_empty() {
        let s1 = vv![[10, 50], [60, 120], [140, 210]];
        let s2 = vv![[0, 15], [60, 70]];
        let d = 12;
        assert_eq!(Solution::min_available_duration(s1, s2, d), []);
    }

    #[test]
    fn s140t210_10t50_60t120_s60t70_0t15_d8_produces_60t68() {
        let s1 = vv![[140, 210], [10, 50], [60, 120]];
        let s2 = vv![[60, 70], [0, 15]];
        let d = 8;
        assert_eq!(Solution::min_available_duration(s1, s2, d), [60, 68]);
    }
    #[test]
    fn s0t9_s0t8_d8_produces_0t8() {
        let s1 = vv![[0, 9]];
        let s2 = vv![[0, 8]];
        let d = 8;
        assert_eq!(Solution::min_available_duration(s1, s2, d), [0, 8]);
    }
    #[test]
    fn s0t9_s0t7_d8_produces_empty() {
        let s1 = vv![[0, 9]];
        let s2 = vv![[0, 7]];
        let d = 8;
        assert_eq!(Solution::min_available_duration(s1, s2, d), []);
    }

    mod performance {
        use super::*;

        #[test]
        fn s10k_s10k_d1_produces_empty() {
            let s1 = (0..20000).step_by(2).map(|i| vec![i, i + 1]).collect();
            let s2 = (20000..40000).step_by(2).map(|i| vec![i, i + 1]).collect();
            let d = 1;
            assert_eq!(Solution::min_available_duration(s1, s2, d), []);
        }
        #[test]
        fn s10k_s10k_d1_produces_empty_2() {
            let s1 = (0..40000).step_by(4).map(|i| vec![i, i + 1]).collect();
            let s2 = (0..40000).step_by(4).map(|i| vec![i + 2, i + 3]).collect();
            let d = 1;
            assert_eq!(Solution::min_available_duration(s1, s2, d), []);
        }
        #[test]
        fn s10k_s10k_d1_produces_41000t41001() {
            let mut s1: Vec<_> = (0..40000).step_by(4).map(|i| vec![i, i + 1]).collect();
            let mut s2: Vec<_> = (0..40000).step_by(4).map(|i| vec![i + 2, i + 3]).collect();
            s1.push(vec![41000, 41002]);
            s2.push(vec![41000, 41002]);
            let d = 1;
            assert_eq!(Solution::min_available_duration(s1, s2, d), [41000, 41001]);
        }
    }

    mod test_macros {
        macro_rules! vs { ($($s:tt),*) => { vec![$($s.to_string()),*] }; }
        macro_rules! vvs { ($($s:tt),*) => { vec![$(vs!$s),*] }; }

        #[test]
        fn test_macro_vv_i32() {
            let v = vv![[1, 2, 3], [4, 5, 6]];
            assert_eq!(v, vec![vec![1, 2, 3], vec![4, 5, 6]]);
        }
        #[test]
        fn test_macro_vv_str() {
            let v = vv![["1", "2"], ["3", "4", "5"]];
            assert_eq!(v, vec![vec!["1", "2"], vec!["3", "4", "5"]]);
        }
        #[test]
        fn test_macro_vvs() {
            let v = vvs![["1", "2"], ["3", "4", "5"]];
            assert_eq!(v, vec![vec!["1", "2"], vec!["3", "4", "5"]]);
        }
    }
}
