#![allow(dead_code)]
//! \#1423. Maximum Points You Can Obtain from Cards
//! ================================================
//!
//! There are several cards __arranged in a row__, and each card has an associated number of points.
//! The points are given in the integer array `cardPoints`.
//!
//! In one step, you can take one card from the beginning or from the end of the row.
//! You have to take exactly `k` cards.
//!
//! Your score is the sum of the points of the cards you have taken.
//!
//! Given the integer array `cardPoints` and the integer `k`, return the _maximum score_ you can obtain.
//!
//! __Constraints:__
//!
//! - `1 <= cardPoints.length <= 100_000`
//! - `1 <= cardPoints[i] <= 10_000`
//! - `1 <= k <= cardPoints.length`
//!
//! <https://leetcode.com/problems/maximum-points-you-can-obtain-from-cards>

pub struct Solution;
impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let len = card_points.len() - k as usize;
        let mut curr = card_points.iter().take(len).sum::<i32>();
        let mut total = curr;
        let mut min = curr;
        for l in 0..card_points.len() - len {
            let r = l + len;
            curr = curr - card_points[l] + card_points[r];
            total += card_points[r];
            min = min.min(curr);
        }
        total - min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cp_2_k_1() {
        assert_eq!(Solution::max_score(vec![2], 1), 2);
    }
    #[test]
    fn cp_1_2_3_4_5_6_1_k_3() {
        let cp = vec![1, 2, 3, 4, 5, 6, 1];
        assert_eq!(Solution::max_score(cp, 3), 12);
        // Explanation: After the first step, your score will always be 1.
        // However, choosing the rightmost card first will maximize your total score.
        // The optimal strategy is to take the three cards on the right, giving a final score of 1 + 6 + 5 = 12.
    }
    #[test]
    fn cp_2_2_2_k_2() {
        let cp = vec![2, 2, 2];
        assert_eq!(Solution::max_score(cp, 2), 4);
        // Explanation: Regardless of which two cards you take, your score will always be 4.
    }
    #[test]
    fn cp_9_7_7_9_7_7_9_k_7() {
        let cp = vec![9, 7, 7, 9, 7, 7, 9];
        assert_eq!(Solution::max_score(cp, 7), 55);
        // Explanation: You have to take all the cards.
        // Your score is the sum of points of all cards.
    }
}
