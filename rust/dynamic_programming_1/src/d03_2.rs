#![allow(dead_code)]
/// 213. House Robber II
/// ====================
///
/// You are a professional robber planning to rob houses along a street.
/// Each house has a certain amount of money stashed.
/// All houses at this place are __arranged in a circle__.
/// That means the first house is the neighbor of the last one.
/// Meanwhile, adjacent houses have a security system connected,
/// and __it will automatically contact the police if two adjacent houses were broken into on the same night__.
///
/// Given an integer array `nums` representing the amount of money of each house,
/// return _the maximum amount of money you can rob tonight __without alerting the police___.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 100`
/// - `0 <= nums[i] <= 1000`
///
/// https://leetcode.com/problems/house-robber-ii/
struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn rob_linear(ns: &[i32]) -> i32 {
            let (mut a, mut b) = (0, 0);
            for &n in ns {
                std::mem::swap(&mut a, &mut b);
                a = b.max(n + a);
            }
            a
        }
        match nums.len() {
            1 => nums[0],
            _ => rob_linear(&nums[..nums.len() - 1]).max(rob_linear(&nums[1..])),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_2_3_2() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        // Explanation: You cannot rob house 1 (money = 2) and then rob house 3 (money = 2),
        // because they are adjacent houses.
    }
    #[test]
    fn n_1_2_3_1() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        // Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
        // Total amount you can rob = 1 + 3 = 4.
    }
    #[test]
    fn n_1_2_3() {
        assert_eq!(Solution::rob(vec![1, 2, 3]), 3);
    }
    #[test]
    fn n_1() {
        assert_eq!(Solution::rob(vec![1]), 1);
    }
    #[test]
    fn n_1_2() {
        assert_eq!(Solution::rob(vec![1, 2]), 2);
    }

    #[test]
    fn n_100x1() {
        assert_eq!(Solution::rob(vec![1; 100]), 50);
    }
}
