#![allow(dead_code)]
/// 605. Can Place Flowers
/// ======================
///
/// You have a long flowerbed in which some of the plots are planted, and some are not.
/// However, flowers cannot be planted in __adjacent__ plots.
///
/// Given an integer array `flowerbed` containing `0`'s and `1`'s, where `0` means empty and `1` means not empty,
/// and an integer `n`,
/// return if `n` new flowers can be planted in the `flowerbed` without violating the no-adjacent-flowers rule.
///
/// __Constraints:__
///
/// - `1 <= flowerbed.length <= 20_000`
/// - `flowerbed[i]` is `0` or `1`.
/// - There are no two adjacent flowers in `flowerbed`.
/// - `0 <= n <= flowerbed.length`
///
/// https://leetcode.com/problems/can-place-flowers/
struct Solution;
impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        println!("can_place_flowers({:?}, {})", flowerbed, n);
        if flowerbed.len() < 2 {
            n == 0 || (n == 1 && flowerbed[0] == 0)
        } else {
            let l = flowerbed.len();

            if flowerbed[0] == 0 && flowerbed[1] == 0 {
                flowerbed[0] = 1;
                n -= 1;
            }
            for i in 1..l - 1 {
                if flowerbed[i - 1] == 0 && flowerbed[i] == 0 && flowerbed[i + 1] == 0 {
                    flowerbed[i] = 1;
                    n -= 1;
                }
            }
            if flowerbed[l - 2] == 0 && flowerbed[l - 1] == 0 {
                flowerbed[0] = 1;
                n -= 1;
            }
            n <= 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn f_0_n_0() { assert!( Solution::can_place_flowers(vec![0], 0)); }
    #[rustfmt::skip] #[test] fn f_1_n_0() { assert!( Solution::can_place_flowers(vec![1], 0)); }
    #[rustfmt::skip] #[test] fn f_0_n_1() { assert!( Solution::can_place_flowers(vec![0], 1)); }
    #[rustfmt::skip] #[test] fn f_0_n_2() { assert!(!Solution::can_place_flowers(vec![0], 2)); }
    #[rustfmt::skip] #[test] fn f_1_n_1() { assert!(!Solution::can_place_flowers(vec![1], 1)); }

    #[rustfmt::skip] #[test] fn f_00_n_0() { assert!( Solution::can_place_flowers(vec![0,0], 0)); }
    #[rustfmt::skip] #[test] fn f_01_n_0() { assert!( Solution::can_place_flowers(vec![0,1], 0)); }
    #[rustfmt::skip] #[test] fn f_10_n_0() { assert!( Solution::can_place_flowers(vec![1,0], 0)); }
    #[rustfmt::skip] #[test] fn f_00_n_1() { assert!( Solution::can_place_flowers(vec![0,0], 1)); }
    #[rustfmt::skip] #[test] fn f_01_n_1() { assert!(!Solution::can_place_flowers(vec![0,1], 1)); }
    #[rustfmt::skip] #[test] fn f_10_n_1() { assert!(!Solution::can_place_flowers(vec![1,0], 1)); }

    #[rustfmt::skip] #[test] fn f_000_n_0() { assert!( Solution::can_place_flowers(vec![0,0,0], 0)); }
    #[rustfmt::skip] #[test] fn f_001_n_0() { assert!( Solution::can_place_flowers(vec![0,0,1], 0)); }
    #[rustfmt::skip] #[test] fn f_010_n_0() { assert!( Solution::can_place_flowers(vec![0,1,0], 0)); }
    #[rustfmt::skip] #[test] fn f_100_n_0() { assert!( Solution::can_place_flowers(vec![1,0,0], 0)); }
    #[rustfmt::skip] #[test] fn f_101_n_0() { assert!( Solution::can_place_flowers(vec![1,0,1], 0)); }
    #[rustfmt::skip] #[test] fn f_000_n_1() { assert!( Solution::can_place_flowers(vec![0,0,0], 1)); }
    #[rustfmt::skip] #[test] fn f_000_n_2() { assert!( Solution::can_place_flowers(vec![0,0,0], 2)); }
    #[rustfmt::skip] #[test] fn f_000_n_3() { assert!(!Solution::can_place_flowers(vec![0,0,0], 3)); }

    #[rustfmt::skip] #[test] fn f_10001_n_1() { assert!( Solution::can_place_flowers(vec![1,0,0,0,1], 1)); }
    #[rustfmt::skip] #[test] fn f_10001_n_2() { assert!(!Solution::can_place_flowers(vec![1,0,0,0,1], 2)); }
}
