#![allow(dead_code)]
/// 55. Jump Game
/// =============
///
/// You are given an integer array `nums`.
/// You are initially positioned at the array's __first index__,
/// and each element in the array represents your maximum jump length at that position.
///
/// Return _`true` if you can reach the last index, or `false` otherwise_.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 10_000`
/// - `0 <= nums[i] <= 100_000`
///
/// https://leetcode.com/problems/jump-game/
struct Solution;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut lg = nums.len() - 1;
        for i in (0..nums.len()).rev() {
            if i + nums[i] as usize >= lg {
                lg = i;
            }
        }
        lg == 0
    }

    pub fn can_jump_dp(nums: Vec<i32>) -> bool {
        let li = nums.len() - 1;
        let mut dp = vec![false; nums.len()];
        dp[li] = true;
        for i in (0..nums.len()).rev() {
            dp[i] = (i..=(i + nums[i] as usize).min(li)).rev().any(|i| dp[i]);
        }
        dp[0]
    }
    pub fn can_jump_dfs_with_memo(nums: Vec<i32>) -> bool {
        fn dfs(i: usize, seen: &mut Vec<bool>, ns: &[i32]) -> bool {
            if i >= ns.len() - 1 {
                true
            } else if seen[i] {
                false
            } else {
                seen[i] = true;
                (1..=ns[i]).rev().any(|j| dfs(i + j as usize, seen, ns))
            }
        }
        dfs(0, &mut vec![false; nums.len()], &nums)
    }
    pub fn can_jump_dfs(nums: Vec<i32>) -> bool {
        fn dfs(i: usize, ns: &[i32]) -> bool {
            if i >= ns.len() - 1 {
                true
            } else {
                (1..=ns[i]).rev().any(|j| dfs(i + j as usize, ns))
            }
        }
        dfs(0, &nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_2_3_1_1_4() {
        let nums = vec![2, 3, 1, 1, 4];
        assert!(Solution::can_jump(nums));
        // Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
    }
    #[test]
    fn n_3_2_1_0_4() {
        let nums = vec![3, 2, 1, 0, 4];
        assert!(!Solution::can_jump(nums));
        // Explanation: You will always arrive at index 3 no matter what.
        // Its maximum jump length is 0, which makes it impossible to reach the last index.
    }
    #[test]
    fn n_1() {
        assert!(Solution::can_jump(vec![1]));
    }
    #[test]
    fn n_0() {
        assert!(Solution::can_jump(vec![0]));
    }

    #[test]
    fn n_9998to0_0() {
        let nums = (0..9998).rev().chain(std::iter::once(0)).collect();
        assert!(!Solution::can_jump(nums));
    }
}
