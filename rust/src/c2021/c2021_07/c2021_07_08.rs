#![allow(dead_code)]
/// Maximum Length of Repeated Subarray
/// ===================================
///
/// Given two integer arrays `nums1` and `nums2`,
/// return _the maximum length of a subarray that appears in __both__ arrays_.
///
/// __Constraints:__
///
/// - `1 <= nums1.length, nums2.length <= 1000`
/// - `0 <= nums1[i], nums2[i] <= 100`
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/609/week-2-july-8th-july-14th/3807/
struct Solution;
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (l1, l2) = (nums1.len(), nums2.len());
        let mut dp = vec![vec![0; l2 + 1]; l1 + 1];
        let mut r = 0;
        for i1 in 0..l1 {
            for i2 in 0..l2 {
                if nums1[i1] == nums2[i2] {
                    dp[i1 + 1][i2 + 1] = dp[i1][i2] + 1;
                    r = r.max(dp[i1 + 1][i2 + 1]);
                }
            }
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_1_2_3_2_1_b_3_2_1_4_7_produces_3() {
        let nums1 = vec![1, 2, 3, 2, 1];
        let nums2 = vec![3, 2, 1, 4, 7];
        assert_eq!(Solution::find_length(nums1, nums2), 3);
        // Explanation: The repeated subarray with maximum length is [3,2,1].
    }
    #[test]
    fn a_0_0_0_0_0_b_0_0_0_0_0_produces_5() {
        let nums1 = vec![0, 0, 0, 0, 0];
        let nums2 = vec![0, 0, 0, 0, 0];
        assert_eq!(Solution::find_length(nums1, nums2), 5);
    }
    #[test]
    fn a_1_0_2_0_3_b_1_2_3_produces_1() {
        let nums1 = vec![1, 0, 2, 0, 3];
        let nums2 = vec![1, 2, 3];
        assert_eq!(Solution::find_length(nums1, nums2), 1);
    }

    mod performance {
        use super::*;

        #[test]
        fn a_1k1s_b_1k2s_produces_0() {
            let nums1 = vec![1; 1000];
            let nums2 = vec![2; 1000];
            assert_eq!(Solution::find_length(nums1, nums2), 0);
        }
        #[test]
        fn a_1k1s_b_1k1s_produces_1000() {
            let nums1 = vec![1; 1000];
            let nums2 = vec![1; 1000];
            assert_eq!(Solution::find_length(nums1, nums2), 1000);
        }
    }
}
