#![allow(dead_code)]
//! \#1465. Maximum Area of a Piece of Cake After Horizontal and Vertical Cuts
//! ==========================================================================
//!
//! You are given a rectangular cake of size `h x w` and
//! two arrays of integers `horizontalCuts` and `verticalCuts` where:
//!
//! - `horizontalCuts[i]` is the distance from the top of the rectangular cake to the `i`th horizontal cut
//! - `verticalCuts[j]` is the distance from the left of the rectangular cake to the `j`th vertical cut
//!
//! Return _the maximum area of a piece of cake after you cut at each horizontal and
//! vertical position provided in the arrays `horizontalCuts` and `verticalCuts`_.
//! Since the answer can be a large number, return this __modulo__ `1_000_000_007`.
//!
//! __Constraints:__
//!
//! - `2 <= h, w <= 1_000_000_000`
//! - `1 <= horizontalCuts.length <= min(h - 1, 100_000)`
//! - `1 <= verticalCuts.length <= min(w - 1, 100_000)`
//! - `1 <= horizontalCuts[i] < h`
//! - `1 <= verticalCuts[i] < w`
//! - All the elements in `horizontalCuts` are distinct.
//! - All the elements in `verticalCuts` are distinct.
//!
//! <https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts>

pub struct Solution;
impl Solution {
    pub fn max_area(h: i32, w: i32, mut hc: Vec<i32>, mut vc: Vec<i32>) -> i32 {
        hc.push(0);
        hc.push(h);
        hc.sort_unstable();
        vc.push(0);
        vc.push(w);
        vc.sort_unstable();
        let max_h = hc.windows(2).map(|w| w[1] - w[0]).max().unwrap() as i64;
        let max_v = vc.windows(2).map(|w| w[1] - w[0]).max().unwrap() as i64;

        ((max_h * max_v) % 1_000_000_007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h5_w4_hc_1_2_4_vc_1_3() {
        assert_eq!(Solution::max_area(5, 4, vec![1, 2, 4], vec![1, 3]), 4);
        //  0   1   2   3   4
        // 0    |       |
        //      |       |
        // 1----+-------+----
        //      |       |
        // 2----+-------+----
        //      | x   x |
        // 3    |       |
        //      | x   x |
        // 4----+-------+----
        //      |       |
        // 5    |       |
        // Explanation: The figure above represents the given rectangular cake.
        // After you cut the cake, the marked of cake has the maximum area.
    }
    #[test]
    fn h5_w4_hc_3_1_vc_1() {
        assert_eq!(Solution::max_area(5, 4, vec![3, 1], vec![1]), 6);
        //  0   1   2   3   4
        // 0    |
        //      |
        // 1----+------------
        //      | x   x   x
        // 2    |
        //      | x   x   x
        // 3----+------------
        //      | y   y   y
        // 4    |
        //      | y   y   y
        // 5    |
        // Explanation: The figure above represents the given rectangular cake.
        // After you cut the cake, the marked pieces of cake have the maximum area.
    }
    #[test]
    fn h5_w4_hc_3_vc_3() {
        assert_eq!(Solution::max_area(5, 4, vec![3], vec![3]), 9);
    }
}
