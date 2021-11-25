#![allow(dead_code)]
/// 53. Maximum Subarray
/// ====================
///
/// Given an integer array `nums`, find the contiguous subarray (containing at least one number)
/// which has the largest sum and return _its sum_.
///
/// A __subarray__ is a __contiguous__ part of an array.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 100_000`
/// - `-10_000 <= nums[i] <= 10_000`
///
/// __Follow up:__ If you have figured out the `O(n)` solution,
/// try coding another solution using the __divide and conquer__ approach,
/// which is more subtle.
///
/// https://leetcode.com/problems/maximum-subarray/
struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        println!("max_sub_array({:?})", nums);
        let mut curr = nums[0];
        let mut max = curr;
        for n in nums.into_iter().skip(1) {
            curr = n.max(curr + n);
            max = max.max(curr);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_m2p1m3p4m1p2p1m5p4() {
        let n = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(Solution::max_sub_array(n), 6);
        // Explanation: [4,-1,2,1] has the largest sum = 6.
    }
    #[test]
    fn n_p1() {
        let n = vec![1];
        assert_eq!(Solution::max_sub_array(n), 1);
    }
    #[test]
    fn n_p5p4m1p7p8() {
        let n = vec![5, 4, -1, 7, 8];
        assert_eq!(Solution::max_sub_array(n), 23);
    }
}
