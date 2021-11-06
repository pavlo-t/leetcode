#![allow(dead_code)]
/// 260. Single Number III
/// ======================
///
/// Given an integer array `nums`,
/// in which exactly two elements appear only once and all the other elements appear exactly twice.
/// Find the two elements that appear only once.
/// You can return the answer in __any order__.
///
/// You must write an algorithm that runs in linear runtime complexity and uses only constant extra space.
///
/// __Constraints:__
///
/// - `2 <= nums.length <= 30_000`
/// - `-2**31 <= nums[i] <= 2**31 - 1`
/// - Each integer in `nums` will appear twice, only two integers will appear once.
///
/// https://leetcode.com/problems/single-number-iii/
struct Solution;
impl Solution {
    /// https://russianblogs.com/article/5765861971/
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        println!("single_number({:?})", nums);
        let xor = |x: i32, n: &i32| x ^ n;

        // `x = a ^ b`; numbers that appear twice are erradicated
        let x = nums.iter().fold(0, xor);

        // `a` and `b` differ at `k` binary position
        //let k = (0..=32).map(|k| 1 << k).find(|k| x & k != 0).unwrap();
        let k = x & (-x);

        // numbers that have `1` at `k` form `a` when XORed; those that have `0` - form `b`
        let a = nums.iter().filter(|&n| n & k != 0).fold(0, xor);
        let b = x ^ a;

        vec![a, b]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[rustfmt::skip]
    fn check(n: Vec<i32>, e: [i32; 2]) {
        let e = e.into_iter().collect::<HashSet<_>>();
        let r = Solution::single_number(n).into_iter().collect::<HashSet<_>>();
        assert_eq!(r, e);
    }

    #[rustfmt::skip] #[test] fn n_1_2_1_3_2_5() { check(vec![1, 2, 1, 3, 2, 5], [3, 5]); }
    #[rustfmt::skip] #[test]        fn n_m1p0() { check(vec![-1, 0]           , [-1, 0]); }
    #[rustfmt::skip] #[test]         fn n_0_1() { check(vec![0, 1]            , [1, 0]); }
}
