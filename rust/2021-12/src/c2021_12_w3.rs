#![allow(dead_code)]
/// 624. Maximum Distance in Arrays
/// ===============================
///
/// You are given `m` `arrays`, where each array is sorted in __ascending order__.
///
/// You can pick up two integers from two different arrays (each array picks one) and calculate the distance.
/// We define the distance between two integers `a` and `b` to be their absolute difference `|a - b|`.
///
/// Return _the maximum distance_.
///
/// __Constraints:__
///
/// - `m == arrays.length`
/// - `2 <= m <= 100_000`
/// - `1 <= arrays[i].length <= 500`
/// - `-10_000 <= arrays[i][j] <= 10_000`
/// - `arrays[i]` is sorted in __ascending order__.
/// - There will be at most `100_000` integers in all the arrays.
///
/// https://leetcode.com/problems/maximum-distance-in-arrays/
struct Solution;
impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        //println!("max_distance({:?})", arrays);
        fn min_max(array: &[i32]) -> (i32, i32) {
            (array[0], array.last().unwrap().to_owned())
        }
        let mut result = 0;
        let (mut min, mut max) = min_max(&arrays[0]);
        for array in arrays.iter().skip(1) {
            let (curr_min, curr_max) = min_max(array);
            result = result.max(curr_max - min).max(max - curr_min);
            min = min.min(curr_min);
            max = max.max(curr_max);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn a123_a45_a123() {
        let a = vv![[1, 2, 3], [4, 5], [1, 2, 3]];
        assert_eq!(Solution::max_distance(a), 4);
        // Explanation: One way to reach the maximum distance 4
        // is to pick 1 in the first or third array and pick 5 in the second array.
    }
    #[test]
    fn a1_a1() {
        let a = vv![[1], [1]];
        assert_eq!(Solution::max_distance(a), 0);
    }
    #[test]
    fn a1_a2() {
        let a = vv![[1], [2]];
        assert_eq!(Solution::max_distance(a), 1);
    }
    #[test]
    fn a14_a05() {
        let a = vv![[1, 4], [0, 5]];
        assert_eq!(Solution::max_distance(a), 4);
    }
    #[test]
    fn arr_100000x500x1() {
        let a = vec![vec![1; 500]; 100_000];
        assert_eq!(Solution::max_distance(a), 0);
    }
}
