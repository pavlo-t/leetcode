#![allow(dead_code)]
//! \#462. Minimum Moves to Equal Array Elements II
//! ===============================================
//!
//! Given an integer array `nums` of size `n`,
//! return _the minimum number of moves required to make all array elements equal_.
//!
//! In one move, you can increment or decrement an element of the array by `1`.
//!
//! Test cases are designed so that the answer will fit in a __32-bit__ integer.
//!
//! __Constraints:__
//!
//! - `1 <= nums.length <= 100_000`
//! - `-1_000_000_000 <= nums[i] <= 1_000_000_000`
//!
//! <https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii>

pub struct Solution;
impl Solution {
    /// using median - figured after looking at the old solution `/rust/2020_2021-09/src/c2021/c2021_05/c2021_05_19.rs`
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let median = nums[nums.len() / 2];
        nums.into_iter().map(|n| (median - n).abs()).sum()
    }

    /// `14:20‥14:33` using average (__wrong answer__)
    pub fn min_moves2_avg(nums: Vec<i32>) -> i32 {
        let avg = (nums.iter().map(|&n| n as i64).sum::<i64>() / (nums.len() as i64)) as i32;
        fn count_changes(target: i32, nums: &[i32]) -> i32 {
            nums.iter().map(|&n| (target - n).abs()).sum()
        }
        count_changes(avg, &nums).min(count_changes(avg + 1, &nums))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n1_2_3() {
        assert_eq!(Solution::min_moves2(vec![1, 2, 3]), 2);
        // Explanation:
        // Only two moves are needed (remember each move increments or decrements one element):
        // [1,2,3]  =>  [2,2,3]  =>  [2,2,2]
    }
    #[test]
    fn n1_2_3_4() {
        assert_eq!(Solution::min_moves2(vec![1, 2, 3, 4]), 4);
    }
    #[test]
    fn n1_2_3_4_5() {
        assert_eq!(Solution::min_moves2(vec![1, 2, 3, 4, 5]), 6);
    }
    #[test]
    fn n1_2_2_4_5() {
        assert_eq!(Solution::min_moves2(vec![1, 2, 2, 4, 5]), 6);
    }
    #[test]
    fn n1_3_3_4_5() {
        assert_eq!(Solution::min_moves2(vec![1, 3, 3, 4, 5]), 5);
    }
    #[test]
    fn n1_4_4_4_5() {
        assert_eq!(Solution::min_moves2(vec![1, 4, 4, 4, 5]), 4);
    }
    #[test]
    fn n1_1_1_1_1_10() {
        assert_eq!(Solution::min_moves2(vec![1, 1, 1, 1, 1, 10]), 9);
    }
    #[test]
    fn n1_10_10_10_10_10() {
        assert_eq!(Solution::min_moves2(vec![1, 10, 10, 10, 10, 10]), 9);
    }
    #[test]
    fn n1_10_2_9() {
        assert_eq!(Solution::min_moves2(vec![1, 10, 2, 9]), 16);
    }
    #[test]
    fn n1_1_20() {
        assert_eq!(Solution::min_moves2(vec![1, 1, 20]), 19);
    }
    #[test]
    fn n1_2_3_4_9() {
        assert_eq!(Solution::min_moves2(vec![1, 2, 3, 4, 9]), 10);
    }
    #[test]
    fn n1_1_1_1_1_2_20() {
        assert_eq!(Solution::min_moves2(vec![1, 1, 1, 1, 1, 2, 20]), 20);
    }
}
