#![allow(dead_code)]

/// Maximum Erasure Value
/// =====================
///
/// You are given an array of positive integers `nums` and want to erase a subarray containing __unique elements__.
/// The __score__ you get by erasing the subarray is equal to the __sum__ of its elements.
///
/// Return _the __maximum score__ you can get by erasing __exactly one__ subarray_.
///
/// An array `b` is called to be a subarray of `a` if it forms a contiguous subsequence of `a`,
/// that is, if it is equal to `a[l],a[l+1],...,a[r]` for some `(l,r)`.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 100_000`
/// - `1 <= nums[i] <= 10_000`
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/601/week-4-may-22nd-may-28th/3758/
struct Solution;
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut result = 0;
        let mut sum = 0;
        let mut sns = HashSet::new();
        let mut l = 0;
        let mut r = 0;

        while r < nums.len() {
            if sns.contains(&nums[r]) {
                sns.remove(&nums[l]);
                sum -= nums[l];
                l += 1;
            } else {
                sns.insert(nums[r]);
                sum += nums[r];
                result = result.max(sum);
                r += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![4, 2, 4, 5, 6];
        assert_eq!(Solution::maximum_unique_subarray(nums), 17);
        // Explanation: The optimal subarray here is [2,4,5,6].
    }
    #[test]
    fn example2() {
        let nums = vec![5, 2, 1, 2, 5, 2, 1, 2, 5];
        assert_eq!(Solution::maximum_unique_subarray(nums), 8);
        // Explanation: The optimal subarray here is [5,2,1] or [1,2,5].
    }

    #[test]
    fn nums_3_produces_3() {
        assert_eq!(Solution::maximum_unique_subarray(vec![3]), 3);
    }

    #[test]
    fn nums_1to10k_repeat_10_produces_50_005_000() {
        let nums = (1..=100_000).map(|i| i % 10000 + 1).collect();
        assert_eq!(Solution::maximum_unique_subarray(nums), 50005000);
    }
    #[test]
    fn nums_100k_3s_produces_3() {
        assert_eq!(Solution::maximum_unique_subarray(vec![3; 100_000]), 3);
    }
}
