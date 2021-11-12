#![allow(dead_code)]
/// 300. Longest Increasing Subsequence
/// ===================================
///
/// Given an integer array `nums`, return the length of the longest strictly increasing subsequence.
///
/// A __subsequence__ is a sequence that can be derived from an array by deleting some or no elements
/// without changing the order of the remaining elements.
/// For example, `[3,6,2,7]` is a subsequence of the array `[0,3,1,6,2,2,7]`.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 2500`
/// - `-10_000 <= nums[i] <= 10_000`
///
/// __Follow up:__ Can you come up with an algorithm that runs in `O(n log(n))` time complexity?
///
/// https://leetcode.com/problems/longest-increasing-subsequence/
struct Solution;
impl Solution {
    pub fn length_of_lis_rec(nums: Vec<i32>) -> i32 {
        println!("length_of_lis({:?})", nums);
        /// t: last taken index + 1; if t == 0 - none were taken yet
        fn rec(i: usize, t: usize, ns: &[i32]) -> i32 {
            if i == ns.len() {
                0
            } else if t == 0 || ns[t - 1] < ns[i] {
                rec(i + 1, t, ns).max(1 + rec(i + 1, i + 1, ns))
            } else {
                rec(i + 1, t, ns)
            }
        }
        rec(0, 0, &nums)
    }
    pub fn length_of_lis_rec_with_memo(nums: Vec<i32>) -> i32 {
        println!("length_of_lis({:?})", nums);
        let mut memo = vec![vec![-1; nums.len() + 2]; nums.len() + 1];
        /// t: last taken index + 1; if t == 0 - none were taken yet
        fn rec(i: usize, t: usize, ns: &[i32], memo: &mut Vec<Vec<i32>>) -> i32 {
            if memo[i][t] == -1 {
                memo[i][t] = if i == ns.len() {
                    0
                } else if t == 0 || ns[t - 1] < ns[i] {
                    rec(i + 1, t, ns, memo).max(1 + rec(i + 1, i + 1, ns, memo))
                } else {
                    rec(i + 1, t, ns, memo)
                };
            }
            memo[i][t]
        }
        rec(0, 0, &nums, &mut memo)
    }
    pub fn length_of_lis_dp_vec_vec(nums: Vec<i32>) -> i32 {
        println!("length_of_lis({:?})", nums);
        let n = nums.len();
        let mut dp = vec![vec![0; n + 2]; n + 1];
        for i in (0..n).rev() {
            for t in (0..=i).rev() {
                dp[i][t] = if t == 0 || nums[t - 1] < nums[i] {
                    dp[i + 1][t].max(1 + dp[i + 1][i + 1])
                } else {
                    dp[i + 1][t]
                }
            }
        }
        dp[0][0]
    }
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        println!("length_of_lis({:?})", nums);
        let n = nums.len();
        let mut dp = vec![0; n + 1];
        for i in (0..n).rev() {
            let next = 1 + dp[i + 1];
            dp[0] = dp[0].max(next);
            for t in 1..=i {
                if nums[t - 1] < nums[i] {
                    dp[t] = dp[t].max(next);
                }
            }
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1() { assert_eq!(Solution::length_of_lis(vec![1  ]), 1); }
    #[rustfmt::skip] #[test] fn n_11(){ assert_eq!(Solution::length_of_lis(vec![1,1]), 1); }
    #[rustfmt::skip] #[test] fn n_21(){ assert_eq!(Solution::length_of_lis(vec![2,1]), 1); }
    #[rustfmt::skip] #[test] fn n_12(){ assert_eq!(Solution::length_of_lis(vec![1,2]), 2); }
    #[test]
    fn n_10_9_2_5_3_7_101_18() {
        let n = vec![10, 9, 2, 5, 3, 7, 101, 18];
        assert_eq!(Solution::length_of_lis(n), 4);
        // Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
    }
    #[test]
    fn n_010323() {
        let n = vec![0, 1, 0, 3, 2, 3];
        assert_eq!(Solution::length_of_lis(n), 4);
    }
    #[test]
    fn n_7777777() {
        let n = vec![7, 7, 7, 7, 7, 7, 7];
        assert_eq!(Solution::length_of_lis(n), 1);
    }

    #[test]
    fn n_1_repeat_2500() {
        let n = vec![1; 2500];
        assert_eq!(Solution::length_of_lis(n), 1);
    }
    #[test]
    fn n_1_to_2500() {
        let n = (1..=2500).collect();
        assert_eq!(Solution::length_of_lis(n), 2500);
    }
}
