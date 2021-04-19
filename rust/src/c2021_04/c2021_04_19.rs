#![allow(dead_code)]
/// Combination Sum IV
/// ==================
///
/// Given an array of __distinct__ integers `nums` and a target integer `target`,
/// return _the number of possible combinations that add up to_ `target`.
///
/// The answer is __guaranteed__ to fit in a __32-bit__ integer.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 200`
/// - `1 <= nums[i] <= 1000`
/// - All the elements of `nums` are __unique__.
/// - `1 <= target <= 1000`
///
/// __Follow up:__
/// What if negative numbers are allowed in the given array?
/// How does it change the problem?
/// What limitation we need to add to the question to allow negative numbers?
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/595/week-3-april-15th-april-21st/3713/
struct Solution;
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        use std::collections::HashMap;
        fn count(nums: &[i32], target: i32, cache: &mut HashMap<i32, i32>) -> i32 {
            if target < 0 {
                0
            } else if target == 0 {
                1
            } else if let Some(&r) = cache.get(&target) {
                r
            } else {
                let r = nums.iter().map(|&n| count(nums, target - n, cache)).sum();
                cache.insert(target, r);
                r
            }
        }
        count(&nums, target, &mut HashMap::new())
    }

    pub fn combination_sum4_brute_force(nums: Vec<i32>, target: i32) -> i32 {
        fn count(nums: &[i32], target: i32) -> i32 {
            if target < 0 {
                0
            } else if target == 0 {
                1
            } else {
                nums.iter().map(|&n| count(nums, target - n)).sum()
            }
        }
        count(&nums, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_n123t4_produces_7() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::combination_sum4(nums, 4), 7);
        // Explanation:
        // The possible combination ways are:
        // (1, 1, 1, 1)
        // (1, 1, 2)
        // (1, 2, 1)
        // (1, 3)
        // (2, 1, 1)
        // (2, 2)
        // (3, 1)
        // Note that different sequences are counted as different combinations.
    }
    #[test]
    fn example2_n9t3_produces_0() {
        assert_eq!(Solution::combination_sum4(vec![9], 3), 0);
    }

    mod performance {
        use super::*;

        #[test]
        fn test_n1to200t10_produces_512() {
            let nums = (1..=200).collect();
            assert_eq!(Solution::combination_sum4(nums, 10), 512);
        }
        #[test]
        fn test_n1to200t31_produces_1_073_741_824() {
            let nums = (1..=200).collect();
            assert_eq!(Solution::combination_sum4(nums, 31), 1_073_741_824);
        }
    }
}
