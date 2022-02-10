#![allow(dead_code)]
/// 560. Subarray Sum Equals K
/// ==========================
///
/// Given an array of integers `nums` and an integer `k`,
/// return _the total number of continuous subarrays whose sum equals to `k`_.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 20_000`
/// - `-1000 <= nums[i] <= 1000`
/// - `-100_000_000 <= k <= 100_000_000`
///
/// https://leetcode.com/problems/subarray-sum-equals-k/
struct Solution;
impl Solution {
    pub fn subarray_sum_brute_force(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        for l in 0..nums.len() {
            let mut sum = 0;
            for r in l..nums.len() {
                sum += nums[r];
                result += (sum == k) as i32;
            }
        }
        result
    }

    /// from other submissions
    /// https://leetcode.com/submissions/detail/638801949/
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        let mut result = 0;
        let mut sum = 0;
        let mut sum_counts = HashMap::new();
        sum_counts.insert(0, 1);
        for num in nums {
            sum += num;
            if let Some(count) = sum_counts.get(&(sum - k)) {
                result += count;
            }
            *sum_counts.entry(sum).or_insert(0) += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_0_k_0() { assert_eq!(Solution::subarray_sum(vec![0], 0), 1); }
    #[rustfmt::skip] #[test] fn n_0_k_1() { assert_eq!(Solution::subarray_sum(vec![0], 1), 0); }
    #[rustfmt::skip] #[test] fn n_1_k_0() { assert_eq!(Solution::subarray_sum(vec![1], 0), 0); }
    #[rustfmt::skip] #[test] fn n_1_k_1() { assert_eq!(Solution::subarray_sum(vec![1], 1), 1); }
    #[rustfmt::skip] #[test] fn n_1_k_2() { assert_eq!(Solution::subarray_sum(vec![1], 2), 0); }

    #[rustfmt::skip] #[test] fn n_0_1_k_0() { assert_eq!(Solution::subarray_sum(vec![0,1], 0), 1); }
    #[rustfmt::skip] #[test] fn n_1_1_k_1() { assert_eq!(Solution::subarray_sum(vec![1,1], 1), 2); }
    #[rustfmt::skip] #[test] fn n_1_1_k_2() { assert_eq!(Solution::subarray_sum(vec![1,1], 2), 1); }
    #[rustfmt::skip] #[test] fn n_1_1_k_3() { assert_eq!(Solution::subarray_sum(vec![1,1], 3), 0); }

    #[rustfmt::skip] #[test] fn n_1_1_1_k_1() { assert_eq!(Solution::subarray_sum(vec![1,1,1], 1), 3); }
    #[rustfmt::skip] #[test] fn n_1_1_1_k_2() { assert_eq!(Solution::subarray_sum(vec![1,1,1], 2), 2); }

    #[rustfmt::skip] #[test] fn n_1_2_3_k_3() { assert_eq!(Solution::subarray_sum(vec![1,2,3], 3), 2); }

    #[rustfmt::skip] #[test] fn n_1_repeat_20000_k_2() { assert_eq!(Solution::subarray_sum(vec![1; 20000], 2), 19999); }
}
