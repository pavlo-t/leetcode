#![allow(dead_code)]
/// 198. House Robber
/// =================
///
/// You are a professional robber planning to rob houses along a street.
/// Each house has a certain amount of money stashed,
/// the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected
/// and __it will automatically contact the police if two adjacent houses were broken into on the same night__.
///
/// Given an integer array `nums` representing the amount of money of each house,
/// return _the maximum amount of money you can rob tonight __without alerting the police___.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 100`
/// - `0 <= nums[i] <= 400`
///
/// https://leetcode.com/problems/house-robber/
struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut a, mut b) = (0, 0);
        for i in 0..nums.len() {
            std::mem::swap(&mut a, &mut b);
            a = b.max(nums[i] + a);
        }
        a
    }
    pub fn rob_dp(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 2];
        for i in (0..nums.len()).rev() {
            dp[i] = dp[i + 1].max(nums[i] + dp[i + 2])
        }
        dp[0]
    }
    pub fn rob_dfs_with_memo(nums: Vec<i32>) -> i32 {
        fn dfs(i: usize, memo: &mut Vec<i32>, ns: &[i32]) -> i32 {
            if i >= ns.len() {
                0
            } else if memo[i] >= 0 {
                memo[i]
            } else {
                memo[i] = (ns[i] + dfs(i + 2, memo, ns)).max(dfs(i + 1, memo, ns));
                memo[i]
            }
        }
        dfs(0, &mut vec![-1; nums.len()], &nums)
    }
    pub fn rob_dfs(nums: Vec<i32>) -> i32 {
        fn dfs(i: usize, ns: &[i32]) -> i32 {
            if i >= ns.len() {
                0
            } else {
                (ns[i] + dfs(i + 2, ns)).max(dfs(i + 1, ns))
            }
        }
        dfs(0, &nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_1_2_3_1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::rob(nums), 4);
        // Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
        // Total amount you can rob = 1 + 3 = 4.
    }
    #[test]
    fn n_2_7_9_3_1() {
        let nums = vec![2, 7, 9, 3, 1];
        assert_eq!(Solution::rob(nums), 12);
        // Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
        // Total amount you can rob = 2 + 9 + 1 = 12.
    }
    #[test]
    fn n_1_2() {
        assert_eq!(Solution::rob(vec![1, 2]), 2);
    }
    #[test]
    fn n_1() {
        assert_eq!(Solution::rob(vec![1]), 1);
    }
    #[test]
    fn n_2_1_1_2() {
        assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 4);
    }

    //#[ignore]
    #[test]
    fn n_100x1() {
        let nums = vec![1; 100];
        assert_eq!(Solution::rob(nums), 50);
    }
}
