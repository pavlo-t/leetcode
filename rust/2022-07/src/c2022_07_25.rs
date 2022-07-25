#![allow(dead_code)]
//! \#34. Find First and Last Position of Element in Sorted Array
//! =============================================================
//!
//! <https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array>
//!
//! Given an array of integers `nums` sorted in non-decreasing order,
//! find the starting and ending position of a given `target` value.
//!
//! If `target` is not found in the array, return `[-1, -1]`.
//!
//! You must write an algorithm with `O(log n)` runtime complexity.
//!
//! __Constraints:__
//!
//! - `0 <= nums.length <= 100_000`
//! - `-1_000_000_000 <= nums[i] <= 1_000_000_000`
//! - `nums` is a non-decreasing array.
//! - `-1_000_000_000 <= target <= 1_000_000_000`

pub struct Solution;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering;

        if nums.is_empty() {
            vec![-1, -1]
        } else {
            let n = nums.len();

            enum Next {
                MoveLeft,
                MoveRight,
                Found,
            }

            fn binary_search<N>(last_idx: usize, next_fn: N) -> usize
            where
                N: Fn(usize) -> Next,
            {
                let (mut l, mut r) = (0, last_idx);
                while l < r {
                    let m = (l + r) / 2;
                    match next_fn(m) {
                        Next::MoveLeft => l = (m + 1).min(last_idx),
                        Next::MoveRight => r = m.saturating_sub(1),
                        Next::Found => {
                            l = m;
                            break;
                        }
                    };
                }
                l
            }

            let next_fn = |m: usize| match nums[m].cmp(&target) {
                Ordering::Less => Next::MoveLeft,
                Ordering::Equal if m == 0 || nums[m - 1] < target => Next::Found,
                _ => Next::MoveRight,
            };

            match binary_search(n - 1, next_fn) {
                first if nums[first] == target => {
                    let next_fn = |m: usize| match nums[m].cmp(&target) {
                        Ordering::Greater => Next::MoveRight,
                        Ordering::Equal if m == n - 1 || nums[m + 1] > target => Next::Found,
                        _ => Next::MoveLeft,
                    };
                    vec![first as i32, binary_search(n - 1, next_fn) as i32]
                }
                _ => vec![-1, -1],
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn empty_t_1() { assert_eq!(Solution::search_range(vec![], 1), [-1, -1]); }

    #[rustfmt::skip] #[test] fn p1_t_0() { assert_eq!(Solution::search_range(vec![1], 0), [-1, -1]); }
    #[rustfmt::skip] #[test] fn p1_t_1() { assert_eq!(Solution::search_range(vec![1], 1), [0, 0]); }
    #[rustfmt::skip] #[test] fn p1_t_2() { assert_eq!(Solution::search_range(vec![1], 2), [-1, -1]); }

    #[rustfmt::skip] #[test] fn p1p1_t_0() { assert_eq!(Solution::search_range(vec![1,1], 0), [-1, -1]); }
    #[rustfmt::skip] #[test] fn p1p1_t_1() { assert_eq!(Solution::search_range(vec![1,1], 1), [0, 1]); }
    #[rustfmt::skip] #[test] fn p1p1_t_2() { assert_eq!(Solution::search_range(vec![1,1], 2), [-1, -1]); }

    #[rustfmt::skip] #[test] fn p0p2p4_t_2() { assert_eq!(Solution::search_range(vec![0,2,4], 2), [1, 1]); }
    #[rustfmt::skip] #[test] fn p1p2p3_t_2() { assert_eq!(Solution::search_range(vec![1,2,3], 2), [1, 1]); }

    #[rustfmt::skip] #[test] fn p0p2p2p4_t_2() { assert_eq!(Solution::search_range(vec![0,2,2,4], 2), [1, 2]); }
    #[rustfmt::skip] #[test] fn p1p2p2p3_t_2() { assert_eq!(Solution::search_range(vec![1,2,2,3], 2), [1, 2]); }

    #[test]
    fn p5p7p7p8p8p10_t_8() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        assert_eq!(Solution::search_range(nums, 8), [3, 4]);
    }
    #[test]
    fn p5p7p7p8p8p10_t_6() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        assert_eq!(Solution::search_range(nums, 6), [-1, -1]);
    }
}
