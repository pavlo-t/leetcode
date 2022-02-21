#![allow(dead_code)]
/// 169. Majority Element
/// =====================
///
/// Given an array `nums` of size `n`, return _the majority element_.
///
/// The majority element is the element that appears more than `⌊n / 2⌋` times.
/// You may assume that the majority element always exists in the array.
///
/// __Constraints:__
///
/// - `n == nums.length`
/// - `1 <= n <= 50_000`
/// - `-2**31 <= nums[i] <= 2**31 - 1`
///
/// __Follow-up:__ Could you solve the problem in linear time and in `O(1)` space?
///
/// https://leetcode.com/problems/majority-element/
struct Solution;
impl Solution {
    pub fn majority_element_my_hash_map(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut counts = HashMap::new();
        let half_len = nums.len() / 2;
        for n in nums {
            *counts.entry(n).or_insert(0) += 1;
        }
        for (&n, &count) in counts.iter() {
            if count > half_len {
                return n;
            }
        }
        -1
    }

    /// Approach 6: Boyer-Moore Voting Algorithm
    /// https://leetcode.com/problems/majority-element/solution/
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = 0;
        for n in nums {
            if count == 0 {
                candidate = n;
            }
            count += if n == candidate { 1 } else { -1 };
        }
        candidate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_3_2_3() {
        let n = vec![3, 2, 3];
        assert_eq!(Solution::majority_element(n), 3);
    }
    #[test]
    fn n_2_2_1_1_1_2_2() {
        let n = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(Solution::majority_element(n), 2);
    }
}
