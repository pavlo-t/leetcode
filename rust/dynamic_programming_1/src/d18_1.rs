#![allow(dead_code)]
/// 300. Longest Increasing Subsequence
/// ===================================
///
/// Given an integer array `nums`, return the length of the longest strictly increasing subsequence.
///
/// A __subsequence__ is a sequence that can be derived from an array by deleting some
/// or no elements without changing the order of the remaining elements.
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
    /// 20:20-20:50
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        println!("length_of_lis({:?})", nums);
        use std::collections::HashMap;
        let mut ns_sorted = nums.clone();
        ns_sorted.sort_unstable();
        let mut idxs: HashMap<i32, usize> = HashMap::new();
        let mut n = 1;
        for i in ns_sorted {
            if !idxs.contains_key(&i) {
                idxs.insert(i, n);
                n += 1;
            }
        }
        let mut dp = vec![0; n];
        for i in (0..nums.len()).rev() {
            let &ni = idxs.get(&nums[i]).unwrap();
            for j in 0..ni {
                dp[j] = dp[j].max(dp[ni] + 1);
            }
        }
        dp[0]
    }
    /// 20:14-20:20
    pub fn length_of_lis_dp_vec_20001(nums: Vec<i32>) -> i32 {
        println!("length_of_lis({:?})", nums);
        let n = nums.len();
        let mut dp = vec![0; 20001];
        for i in (0..n).rev() {
            let ni = (nums[i] + 10000) as usize;
            for j in 0..ni {
                dp[j] = dp[j].max(dp[ni] + 1);
            }
        }
        dp[0]
    }
    /// 19:51-20:14
    pub fn length_of_lis_dp_vec_vec(nums: Vec<i32>) -> i32 {
        println!("length_of_lis({:?})", nums);
        let n = nums.len();
        let mut dp = vec![vec![0; 20001]; n + 1];
        for i in (0..n).rev() {
            let ni = (nums[i] + 10000) as usize;
            for j in 0..ni {
                dp[i][j] = dp[i + 1][j].max(dp[i + 1][ni] + 1);
            }
            for j in ni..20001 {
                dp[i][j] = dp[i + 1][j];
            }
        }
        dp[0][0]
    }
    /// 19:41-19:51
    pub fn length_of_lis_rec_with_memo(nums: Vec<i32>) -> i32 {
        println!("length_of_lis({:?})", nums);
        fn rec(i: usize, m: i32, ns: &[i32], memo: &mut Vec<Vec<i32>>) -> i32 {
            let mi = (m + 10000) as usize;
            if i == ns.len() {
                0
            } else if memo[i][mi] >= 0 {
                memo[i][mi]
            } else {
                memo[i][mi] = if m >= ns[i] {
                    rec(i + 1, m, ns, memo)
                } else {
                    rec(i + 1, m, ns, memo).max(1 + rec(i + 1, ns[i], ns, memo))
                };
                memo[i][mi]
            }
        }
        let mut memo = vec![vec![-1; 20001]; nums.len()];
        rec(0, -10000, &nums, &mut memo)
    }
    /// 19:34-19:41
    pub fn length_of_lis_rec(nums: Vec<i32>) -> i32 {
        println!("length_of_lis({:?})", nums);
        fn rec(i: usize, m: i32, ns: &[i32]) -> i32 {
            if i == ns.len() {
                0
            } else if m >= ns[i] {
                rec(i + 1, m, ns)
            } else {
                rec(i + 1, m, ns).max(1 + rec(i + 1, ns[i], ns))
            }
        }
        rec(0, i32::MIN, &nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ns_10_9_2_5_3_7_101_18() {
        let n = vec![10, 9, 2, 5, 3, 7, 101, 18];
        assert_eq!(Solution::length_of_lis(n), 4);
        // Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
    }
    #[test]
    fn ns_0_1_0_3_2_3() {
        let n = vec![0, 1, 0, 3, 2, 3];
        assert_eq!(Solution::length_of_lis(n), 4);
    }
    #[test]
    fn ns_7_7_7_7_7_7_7() {
        let n = vec![7, 7, 7, 7, 7, 7, 7];
        assert_eq!(Solution::length_of_lis(n), 1);
    }

    #[test]
    fn ns_1to2500() {
        let n = (1..=2500).collect();
        assert_eq!(Solution::length_of_lis(n), 2500);
    }
}
