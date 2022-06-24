#![allow(dead_code)]
//! \#88. Merge Sorted Array
//! ========================
//!
//! You are given two integer arrays `nums1` and `nums2`, sorted in __non-decreasing order__,
//! and two integers `m` and `n`, representing the number of elements in `nums1` and `nums2` respectively.
//!
//! __Merge__ `nums1` and `nums2` into a single array sorted in __non-decreasing order__.
//!
//! The final sorted array should not be returned by the function,
//! but instead be _stored inside the array_ `nums1`.
//! To accommodate this, `nums1` has a length of `m + n`,
//! where the first `m` elements denote the elements that should be merged,
//! and the last `n` elements are set to `0` and should be ignored.
//! `nums2` has a length of `n`.
//!
//! __Constraints:__
//!
//! - `nums1.length == m + n`
//! - `nums2.length == n`
//! - `0 <= m, n <= 200`
//! - `1 <= m + n <= 200`
//! - `-1_000_000_000 <= nums1[i], nums2[j] <= 1_000_000_000`
//!
//! __Follow up:__ Can you come up with an algorithm that runs in `O(m + n)` time?
//!
//! <https://leetcode.com/problems/merge-sorted-array>

pub struct Solution;
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut m, mut n) = (m as usize, n as usize);
        for i in (0..nums1.len()).rev() {
            if n == 0 {
                break;
            } else if m == 0 || nums1[m - 1] < nums2[n - 1] {
                n -= 1;
                nums1[i] = nums2[n];
            } else {
                m -= 1;
                nums1[i] = nums1[m];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ns1_123000_m_3_ns2_256_n_3() {
        let mut ns1 = vec![1, 2, 3, 0, 0, 0];
        let mut ns2 = vec![2, 5, 6];
        Solution::merge(&mut ns1, 3, &mut ns2, 3);
        assert_eq!(ns1, [1, 2, 2, 3, 5, 6]);
        // Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
        // The result of the merge is [1,2,2,3,5,6] with the underlined elements coming from nums1.
    }
    #[test]
    fn ns1_1_m_1_ns2_empty_n_0() {
        let mut ns1 = vec![1];
        let mut ns2 = vec![];
        Solution::merge(&mut ns1, 1, &mut ns2, 0);
        assert_eq!(ns1, [1]);
        // Explanation: The arrays we are merging are [1] and [].
        // The result of the merge is [1].
    }
    #[test]
    fn ns1_0_m_0_ns2_1_n_1() {
        let mut ns1 = vec![0];
        let mut ns2 = vec![1];
        Solution::merge(&mut ns1, 0, &mut ns2, 1);
        assert_eq!(ns1, [1]);
        // Explanation: The arrays we are merging are [] and [1].
        // The result of the merge is [1].
        // Note that because m = 0, there are no elements in nums1.
        // The 0 is only there to ensure the merge result can fit in nums1.
    }
}
