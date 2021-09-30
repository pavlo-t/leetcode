#![allow(dead_code)]

/// ### 5630. Maximum Erasure Value
///
/// https://leetcode.com/contest/weekly-contest-220/problems/maximum-erasure-value/
struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut sums = nums.clone();
        for i in 1..nums.len() {
            sums[i] += sums[i - 1]
        }

        let mut l = 0;
        let mut r = 0;

        while r < nums.len() {
            for i in l..r {
                if nums[i] == nums[r] {
                    result = result.max(sums[r - 1] - if l == 0 { 0 } else { sums[l - 1] });
                    l = i + 1;
                    break;
                }
            }

            r += 1;
        }
        result = result.max(sums[r - 1] - if l == 0 { 0 } else { sums[l - 1] });

        result
    }

    pub fn maximum_unique_subarray_bruteforce(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut l = 0;
        let mut r = 0;

        while r < nums.len() {
            for i in l..r {
                if nums[i] == nums[r] {
                    result = result.max(nums[l..r].iter().sum());
                    l = i + 1;
                    break;
                }
            }

            r += 1;
        }
        result = result.max(nums[l..r].iter().sum());

        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![4, 2, 4, 5, 6];
        assert_eq!(Solution::maximum_unique_subarray(nums), 17);
        // Explanation: The optimal subarray here is [2,4,5,6].
    }

    #[test]
    fn example2() {
        let nums = vec![5, 2, 1, 2, 5, 2, 1, 2, 5];
        assert_eq!(Solution::maximum_unique_subarray(nums), 8);
        // Explanation: The optimal subarray here is [5,2,1] or [1,2,5].
    }

    #[test]
    fn test_1to10000_repeat10() {
        let nums = (0..100000).map(|n| n % 10000 + 1).collect();
        assert_eq!(Solution::maximum_unique_subarray(nums), 50005000);
    }
}