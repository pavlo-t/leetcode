#![allow(dead_code)]
/// Longest Increasing Subsequence
/// ==============================
///
/// Given an integer array `nums`, return the length of the longest strictly increasing subsequence.
///
/// A __subsequence__ is a sequence that can be derived from an array by deleting some or
/// no elements without changing the order of the remaining elements.
/// For example, `[3,6,2,7]` is a subsequence of the array `[0,3,1,6,2,2,7]`.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 2500`
/// - `-10_000 <= nums[i] <= 10_000`
///
/// __Follow up:__ Can you come up with an algorithm that runs in `O(n log(n))` time complexity?
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/609/week-2-july-8th-july-14th/3808/
struct Solution;
impl Solution {
    /// Approach 3: Improve With Binary Search
    ///
    /// https://leetcode.com/problems/longest-increasing-subsequence/solution/
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = Vec::new();
        for x in nums {
            match dp.binary_search(&x) {
                Err(i) => {
                    if i == dp.len() {
                        dp.push(x);
                    } else {
                        dp[i] = x;
                    }
                }
                _ => (),
            }
        }
        dp.len() as i32
    }

    pub fn length_of_lis_my(nums: Vec<i32>) -> i32 {
        let mut dp = nums.iter().map(|&n| (n, 1)).collect::<Vec<_>>();
        for i in 1..nums.len() {
            let mut max = usize::MAX;
            for j in 0..i {
                if dp[j].0 < nums[i] && (max == usize::MAX || dp[j].1 > dp[max].1) {
                    max = j;
                }
            }
            if max < usize::MAX {
                let n = dp[i].0;
                let l = dp[max].1 + 1;
                dp[i] = (n, l);
            }
        }
        dp.into_iter().map(|(_, l)| l).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ns_10_9_2_5_3_7_101_18_produces_4() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        assert_eq!(Solution::length_of_lis(nums), 4);
        //Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
    }
    #[test]
    fn ns_0_1_0_3_2_3() {
        let nums = vec![0, 1, 0, 3, 2, 3];
        assert_eq!(Solution::length_of_lis(nums), 4);
    }
    #[test]
    fn ns_7_7_7_7_7_7_7_produces_1() {
        let nums = vec![7, 7, 7, 7, 7, 7, 7];
        assert_eq!(Solution::length_of_lis(nums), 1);
    }

    mod performance {
        use super::*;

        #[test]
        fn ns_2500x1_produces_1() {
            let nums = vec![1; 2500];
            assert_eq!(Solution::length_of_lis(nums), 1);
        }
        #[test]
        fn ns_1to2500_produces_2500() {
            let nums = (1..=2500).collect();
            assert_eq!(Solution::length_of_lis(nums), 2500);
        }
    }
}
