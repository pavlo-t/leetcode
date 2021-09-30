#![allow(dead_code)]
/// Max Consecutive Ones
/// ====================
///
/// Given a binary array `nums`, return _the maximum number of consecutive `1`'s in the array_.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 100_000`
/// - `nums[i]` is either `0` or `1`.
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/638/week-3-september-15th-september-21st/3982/
struct Solution;
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |(m, c), &e| match e {
                0 => (m, 0),
                1 => (m.max(c + 1), c + 1),
                _ => unreachable!(),
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_110111_produces_3() {
        let nums = vec![1, 1, 0, 1, 1, 1];
        assert_eq!(Solution::find_max_consecutive_ones(nums), 3);
        // Explanation: The first two digits or the last three digits are consecutive 1s.
        // The maximum number of consecutive 1s is 3.
    }
    #[test]
    fn n_101101_produces_2() {
        let nums = vec![1, 0, 1, 1, 0, 1];
        assert_eq!(Solution::find_max_consecutive_ones(nums), 2);
    }
}
