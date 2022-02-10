#![allow(dead_code, non_snake_case)]
/// 1274. Number of Ships in a Rectangle
/// ====================================
///
/// (_This problem is an __interactive problem___.)
///
/// Each ship is located at an integer point on the sea represented by a cartesian plane,
/// and each integer point may contain at most 1 ship.
///
/// You have a function `Sea.hasShips(topRight, bottomLeft)` which takes two points as arguments
/// and returns `true` if there is at least one ship in the rectangle represented by the two points,
/// including on the boundary.
///
/// Given two points: the top right and bottom left corners of a rectangle,
/// return the number of ships present in that rectangle.
/// It is guaranteed that there are __at most 10 ships__ in that rectangle.
///
/// Submissions making __more than 400 calls__ to `hasShips` will be judged _Wrong Answer_.
/// Also, any solutions that attempt to circumvent the judge will result in disqualification.
///
/// __Constraints:__
///
/// - On the input `ships` is only given to initialize the map internally. You must solve this problem "blindfolded".
///   In other words, you must find the answer using the given `hasShips` API, without knowing the `ships` position.
/// - `0 <= bottomLeft[0] <= topRight[0] <= 1000`
/// - `0 <= bottomLeft[1] <= topRight[1] <= 1000`
/// - `topRight != bottomLeft`
///
/// https://leetcode.com/problems/number-of-ships-in-a-rectangle/
struct Solution;
impl Solution {
    pub fn count_ships(sea: &Sea, top_right: Vec<i32>, bottom_left: Vec<i32>) -> i32 {
        let (x0, y0) = (bottom_left[0], bottom_left[1]);
        let (x3, y3) = (top_right[0], top_right[1]);
        let (x1, y1) = (x0 + (x3 - x0) / 2, y0 + (y3 - y0) / 2);
        let (x2, y2) = (x1 + 1, y1 + 1);
        use std::iter::once;

        once((x1, y1, x0, y0))
            .chain(once((x1, y3, x0, y2)))
            .chain(once((x3, y1, x2, y0)))
            .chain(once((x3, y3, x2, y2)))
            .filter(|&(right, top, left, bottom)| {
                sea.hasShips(vec![right, top], vec![left, bottom])
            })
            .map(|(right, top, left, bottom)| {
                if right == left && top == bottom {
                    1
                } else {
                    Self::count_ships(sea, vec![right, top], vec![left, bottom])
                }
            })
            .sum::<i32>()
    }
}

use std::cell::RefCell;
/// This is Sea's API interface.
/// You should not implement it, or speculate about its implementation
#[derive(Debug)]
struct Sea {
    ships: Vec<[usize; 2]>,
    calls_remaining: RefCell<usize>,
}
impl Sea {
    #[rustfmt::skip]    fn new(ships: Vec<[usize; 2]>) -> Self { Self { ships, calls_remaining: RefCell::new(400) } }

    pub fn hasShips(&self, top_right: Vec<i32>, bottom_left: Vec<i32>) -> bool {
        //println!("hasShips({top_right:?}, {bottom_left:?})");
        *self.calls_remaining.borrow_mut() -= 1;
        let (x0, x1) = (bottom_left[0] as usize, top_right[0] as usize);
        let (y0, y1) = (bottom_left[1] as usize, top_right[1] as usize);
        self.ships
            .iter()
            .any(|&[x, y]| x0 <= x && x <= x1 && y0 <= y && y <= y1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s_1y1_2y2_3y3_5y5_tr_4_4_bl_0_0() {
        let s = Sea::new(vec![[1, 1], [2, 2], [3, 3], [5, 5]]);
        assert_eq!(Solution::count_ships(&s, vec![4, 4], vec![0, 0]), 3);
        // Explanation: From [0,0] to [4,4] we can count 3 ships within the range.
    }
    #[test]
    fn s_1y1_2y2_3y3_tr_1000_1000_bl_0_0() {
        let s = Sea::new(vec![[1, 1], [2, 2], [3, 3]]);
        assert_eq!(Solution::count_ships(&s, vec![1000, 1000], vec![0, 0]), 3);
    }
    #[test]
    fn s_0y0_1000y1000_0y1000_1000y0_0y500_500y0_500y500_500y1000_1000y500_501y501_tr_1000_1000_bl_0_0(
    ) {
        let s = vec![
            [0, 0],
            [1000, 1000],
            [0, 1000],
            [1000, 0],
            [0, 500],
            [500, 0],
            [500, 500],
            [500, 1000],
            [1000, 500],
            [501, 501],
        ];
        let s = Sea::new(s);
        assert_eq!(Solution::count_ships(&s, vec![1000, 1000], vec![0, 0]), 10);
    }
}
