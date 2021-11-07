#![allow(dead_code)]
/// 53. Maximum Subarray
/// ====================
///
/// See `d05_1.rs` in "Dynamic programming 1"
///
/// https://leetcode.com/problems/maximum-subarray/
struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut best, mut curr) = (nums[0], nums[0]);
        for n in nums.into_iter().skip(1) {
            curr = n + curr.max(0);
            best = best.max(curr);
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_m2p1m3p4m1p2p1m5p4() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(Solution::max_sub_array(nums), 6);
        // Explanation: [4,-1,2,1] has the largest sum = 6.
    }
    #[test]
    fn n_p5p4m1p7p8() {
        let nums = vec![5, 4, -1, 7, 8];
        assert_eq!(Solution::max_sub_array(nums), 23);
    }
    #[rustfmt::skip] #[test] fn n_p1() { assert_eq!(Solution::max_sub_array(vec![ 1]), 1); }
    #[rustfmt::skip] #[test] fn n_m1() { assert_eq!(Solution::max_sub_array(vec![-1]), -1); }

    #[rustfmt::skip] #[test] fn n_p1m1() { assert_eq!(Solution::max_sub_array(vec![ 1, -1]), 1); }
    #[rustfmt::skip] #[test] fn n_m1p1() { assert_eq!(Solution::max_sub_array(vec![-1,  1]), 1); }
    #[rustfmt::skip] #[test] fn n_p1p1() { assert_eq!(Solution::max_sub_array(vec![ 1,  1]), 2); }

    #[rustfmt::skip] #[test] fn n_m1m1m1() { assert_eq!(Solution::max_sub_array(vec![-1, -1, -1]), -1); }
    #[rustfmt::skip] #[test] fn n_m1p1m1() { assert_eq!(Solution::max_sub_array(vec![-1,  1, -1]),  1); }
    #[rustfmt::skip] #[test] fn n_p1m1p1() { assert_eq!(Solution::max_sub_array(vec![ 1, -1,  1]),  1); }
    #[rustfmt::skip] #[test] fn n_m1p1p1() { assert_eq!(Solution::max_sub_array(vec![-1,  1,  1]),  2); }
    #[rustfmt::skip] #[test] fn n_p1p1m1() { assert_eq!(Solution::max_sub_array(vec![ 1,  1, -1]),  2); }
    #[rustfmt::skip] #[test] fn n_p1p1p1() { assert_eq!(Solution::max_sub_array(vec![ 1,  1,  1]),  3); }

    #[test]
    fn n_100000x1() {
        let nums = vec![1; 100_000];
        assert_eq!(Solution::max_sub_array(nums), 100_000);
    }
}
