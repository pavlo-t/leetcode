#![allow(dead_code)]
/// 1675. Minimize Deviation in Array
/// =================================
///
/// You are given an array `nums` of `n` positive integers.
///
/// You can perform two types of operations on any element of the array any number of times:
///
/// - If the element is __even__, __divide__ it by `2`.
///   For example, if the array is `[1,2,3,4]`,
///   then you can do this operation on the last element, and the array will be `[1,2,3,2]`.
/// - If the element is __odd__, __multiply__ it by `2`.
///   For example, if the array is `[1,2,3,4]`,
///   then you can do this operation on the first element, and the array will be `[2,2,3,4]`.
///
/// The __deviation__ of the array is the __maximum difference__ between any two elements in the array.
///
/// Return _the __minimum deviation__ the array can have after performing some number of operations_.
///
/// __Constraints:__
///
/// - `n == nums.length`
/// - `2 <= n <= 100_000`
/// - `1 <= nums[i] <= 1_000_000_000`
///
/// https://leetcode.com/problems/minimize-deviation-in-array/
struct Solution;
impl Solution {
    /// from old solutions `src/main/scala/challenge/c2021/c2021_01/c2021_01_30.scala`
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;
        let mut evens = BinaryHeap::new();
        let mut min = i32::MAX;
        for n in nums {
            let even = if n % 2 == 0 { n } else { n * 2 };
            evens.push(even);
            min = min.min(even);
        }

        let mut result = i32::MAX;
        while let Some(n) = evens.pop() {
            result = result.min(n - min);
            if n % 2 == 0 {
                evens.push(n / 2);
                min = min.min(n / 2);
            } else {
                return result;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1_1() { assert_eq!(Solution::minimum_deviation(vec![1, 1]), 0); }
    #[rustfmt::skip] #[test] fn n_1_16() { assert_eq!(Solution::minimum_deviation(vec![1, 16]), 0); }
    #[rustfmt::skip] #[test] fn n_1_2() { assert_eq!(Solution::minimum_deviation(vec![1, 2]), 0); }
    #[rustfmt::skip] #[test] fn n_1_2_3() { assert_eq!(Solution::minimum_deviation(vec![1, 2, 3]), 1); }
    #[rustfmt::skip] #[test] fn n_3_5() { assert_eq!(Solution::minimum_deviation(vec![3, 5]), 1); }

    #[test]
    fn n_1_2_3_4() {
        assert_eq!(Solution::minimum_deviation(vec![1, 2, 3, 4]), 1);
        // Explanation: You can transform the array to [1,2,3,2], then to [2,2,3,2],
        // then the deviation will be 3 - 2 = 1.
    }
    #[test]
    fn n_1_2_3_4_5() {
        assert_eq!(Solution::minimum_deviation(vec![1, 2, 3, 4, 5]), 3);
    }
    #[test]
    fn n_4_1_5_20_3() {
        assert_eq!(Solution::minimum_deviation(vec![4, 1, 5, 20, 3]), 3);
        // Explanation: You can transform the array after two operations to [4,2,5,5,3],
        // then the deviation will be 5 - 2 = 3.
    }
    #[test]
    fn n_2_10_8() {
        assert_eq!(Solution::minimum_deviation(vec![2, 10, 8]), 3);
    }
}
