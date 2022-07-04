#![allow(dead_code)]
//! \#135. Candy
//! ============
//!
//! There are `n` children standing in a line.
//! Each child is assigned a rating value given in the integer array `ratings`.
//!
//! You are giving candies to these children subjected to the following requirements:
//!
//! - Each child must have at least one candy.
//! - Children with a higher rating get more candies than their neighbors.
//!
//! Return _the minimum number of candies you need to have to distribute the candies to the children_.
//!
//! __Constraints:__
//!
//! - `1 <= ratings.length <= 20_000`
//! - `0 <= ratings[i] <= 20_000`
//!
//! <https://leetcode.com/problems/candy>

pub struct Solution;
impl Solution {
    pub fn candy_brute_force(ratings: Vec<i32>) -> i32 {
        fn needs_more(i: usize, ratings: &[i32], candies: &[i32]) -> bool {
            fn needs_because_of(i: usize, o: usize, ratings: &[i32], candies: &[i32]) -> bool {
                ratings[i] > ratings[o] && candies[i] <= candies[o]
            }
            (i > 0 && needs_because_of(i, i - 1, ratings, candies))
                || (i < ratings.len() - 1 && needs_because_of(i, i + 1, ratings, candies))
        }
        let n = ratings.len();
        let mut curr = vec![1; n];
        let mut prev = vec![1; n];
        let mut someone_needed = true;
        while someone_needed {
            someone_needed = false;
            for i in 0..n {
                curr[i] = prev[i];
                if needs_more(i, &ratings, &prev) {
                    someone_needed = true;
                    curr[i] += 1;
                }
            }
            std::mem::swap(&mut curr, &mut prev);
        }

        prev.into_iter().sum()
    }

    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut candies = vec![1; n];
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }
        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] && candies[i] <= candies[i + 1] {
                candies[i] = candies[i + 1] + 1;
            }
        }
        candies.into_iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn r_1() { assert_eq!(Solution::candy(vec![1]), 1); }

    #[rustfmt::skip] #[test] fn r_1_1() { assert_eq!(Solution::candy(vec![1,1]), 2); }
    #[rustfmt::skip] #[test] fn r_2_1() { assert_eq!(Solution::candy(vec![2,1]), 3); }
    #[rustfmt::skip] #[test] fn r_1_2() { assert_eq!(Solution::candy(vec![1,2]), 3); }

    #[rustfmt::skip] #[test] fn r_1_1_1() { assert_eq!(Solution::candy(vec![1,1,1]), 3); }
    #[rustfmt::skip] #[test] fn r_2_1_1() { assert_eq!(Solution::candy(vec![2,1,1]), 4); }
    #[rustfmt::skip] #[test] fn r_1_2_1() { assert_eq!(Solution::candy(vec![1,2,1]), 4); }
    #[rustfmt::skip] #[test] fn r_1_1_2() { assert_eq!(Solution::candy(vec![1,1,2]), 4); }
    #[rustfmt::skip] #[test] fn r_2_2_1() { assert_eq!(Solution::candy(vec![2,2,1]), 4); }
    #[rustfmt::skip] #[test] fn r_1_2_2() { assert_eq!(Solution::candy(vec![1,2,2]), 4); }
    #[rustfmt::skip] #[test] fn r_2_1_2() { assert_eq!(Solution::candy(vec![2,1,2]), 5); }
    #[rustfmt::skip] #[test] fn r_1_0_2() { assert_eq!(Solution::candy(vec![1,0,2]), 5); }
    #[rustfmt::skip] #[test] fn r_1_2_3() { assert_eq!(Solution::candy(vec![1,2,3]), 6); }

    #[rustfmt::skip] #[test] fn r_1_2_3_4_5() { assert_eq!(Solution::candy(vec![1,2,3,4,5]), 15); }
    #[rustfmt::skip] #[test] fn r_1_2_1_2_3() { assert_eq!(Solution::candy(vec![1,2,1,2,3]), 9); }
    #[rustfmt::skip] #[test] fn r_5_4_3_2_1() { assert_eq!(Solution::candy(vec![5,4,3,2,1]), 15); }
    #[rustfmt::skip] #[test] fn r_1_2_1_2_1() { assert_eq!(Solution::candy(vec![1,2,1,2,1]), 7); }
    #[rustfmt::skip] #[test] fn r_1_2_3_2_1() { assert_eq!(Solution::candy(vec![1,2,3,2,1]), 9); }

    #[rustfmt::skip] #[test] fn r_1_4_3_2_1() { assert_eq!(Solution::candy(vec![1,4,3,2,1]), 11); }

    //#[ignore]
    #[test]
    fn r_1_to_20000() {
        let ratings = (1..=20000).collect();
        assert_eq!(Solution::candy(ratings), 200010000);
    }
    //#[ignore]
    #[test]
    fn r_20000_to_1() {
        let ratings = (1..=20000).rev().collect();
        assert_eq!(Solution::candy(ratings), 200010000);
    }
}
