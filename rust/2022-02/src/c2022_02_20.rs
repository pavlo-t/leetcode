#![allow(dead_code)]
/// 1288. Remove Covered Intervals
/// ==============================
///
/// Given an array `intervals` where `intervals[i] = [li, ri]` represent the interval `[li, ri)`,
/// remove all intervals that are covered by another interval in the list.
///
/// The interval `[a, b)` is covered by the interval `[c, d)` if and only if `c <= a` and `b <= d`.
///
/// Return _the number of remaining intervals_.
///
/// __Constraints:__
///
/// - `1 <= intervals.length <= 1000`
/// - `intervals[i].length == 2`
/// - `0 <= li <= ri <= 100_000`
/// - All the given intervals are __unique__.
///
/// https://leetcode.com/problems/remove-covered-intervals/
struct Solution;
impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        println!("remove_covered_intervals({intervals:?})");
        let n = intervals.len();
        let mut covered = vec![false; n];
        for i in 0..n {
            let (a, b) = (intervals[i][0], intervals[i][1]);
            for j in (0..n).filter(|&j| j != i) {
                let (c, d) = (intervals[j][0], intervals[j][1]);
                if c <= a && b <= d {
                    covered[i] = true;
                }
            }
        }
        covered.into_iter().filter(|&c| !c).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn i_1_4_3_6_2_8() {
        let i = vv![[1,4],[3,6],[2,8]];
        assert_eq!(Solution::remove_covered_intervals(i), 2);
        // Explanation: Interval [3,6] is covered by [2,8], therefore it is removed.
    }
    #[test]
    fn i_1_4_2_3() {
        let i = vv![[1,4],[2,3]];
        assert_eq!(Solution::remove_covered_intervals(i), 1);
    }
}
