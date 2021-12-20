#![allow(dead_code)]
/// 1200. Minimum Absolute Difference
/// =================================
///
/// Given an array of __distinct__ integers `arr`,
/// find all pairs of elements with the minimum absolute difference of any two elements.
///
/// Return a list of pairs in ascending order(with respect to pairs), each pair `[a, b]` follows
///
/// - `a`, `b` are from `arr`
/// - `a < b`
/// - `b - a` equals to the minimum absolute difference of any two elements in `arr`
///
/// __Constraints:__
///
/// - `2 <= arr.length <= 100_000`
/// - `-1_000_000 <= arr[i] <= 1_000_000`
///
/// https://leetcode.com/problems/minimum-absolute-difference/
struct Solution;
impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        println!("minimum_abs_difference({:?})", arr);
        arr.sort_unstable();

        let min_diff = arr.windows(2).map(|w| w[1] - w[0]).min().unwrap();

        arr.windows(2)
            .filter(|w| (w[1] - w[0]) == min_diff)
            .map(|w| vec![w[0], w[1]])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a4213() {
        let a = vec![4, 2, 1, 3];
        let e = [[1, 2], [2, 3], [3, 4]];
        assert_eq!(Solution::minimum_abs_difference(a), e);
        // Explanation: The minimum absolute difference is 1.
        // List all pairs with difference equal to 1 in ascending order.
    }
    #[test]
    fn a_1_3_6_10_15() {
        let a = vec![1, 3, 6, 10, 15];
        let e = [[1, 3]];
        assert_eq!(Solution::minimum_abs_difference(a), e);
    }
    #[test]
    fn a_p3p8m10p23p19m4m14p27() {
        let a = vec![3, 8, -10, 23, 19, -4, -14, 27];
        let e = [[-14, -10], [19, 23], [23, 27]];
        assert_eq!(Solution::minimum_abs_difference(a), e);
    }
}
