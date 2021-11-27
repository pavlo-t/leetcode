#![allow(dead_code)]
/// 238. Product of Array Except Self
/// =================================
///
/// Given an integer array `nums`, return _an array `answer`
/// such that `answer[i]` is equal to the product of all the elements of `nums` except `nums[i]`_.
///
/// The product of any prefix or suffix of `nums` is __guaranteed__ to fit in a __32-bit__ integer.
///
/// You must write an algorithm that runs in `O(n)` time and without using the division operation.
///
/// __Constraints:__
///
/// - `2 <= nums.length <= 100_000`
/// - `-30 <= nums[i] <= 30`
/// - The product of any prefix or suffix of `nums` is __guaranteed__ to fit in a __32-bit__ integer.
///
/// __Follow up:__ Can you solve the problem in `O(1)` extra space complexity?
/// (The output array __does not__ count as extra space for space complexity analysis.)
///
/// https://leetcode.com/problems/product-of-array-except-self/
struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        println!("product_except_self({:?})", nums);
        let mut result = vec![1; nums.len()];
        for i in 1..nums.len() {
            result[i] = result[i - 1] * nums[i - 1];
        }
        let mut product = 1;
        for i in (0..nums.len() - 1).rev() {
            product *= nums[i + 1];
            result[i] *= product;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n11() { assert_eq!(Solution::product_except_self(vec![1,1]), [1,1]); }
    #[rustfmt::skip] #[test] fn n12() { assert_eq!(Solution::product_except_self(vec![1,2]), [2,1]); }
    #[test]
    fn n1234() {
        let n = vec![1, 2, 3, 4];
        let e = [24, 12, 8, 6];
        assert_eq!(Solution::product_except_self(n), e);
    }
    #[test]
    fn n_m1p1n0m3p3() {
        let n = vec![-1, 1, 0, -3, 3];
        let e = [0, 0, 9, 0, 0];
        assert_eq!(Solution::product_except_self(n), e);
    }
}
