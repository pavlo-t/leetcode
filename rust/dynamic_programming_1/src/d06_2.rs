#![allow(dead_code)]
/// 1567. Maximum Length of Subarray With Positive Product
/// ======================================================
///
/// Given an array of integers `nums`,
/// find the maximum length of a subarray where the product of all its elements is positive.
///
/// A subarray of an array is a consecutive sequence of zero or more values taken out of that array.
///
/// Return _the maximum length of a subarray with positive product_.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 100_000`
/// - `-10^9 <= nums[i] <= 10^9`
///
/// https://leetcode.com/problems/maximum-length-of-subarray-with-positive-product/
struct Solution;
impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        //println!("get_max_len({:?})", nums);
        let mut result = 0;
        let mut pos = 0;
        let mut neg = 0;
        for i in 0..nums.len() {
            if nums[i] > 0 {
                pos = pos + 1;
                neg = if neg > 0 { neg + 1 } else { 0 };
            } else if nums[i] < 0 {
                let prev_pos = pos;
                pos = if neg > 0 { neg + 1 } else { 0 };
                neg = prev_pos + 1;
            } else {
                pos = 0;
                neg = 0;
            }
            result = result.max(pos);
        }
        result
    }
    pub fn get_max_len_dp_vector(nums: Vec<i32>) -> i32 {
        //println!("get_max_len({:?})", nums);
        let mut dp = vec![(0, 0); nums.len() + 1];
        for i in 0..nums.len() {
            let (pp, pn) = dp[i];
            if nums[i] > 0 {
                dp[i + 1].0 = pp + 1;
                dp[i + 1].1 = if pn == 0 { 0 } else { pn + 1 };
            } else if nums[i] < 0 {
                dp[i + 1].0 = if pn == 0 { 0 } else { pn + 1 };
                dp[i + 1].1 = pp + 1;
            }
        }
        dp.into_iter().map(|(p, _)| p).max().unwrap_or(0)
    }
    pub fn get_max_len_brute_force_improved(nums: Vec<i32>) -> i32 {
        println!("get_max_len({:?})", nums);
        let mut result = 0;
        for l in 0..nums.len() {
            let mut len = 0;
            let mut positive = true;
            for r in l..nums.len() {
                if nums[r] == 0 {
                    break;
                }
                len += 1;
                if nums[r] < 0 {
                    positive = !positive;
                }
                if positive {
                    result = result.max(len);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1m2m3p4() {
        let nums = vec![1, -2, -3, 4];
        assert_eq!(Solution::get_max_len(nums), 4);
        // Explanation: The array nums already has a positive product of 24.
    }
    #[test]
    fn n0p1m2m3m4() {
        let nums = vec![0, 1, -2, -3, -4];
        assert_eq!(Solution::get_max_len(nums), 3);
        // Explanation: The longest subarray with positive product is [1,-2,-3] which has a product of 6.
        // Notice that we cannot include 0 in the subarray since that'll make the product 0 which is not positive.
    }
    #[test]
    fn m1m2m3n0p1() {
        let nums = vec![-1, -2, -3, 0, 1];
        assert_eq!(Solution::get_max_len(nums), 2);
        // Explanation: The longest subarray with positive product is [-1,-2] or [-2,-3].
    }
    #[test]
    fn m1p2() {
        let nums = vec![-1, 2];
        assert_eq!(Solution::get_max_len(nums), 1);
    }
    #[test]
    fn p1p2p3p5m6p4n0p10() {
        let nums = vec![1, 2, 3, 5, -6, 4, 0, 10];
        assert_eq!(Solution::get_max_len(nums), 4);
    }

    #[rustfmt::skip] #[test] fn m1() { assert_eq!(Solution::get_max_len(vec![-1]), 0); }
    #[rustfmt::skip] #[test] fn n0() { assert_eq!(Solution::get_max_len(vec![ 0]), 0); }
    #[rustfmt::skip] #[test] fn p1() { assert_eq!(Solution::get_max_len(vec![ 1]), 1); }

    #[rustfmt::skip] #[test] fn m1m1() { assert_eq!(Solution::get_max_len(vec![-1,-1]), 2); }
    #[rustfmt::skip] #[test] fn m1n0() { assert_eq!(Solution::get_max_len(vec![-1, 0]), 0); }
    #[rustfmt::skip] #[test] fn m1p1() { assert_eq!(Solution::get_max_len(vec![-1, 1]), 1); }
    #[rustfmt::skip] #[test] fn n0m1() { assert_eq!(Solution::get_max_len(vec![ 0,-1]), 0); }
    #[rustfmt::skip] #[test] fn n0n0() { assert_eq!(Solution::get_max_len(vec![ 0, 0]), 0); }
    #[rustfmt::skip] #[test] fn n0p1() { assert_eq!(Solution::get_max_len(vec![ 0, 1]), 1); }
    #[rustfmt::skip] #[test] fn p1m1() { assert_eq!(Solution::get_max_len(vec![ 1,-1]), 1); }
    #[rustfmt::skip] #[test] fn p1n0() { assert_eq!(Solution::get_max_len(vec![ 1, 0]), 1); }
    #[rustfmt::skip] #[test] fn p1p1() { assert_eq!(Solution::get_max_len(vec![ 1, 1]), 2); }

    #[rustfmt::skip] #[test] fn m1m1m1() { assert_eq!(Solution::get_max_len(vec![-1,-1,-1]), 2); }
    #[rustfmt::skip] #[test] fn m1m1n0() { assert_eq!(Solution::get_max_len(vec![-1,-1, 0]), 2); }
    #[rustfmt::skip] #[test] fn m1m1p1() { assert_eq!(Solution::get_max_len(vec![-1,-1, 1]), 3); }
    #[rustfmt::skip] #[test] fn m1n0m1() { assert_eq!(Solution::get_max_len(vec![-1, 0,-1]), 0); }
    #[rustfmt::skip] #[test] fn m1n0n0() { assert_eq!(Solution::get_max_len(vec![-1, 0, 0]), 0); }
    #[rustfmt::skip] #[test] fn m1n0p1() { assert_eq!(Solution::get_max_len(vec![-1, 0, 1]), 1); }
    #[rustfmt::skip] #[test] fn m1p1m1() { assert_eq!(Solution::get_max_len(vec![-1, 1,-1]), 3); }
    #[rustfmt::skip] #[test] fn m1p1n0() { assert_eq!(Solution::get_max_len(vec![-1, 1, 0]), 1); }
    #[rustfmt::skip] #[test] fn m1p1p1() { assert_eq!(Solution::get_max_len(vec![-1, 1, 1]), 2); }

    #[rustfmt::skip] #[test] fn n0m1m1() { assert_eq!(Solution::get_max_len(vec![ 0,-1,-1]), 2); }
    #[rustfmt::skip] #[test] fn n0m1n0() { assert_eq!(Solution::get_max_len(vec![ 0,-1, 0]), 0); }
    #[rustfmt::skip] #[test] fn n0m1p1() { assert_eq!(Solution::get_max_len(vec![ 0,-1, 1]), 1); }
    #[rustfmt::skip] #[test] fn n0n0m1() { assert_eq!(Solution::get_max_len(vec![ 0, 0,-1]), 0); }
    #[rustfmt::skip] #[test] fn n0n0n0() { assert_eq!(Solution::get_max_len(vec![ 0, 0, 0]), 0); }
    #[rustfmt::skip] #[test] fn n0n0p1() { assert_eq!(Solution::get_max_len(vec![ 0, 0, 1]), 1); }
    #[rustfmt::skip] #[test] fn n0p1m1() { assert_eq!(Solution::get_max_len(vec![ 0, 1,-1]), 1); }
    #[rustfmt::skip] #[test] fn n0p1n0() { assert_eq!(Solution::get_max_len(vec![ 0, 1, 0]), 1); }
    #[rustfmt::skip] #[test] fn n0p1p1() { assert_eq!(Solution::get_max_len(vec![ 0, 1, 1]), 2); }

    #[rustfmt::skip] #[test] fn p1m1m1() { assert_eq!(Solution::get_max_len(vec![ 1,-1,-1]), 3); }
    #[rustfmt::skip] #[test] fn p1m1n0() { assert_eq!(Solution::get_max_len(vec![ 1,-1, 0]), 1); }
    #[rustfmt::skip] #[test] fn p1m1p1() { assert_eq!(Solution::get_max_len(vec![ 1,-1, 1]), 1); }
    #[rustfmt::skip] #[test] fn p1n0m1() { assert_eq!(Solution::get_max_len(vec![ 1, 0,-1]), 1); }
    #[rustfmt::skip] #[test] fn p1n0n0() { assert_eq!(Solution::get_max_len(vec![ 1, 0, 0]), 1); }
    #[rustfmt::skip] #[test] fn p1n0p1() { assert_eq!(Solution::get_max_len(vec![ 1, 0, 1]), 1); }
    #[rustfmt::skip] #[test] fn p1p1m1() { assert_eq!(Solution::get_max_len(vec![ 1, 1,-1]), 2); }
    #[rustfmt::skip] #[test] fn p1p1n0() { assert_eq!(Solution::get_max_len(vec![ 1, 1, 0]), 2); }
    #[rustfmt::skip] #[test] fn p1p1p1() { assert_eq!(Solution::get_max_len(vec![ 1, 1, 1]), 3); }

    //#[ignore]
    #[test]
    fn ns_100000x0() {
        let nums = vec![0; 100_000];
        assert_eq!(Solution::get_max_len(nums), 0);
    }
    //#[ignore]
    #[test]
    fn ns_100000x1() {
        let nums = vec![1; 100_000];
        assert_eq!(Solution::get_max_len(nums), 100_000);
    }
}
