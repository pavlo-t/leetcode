#![allow(dead_code)]
/// Find K Closest Elements
/// =======================
///
/// Given a __sorted__ integer array `arr`, two integers `k` and `x`,
/// return the `k` closest integers to `x` in the array.
/// The result should also be sorted in ascending order.
///
/// An integer `a` is closer to `x` than an integer `b` if:
///
/// - `|a - x| < |b - x|`, or
/// - `|a - x| == |b - x|` and `a < b`
///
/// __Constraints:__
///
/// - `1 <= k <= arr.length`
/// - `1 <= arr.length <= 10_000`
/// - `arr` is sorted in __ascending__ order.
/// - `-10_000 <= arr[i], x <= 10_000`
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/608/week-1-july-1st-july-7th/3800/
struct Solution;
impl Solution {
    /// Approach 3: Binary Search To Find The Left Bound
    ///
    /// https://leetcode.com/problems/find-k-closest-elements/solution/
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let mut l = 0;
        let mut r = arr.len() - k;
        while l < r {
            let m = l + (r - l) / 2;
            if x - arr[m] > arr[m + k] - x {
                l = m + 1;
            } else {
                r = m;
            }
        }
        arr[l..(l + k)].into()
    }

    pub fn find_closest_elements_my(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        fn closer(a: i32, b: i32, x: i32) -> bool {
            let ad = (a - x).abs();
            let bd = (b - x).abs();

            ad < bd || (ad == bd && a < b)
        }

        let mut l = 0;
        let mut r = k as usize;

        while r < arr.len() && !closer(arr[l], arr[r], x) {
            l += 1;
            r += 1;
        }

        arr[l..r].into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work_with_x_in_arr() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::find_closest_elements(arr, 4, 3), [1, 2, 3, 4]);
    }
    #[test]
    fn should_work_with_x_not_in_arr() {
        let arr = vec![1, 2, 4, 5];
        assert_eq!(Solution::find_closest_elements(arr, 3, 3), [1, 2, 4]);
    }
    #[test]
    fn should_work_with_k_eq_arr_len() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::find_closest_elements(arr, 5, 1), [1, 2, 3, 4, 5]);
    }
    #[test]
    fn should_work_with_x_lt_smallest_element() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::find_closest_elements(arr, 4, -1), [1, 2, 3, 4]);
    }
    #[test]
    fn should_work_with_x_gt_greatest_element() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::find_closest_elements(arr, 4, 100), [2, 3, 4, 5]);
    }
    #[test]
    fn should_work_with_repeated_elements() {
        let arr = vec![1, 1, 1, 1, 1, 2, 3, 4, 5];
        assert_eq!(Solution::find_closest_elements(arr, 4, 2), [1, 1, 1, 2]);
    }
}
