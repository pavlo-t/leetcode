#![allow(dead_code)]
/// \#1695. Maximum Erasure Value
/// =============================
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
/// https://leetcode.com/problems/maximum-erasure-value
struct Solution;
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut subarray: HashSet<i32> = HashSet::new();
        let mut sum = 0;
        let mut max = 0;
        let (mut l, mut r) = (0, 0);
        while r < nums.len() {
            while subarray.contains(&nums[r]) {
                subarray.remove(&nums[l]);
                sum -= nums[l];
                l += 1;
            }
            subarray.insert(nums[r]);
            sum += nums[r];
            max = max.max(sum);
            r += 1;
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_4_2_4_5_6() {
        let n = vec![4, 2, 4, 5, 6];
        assert_eq!(Solution::maximum_unique_subarray(n), 17);
        // Explanation: The optimal subarray here is [2,4,5,6].
    }
    #[test]
    fn n_5_2_1_2_5_2_1_2_5() {
        let n = vec![5, 2, 1, 2, 5, 2, 1, 2, 5];
        assert_eq!(Solution::maximum_unique_subarray(n), 8);
        // Explanation: The optimal subarray here is [5,2,1] or [1,2,5].
    }
    #[test]
    fn n_1x100000() {
        assert_eq!(Solution::maximum_unique_subarray(vec![1; 100000]), 1);
    }
    #[test]
    fn n_1to10000x10() {
        let nums = (0..100_000).map(|n| n % 10000 + 1).collect();
        assert_eq!(Solution::maximum_unique_subarray(nums), 50_005_000);
    }
}
