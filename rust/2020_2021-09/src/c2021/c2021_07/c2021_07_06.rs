#![allow(dead_code)]
/// Reduce Array Size to The Half
/// =============================
///
/// Given an array `arr`,
/// you can choose a set of integers and remove all the occurrences of these integers in the array.
///
/// Return _the minimum size of the set_ so that __at least__ half of the integers of the array are removed.
///
/// __Constraints:__
///
/// - `1 <= arr.length <= 100_000`
/// - `arr.length` is even.
/// - `1 <= arr[i] <= 100_000`
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/608/week-1-july-1st-july-7th/3804/
struct Solution;
impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut l = arr.len() / 2;
        let mut counts = HashMap::new();
        for i in arr {
            *counts.entry(i).or_insert(0usize) += 1;
        }

        let mut counts = counts.values().map(|&i| i).collect::<Vec<_>>();
        counts.sort_unstable_by(|a, b| b.cmp(a));

        let mut r = 0;
        for i in counts {
            r += 1;
            l = l.saturating_sub(i);
            if l == 0 {
                return r;
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_3_3_3_3_5_5_5_2_2_7_produces_2() {
        let arr = vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7];
        assert_eq!(Solution::min_set_size(arr), 2);
        // Explanation: Choosing {3,7} will make the new array [5,5,5,2,2] which has size 5 (i.e equal to half of the size of the old array).
        // Possible sets of size 2 are {3,5},{3,2},{5,2}.
        // Choosing set {2,7} is not possible as it will make the new array [3,3,3,3,5,5,5] which has size greater than half of the size of the old array.
    }
    #[test]
    fn a_7_7_7_7_7_7_produces_1() {
        let arr = vec![7, 7, 7, 7, 7, 7];
        assert_eq!(Solution::min_set_size(arr), 1);
        //Explanation: The only possible set you can choose is {7}. This will make the new array empty.
    }
    #[test]
    fn a_1_9_produces_1() {
        let arr = vec![1, 9];
        assert_eq!(Solution::min_set_size(arr), 1);
    }
    #[test]
    fn a_1000_1000_3_7_produces_1() {
        let arr = vec![1000, 1000, 3, 7];
        assert_eq!(Solution::min_set_size(arr), 1);
    }
    #[test]
    fn a_1_2_3_4_5_6_7_8_9_10_produces_5() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(Solution::min_set_size(arr), 5);
    }
}
