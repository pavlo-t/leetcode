#![allow(dead_code)]

use std::collections::HashSet;

/// # Set Mismatch
///
/// You have a set of integers `s`, which originally contains all the numbers from `1` to `n`.
/// Unfortunately, due to some error, one of the numbers in `s` got duplicated to another number
/// in the set, which results in __repetition of one__ number and __loss of another__ number.
///
/// You are given an integer array `nums` representing the data status of this set after the error.
///
/// Find the number that occurs twice and the number that is missing and
/// _return them in the form of an array_.
///
/// __Constraints:__
///
/// - `2 <= nums.length <= 10_000`
/// - `1 <= nums[i] <= 10_000`
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/588/week-1-march-1st-march-7th/3658/
struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut diff = 0;
        let mut square_diff = 0;
        for (i, n) in nums.iter().enumerate() {
            let e = i as i32 + 1;
            diff += e - n;
            square_diff += e * e - n * n;
        }
        let sum = square_diff / diff;
        vec![(sum - diff) / 2, (sum + diff) / 2]
    }

    pub fn find_error_nums_set(nums: Vec<i32>) -> Vec<i32> {
        let mut rep = 0;
        let mut set = HashSet::with_capacity(nums.len() - 1);
        for n in &nums {
            if set.contains(n) {
                rep = *n;
            } else {
                set.insert(*n);
            }
        }
        for i in 1..=nums.len() as i32 {
            if !set.contains(&i) {
                return vec![rep, i];
            }
        }
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_n_1224_should_produce_2_3() {
        let nums = vec![1, 2, 2, 4];
        assert_eq!(Solution::find_error_nums(nums), [2, 3]);
    }

    #[test]
    fn example2_ns_11_should_produce_1_2() {
        let nums = vec![1, 1];
        assert_eq!(Solution::find_error_nums(nums), [1, 2]);
    }
}
