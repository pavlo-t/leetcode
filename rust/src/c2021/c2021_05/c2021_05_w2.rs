#![allow(dead_code)]
/// Put Boxes Into the Warehouse I
/// ==============================
///
/// You are given two arrays of positive integers, `boxes` and `warehouse`, representing the
/// heights of some boxes of unit width and the heights of `n` rooms in a warehouse respectively.
/// The warehouse's rooms are labelled from `0` to `n - 1` from left to right where
/// `warehouse[i]` (0-indexed) is the height of the `i`th room.
///
/// Boxes are put into the warehouse by the following rules:
///
/// - Boxes cannot be stacked.
/// - You can rearrange the insertion order of the boxes.
/// - Boxes can only be pushed into the warehouse from left to right only.
/// - If the height of some room in the warehouse is less than the height of a box,
///   then that box and all other boxes behind it will be stopped before that room.
///
/// Return _the maximum number of boxes you can put into the warehouse_.
///
/// __Constraints:__
///
/// - `1 <= boxes.length, warehouse.length <= 100_000`
/// - `1 <= boxes[i], warehouse[i] <= 1_000_000_000`
///
/// https://leetcode.com/explore/featured/card/may-leetcoding-challenge-2021/599/week-2-may-8th-may-14th/3735/
struct Solution;
impl Solution {
    pub fn max_boxes_in_warehouse(mut boxes: Vec<i32>, warehouse: Vec<i32>) -> i32 {
        boxes.sort_unstable();

        warehouse.into_iter().fold(0, |rsf, h| {
            while let Some(b) = boxes.pop() {
                if b <= h {
                    return rsf + 1;
                }
            }
            rsf
        })
    }

    pub fn max_boxes_in_warehouse_iter_mut(mut boxes: Vec<i32>, mut warehouse: Vec<i32>) -> i32 {
        boxes.sort_unstable();
        let mut min_height = warehouse[0];
        warehouse.iter_mut().for_each(|h| {
            min_height = min_height.min(*h);
            *h = min_height;
        });

        boxes.into_iter().fold(0, |rsf, b| {
            while let Some(h) = warehouse.pop() {
                if h >= b {
                    return rsf + 1;
                }
            }
            rsf
        })
    }

    pub fn max_boxes_in_warehouse_my_1_fold(mut boxes: Vec<i32>, warehouse: Vec<i32>) -> i32 {
        boxes.sort_unstable();
        let (mut wh, _) = warehouse
            .into_iter()
            .fold((vec![], i32::MAX), |(mut acc, min), h| {
                acc.push(min.min(h));
                (acc, min.min(h))
            });

        boxes.into_iter().fold(0, |rsf, b| {
            while let Some(h) = wh.pop() {
                if h >= b {
                    return rsf + 1;
                }
            }
            rsf
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let boxes = vec![4, 3, 4, 1];
        let warehouse = vec![5, 3, 3, 4, 1];
        assert_eq!(Solution::max_boxes_in_warehouse(boxes, warehouse), 3);
        // Explanation:
        //           _          ->      _
        //            |   _     ->       |   _
        // 0   2      |__| |    ->  0   2|__| |
        // 0 1 2           |    ->  0   2    1|
        // 0 1 2           |_   ->  0   2    1|_
        // 0 1 2 3           |  ->  0   2    1 3|
        //
        // We can first put the box of height 1 in room 4.
        // Then we can put the box of height 3 in either of the 3 rooms 1, 2, or 3.
        // Lastly, we can put one box of height 4 in room 0.
        // There is no way we can fit all 4 boxes in the warehouse.
    }
    #[test]
    fn example2() {
        let boxes = vec![1, 2, 2, 3, 4];
        let warehouse = vec![3, 4, 1, 2];
        assert_eq!(Solution::max_boxes_in_warehouse(boxes, warehouse), 3);
        // Explanation:
        //               _       ->          _
        //         4   _| |      ->    4   _| |
        //       3 4      |  _   ->    4   3  |  _
        //   1 2 3 4      |_| |  ->  1 4   3 2|_| |
        // 0 1 2 3 4          |  ->  1 4   3 2   0|
        //
        // Notice that it's not possible to put the box of height 4 into the warehouse since it
        // cannot pass the first room of height 3.
        // Also, for the last two rooms, 2 and 3, only boxes of height 1 can fit.
        // We can fit 3 boxes maximum as shown above.
        // Box 0 can also be put in room 2 instead.
        // Swapping boxes 2 and 3 is also valid, or swapping one of them with box 1.
    }
    #[test]
    fn example3() {
        let boxes = vec![1, 2, 3];
        let warehouse = vec![1, 2, 3, 4];
        assert_eq!(Solution::max_boxes_in_warehouse(boxes, warehouse), 1);
        // Explanation:
        // Since the first room in the warehouse is of height 1, we can only put boxes of height 1.
    }
    #[test]
    fn example4() {
        let boxes = vec![4, 5, 6];
        let warehouse = vec![3, 3, 3, 3, 3];
        assert_eq!(Solution::max_boxes_in_warehouse(boxes, warehouse), 0);
    }

    mod performance {
        use super::*;

        #[test]
        fn bs100k1s_wh100k1s_produces_100k() {
            let boxes = vec![1; 100_000];
            let warehouse = vec![1; 100_000];
            assert_eq!(Solution::max_boxes_in_warehouse(boxes, warehouse), 100_000);
        }
        #[test]
        fn bs100k2s_wh100k1s_produces_0() {
            let boxes = vec![2; 100_000];
            let warehouse = vec![1; 100_000];
            assert_eq!(Solution::max_boxes_in_warehouse(boxes, warehouse), 0);
        }
    }
}
