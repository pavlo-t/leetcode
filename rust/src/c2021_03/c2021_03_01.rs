#![allow(dead_code)]

use std::collections::HashSet;

/// # Distribute Candies
///
/// Alice has `n` candies, where the `ith` candy is of type `candyType[i]`.
/// Alice noticed that she started to gain weight, so she visited a doctor.
///
/// The doctor advised Alice to only eat `n / 2` of the candies she has (`n` is always even).
/// Alice likes her candies very much, and she wants to eat the maximum number
/// of different types of candies while still following the doctor's advice.
///
/// Given the integer array `candyType` of length `n`, return _the __maximum__ number of different
/// types of candies she can eat if she only eats `n / 2` of them_.
///
/// __Constraints:__
///
/// - `n == candyType.length`
/// - `2 <= n <= 10_000`
/// - `n` is even.
/// - `-100_000 <= candyType[i] <= 100_000`
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/588/week-1-march-1st-march-7th/3657/
struct Solution;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let n = candy_type.len();
        let set = candy_type.into_iter().collect::<HashSet<_>>();
        (set.len().min(n / 2)) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let candy_type = vec![1, 1, 2, 2, 3, 3];
        assert_eq!(Solution::distribute_candies(candy_type), 3);
        // Explanation:
        // Alice can only eat 6 / 2 = 3 candies.
        // Since there are only 3 types, she can eat one of each type.
    }

    #[test]
    fn example2() {
        let candy_type = vec![1, 1, 2, 3];
        assert_eq!(Solution::distribute_candies(candy_type), 2);
        // Explanation:
        // Alice can only eat 4 / 2 = 2 candies.
        // Whether she eats types [1,2], [1,3], or [2,3], she still can only eat 2 different types.
    }

    #[test]
    fn example3() {
        let candy_type = vec![6, 6, 6, 6];
        assert_eq!(Solution::distribute_candies(candy_type), 1);
        // Explanation:
        // Alice can only eat 4 / 2 = 2 candies.
        // Even though she can eat 2 candies, she only has 1 type.
    }
}
