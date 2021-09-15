#![allow(dead_code)]
/// Longest Turbulent Subarray
/// ==========================
///
/// Given an integer array `arr`, return _the length of a maximum size turbulent subarray of `arr`_.
///
/// A subarray is __turbulent__ if the comparison sign flips between each adjacent pair of elements in the subarray.
///
/// More formally, a subarray `[arr[i], arr[i + 1], ..., arr[j]]` of `arr` is said to be turbulent if and only if:
///
/// - For `i <= k < j`:
///   - `arr[k] > arr[k + 1]` when `k` is odd, and
///   - `arr[k] < arr[k + 1]` when `k` is even.
/// - Or, for `i <= k < j`:
///   - `arr[k] > arr[k + 1]` when `k` is even, and
///   - `arr[k] < arr[k + 1]` when `k` is odd.
///
/// __Constraints:__
///
/// - `1 <= arr.length <= 40_000`
/// - `0 <= arr[i] <= 1_000_000_000`
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/638/week-3-september-15th-september-21st/3976/
struct Solution;
impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        if arr.len() < 2 {
            1
        } else {
            let mut ps = 0;
            let mut curr = 1;
            let mut max_size = 1;
            for i in 1..arr.len() {
                let cs = (arr[i] - arr[i - 1]).signum();
                curr = match cs {
                    0 => 1,
                    cs if cs == ps => 2,
                    _ => curr + 1,
                };
                max_size = max_size.max(curr);
                ps = cs;
            }
            max_size
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_9_4_2_10_7_8_8_1_9_produces_5() {
        let arr = vec![9, 4, 2, 10, 7, 8, 8, 1, 9];
        assert_eq!(Solution::max_turbulence_size(arr), 5);
        // Explanation: arr[1] > arr[2] < arr[3] > arr[4] < arr[5]
    }
    #[test]
    fn a_4_8_12_16_produces_2() {
        let arr = vec![4, 8, 12, 16];
        assert_eq!(Solution::max_turbulence_size(arr), 2);
    }
    #[test]
    fn a_100_produces_1() {
        let arr = vec![100];
        assert_eq!(Solution::max_turbulence_size(arr), 1);
    }
    #[test]
    fn a_1_1_produces_1() {
        let arr = vec![1, 1];
        assert_eq!(Solution::max_turbulence_size(arr), 1);
    }
    #[test]
    fn a_1_2_produces_2() {
        let arr = vec![1, 2];
        assert_eq!(Solution::max_turbulence_size(arr), 2);
    }
    #[test]
    fn a_2_1_produces_2() {
        let arr = vec![2, 1];
        assert_eq!(Solution::max_turbulence_size(arr), 2);
    }
    #[test]
    fn a_1_1_2_2_produces_2() {
        let arr = vec![1, 1, 2, 2];
        assert_eq!(Solution::max_turbulence_size(arr), 2);
    }
    #[test]
    fn a_1_2_3_4_5_6_7_8_1_9_produces_4() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 1, 9];
        assert_eq!(Solution::max_turbulence_size(arr), 4);
    }
}
