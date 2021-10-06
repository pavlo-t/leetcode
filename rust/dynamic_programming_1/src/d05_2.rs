#![allow(dead_code)]
/// 918. Maximum Sum Circular Subarray
/// ==================================
///
/// Given a __circular integer array__ `nums` of length `n`,
/// return _the maximum possible sum of a non-empty __subarray__ of `nums`_.
///
/// A __circular array__ means the end of the array connects to the beginning of the array.
/// Formally, the next element of `nums[i]` is `nums[(i + 1) % n]`
/// and the previous element of `nums[i]` is `nums[(i - 1 + n) % n]`.
///
/// A __subarray__ may only include each element of the fixed buffer `nums` at most once.
/// Formally, for a subarray `nums[i], nums[i + 1], ..., nums[j]`,
/// there does not exist `i <= k1, k2 <= j with k1 % n == k2 % n`.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 30_000`
/// - `-30_000 <= nums[i] <= 30_000`
///
/// https://leetcode.com/problems/maximum-sum-circular-subarray/
struct Solution;
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        println!("max_subarray_sum_circular({:?})", nums);
        let z = (i32::MIN, i32::MIN);
        let kadane = |(best, mut curr): (i32, i32), n| {
            curr = n + curr.max(0);
            (best.max(curr), curr)
        };
        let max_subarray_sum = nums.iter().map(|&n| n).fold(z, kadane).0;
        if max_subarray_sum < 0 {
            max_subarray_sum
        } else {
            let min_subarray_sum = nums.iter().map(|&n| -n).fold(z, kadane).0;
            let total = nums.iter().sum::<i32>();
            max_subarray_sum.max(total + min_subarray_sum)
        }
    }

    /// Approach 3: Kadane's (Sign Variant)
    /// https://leetcode.com/problems/maximum-sum-circular-subarray/solution/
    pub fn max_subarray_sum_circular_sign_variant(nums: Vec<i32>) -> i32 {
        println!("max_subarray_sum_circular({:?})", nums);
        fn kadane(ns: &[i32], sign: i32) -> i32 {
            println!("  kadane({:?})", ns);
            let (mut curr, mut result) = (i32::MIN, i32::MIN);
            for i in 0..ns.len() {
                curr = ns[i] * sign + curr.max(0);
                result = result.max(curr);
            }
            result
        }
        let s: i32 = nums.iter().sum();
        let res1 = kadane(&nums, 1);
        let res2 = s.saturating_add(kadane(&nums[1..], -1));
        let res3 = s.saturating_add(kadane(&nums[..nums.len() - 1], -1));
        res1.max(res2).max(res3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1m2p3m2() {
        let nums = vec![1, -2, 3, -2];
        assert_eq!(Solution::max_subarray_sum_circular(nums), 3);
        // Explanation: Subarray [3] has maximum sum 3
    }
    #[test]
    fn p5m3p5() {
        let nums = vec![5, -3, 5];
        assert_eq!(Solution::max_subarray_sum_circular(nums), 10);
        // Explanation: Subarray [5,5] has maximum sum 5 + 5 = 10
    }
    #[test]
    fn p3m1p2m1() {
        let nums = vec![3, -1, 2, -1];
        assert_eq!(Solution::max_subarray_sum_circular(nums), 4);
        // Explanation: Subarray [2,-1,3] has maximum sum 2 + (-1) + 3 = 4
    }
    #[test]
    fn p3m2p2m3() {
        let nums = vec![3, -2, 2, -3];
        assert_eq!(Solution::max_subarray_sum_circular(nums), 3);
        // Explanation: Subarray [3] and [3,-2,2] both have maximum sum 3
    }
    #[test]
    fn m2m3m1() {
        let nums = vec![-2, -3, -1];
        assert_eq!(Solution::max_subarray_sum_circular(nums), -1);
        // Explanation: Subarray [-1] has maximum sum -1
    }
    #[rustfmt::skip] #[test] fn n0() { assert_eq!(Solution::max_subarray_sum_circular(vec![ 0]),  0); }
    #[rustfmt::skip] #[test] fn p1() { assert_eq!(Solution::max_subarray_sum_circular(vec![ 1]),  1); }
    #[rustfmt::skip] #[test] fn m1() { assert_eq!(Solution::max_subarray_sum_circular(vec![-1]), -1); }
    #[rustfmt::skip] #[test] fn p1p1() { assert_eq!(Solution::max_subarray_sum_circular(vec![ 1, 1]), 2); }
    #[rustfmt::skip] #[test] fn m1p1() { assert_eq!(Solution::max_subarray_sum_circular(vec![-1, 1]), 1); }
    #[rustfmt::skip] #[test] fn p1m1() { assert_eq!(Solution::max_subarray_sum_circular(vec![ 1,-1]), 1); }
    #[rustfmt::skip] #[test] fn m1n0() { assert_eq!(Solution::max_subarray_sum_circular(vec![-1, 0]), 0); }
    #[rustfmt::skip] #[test] fn n0m1() { assert_eq!(Solution::max_subarray_sum_circular(vec![ 0,-1]), 0); }

    #[test]
    fn n_30000x1() {
        let nums = vec![1; 30000];
        assert_eq!(Solution::max_subarray_sum_circular(nums), 30000);
    }
    #[test]
    fn n_30000xm1() {
        let nums = vec![-1; 30000];
        assert_eq!(Solution::max_subarray_sum_circular(nums), -1);
    }
}
