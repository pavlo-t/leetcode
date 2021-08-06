#![allow(dead_code)]
/// Stone Game
/// ==========
///
/// Alex and Lee play a game with piles of stones.
/// There are an even number of piles __arranged in a row__,
/// and each pile has a positive integer number of stones `piles[i]`.
///
/// The objective of the game is to end with the most stones.
/// The total number of stones is odd, so there are no ties.
///
/// Alex and Lee take turns, with Alex starting first.
/// Each turn, a player takes the entire pile of stones from either the beginning or the end of the row.
/// This continues until there are no more piles left, at which point the person with the most stones wins.
///
/// Assuming Alex and Lee play optimally, return `True` if and only if Alex wins the game.
///
/// __Constraints:__
///
/// - `2 <= piles.length <= 500`
/// - `piles.length` is even.
/// - `1 <= piles[i] <= 500`
/// - `sum(piles)` is odd.
///
/// https://leetcode.com/explore/featured/card/august-leetcoding-challenge-2021/613/week-1-august-1st-august-7th/3870/
struct Solution;
impl Solution {
    pub fn stone_game(_piles: Vec<i32>) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let piles = vec![5, 3, 4, 5];
        assert!(Solution::stone_game(piles));
        // Explanation:
        // Alex starts first, and can only take the first 5 or the last 5.
        // Say he takes the first 5, so that the row becomes [3, 4, 5].
        // If Lee takes 3, then the board is [4, 5], and Alex takes 5 to win with 10 points.
        // If Lee takes the last 5, then the board is [3, 4], and Alex takes 4 to win with 9 points.
        // This demonstrated that taking the first 5 was a winning move for Alex, so we return true.
    }
}
