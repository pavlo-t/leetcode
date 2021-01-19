#![allow(dead_code)]

/// ### Minimum Operations to Reduce X to Zero
/// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3603/
struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let target = nums.iter().sum::<i32>() - x;
        if target < 0 {
            -1
        } else if target == 0 {
            nums.len() as i32
        } else {
            let mut l = 0;
            let mut r = 0;
            let mut max_len = 0;
            let mut subarray_sum = 0;

            while r < nums.len() {
                let new_sum = subarray_sum + nums[r];
                if new_sum > target {
                    subarray_sum -= nums[l];
                    l += 1;
                } else if new_sum == target {
                    subarray_sum = new_sum - nums[l];
                    max_len = max_len.max(r - l + 1);
                    l += 1;
                    r += 1;
                } else {
                    subarray_sum = new_sum;
                    r += 1;
                }
            }

            if max_len == 0 { -1 } else { (nums.len() - max_len) as i32 }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_n11423_x5_is_2() {
        let nums = vec![1, 1, 4, 2, 3];
        assert_eq!(Solution::min_operations(nums, 5), 2);
        // Explanation: The optimal solution is to remove the last two elements to reduce x to zero.
    }

    #[test]
    fn example2_n56789_x4_is_m1() {
        let nums = vec![5, 6, 7, 8, 9];
        assert_eq!(Solution::min_operations(nums, 4), -1);
    }

    #[test]
    fn example3_n3_2_20_1_1_3_x10_is_5() {
        let nums = vec![3, 2, 20, 1, 1, 3];
        assert_eq!(Solution::min_operations(nums, 10), 5);
        // Explanation:
        // The optimal solution is to remove the last three elements and the first two elements
        // (5 operations in total) to reduce x to zero.
    }

    #[test]
    fn n111_x4_is_m1() {
        let nums = vec![1, 1, 1];
        assert_eq!(Solution::min_operations(nums, 4), -1);
    }

    #[test]
    fn n111_x3_is_m3() {
        let nums = vec![1, 1, 1];
        assert_eq!(Solution::min_operations(nums, 3), 3);
    }
}