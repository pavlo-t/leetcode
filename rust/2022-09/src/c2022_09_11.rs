#![allow(dead_code)]
//! \#1383. Maximum Performance of a Team
//! =====================================
//!
//! <https://leetcode.com/problems/maximum-performance-of-a-team>
//!
//! You are given two integers `n` and `k` and two integer arrays `speed` and `efficiency` both of length `n`.
//! There are `n` engineers numbered from `1` to `n`.
//! `speed[i]` and `efficiency[i]` represent the speed and efficiency of the `i`th engineer respectively.
//!
//! Choose __at most__ `k` different engineers out of the `n` engineers to form a team with the maximum __performance__.
//!
//! The performance of a team is the sum of their engineers' speeds multiplied by the minimum efficiency
//! among their engineers.
//!
//! Return _the maximum performance of this team_.
//! Since the answer can be a huge number, return it __modulo__ `1_000_000_007`.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_09::c2022_09_11::*;
//! let speed      = vec![2, 10, 3, 1, 5, 8];
//! let efficiency = vec![5,  4, 3, 9, 7, 2];
//! assert_eq!(Solution::max_performance(6, speed, efficiency, 2), 60);
//! ```
//!
//! __Explanation:__
//! We have the maximum performance of the team by selecting engineer `2` (with `speed=10` and `efficiency=4`) and
//! engineer `5` (with `speed=5` and `efficiency=7`).
//! That is, `performance = (10 + 5) * min(4, 7) = 60`.
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_09::c2022_09_11::*;
//! let speed      = vec![2, 10, 3, 1, 5, 8];
//! let efficiency = vec![5,  4, 3, 9, 7, 2];
//! assert_eq!(Solution::max_performance(6, speed, efficiency, 3), 68);
//! ```
//!
//! __Explanation:__
//! This is the same example as the first but `k = 3`.
//! We can select engineer `1`, engineer `2` and engineer `5` to get the maximum performance of the team.
//! That is, `performance = (2 + 10 + 5) * min(5, 4, 7) = 68`.
//!
//! ###### Example 3
//!
//! ```
//! # use c2022_09::c2022_09_11::*;
//! let speed      = vec![2, 10, 3, 1, 5, 8];
//! let efficiency = vec![5,  4, 3, 9, 7, 2];
//! assert_eq!(Solution::max_performance(6, speed, efficiency, 4), 72);
//! ```
//!
//! ##### Constraints
//!
//! - `1 <= k <= n <= 100_000`
//! - `speed.length == n`
//! - `efficiency.length == n`
//! - `1 <= speed[i] <= 100_000`
//! - `1 <= efficiency[i] <= 100_000_000`

pub struct Solution;
impl Solution {
    pub fn max_performance(_n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut efficiency_speed = efficiency.into_iter().zip(speed).collect::<Vec<_>>();
        efficiency_speed.sort_unstable_by_key(|(efficiency, _)| -efficiency);

        let k = k as usize;
        let mut total_speed = 0i64;
        let mut result = 0i64;
        let mut min_heap = BinaryHeap::new();

        for (efficiency, speed) in efficiency_speed {
            total_speed += speed as i64;
            min_heap.push(Reverse(speed));
            if min_heap.len() > k {
                total_speed -= min_heap.pop().unwrap().0 as i64;
            }
            result = result.max(total_speed * efficiency as i64);
        }

        (result % 1_000_000_007i64) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let speed = vec![2, 10, 3, 1, 5, 8];
        let efficiency = vec![5, 4, 3, 9, 7, 2];
        assert_eq!(Solution::max_performance(6, speed, efficiency, 2), 60);
    }

    #[test]
    fn example_2() {
        let speed = vec![2, 10, 3, 1, 5, 8];
        let efficiency = vec![5, 4, 3, 9, 7, 2];
        assert_eq!(Solution::max_performance(6, speed, efficiency, 3), 68);
    }

    #[test]
    fn example_3() {
        let speed = vec![2, 10, 3, 1, 5, 8];
        let efficiency = vec![5, 4, 3, 9, 7, 2];
        assert_eq!(Solution::max_performance(6, speed, efficiency, 4), 72);
    }

    //#[ignore]
    #[test]
    fn s_100000x100000_e_100000x100000000_k_99999() {
        let n = 100_000;
        let speed = vec![100_000; n];
        let efficiency = vec![100_000_000; n];
        let n = n as i32;
        let k = 99_999;
        assert_eq!(Solution::max_performance(n, speed, efficiency, k), 70049);
    }

    //#[ignore]
    #[test]
    fn s_100000x100000_e_100000x100000000_k_100000() {
        let n = 100_000;
        let speed = vec![100_000; n];
        let efficiency = vec![100_000_000; n];
        let n = n as i32;
        let k = 100_000;
        assert_eq!(Solution::max_performance(n, speed, efficiency, k), 49);
    }
}
