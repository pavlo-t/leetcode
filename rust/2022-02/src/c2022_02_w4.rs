#![allow(dead_code)]
/// 1064. Fixed Point
/// =================
///
/// Given an array of distinct integers `arr`,
/// where `arr` is sorted in __ascending order__,
/// return the smallest index `i` that satisfies `arr[i] == i`.
/// If there is no such index, return `-1`.
///
/// __Constraints:__
///
/// - `1 <= arr.length < 10_000`
/// - `-1_000_000_000 <= arr[i] <= 1_000_000_000`
///
/// __Follow up:__ The `O(n)` solution is very straightforward. Can we do better?
///
/// https://leetcode.com/problems/fixed-point/
struct Solution;
impl Solution {
    pub fn fixed_point_linear(arr: Vec<i32>) -> i32 {
        arr.into_iter()
            .enumerate()
            .find(|&(i, v)| i as i32 == v)
            .map(|(_, v)| v)
            .unwrap_or(-1)
    }

    pub fn fixed_point(arr: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, arr.len() - 1);
        while l < r {
            let m = l + (r - l) / 2;
            if arr[m] < m as i32 {
                l = m + 1;
            } else {
                r = m;
            }
        }

        if arr[l] == l as i32 {
            l as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn m10_m5_n0_p3_p7() {
        let a = vec![-10, -5, 0, 3, 7];
        assert_eq!(Solution::fixed_point(a), 3);
        // Explanation: For the given array, arr[0] = -10, arr[1] = -5, arr[2] = 0, arr[3] = 3, thus the output is 3.
    }
    #[test]
    fn n0_p2_p5_p8_p8_p17() {
        let a = vec![0, 2, 5, 8, 17];
        assert_eq!(Solution::fixed_point(a), 0);
        // Explanation: arr[0] = 0, thus the output is 0.
    }
    #[test]
    fn m10_m5_p3_p4_p7_p9() {
        let a = vec![-10, -5, 3, 4, 7, 9];
        assert_eq!(Solution::fixed_point(a), -1);
        // Explanation: There is no such i that arr[i] == i, thus the output is -1.
    }
}
