#![allow(dead_code)]
/// 45. Jump Game II
/// ================
///
/// Given an array of non-negative integers `nums`,
/// you are initially positioned at the first index of the array.
///
/// Each element in the array represents your maximum jump length at that position.
///
/// Your goal is to reach the last index in the minimum number of jumps.
///
/// You can assume that you can always reach the last index.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 10_000`
/// - `0 <= nums[i] <= 1000`
///
/// https://leetcode.com/problems/jump-game-ii/
struct Solution;
impl Solution {
    /// Approach: Greedy
    /// https://leetcode.com/problems/jump-game-ii/solution/
    pub fn jump(nums: Vec<i32>) -> i32 {
        let (mut jumps, mut curr_jump_end, mut farthest) = (0, 0, 0);
        for i in 0..nums.len() - 1 {
            farthest = farthest.max(i + nums[i] as usize);
            if i == curr_jump_end {
                jumps += 1;
                curr_jump_end = farthest;
            }
        }
        jumps
    }
    pub fn jump_dp(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp: Vec<i32> = vec![0; n];
        for i in (0..n - 1).rev() {
            let mut res = i32::MAX;
            for j in i + 1..=(i + nums[i] as usize).min(n - 1) {
                res = res.min(dp[j].saturating_add(1));
            }
            dp[i] = res;
        }
        dp[0]
    }
    pub fn jump_dfs_with_memo(nums: Vec<i32>) -> i32 {
        fn dfs(i: usize, memo: &mut Vec<i32>, ns: &[i32]) -> i32 {
            if i >= ns.len() - 1 {
                0
            } else if memo[i] >= 0 {
                memo[i]
            } else {
                let mut res = i32::MAX;
                for j in i + 1..=(i + ns[i] as usize) {
                    res = res.min(dfs(j, memo, ns).saturating_add(1));
                }
                memo[i] = res;
                res
            }
        }
        dfs(0, &mut vec![-1; nums.len()], &nums)
    }
    pub fn jump_dfs(nums: Vec<i32>) -> i32 {
        fn dfs(i: usize, ns: &[i32]) -> i32 {
            if i >= ns.len() - 1 {
                0
            } else {
                let mut res = i32::MAX;
                for j in i + 1..=(i + ns[i] as usize) {
                    res = res.min(dfs(j, ns).saturating_add(1));
                }
                res
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
        assert_eq!(Solution::jump(nums), 2);
        // Explanation: The minimum number of jumps to reach the last index is 2.
        // Jump 1 step from index 0 to 1, then 3 steps to the last index.
    }
    #[test]
    fn n_2_3_0_1_4() {
        let nums = vec![2, 3, 0, 1, 4];
        assert_eq!(Solution::jump(nums), 2);
    }
    #[test]
    fn n_0() {
        let nums = vec![0];
        assert_eq!(Solution::jump(nums), 0);
    }
    #[test]
    fn n_1() {
        let nums = vec![1];
        assert_eq!(Solution::jump(nums), 0);
    }

    /// If getting stack overflow:
    ///
    /// ```sh
    /// thread 'd04_2::tests::n_10000x1000' has overflowed its stack
    /// fatal runtime error: stack overflow
    /// ```
    ///
    /// Add `RUST_MIN_STACK=67108864` to env:
    ///
    /// ```sh
    /// RUST_MIN_STACK=67108864 cargo test --lib d04_2
    /// ```
    #[test]
    fn n_10000x1000() {
        let nums = vec![1000; 10_000];
        assert_eq!(Solution::jump(nums), 10);
    }
}
