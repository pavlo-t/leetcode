#![allow(dead_code)]
/// Partition Array into Disjoint Intervals
/// =======================================
///
/// Given an array `nums`, partition it into two (contiguous) subarrays `left` and `right` so that:
///
/// - Every element in `left` is less than or equal to every element in `right`.
/// - `left` and `right` are non-empty.
/// - `left` has the smallest possible size.
///
/// Return the __length__ of `left` after such a partitioning.
/// It is guaranteed that such a partitioning exists.
///
/// __Note:__
///
/// 1. `2 <= nums.length <= 30000`
/// 2. `0 <= nums[i] <= 1_000_000`
/// 3. It is guaranteed there is at least one way to partition `nums` as described.
///
/// https://leetcode.com/explore/challenge/card/july-leetcoding-challenge-2021/611/week-4-july-22nd-july-28th/3823/
struct Solution;
impl Solution {
    /// https://leetcode.com/problems/partition-array-into-disjoint-intervals/solution/224152
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut max_l = nums[0];
        let mut max_so_far = nums[0];
        let mut result = 1;
        for i in 1..nums.len() {
            if nums[i] < max_l {
                result = i + 1;
                max_l = max_so_far;
            } else {
                max_so_far = max_so_far.max(nums[i]);
            }
        }
        result as i32
    }

    pub fn partition_disjoint_my(nums: Vec<i32>) -> i32 {
        let mut min_r = nums.clone();
        for i in (0..nums.len() - 1).rev() {
            min_r[i] = min_r[i].min(min_r[i + 1]);
        }
        let mut max = nums[0];
        for i in 1..nums.len() {
            if max <= min_r[i] {
                return i as i32;
            }
            max = max.max(nums[i]);
        }
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_5_0_3_8_6_produces_3() {
        let nums = vec![5, 0, 3, 8, 6];
        assert_eq!(Solution::partition_disjoint(nums), 3);
        // Explanation: left = [5,0,3], right = [8,6]
    }
    #[test]
    fn n_1_1_1_0_6_12_produces_4() {
        let nums = vec![1, 1, 1, 0, 6, 12];
        assert_eq!(Solution::partition_disjoint(nums), 4);
        // Explanation: left = [1,1,1,0], right = [6,12]
    }
    #[test]
    fn n_1_1_1_1_0_produces_5() {
        let nums = vec![1, 1, 1, 1, 0];
        assert_eq!(Solution::partition_disjoint(nums), 5);
    }
    #[test]
    fn n_1_1_1_1_1_produces_1() {
        let nums = vec![1, 1, 1, 1, 1];
        assert_eq!(Solution::partition_disjoint(nums), 1);
    }
}
