#![allow(dead_code)]
/// Find Peak Element
/// =================
///
/// A peak element is an element that is strictly greater than its neighbors.
///
/// Given an integer array `nums`, find a peak element, and return its index.
/// If the array contains multiple peaks, return the index to __any of the peaks__.
///
/// You may imagine that `nums[-1] = nums[n] = -∞`.
///
/// You must write an algorithm that runs in `O(log n)` time.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 1000`
/// - `-2^31 <= nums[i] <= 2^31 - 1`
/// - `nums[i] != nums[i + 1]` for all valid `i`.
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/609/week-2-july-8th-july-14th/3812/
struct Solution;
impl Solution {
    /// https://leetcode.com/problems/find-peak-element/solution/
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        fn bs(l: usize, r: usize, ns: &[i32]) -> usize {
            if l == r {
                l
            } else {
                let m = (l + r) / 2;
                if ns[m] > ns[m + 1] {
                    bs(l, m, ns)
                } else {
                    bs(m + 1, r, ns)
                }
            }
        }
        bs(0, nums.len() - 1, &nums) as i32
    }

    pub fn find_peak_element_my_linear(nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => 0,
            _ if nums[0] > nums[1] => 0,
            2 => 1,
            l => {
                for i in 1..nums.len() - 1 {
                    if nums[i - 1] < nums[i] && nums[i] > nums[i + 1] {
                        return i as i32;
                    }
                }
                (l - 1) as i32
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn n_1_2_3_1_produces_2() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::find_peak_element(nums), 2);
        // Explanation: 3 is a peak element and your function should return the index number 2.
    }
    #[test]
    fn n_1_2_1_3_5_6_4_produces_1_or_5() {
        let nums = vec![1, 2, 1, 3, 5, 6, 4];
        let accepted = [1, 5].iter().map(|&n| n).collect::<HashSet<_>>();
        let result = Solution::find_peak_element(nums);
        assert!(accepted.contains(&result));
        // Explanation: Your function can return either index number 1 where the peak element is 2,
        // or index number 5 where the peak element is 6.
    }
    #[test]
    fn n_7_6_5_4_3_2_1_produces_0() {
        let nums = vec![7, 6, 5, 4, 3, 2, 1];
        assert_eq!(Solution::find_peak_element(nums), 0);
    }
    #[test]
    fn n_1_2_3_4_5_6_7_produces_6() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(Solution::find_peak_element(nums), 6);
    }
}
