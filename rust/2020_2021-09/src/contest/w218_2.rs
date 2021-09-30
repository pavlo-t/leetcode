#![allow(dead_code, unused_imports)]

/// ### 5618. Max Number of K-Sum Pairs
///
/// https://leetcode.com/contest/weekly-contest-218/problems/max-number-of-k-sum-pairs/
struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::LinkedList;
        let mut nums = nums;
        nums.sort();
        let mut nums: LinkedList<_> = nums.iter().collect();

        let mut result = 0;

        while nums.len() > 1 {
            let f = nums.pop_front().unwrap();
            let b = nums.pop_back().unwrap();

            if f + b == k {
                result += 1;
            } else if nums.len() == 0 {
                return result;
            } else if f + b < k {
                nums.push_back(b);
            } else {
                nums.push_front(f);
            }
        }

        result
    }

    pub fn max_operations_brute_force(nums: Vec<i32>, k: i32) -> i32 {
        let mut unpaired = Vec::with_capacity(nums.len());
        let mut result = 0;

        for n in nums.iter() {
            if let Some(idx) = unpaired.iter().position(|&&x| x == k - n) {
                result += 1;
                unpaired.remove(idx);
            } else {
                unpaired.push(n);
            }
        }

        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 2, 3, 4];
        let k = 5;
        let expected = 2;
        assert_eq!(Solution::max_operations(nums, k), expected);
        // Explanation: Starting with nums = [1,2,3,4]:
        // - Remove numbers 1 and 4, then nums = [2,3]
        // - Remove numbers 2 and 3, then nums = []
        // There are no more pairs that sum up to 5, hence a total of 2 operations.
    }

    #[test]
    fn example2() {
        let nums = vec![3, 1, 3, 4, 3];
        let k = 6;
        let expected = 1;
        assert_eq!(Solution::max_operations(nums, k), expected);
        // Explanation: Starting with nums = [3,1,3,4,3]:
        // - Remove the first two 3's, then nums = [1,4,3]
        // There are no more pairs that sum up to 6, hence a total of 1 operation.
    }

    #[test]
    fn test_12555666_k7_is_2() {
        let nums = vec![1, 2, 5, 5, 5, 6, 6, 6];
        let k = 7;
        let expected = 2;
        assert_eq!(Solution::max_operations(nums, k), expected);
    }

    #[test]
    fn test_11122256_k7_is_2() {
        let nums = vec![1, 1, 1, 2, 2, 2, 5, 6];
        let k = 7;
        let expected = 2;
        assert_eq!(Solution::max_operations(nums, k), expected);
    }

    #[test]
    fn test_10k_elements() {
        let nums = (1..=10000).collect();
        let k = 10001;
        let expected = 5000;
        assert_eq!(Solution::max_operations(nums, k), expected);
    }

    #[test]
    fn test_100k_elements() {
        let nums = (1..=100000).collect();
        let k = 100001;
        let expected = 50000;
        assert_eq!(Solution::max_operations(nums, k), expected);
    }
}