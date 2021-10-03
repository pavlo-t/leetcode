#![allow(dead_code)]
/// 55. Jump Game
/// =============
///
/// You are given an integer array `nums`.
/// You are initially positioned at the array's __first index__,
/// and each element in the array represents your maximum jump length at that position.
///
/// Return _`true` if you can reach the last index, or `false` otherwise_
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 10_000`
/// - `0 <= nums[i] <= 100_000`
///
/// https://leetcode.com/problems/jump-game/
struct Solution;
impl Solution {
    /// Approach 4: Greedy
    /// https://leetcode.com/problems/jump-game/solution/
    pub fn can_jump(nums: Vec<i32>) -> bool {
        println!("can_jump {:?}", nums);
        let n = nums.len();
        let mut first_good = n - 1;
        for i in (0..n - 1).rev() {
            if i + nums[i] as usize >= first_good {
                first_good = i;
            }
        }
        first_good == 0
    }
    pub fn can_jump_dp_bottom_up(nums: Vec<i32>) -> bool {
        println!("can_jump {:?}", nums);
        let n = nums.len();
        let mut dp = vec![false; n];
        dp[n - 1] = true;
        for i in (0..n - 1).rev() {
            dp[i] = (i..=(i + nums[i] as usize)).rev().find(|&j| j >= n || dp[j]).is_some();
        }
        dp[0]
    }
    pub fn can_jump_dfs_with_memo(nums: Vec<i32>) -> bool {
        println!("can_jump {:?}", nums);
        fn dfs(i: usize, seen: &mut Vec<bool>, ns: &[i32]) -> bool {
            if i + 1 >= ns.len() {
                true
            } else if ns[i] == 0 || seen[i] {
                false
            } else {
                seen[i] = true;
                let mut ni = ns[i] as usize;
                while ni > 0 {
                    if dfs(i + ni, seen, ns) {
                        return true;
                    }
                    ni -= 1;
                }
                false
            }
        }
        dfs(0, &mut vec![false; nums.len()], &nums)
    }
    pub fn can_jump_dfs(nums: Vec<i32>) -> bool {
        println!("can_jump {:?}", nums);
        fn dfs(i: usize, ns: &[i32]) -> bool {
            if i + 1 >= ns.len() {
                true
            } else if ns[i] == 0 {
                false
            } else {
                let mut ni = ns[i] as usize;
                while ni > 0 {
                    if dfs(i + ni, ns) {
                        return true;
                    }
                    ni -= 1;
                }
                false
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
    fn n_10000x1() {
        let nums = vec![1; 10_000];
        assert!(Solution::can_jump(nums));
    }
    #[test]
    fn n_1x9998_9999x0() {
        let mut nums = vec![0; 10_000];
        nums[0] = 9998;
        assert!(!Solution::can_jump(nums));
    }
    #[test]
    fn n_9998_to_0_0() {
        let mut nums: Vec<i32> = (0..9998).rev().collect();
        nums.push(0);
        assert!(!Solution::can_jump(nums));
    }
}
