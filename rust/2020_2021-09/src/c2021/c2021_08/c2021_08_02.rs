#![allow(dead_code)]
/// Two Sum
///
/// Given an array of integers `nums` and an integer `target`,
/// return _indices of the two numbers such that they add up to `target`_.
///
/// You may assume that each input would have ___exactly_ one solution__,
/// and you may not use the _same_ element twice.
///
/// You can return the answer in any order.
///
/// __Constraints:__
///
/// - `2 <= nums.length <= 10_000`
/// - `-1_000_000_000 <= nums[i] <= 1_000_000_000`
/// - `-1_000_000_000 <= target <= 1_000_000_000`
/// - __Only one valid answer exists__.
///
/// __Follow-up:__ Can you come up with an algorithm that is less than `O(n^2)` time complexity?
///
/// https://leetcode.com/explore/featured/card/august-leetcoding-challenge-2021/613/week-1-august-1st-august-7th/3836/
struct Solution;
impl Solution {
    /// Approach 3: One-pass Hash Table
    ///
    /// https://leetcode.com/problems/two-sum/solution/
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        for (i, n) in nums.into_iter().enumerate() {
            if let Some(&j) = map.get(&(target - n)) {
                return vec![j as i32, i as i32];
            } else {
                map.insert(n, i);
            }
        }
        unreachable!();
    }

    pub fn two_sum_my_n_log_n(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums = nums.into_iter().enumerate().map(|(i, n)| (n, i)).collect::<Vec<_>>();
        nums.sort_unstable();
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let sum = nums[l].0 + nums[r].0;
            if sum < target {
                l += 1;
            } else if sum > target {
                r -= 1;
            } else {
                return vec![nums[l].1 as i32, nums[r].1 as i32];
            }
        }
        unreachable!();
    }

    pub fn two_sum_my_quadratic_time(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for l in 0..nums.len() - 1 {
            for r in l + 1..nums.len() {
                if nums[l] + nums[r] == target {
                    return vec![l as i32, r as i32];
                }
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_p2p7p11p15_t9_produces_0_1() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(Solution::two_sum(nums, 9), [0, 1]);
        // Output: Because nums[0] + nums[1] == 9, we return [0, 1].
    }
    #[test]
    fn n_p3p2p4_t_6_produces_1_2() {
        let nums = vec![3, 2, 4];
        assert_eq!(Solution::two_sum(nums, 6), [1, 2]);
    }
    #[test]
    fn n_p3p3_t_6_produces_0_1() {
        let nums = vec![3, 3];
        assert_eq!(Solution::two_sum(nums, 6), [0, 1]);
    }

    #[test]
    fn n_1to10000_t_19999_produces_9998_9999() {
        let nums = (1..=10000).collect();
        assert_eq!(Solution::two_sum(nums, 19999), [9998, 9999]);
    }
}
