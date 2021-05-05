#![allow(dead_code)]
/// Jump Game II
/// ============
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
/// - `1 <= nums.length <= 1000`
/// - `0 <= nums[i] <= 100_000`
///
/// https://leetcode.com/explore/featured/card/may-leetcoding-challenge-2021/598/week-1-may-1st-may-7th/3732/
struct Solution;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut current_jump_end = 0;
        let mut farthest = 0;
        let mut jumps = 0;
        for i in 0..nums.len() - 1 {
            farthest = farthest.max(i + nums[i] as usize);
            if i == current_jump_end {
                jumps += 1;
                current_jump_end = farthest;
            }
        }
        jumps
    }

    pub fn jump_my_rec(nums: Vec<i32>) -> i32 {
        fn best_jump(c: usize, ns: &[i32]) -> usize {
            (c + 1..=(c + ns[c] as usize).min(ns.len() - 1))
                .fold((0, 0), |(p, pj), n| match n + ns[n] as usize {
                    nj if n == ns.len() - 1 => (n, nj),
                    nj if p == ns.len() - 1 || pj > nj => (p, pj),
                    nj => (n, nj),
                })
                .0
        }
        fn rec(c: usize, ns: &[i32]) -> i32 {
            if c >= ns.len() - 1 {
                0
            } else {
                1 + rec(best_jump(c, ns), ns)
            }
        }
        rec(0, &nums)
    }

    pub fn jump_my_iteration(nums: Vec<i32>) -> i32 {
        let mut jumps = 0;
        let mut c = 0;
        loop {
            if c >= nums.len() - 1 {
                return jumps;
            }
            c = (c + 1..=(c + nums[c] as usize).min(nums.len() - 1))
                .fold((0, 0), |(p, pj), n| match n + nums[n] as usize {
                    nj if n == nums.len() - 1 => (n, nj),
                    nj if p == nums.len() - 1 || pj > nj => (p, pj),
                    nj => (n, nj),
                })
                .0;
            jumps += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_ns23114_produces_2() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(Solution::jump(nums), 2);
        // Explanation:
        // The minimum number of jumps to reach the last index is 2.
        // Jump 1 step from index 0 to 1, then 3 steps to the last index.
    }
    #[test]
    fn example2_ns23014_produces_2() {
        let nums = vec![2, 3, 0, 1, 4];
        assert_eq!(Solution::jump(nums), 2);
    }

    #[test]
    fn test68_ns231_produces_1() {
        let nums = vec![2, 3, 1];
        assert_eq!(Solution::jump(nums), 1);
    }
    #[test]
    fn test91_ns0_produces_0() {
        let nums = vec![0];
        assert_eq!(Solution::jump(nums), 0);
    }

    #[test]
    fn ns203014_produces_2() {
        let nums = vec![2, 0, 3, 0, 1, 4];
        assert_eq!(Solution::jump(nums), 2);
    }

    mod performance {
        use super::*;

        #[test]
        fn n1x1000_produces_999() {
            let nums = vec![1; 1000];
            assert_eq!(Solution::jump(nums), 999);
        }
    }
}
