#![allow(dead_code)]
//! \#128. Longest Consecutive Sequence
//! ===================================
//!
//! Given an unsorted array of integers `nums`, return _the length of the longest consecutive elements sequence_.
//!
//! You must write an algorithm that runs in `O(n)` time.
//!
//! __Constraints:__
//!
//! - `0 <= nums.length <= 100_000`
//! - `-1_000_000_000 <= nums[i] <= 1_000_000_000`
//!
//! <https://leetcode.com/problems/longest-consecutive-sequence>

pub struct Solution;
impl Solution {
    pub fn longest_consecutive_my_map_rec_with_memo(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut idxs: HashMap<i32, usize> = HashMap::with_capacity(n);
        for (i, &n) in nums.iter().enumerate() {
            *idxs.entry(n).or_default() = i;
        }

        fn rec(i: usize, nums: &[i32], idxs: &HashMap<i32, usize>, memo: &mut Vec<i32>) -> i32 {
            if memo[i] == -1 {
                memo[i] = if let Some(&j) = idxs.get(&(nums[i] + 1)) {
                    1 + rec(j, nums, idxs, memo)
                } else {
                    1
                };
            }
            memo[i]
        }

        let mut memo = vec![-1; n];
        (0..n)
            .map(|i| rec(i, &nums, &idxs, &mut memo))
            .max()
            .unwrap()
    }
    /// after reading <https://leetcode.com/problems/longest-consecutive-sequence/solution>
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        if nums.len() == 0 {
            return 0;
        }

        let mut num_set = HashSet::with_capacity(nums.len());
        for &n in nums.iter() {
            num_set.insert(n);
        }

        let mut longest_streak = 0;
        for mut n in nums {
            if !num_set.contains(&(n - 1)) {
                let mut curr_streak = 1;
                while num_set.contains(&(n + 1)) {
                    n += 1;
                    curr_streak += 1;
                }
                longest_streak = longest_streak.max(curr_streak);
            }
        }
        longest_streak
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_empty() { assert_eq!(Solution::longest_consecutive(vec![]), 0); }
    #[rustfmt::skip] #[test] fn n_1() { assert_eq!(Solution::longest_consecutive(vec![1]), 1); }
    #[rustfmt::skip] #[test] fn n_1_2() { assert_eq!(Solution::longest_consecutive(vec![1,2]), 2); }
    #[rustfmt::skip] #[test] fn n_2_1() { assert_eq!(Solution::longest_consecutive(vec![2,1]), 2); }
    #[rustfmt::skip] #[test] fn n_1_1() { assert_eq!(Solution::longest_consecutive(vec![1,1]), 1); }
    #[rustfmt::skip] #[test] fn n_1_3() { assert_eq!(Solution::longest_consecutive(vec![1,3]), 1); }

    #[test]
    fn n_100_4_200_1_3_2() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        // Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
    }
    #[test]
    fn n_0_3_7_2_5_8_4_6_0_1() {
        let n = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(Solution::longest_consecutive(n), 9);
    }

    /// Got stack overflow with default stack
    #[test]
    fn n_0_until_100_000() {
        let child = std::thread::Builder::new()
            .stack_size(2usize.pow(31))
            .spawn(|| {
                let n = (0..100_000).collect();
                assert_eq!(Solution::longest_consecutive(n), 100_000);
            })
            .unwrap();
        child.join().unwrap();
    }
}
