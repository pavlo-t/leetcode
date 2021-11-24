#![allow(dead_code)]
/// 986. Interval List Intersections
/// ================================
///
/// You are given two lists of closed intervals, `firstList` and `secondList`,
/// where `firstList[i] = [starti, endi]` and `secondList[j] = [startj, endj]`.
/// Each list of intervals is pairwise __disjoint__ and in __sorted order__.
///
/// Return _the intersection of these two interval lists_.
///
/// A __closed interval__ `[a, b]` (with `a <= b`) denotes the set of real numbers `x` with `a <= x <= b`.
///
/// The __intersection__ of two closed intervals
/// is a set of real numbers that are either empty or represented as a closed interval.
/// For example, the intersection of `[1, 3]` and `[2, 4]` is `[2, 3]`.
///
/// __Constraints:__
///
/// - `0 <= firstList.length, secondList.length <= 1000`
/// - `firstList.length + secondList.length >= 1`
/// - `0 <= start_i < end_i <= 10**9`
/// - `end_i < start_i+1`
/// - `0 <= start_j < end_j <= 10**9`
/// - `end_j < start_j+1`
///
/// https://leetcode.com/problems/interval-list-intersections/
struct Solution;
impl Solution {
    pub fn interval_intersection_my(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        println!("interval_intersection({:?}, {:?})", a, b);
        let (mut i, mut j) = (0, 0);
        let mut result = vec![];
        while i < a.len() && j < b.len() {
            if a[i][0] <= b[j][1] && a[i][1] >= b[j][0] {
                result.push(vec![a[i][0].max(b[j][0]), a[i][1].min(b[j][1])]);
                if a[i][1] < b[j][1] {
                    i += 1;
                } else if a[i][1] > b[j][1] {
                    j += 1;
                } else {
                    i += 1;
                    j += 1;
                }
            } else if a[i][1] < b[j][0] {
                i += 1;
            } else {
                j += 1;
            }
        }
        result
    }
    /// Approach 1: Merge Intervals - essentially the same, but a bit simpler code
    /// https://leetcode.com/problems/interval-list-intersections/solution/
    pub fn interval_intersection(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        println!("interval_intersection({:?}, {:?})", a, b);
        let (mut i, mut j) = (0, 0);
        let mut result = vec![];
        while i < a.len() && j < b.len() {
            let (lo, hi) = (a[i][0].max(b[j][0]), a[i][1].min(b[j][1]));
            if lo <= hi {
                result.push(vec![lo, hi]);
            }

            if a[i][1] < b[j][1] {
                i += 1;
            } else {
                j += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn a_1t7_b_3t10() {
        let a = vv![[1, 7]];
        let b = vv![[3, 10]];
        let e = vv![[3, 7]];
        assert_eq!(Solution::interval_intersection(a, b), e);
    }
    #[test]
    fn a_0t2_5t10_13t23_24t25_b_1t5_8t12_15t24_25t26() {
        let a = vv![[0, 2], [5, 10], [13, 23], [24, 25]];
        let b = vv![[1, 5], [8, 12], [15, 24], [25, 26]];
        let e = vv![[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]];
        assert_eq!(Solution::interval_intersection(a, b), e);
    }
    #[test]
    fn a_1t3_5t9_b_empty() {
        let a = vv![[1, 3], [5, 9]];
        let b = vv![];
        let e: Vec<Vec<i32>> = vv![];
        assert_eq!(Solution::interval_intersection(a, b), e);
    }
    #[test]
    fn a_empty_b_4t8_10t12() {
        let a = vv![];
        let b = vv![[4, 8], [10, 12]];
        let e: Vec<Vec<i32>> = vv![];
        assert_eq!(Solution::interval_intersection(a, b), e);
    }
    #[test]
    fn a_1t7_b_8t10() {
        let a = vv![[1, 7]];
        let b = vv![[8, 10]];
        let e: Vec<Vec<i32>> = vv![];
        assert_eq!(Solution::interval_intersection(a, b), e);
    }
}
