#![allow(dead_code)]
/// Maximum Size Subarray Sum Equals k
/// ==================================
///
/// Given an integer array `nums` and an integer `k`,
/// return _the maximum length of a subarray that sums to `k`_.
/// If there isn't one, return `0` instead.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 200_000`
/// - `-10_000 <= nums[i] <= 10_000`
/// - `-1_000_000_000 <= k <= 1_000_000_000`
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/640/week-5-september-29th-september-30th/3991/
struct Solution;
impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        println!("max_sub_array_len({:?}, {:?})", nums, k);

        use std::collections::HashMap;

        let n = nums.len();
        // prefix sum
        let mut s = 0;
        // left-most sum indexes
        let mut lsi = HashMap::new();
        lsi.insert(s, 0);
        let mut res = 0;
        for i in 0..n {
            s += nums[i];
            if let Some(&l) = lsi.get(&(s - k)) {
                res = res.max(i + 1 - l);
            }
            if !lsi.contains_key(&s) {
                lsi.insert(s, i + 1);
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_1_m1_5_m2_3_k_3() {
        let n = vec![1, -1, 5, -2, 3];
        assert_eq!(Solution::max_sub_array_len(n, 3), 4);
        // Explanation: The subarray [1, -1, 5, -2] sums to 3 and is the longest.
    }
    #[test]
    fn n_m2_m1_2_1_k_1() {
        let n = vec![-2, -1, 2, 1];
        assert_eq!(Solution::max_sub_array_len(n, 1), 2);
        // Explanation: The subarray [-1, 2] sums to 1 and is the longest.
    }
    #[test]
    fn n_m2_m1_2_1_k_5() {
        let n = vec![-2, -1, 2, 1];
        assert_eq!(Solution::max_sub_array_len(n, 5), 0);
    }

    mod performance {
        use super::*;

        #[test]
        fn n_1x20_000_k_20_000() {
            let n = vec![1; 20_000];
            assert_eq!(Solution::max_sub_array_len(n, 20_000), 20_000);
        }
        //#[ignore]
        #[test]
        fn n_1x200_000_k_200_000() {
            let n = vec![1; 200_000];
            assert_eq!(Solution::max_sub_array_len(n, 200_000), 200_000);
        }
    }
}
