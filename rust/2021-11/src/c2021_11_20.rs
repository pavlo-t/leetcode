#![allow(dead_code)]
/// 540. Single Element in a Sorted Array
/// =====================================
///
/// You are given a sorted array consisting of only integers where every element appears exactly twice,
/// except for one element which appears exactly once.
///
/// Return _the single element that appears only once_.
///
/// Your solution must run in `O(log n)` time and `O(1)` space.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 100_000`
/// - `0 <= nums[i] <= 100_000`
///
/// https://leetcode.com/problems/single-element-in-a-sorted-array/
struct Solution;
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        println!("single_non_duplicate({:?})", nums);
        let rmax = nums.len() - 1;
        let (mut l, mut r) = (0, rmax);
        while l < r {
            let m = l + (r - l) / 2;
            if ((m % 2) == 0) == (nums[m] == nums[m + 1]) {
                l = m + 1;
            } else {
                r = m;
            }
        }
        nums[l]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_3() {
        let n = vec![3];
        assert_eq!(Solution::single_non_duplicate(n), 3);
    }
    #[test]
    fn n_112233445() {
        //          [0, 1, 2, 3, 4, 5, 6, 7, 8];
        let n = vec![1, 1, 2, 2, 3, 3, 4, 4, 5];
        assert_eq!(Solution::single_non_duplicate(n), 5);
    }
    #[test]
    fn n_122334455() {
        //          [0, 1, 2, 3, 4, 5, 6, 7, 8];
        let n = vec![1, 2, 2, 3, 3, 4, 4, 5, 5];
        assert_eq!(Solution::single_non_duplicate(n), 1);
    }
    #[test]
    fn n_112334488() {
        //          [0, 1, 2, 3, 4, 5, 6, 7, 8];
        let n = vec![1, 1, 2, 3, 3, 4, 4, 8, 8];
        assert_eq!(Solution::single_non_duplicate(n), 2);
    }
    #[test]
    fn n_3_3_7_7_10_11_11() {
        let n = vec![3, 3, 7, 7, 10, 11, 11];
        assert_eq!(Solution::single_non_duplicate(n), 10);
    }
}
