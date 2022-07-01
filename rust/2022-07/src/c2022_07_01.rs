#![allow(dead_code)]
//! \#1710. Maximum Units on a Truck
//! ================================
//!
//! You are assigned to put some amount of boxes onto __one truck__.
//! You are given a 2D array `boxTypes`, where `boxTypes[i] = [numberOfBoxes_i, numberOfUnitsPerBox_i]`:
//!
//! - `numberOfBoxes_i` is the number of boxes of type `i`.
//! - `numberOfUnitsPerBox_i` is the number of units in each box of the type `i`.
//!
//! You are also given an integer `truckSize`,
//! which is the __maximum__ number of __boxes__ that can be put on the truck.
//! You can choose any boxes to put on the truck as long as the number of boxes does not exceed `truckSize`.
//!
//! Return _the __maximum__ total number of __units__ that can be put on the truck_.
//!
//! __Constraints:__
//!
//! - `1 <= boxTypes.length <= 1000`
//! - `1 <= numberOfBoxes_i, numberOfUnitsPerBox_i <= 1000`
//! - `1 <= truckSize <= 1_000_000`
//!
//! <https://leetcode.com/problems/maximum-units-on-a-truck>

pub struct Solution;
impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let mut result = 0;
        while truck_size > 0 && !box_types.is_empty() {
            let (number, units_per_box) = box_types.pop().map(|b| (b[0], b[1])).unwrap();
            let can_fit = number.min(truck_size);
            truck_size -= can_fit;
            result += can_fit * units_per_box;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn b_n1u3_n2u2_n3u1_t_4() {
        let b = vv![[1, 3], [2, 2], [3, 1]];
        assert_eq!(Solution::maximum_units(b, 4), 8);
        // Explanation: There are:
        // - 1 box of the first type that contains 3 units.
        // - 2 boxes of the second type that contain 2 units each.
        // - 3 boxes of the third type that contain 1 unit each.
        // You can take all the boxes of the first and second types, and one box of the third type.
        // The total number of units will be = (1 * 3) + (2 * 2) + (1 * 1) = 8.
    }
    #[test]
    fn b_n5u10_n2u5_n4u7_n3u9_t_10() {
        let b = vv![[5, 10], [2, 5], [4, 7], [3, 9]];
        assert_eq!(Solution::maximum_units(b, 10), 91);
    }
}
