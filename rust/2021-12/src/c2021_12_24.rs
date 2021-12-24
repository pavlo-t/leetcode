#![allow(dead_code)]
/// 56. Merge Intervals
/// ===================
///
/// Given an array of `intervals` where `intervals[i] = [starti, endi]`, merge all overlapping intervals,
/// and return _an array of the non-overlapping intervals that cover all the intervals in the input_.
///
/// __Constraints:__
///
/// - `1 <= intervals.length <= 10_000`
/// - `intervals[i].length == 2`
/// - `0 <= starti <= endi <= 10_000`
///
/// https://leetcode.com/problems/merge-intervals/
struct Solution;
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        println!("merge({:?})", intervals);
        intervals.sort_unstable();
        let mut results = vec![intervals[0].clone()];
        let mut last_end = results[0][1];
        for i in intervals.into_iter().skip(1) {
            let (start, end) = (i[0], i[1]);
            if last_end < start {
                last_end = end;
                results.push(vec![start, end]);
            } else if last_end < end {
                last_end = end;
                results.last_mut().unwrap()[1] = end;
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => {vec![$(vec!$x),*]}; }

    #[test]
    fn i_1to3_2to6_8to10_15to18() {
        let i = vv![[1, 3], [2, 6], [8, 10], [15, 18]];
        let e = vv![[1, 6], [8, 10], [15, 18]];
        assert_eq!(Solution::merge(i), e);
        // Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].
    }
    #[test]
    fn i_1to4_4to5() {
        let i = vv![[1, 4], [4, 5]];
        let e = vv![[1, 5]];
        assert_eq!(Solution::merge(i), e);
        // Explanation: Intervals [1,4] and [4,5] are considered overlapping.
    }
    #[test]
    fn i_1to3_2to6_8to10_15to18_0to5() {
        let i = vv![[1, 3], [2, 6], [8, 10], [15, 18], [0, 5]];
        let e = vv![[0, 6], [8, 10], [15, 18]];
        assert_eq!(Solution::merge(i), e);
    }
    #[test]
    fn i_0to5_1to2_2to5_3to6_7to7() {
        let i = vv![[0, 5], [1, 2], [2, 5], [3, 6], [7, 7]];
        let e = vv![[0, 6], [7, 7]];
        assert_eq!(Solution::merge(i), e);
    }
}
