#![allow(dead_code)]
/// 1413. Minimum Value to Get Positive Step by Step Sum
/// ====================================================
///
/// Given an array of integers `nums`,
/// you start with an initial __positive__ value _startValue_.
///
/// In each iteration, you calculate the step by step sum of _startValue_ plus elements in `nums` (from left to right).
///
/// Return the minimum __positive__ value of _startValue_ such that the step by step sum is never less than 1.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 100`
/// - `-100 <= nums[i] <= 100`
///
/// https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum/
struct Solution;
impl Solution {
    /// 15:32-15:39
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        println!("min_start_value({:?})", nums);
        let (min, _) = nums
            .into_iter()
            .fold((i32::MAX, 0), |(min, sum), n| (min.min(sum + n), sum + n));
        1.max(1 - min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n0() { assert_eq!(Solution::min_start_value(vec![ 0]), 1); }
    #[rustfmt::skip] #[test] fn p1() { assert_eq!(Solution::min_start_value(vec![ 1]), 1); }
    #[rustfmt::skip] #[test] fn p2() { assert_eq!(Solution::min_start_value(vec![ 2]), 1); }
    #[rustfmt::skip] #[test] fn m1() { assert_eq!(Solution::min_start_value(vec![-1]), 2); }
    #[rustfmt::skip] #[test] fn m2() { assert_eq!(Solution::min_start_value(vec![-2]), 3); }
    #[test]
    fn m3p2m3p4p2() {
        let n = vec![-3, 2, -3, 4, 2];
        assert_eq!(Solution::min_start_value(n), 5);
        // Explanation: If you choose startValue = 4, in the third iteration your step by step sum is less than 1.
        //                 step by step sum
        //                 startValue = 4 | startValue = 5 | nums
        //                   (4 -3 ) = 1  | (5 -3 ) = 2    |  -3
        //                   (1 +2 ) = 3  | (2 +2 ) = 4    |   2
        //                   (3 -3 ) = 0  | (4 -3 ) = 1    |  -3
        //                   (0 +4 ) = 4  | (1 +4 ) = 5    |   4
        //                   (4 +2 ) = 6  | (5 +2 ) = 7    |   2
    }
    #[test]
    fn p1p2() {
        let n = vec![1, 2];
        assert_eq!(Solution::min_start_value(n), 1);
        // Explanation: Minimum start value should be positive.
    }
    #[test]
    fn p1m2m3() {
        let n = vec![1, -2, -3];
        assert_eq!(Solution::min_start_value(n), 5);
    }
}
