#![allow(dead_code)]
/// 1510. Stone Game IV
/// ===================
///
/// Alice and Bob take turns playing a game, with Alice starting first.
///
/// Initially, there are `n` stones in a pile.
/// On each player's turn, that player makes _a move_ consisting of
/// removing __any__ non-zero __square number__ of stones in the pile.
///
/// Also, if a player cannot make a move, he/she loses the game.
///
/// Given a positive integer `n`, return `true` if and only if Alice wins the game otherwise return `false`,
/// assuming both players play optimally.
///
/// __Constraints:__
///
/// - `1 <= n <= 100_000`
///
/// https://leetcode.com/problems/stone-game-iv/
struct Solution;
impl Solution {
    /// from my last solution in scala
    pub fn winner_square_game(n: i32) -> bool {
        let n = n as usize;
        let mut dp = vec![false; n + 1];
        for i in 0..=n {
            if !dp[i] {
                let mut k = 1;
                while i + k * k <= n {
                    dp[i + k * k] = true;
                    k += 1;
                }
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_1() {
        assert_eq!(Solution::winner_square_game(1), true);
        // Explanation: Alice can remove 1 stone winning the game because Bob doesn't have any moves.
    }
    #[test]
    fn n_2() {
        assert_eq!(Solution::winner_square_game(2), false);
        // Explanation: Alice can only remove 1 stone, after that Bob removes the last one winning the game (2 -> 1 -> 0).
    }
    #[test]
    fn n_3() {
        assert_eq!(Solution::winner_square_game(3), true);
    }
    #[test]
    fn n_4() {
        assert_eq!(Solution::winner_square_game(4), true);
        // Explanation: n is already a perfect square, Alice can win with one move, removing 4 stones (4 -> 0).
    }

    #[test]
    fn n_100000() {
        assert_eq!(Solution::winner_square_game(100000), true);
    }
    #[test]
    fn n_99999() {
        assert_eq!(Solution::winner_square_game(99999), true);
    }
}
