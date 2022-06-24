//! \#167. Two Sum II - Input Array Is Sorted
//! =========================================
//!
//! Given a __1-indexed__ array of integers numbers that is already __sorted in non-decreasing order__,
//! find two numbers such that they add up to a specific `target` number.
//! Let these two numbers be `numbers[index1]` and `numbers[index2]` where `1 <= index1 < index2 <= numbers.length`.
//!
//! Return _the indices of the two numbers, `index1` and `index2`,
//! __added by one__ as an integer array `[index1, index2]` of length 2_.
//!
//! The tests are generated such that there is __exactly one solution__.
//! You __may not__ use the same element twice.
//!
//! Your solution must use only constant extra space.
//!
//! __Constraints:__
//!
//! - `2 <= numbers.length <= 30_000`
//! - `-1000 <= numbers[i] <= 1000`
//! - `numbers` is sorted in __non-decreasing order__.
//! - `-1000 <= target <= 1000`
//! - The tests are generated such that there is __exactly one solution__.
//!
//! <https://leetcode.com/problems/two-sum-ii-input-array-is-sorted>
#![allow(dead_code)]

pub struct Solution;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l, mut r) = (0, numbers.len() - 1);
        while numbers[l] + numbers[r] != target {
            if numbers[l] + numbers[r] < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
        vec![l as i32 + 1, r as i32 + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_2_7_11_15_t_9() {
        let ns = vec![2, 7, 11, 15];
        assert_eq!(Solution::two_sum(ns, 9), [1, 2]);
        // Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].
    }
    #[test]
    fn n_2_3_4_t_6() {
        let ns = vec![2, 3, 4];
        assert_eq!(Solution::two_sum(ns, 6), [1, 3]);
        // Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We return [1, 3].
    }
    #[test]
    fn n_m1_0_t_m1() {
        let ns = vec![-1, 0];
        assert_eq!(Solution::two_sum(ns, -1), [1, 2]);
        // Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We return [1, 2].
    }
}
