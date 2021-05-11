#![allow(dead_code)]
/// Maximum Points You Can Obtain from Cards
/// ========================================
///
/// There are several cards __arranged in a row__, and each card has an associated number of points.
/// The points are given in the integer array `cardPoints`.
///
/// In one step, you can take one card from the beginning or from the end of the row.
/// You have to take exactly `k` cards.
///
/// Your score is the sum of the points of the cards you have taken.
///
/// Given the integer array `cardPoints` and the integer `k`,
/// return the _maximum score_ you can obtain.
///
/// __Constraints:__
///
/// - `1 <= cardPoints.length <= 100_000`
/// - `1 <= cardPoints[i] <= 10_000`
/// - `1 <= k <= cardPoints.length`
///
/// https://leetcode.com/explore/featured/card/may-leetcoding-challenge-2021/599/week-2-may-8th-may-14th/3739/
struct Solution;
impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let r_sum: i32 = card_points.iter().rev().take(k).map(|&i| i).sum();
        (0..k)
            .map(|l| (l, card_points.len() - k + l))
            .fold((r_sum, r_sum), |(max, sum), (l, r)| {
                let sum = sum - card_points[r] + card_points[l];
                (max.max(sum), sum)
            })
            .0
    }

    pub fn max_score_my_2vectors(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let f = |mut acc: Vec<i32>, c| {
            acc.push(*acc.last().unwrap_or(&0) + c);
            acc
        };
        let l_sums = card_points.iter().take(k).fold(vec![0], f);
        let r_sums = card_points.iter().rev().take(k).fold(vec![0], f);

        (0..=k).fold(0, |rsf, i| rsf.max(l_sums[k - i] + r_sums[i]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let card_points = vec![1, 2, 3, 4, 5, 6, 1];
        let k = 3;
        assert_eq!(Solution::max_score(card_points, k), 12);
        // Explanation:
        // After the first step, your score will always be 1.
        // However, choosing the rightmost card first will maximize your total score.
        // The optimal strategy is to take the three cards on the right,
        // giving a final score of 1 + 6 + 5 = 12.
    }
    #[test]
    fn example2() {
        let card_points = vec![2, 2, 2];
        let k = 2;
        assert_eq!(Solution::max_score(card_points, k), 4);
        // Explanation: Regardless of which two cards you take, your score will always be 4.
    }
    #[test]
    fn example3() {
        let card_points = vec![9, 7, 7, 9, 7, 7, 9];
        let k = 7;
        assert_eq!(Solution::max_score(card_points, k), 55);
        // Explanation: You have to take all the cards.
        // Your score is the sum of points of all cards.
    }
    #[test]
    fn example4() {
        let card_points = vec![1, 1000, 1];
        let k = 1;
        assert_eq!(Solution::max_score(card_points, k), 1);
        // Explanation: You cannot take the card in the middle. Your best score is 1.
    }
    #[test]
    fn example5() {
        let card_points = vec![1, 79, 80, 1, 1, 1, 200, 1];
        let k = 3;
        assert_eq!(Solution::max_score(card_points, k), 202);
    }

    mod performance {
        use super::*;

        // #[ignore]
        #[test]
        fn cps1x100k_k100k_produces_100k() {
            let k = 100_000;
            let card_points = vec![1; k as usize];
            assert_eq!(Solution::max_score(card_points, k), k);
        }
    }
}
