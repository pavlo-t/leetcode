#![allow(dead_code)]
/// 1746. Maximum Subarray Sum After One Operation
/// ==============================================
///
/// You are given an integer array `nums`.
/// You must perform __exactly one__ operation where you can __replace__ one element `nums[i]` with `nums[i] * nums[i]`.
///
/// Return _the __maximum__ possible subarray sum after __exactly one__ operation_.
/// The subarray must be non-empty.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 100_000`
/// - `-10_000 <= nums[i] <= 10_000`
///
/// https://leetcode.com/problems/maximum-subarray-sum-after-one-operation/
struct Solution;
impl Solution {
    /// 20:30-20:50
    pub fn max_sum_after_operation_rec(nums: Vec<i32>) -> i32 {
        println!("max_sum_after_operation({:?})", nums);
        fn rec(i: usize, ns: &[i32]) -> (i32, i32) {
            if i == ns.len() {
                (0, 0)
            } else {
                let (prev_o, prev_n) = rec(i + 1, ns);
                let (o, n) = (ns[i] * ns[i], ns[i]);
                (o.max(o + prev_n).max(n + prev_o), n.max(n + prev_n))
            }
        }
        (0..nums.len()).map(|i| rec(i, &nums).0).max().unwrap()
    }
    /// 20:50-20:54
    pub fn max_sum_after_operation_rec_with_memo(nums: Vec<i32>) -> i32 {
        println!("max_sum_after_operation({:?})", nums);
        fn rec(i: usize, ns: &[i32], memo: &mut Vec<[i32; 2]>) -> [i32; 2] {
            if i == ns.len() {
                [0, 0]
            } else if memo[i][0] != i32::MIN {
                memo[i]
            } else {
                let [prev_o, prev_n] = rec(i + 1, ns, memo);
                let (o, n) = (ns[i] * ns[i], ns[i]);
                memo[i] = [o.max(o + prev_n).max(n + prev_o), n.max(n + prev_n)];
                memo[i]
            }
        }
        let mut memo = vec![[i32::MIN; 2]; nums.len()];
        (0..nums.len())
            .map(|i| rec(i, &nums, &mut memo)[0])
            .max()
            .unwrap()
    }
    /// 20:54-20:58
    pub fn max_sum_after_operation_dp_vec(nums: Vec<i32>) -> i32 {
        println!("max_sum_after_operation({:?})", nums);
        let mut dp = vec![[0; 2]; nums.len() + 1];
        let mut result = i32::MIN;
        for i in (0..nums.len()).rev() {
            let n = nums[i];
            let o = n * n;
            dp[i][0] = o.max(o + dp[i + 1][1]).max(n + dp[i + 1][0]);
            dp[i][1] = n.max(n + dp[i + 1][1]);
            result = result.max(dp[i][0]);
        }
        result
    }
    /// 20:58-21:01
    pub fn max_sum_after_operation(nums: Vec<i32>) -> i32 {
        println!("max_sum_after_operation({:?})", nums);
        let (mut do_op, mut no_op) = (0, 0);
        let mut result = i32::MIN;
        for n in nums {
            let o = n * n;
            do_op = o.max(o + no_op).max(n + do_op);
            no_op = n.max(n + no_op);
            result = result.max(do_op);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn p0() { assert_eq!(Solution::max_sum_after_operation(vec![ 0]), 0); }
    #[rustfmt::skip] #[test] fn m1() { assert_eq!(Solution::max_sum_after_operation(vec![-1]), 1); }
    #[rustfmt::skip] #[test] fn p1() { assert_eq!(Solution::max_sum_after_operation(vec![ 1]), 1); }
    #[test]
    fn p10m8m8m8m8m8p9p9p9p9p9() {
        let n = vec![10, -8, -8, -8, -8, -8, 9, 9, 9, 9, 9];
        assert_eq!(Solution::max_sum_after_operation(n), 117);
    }
    #[test]
    fn p2m1m4m3() {
        let n = vec![2, -1, -4, -3];
        assert_eq!(Solution::max_sum_after_operation(n), 17);
        // Explanation: You can perform the operation on index 2 (0-indexed) to make nums = [2,-1,16,-3].
        // Now, the maximum subarray sum is 2 + -1 + 16 = 17.
    }
    #[test]
    fn p1m1p1p1m1m1p1() {
        let n = vec![1, -1, 1, 1, -1, -1, 1];
        assert_eq!(Solution::max_sum_after_operation(n), 4);
        // Explanation: You can perform the operation on index 1 (0-indexed) to make nums = [1,1,1,1,-1,-1,1].
        // Now, the maximum subarray sum is 1 + 1 + 1 + 1 = 4.
    }

    // If getting stack overflow:
    // Add `RUST_MIN_STACK=67108864` to env:
    // RUST_MIN_STACK=67108864 cargo test --lib d06_1
    #[test]
    fn ns_50000_x_p1() {
        let n = vec![1; 50000];
        assert_eq!(Solution::max_sum_after_operation(n), 50000);
    }
    #[test]
    fn ns_50000_x_m10000() {
        let n = vec![-10000; 50000];
        assert_eq!(Solution::max_sum_after_operation(n), 100_000_000);
    }
}
