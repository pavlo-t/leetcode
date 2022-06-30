#![allow(dead_code)]
//! \#1564. Put Boxes Into the Warehouse I
//! ======================================
//!
//! You are given two arrays of positive integers, `boxes` and `warehouse`,
//! representing the heights of some boxes of unit width
//! and the heights of `n` rooms in a warehouse respectively.
//! The warehouse's rooms are labelled from `0` to `n - 1` from left to right
//! where `warehouse[i]` (`0`-indexed) is the height of the `i`<sup>th</sup> room.
//!
//! Boxes are put into the warehouse by the following rules:
//!
//! - Boxes cannot be stacked.
//! - You can rearrange the insertion order of the boxes.
//! - Boxes can only be pushed into the warehouse from left to right only.
//! - If the height of some room in the warehouse is less than the height of a box,
//!   then that box and all other boxes behind it will be stopped before that room.
//!
//! Return _the maximum number of boxes you can put into the warehouse_.
//!
//! __Constraints:__
//!
//! - `n == warehouse.length`
//! - `1 <= boxes.length, warehouse.length <= 100_000`
//! - `1 <= boxes[i], warehouse[i] <= 1_000_000_000`
//!
//! <https://leetcode.com/problems/put-boxes-into-the-warehouse-i>

pub struct Solution;
impl Solution {
    pub fn max_boxes_in_warehouse(mut boxes: Vec<i32>, mut warehouse: Vec<i32>) -> i32 {
        boxes.sort_unstable();

        let mut max_height = warehouse[0];
        for i in 1..warehouse.len() {
            max_height = max_height.min(warehouse[i]);
            warehouse[i] = max_height;
        }

        let mut result = 0;
        let mut i = warehouse.len() - 1;
        for b in boxes {
            while i < warehouse.len() && warehouse[i] < b {
                i = i.wrapping_sub(1);
            }
            if i >= warehouse.len() {
                break;
            } else {
                result += 1;
                i = i.wrapping_sub(1);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn b_4_3_4_1_w_5_3_3_4_1() {
        let b = vec![4, 3, 4, 1];
        let w = vec![5, 3, 3, 4, 1];
        assert_eq!(Solution::max_boxes_in_warehouse(b, w), 3);
        // Explanation:
        // We can first put the box of height 1 in room 4.
        // Then we can put the box of height 3 in either of the 3 rooms 1, 2, or 3.
        // Lastly, we can put one box of height 4 in room 0.
        // There is no way we can fit all 4 boxes in the warehouse.
    }
    #[test]
    fn b_1_2_2_3_4_w_3_4_1_2() {
        let b = vec![1, 2, 2, 3, 4];
        let w = vec![3, 4, 1, 2];
        assert_eq!(Solution::max_boxes_in_warehouse(b, w), 3);
        // Explanation:
        // Notice that it's not possible to put the box of height 4 into the warehouse
        // since it cannot pass the first room of height 3.
        // Also, for the last two rooms, 2 and 3, only boxes of height 1 can fit.
        // We can fit 3 boxes maximum: [2, 3, _, 1].
        // Swapping the 2 and 3 boxes is also valid.
    }
    #[test]
    fn b_1_2_3_w_1_2_3_4() {
        let b = vec![1, 2, 3];
        let w = vec![1, 2, 3, 4];
        assert_eq!(Solution::max_boxes_in_warehouse(b, w), 1);
        // Explanation: Since the first room in the warehouse is of height 1,
        // we can only put boxes of height 1.
    }
}
