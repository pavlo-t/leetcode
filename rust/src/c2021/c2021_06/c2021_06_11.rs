#![allow(dead_code)]
/// Stone Game VII
/// ==============
///
/// Alice and Bob take turns playing a game, with __Alice starting first__.
///
/// There are `n` stones arranged in a row.
/// On each player's turn, they can __remove__ either the leftmost stone or the rightmost stone from the row
/// and receive points equal to the __sum__ of the remaining stones' values in the row.
/// The winner is the one with the higher score when there are no stones left to remove.
///
/// Bob found that he will always lose this game (poor Bob, he always loses),
/// so he decided to __minimize the score's difference__.
/// Alice's goal is to __maximize the difference__ in the score.
///
/// Given an array of integers `stones` where `stones[i]` represents the value of the `i`th stone
/// __from the left__, return _the __difference__ in Alice and Bob's score if they both play __optimally___.
///
/// __Constraints:__
///
/// - `n == stones.length`
/// - `2 <= n <= 1000`
/// - `1 <= stones[i] <= 1000`
///
/// https://leetcode.com/explore/featured/card/june-leetcoding-challenge-2021/604/week-2-june-8th-june-14th/3775/
struct Solution;
impl Solution {
    /// Approach 5: Another Approach using Tabulation
    ///
    /// https://leetcode.com/problems/stone-game-vii/solution/
    pub fn stone_game_vii_leetcode_dp_bottom_up_2(stones: Vec<i32>) -> i32 {
        use std::cmp::max;

        let n = stones.len();
        let mut prefix_sum = vec![0; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + stones[i];
        }

        let mut dp = vec![vec![0; n]; n];
        for start in (0..n - 1).rev() {
            for end in start + 1..n {
                let score_remove_first = prefix_sum[end + 1] - prefix_sum[start + 1];
                let score_remove_last = prefix_sum[end] - prefix_sum[start];
                dp[start][end] = max(
                    score_remove_first - dp[start + 1][end],
                    score_remove_last - dp[start][end - 1],
                );
            }
        }

        dp[0][n - 1]
    }

    /// Approach 4: Bottom Up Dynamic Programming - Tabulation
    ///
    /// https://leetcode.com/problems/stone-game-vii/solution/
    pub fn stone_game_vii_leetcode_dp_bottom_up_1(stones: Vec<i32>) -> i32 {
        use std::cmp::max;

        let n = stones.len();
        let mut prefix_sum = vec![0; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + stones[i];
        }

        let mut dp = vec![vec![0; n]; n];
        for length in 2..=n {
            for start in 0..=(n - length) {
                let end = start + length - 1;
                let score_remove_first = prefix_sum[end + 1] - prefix_sum[start + 1];
                let score_remove_last = prefix_sum[end] - prefix_sum[start];
                dp[start][end] = max(
                    score_remove_first - dp[start + 1][end],
                    score_remove_last - dp[start][end - 1],
                );
            }
        }

        dp[0][n - 1]
    }

    /// Approach 1: Brute Force Using Recursion
    ///
    /// https://leetcode.com/problems/stone-game-vii/solution/
    pub fn stone_game_vii_brute_force_leetcode(stones: Vec<i32>) -> i32 {
        fn find_diff(prefix_sum: &[i32], start: usize, end: usize, alice: bool) -> i32 {
            use std::cmp::{max, min};

            if start == end {
                0
            } else {
                let score_remove_first = prefix_sum[end + 1] - prefix_sum[start + 1];
                let score_remove_last = prefix_sum[end] - prefix_sum[start];

                if alice {
                    max(
                        find_diff(prefix_sum, start + 1, end, !alice) + score_remove_first,
                        find_diff(prefix_sum, start, end - 1, !alice) + score_remove_last,
                    )
                } else {
                    min(
                        find_diff(prefix_sum, start + 1, end, !alice) - score_remove_first,
                        find_diff(prefix_sum, start, end - 1, !alice) - score_remove_last,
                    )
                }
            }
        }

        let n = stones.len();
        let mut prefix_sum = vec![0; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + stones[i];
        }

        find_diff(&prefix_sum, 0, n - 1, true).abs()
    }

    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        fn get_diff(l: usize, r: usize, sum: i32, ss: &[i32], cache: &mut Vec<Vec<i32>>) -> i32 {
            if l >= r {
                0
            } else if cache[l][r] >= 0 {
                cache[l][r]
            } else {
                let ld = sum - ss[l] - get_diff(l + 1, r, sum - ss[l], ss, cache);
                let rd = sum - ss[r] - get_diff(l, r - 1, sum - ss[r], ss, cache);
                cache[l][r] = ld.max(rd);
                cache[l][r]
            }
        }

        let n = stones.len();

        get_diff(
            0,
            n - 1,
            stones.iter().sum(),
            &stones,
            &mut vec![vec![-1; n]; n],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s_5_3_1_4_2_produces_6() {
        let stones = vec![5, 3, 1, 4, 2];
        assert_eq!(Solution::stone_game_vii(stones), 6);
        // Explanation:
        // - Alice removes 2 and gets 5 + 3 + 1 + 4 = 13 points. Alice = 13, Bob = 0, stones = [5,3,1,4].
        // - Bob removes 5 and gets 3 + 1 + 4 = 8 points. Alice = 13, Bob = 8, stones = [3,1,4].
        // - Alice removes 3 and gets 1 + 4 = 5 points. Alice = 18, Bob = 8, stones = [1,4].
        // - Bob removes 1 and gets 4 points. Alice = 18, Bob = 12, stones = [4].
        // - Alice removes 4 and gets 0 points. Alice = 18, Bob = 12, stones = [].
        // The score difference is 18 - 12 = 6.
    }
    #[test]
    fn s_7_90_5_1_100_10_10_2_produces_122() {
        let stones = vec![7, 90, 5, 1, 100, 10, 10, 2];
        assert_eq!(Solution::stone_game_vii(stones), 122);
    }

    #[test]
    fn test_stones1to1000() {
        let stones = (1..=1000).collect();
        assert_eq!(Solution::stone_game_vii(stones), 250500);
    }
    #[test]
    fn test_stones1000to1() {
        let stones = (1..=1000).rev().collect();
        assert_eq!(Solution::stone_game_vii(stones), 250500);
    }
    #[test]
    fn test_stones1to500to1() {
        let mut stones: Vec<_> = (1..=500).collect();
        let more_stones: Vec<_> = (1..500).rev().collect();
        stones.extend(&more_stones);
        assert_eq!(Solution::stone_game_vii(stones), 124750);
    }
}
