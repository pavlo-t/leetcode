#![allow(dead_code)]
//! \#215. Kth Largest Element in an Array
//! ======================================
//!
//! Given an integer array `nums` and an integer `k`, return _the `k`th largest element in the array_.
//!
//! Note that it is the `k`th largest element in the sorted order, not the `k`th distinct element.
//!
//! __Constraints:__
//!
//! - `1 <= k <= nums.length <= 10_000`
//! - `-10_000 <= nums[i] <= 10_000`
//!
//! <https://leetcode.com/problems/kth-largest-element-in-an-array>

pub struct Solution;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;
        let mut heap = BinaryHeap::new();
        for n in nums {
            heap.push(Reverse(n));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        heap.pop().unwrap().0
    }
    pub fn find_kth_largest_sort(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        nums[nums.len() - k as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_3_2_1_5_6_4_k_2() {
        let n = vec![3, 2, 1, 5, 6, 4];
        assert_eq!(Solution::find_kth_largest(n, 2), 5);
    }
    #[test]
    fn n_3_2_3_1_2_4_5_5_6_k_4() {
        let n = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        assert_eq!(Solution::find_kth_largest(n, 4), 4);
    }
}
