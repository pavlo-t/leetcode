#![allow(dead_code)]
/// 941. Valid Mountain Array
/// =========================
///
/// Given an array of integers `arr`, return _`true` if and only if it is a valid mountain array_.
///
/// Recall that `arr` is a mountain array if and only if:
///
/// - `arr.length >= 3`
/// - There exists some `i` with `0 < i < arr.length - 1` such that:
///   - `arr[0] < arr[1] < ... < arr[i - 1] < arr[i]`
///   - `arr[i] > arr[i + 1] > ... > arr[arr.length - 1]`
///
/// __Constraints:__
///
/// - `1 <= arr.length <= 10_000`
/// - `0 <= arr[i] <= 10_000`
///
/// https://leetcode.com/problems/valid-mountain-array/
struct Solution;
impl Solution {
    pub fn valid_mountain_array_my_for_newer_rust(arr: Vec<i32>) -> bool {
        // 0: start; 1: seen increasing; 2: seen decreasing
        let mut state = 0;
        for w in arr.windows(2) {
            state = match (w[0].cmp(&w[1]), state) {
                (std::cmp::Ordering::Less, 0 | 1) => 1,
                (std::cmp::Ordering::Greater, 1 | 2) => 2,
                _ => return false,
            };
        }
        state == 2
    }
    pub fn valid_mountain_array_for_older_rust_at_leetcode(arr: Vec<i32>) -> bool {
        // 0: start; 1: seen increasing; 2: seen decreasing
        let mut state = 0;
        for w in arr.windows(2) {
            state = match (w[0].cmp(&w[1]), state) {
                (std::cmp::Ordering::Less, 0) => 1,
                (std::cmp::Ordering::Less, 1) => 1,
                (std::cmp::Ordering::Greater, 1) => 2,
                (std::cmp::Ordering::Greater, 2) => 2,
                _ => return false,
            };
        }
        state == 2
    }
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        // 0: start; 1: seen increasing; 2: seen decreasing; 3: invalid
        2 == arr
            .windows(2)
            .map(|w| (w[0], w[1]))
            .fold(0, |state, (a, b)| match (a.cmp(&b), state) {
                (std::cmp::Ordering::Less, 0 | 1) => 1,
                (std::cmp::Ordering::Greater, 1 | 2) => 2,
                _ => 3,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_2_1() {
        assert_eq!(Solution::valid_mountain_array(vec![2, 1]), false);
    }
    #[test]
    fn a_3_5_5() {
        assert_eq!(Solution::valid_mountain_array(vec![3, 5, 5]), false);
    }
    #[test]
    fn a_0_3_2_1() {
        assert_eq!(Solution::valid_mountain_array(vec![0, 3, 2, 1]), true);
    }
    #[test]
    fn a_1_2_2_1() {
        assert_eq!(Solution::valid_mountain_array(vec![1, 2, 2, 1]), false);
    }
}
