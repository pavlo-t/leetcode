#![allow(dead_code)]

/// # Shortest Unsorted Continuous Subarray
///
/// Given an integer array `nums`, you need to find one __continuous subarray__
/// that if you only sort this subarray in ascending order,
/// then the whole array will be sorted in ascending order.
///
/// Return _the shortest such subarray and output its length_.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 10_000`
/// - `-100_000 <= nums[i] <= 100_000`
///
/// __Follow up__: Can you solve it in `O(n)` time complexity?
///
/// https://leetcode.com/explore/featured/card/february-leetcoding-challenge-2021/587/week-4-february-22nd-february-28th/3652/
struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut end = -1;

        let mut min = i32::MAX;
        let mut max = i32::MIN;

        let mut l = 0;
        let mut r = nums.len() - 1;

        loop {
            if nums[l] >= max { max = nums[l]; } else { end = l as i32; }
            if nums[r] <= min { min = nums[r]; } else { start = r as i32; }

            if r == 0 { break; }

            l += 1;
            r -= 1;
        }

        end - start + 1
    }

    pub fn find_unsorted_subarray_clone_sort(nums: Vec<i32>) -> i32 {
        let mut ns = nums.iter().map(|&i| i).collect::<Vec<_>>();
        ns.sort_unstable();

        let mut result = ns.len() as i32;
        let mut l = 0;
        while ns[l] == nums[l] {
            result -= 1;
            l += 1;
            if l == ns.len() { break; }
        }
        let mut r = ns.len() - 1;
        while r > l && ns[r] == nums[r] {
            result -= 1;
            r -= 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![2, 6, 4, 8, 10, 9, 15];
        assert_eq!(Solution::find_unsorted_subarray(nums), 5);
        // Explanation: You need to sort [6, 4, 8, 10, 9] in ascending order to make the whole array
        // sorted in ascending order.
    }

    #[test]
    fn example2_n1234_should_produce_0() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::find_unsorted_subarray(nums), 0);
    }

    #[test]
    fn example3_n1_should_produce_0() {
        assert_eq!(Solution::find_unsorted_subarray(vec![1]), 0);
    }
}
