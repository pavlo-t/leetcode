#![allow(dead_code)]
/// 198. House Robber
/// =================
///
/// You are a professional robber planning to rob houses along a street.
/// Each house has a certain amount of money stashed,
/// the only constraint stopping you from robbing each of them
/// is that adjacent houses have security systems connected
/// and __it will automatically contact the police if two adjacent houses were broken into on the same night__.
///
/// Given an integer array `nums` representing the amount of money of each house,
/// return _the maximum amount of money you can rob tonight __without alerting the police___.
///
/// __Constraints:__
///
///- `1 <= nums.length <= 100`
///- `0 <= nums[i] <= 400`
///
/// https://leetcode.com/problems/house-robber/
struct Solution;
impl Solution {
    /// 22:25-22:28
    pub fn rob(nums: Vec<i32>) -> i32 {
        println!("rob({:?})", nums);
        let (mut prev, mut curr) = (0, 0);
        for i in 0..nums.len() {
            std::mem::swap(&mut curr, &mut prev);
            curr = prev.max(nums[i] + curr);
        }
        curr
    }
    /// 22:23-22:25
    pub fn rob_dp_vec(nums: Vec<i32>) -> i32 {
        println!("rob({:?})", nums);
        let mut dp = vec![0; nums.len() + 2];
        for i in (0..nums.len()).rev() {
            dp[i] = dp[i + 1].max(nums[i] + dp[i + 2]);
        }
        dp[0].max(dp[1])
    }
    /// 22:20-22:23
    pub fn rob_rec_with_memo(nums: Vec<i32>) -> i32 {
        println!("rob({:?})", nums);
        fn rec(i: usize, ns: &[i32], memo: &mut Vec<i32>) -> i32 {
            if i >= ns.len() {
                0
            } else if memo[i] >= 0 {
                memo[i]
            } else {
                memo[i] = rec(i + 1, ns, memo).max(ns[i] + rec(i + 2, ns, memo));
                memo[i]
            }
        }
        let mut memo = vec![-1; nums.len()];
        rec(0, &nums, &mut memo).max(rec(1, &nums, &mut memo))
    }
    /// 22:14-22:19
    pub fn rob_rec(nums: Vec<i32>) -> i32 {
        println!("rob({:?})", nums);
        fn rec(i: usize, ns: &[i32]) -> i32 {
            if i >= ns.len() {
                0
            } else {
                rec(i + 1, ns).max(ns[i] + rec(i + 2, ns))
            }
        }
        rec(0, &nums).max(rec(1, &nums))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_3() { assert_eq!(Solution::rob(vec![3]), 3); }
    #[test]
    fn n_1_2_3_1() {
        let n = vec![1, 2, 3, 1];
        assert_eq!(Solution::rob(n), 4);
        // Output: 4
        // Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
        // Total amount you can rob = 1 + 3 = 4.
    }
    #[test]
    fn n_2_7_9_3_1() {
        let n = vec![2, 7, 9, 3, 1];
        assert_eq!(Solution::rob(n), 12);
        // Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
        // Total amount you can rob = 2 + 9 + 1 = 12.
    }
    #[rustfmt::skip] #[test] fn n_1_repeat_100() { assert_eq!(Solution::rob(vec![1; 100]), 50); }
}
