#![allow(dead_code)]

/// Shuffle an Array
/// ================
///
/// Given an integer array `nums`, design an algorithm to randomly shuffle the array.
/// All permutations of the array should be __equally likely__ as a result of the shuffling.
///
/// Implement the `Solution` class:
///
/// - `Solution(int[] nums)` Initializes the object with the integer array nums.
/// - `int[] reset()` Resets the array to its original configuration and returns it.
/// - `int[] shuffle()` Returns a random shuffling of the array.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 200`
/// - `-1_000_000 <= nums[i] <= 1_000_000`
/// - All the elements of `nums` are __unique__.
/// - At most `50_000` calls __in total__ will be made to `reset` and `shuffle`.
///
/// https://leetcode.com/explore/challenge/card/july-leetcoding-challenge-2021/610/week-3-july-15th-july-21st/3820/
struct Solution {
    nums: Vec<i32>,
    rng: rand::rngs::StdRng,
}
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        use rand::prelude::*;
        use rand::rngs::StdRng;

        let rng = StdRng::from_entropy();
        Self { nums, rng }
    }

    /** Resets the array to its original configuration and return it. */
    fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    /** Returns a random shuffling of the array. */
    fn shuffle(&mut self) -> Vec<i32> {
        use rand::Rng;

        let mut nums = self.nums.clone();
        for i in 0..nums.len() {
            let j = self.rng.gen_range(i..nums.len());
            nums.swap(i, j);
        }
        nums
    }
}

struct SolutionBuiltin {
    nums: Vec<i32>,
    rng: rand::rngs::ThreadRng,
}
impl SolutionBuiltin {
    fn new(nums: Vec<i32>) -> Self {
        use rand::thread_rng;

        let rng = thread_rng();
        Self { nums, rng }
    }

    /** Resets the array to its original configuration and return it. */
    fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    /** Returns a random shuffling of the array. */
    fn shuffle(&mut self) -> Vec<i32> {
        use rand::seq::SliceRandom;

        let mut nums = self.nums.clone();
        nums.shuffle(&mut self.rng);
        nums
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let nums_set = nums.iter().collect::<HashSet<_>>();
        let mut s = Solution::new(nums.clone());

        let mut rs = vec![];

        for _ in 0..10 {
            let r = s.shuffle();
            assert_ne!(r, nums);
            assert!(!rs.contains(&r));
            assert_eq!(r.len(), nums.len());
            assert_eq!(r.iter().collect::<HashSet<_>>(), nums_set);

            assert_eq!(s.reset(), nums);

            rs.push(r);
        }
    }
}
