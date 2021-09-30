#![allow(dead_code)]
/// Maximum Units on a Truck
/// ========================
///
/// You are assigned to put some amount of boxes onto __one truck__.
/// You are given a 2D array `boxTypes`,
/// where `boxTypes[i] = [numberOfBoxes_i, numberOfUnitsPerBox_i]`:
///
/// - `numberOfBoxes_i` is the number of boxes of type `i`.
/// - `numberOfUnitsPerBox_i` is the number of units in each box of the type `i`.
///
/// You are also given an integer `truckSize`,
/// which is the __maximum__ number of __boxes__ that can be put on the truck.
/// You can choose any boxes to put on the truck as long as the number of boxes does not exceed `truckSize`.
///
/// Return _the __maximum__ total number of __units__ that can be put on the truck_.
///
/// __Constraints:__
///
/// - `1 <= boxTypes.length <= 1000`
/// - `1 <= numberOfBoxes_i, numberOfUnitsPerBox_i <= 1000`
/// - `1 <= truckSize <= 1_000_000`
///
/// https://leetcode.com/explore/featured/card/june-leetcoding-challenge-2021/604/week-2-june-8th-june-14th/3778/
struct Solution;
impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_unstable_by_key(|b| b[1]);

        let mut result = 0;
        while let Some(b) = box_types.pop() {
            if truck_size > 0 {
                result += b[0].min(truck_size) * b[1];
                truck_size -= b[0];
            } else {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn bt_1u3_2u2_3u1_ts_4_produces_8() {
        let box_types = vv![[1, 3], [2, 2], [3, 1]];
        let truck_size = 4;
        assert_eq!(Solution::maximum_units(box_types, truck_size), 8);
        // Explanation: There are:
        // - 1 box of the first type that contains 3 units.
        // - 2 boxes of the second type that contain 2 units each.
        // - 3 boxes of the third type that contain 1 unit each.
        // You can take all the boxes of the first and second types, and one box of the third type.
        // The total number of units will be = (1 * 3) + (2 * 2) + (1 * 1) = 8.
    }
    #[test]
    fn bt_5u10_2u5_4u7_3u9_ts_10_produces_91() {
        let box_types = vv![[5, 10], [2, 5], [4, 7], [3, 9]];
        let truck_size = 10;
        assert_eq!(Solution::maximum_units(box_types, truck_size), 91);
    }

    #[test]
    fn bt_5u10_ts_10_produces_50() {
        let mut str = String::new();
        str.push_str("[[1000,1000]");
        for _ in 1..1000 {
            str.push_str(",[1000,1000]")
        }
        str.push(']');
        println!("{}", str);
        let box_types = vv![[5, 10]];
        let truck_size = 10;
        assert_eq!(Solution::maximum_units(box_types, truck_size), 50);
    }
    #[test]
    fn bt_1000u1000x1000_ts_1_000_000_produces_1_000_000_000() {
        let box_types = vec![vec![1000, 1000]; 1000];
        let truck_size = 1_000_000;
        assert_eq!(
            Solution::maximum_units(box_types, truck_size),
            1_000_000_000
        );
    }
}
