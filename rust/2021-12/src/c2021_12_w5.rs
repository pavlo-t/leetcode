#![allow(dead_code)]
use std::collections::HashMap;
/// 1244. Design A Leaderboard
/// ==========================
///
/// Design a Leaderboard class, which has 3 functions:
///
/// - `addScore(playerId, score)`: Update the leaderboard by adding `score` to the given player's score.
///   If there is no player with such id in the leaderboard, add him to the leaderboard with the given `score`.
/// - `top(K)`: Return the score sum of the top `K` players.
/// - `reset(playerId)`: Reset the score of the player with the given id to 0
///   (in other words erase it from the leaderboard).
///   It is guaranteed that the player was added to the leaderboard before calling this function.
///
/// Initially, the leaderboard is empty.
///
/// __Constraints:__
///
/// - `1 <= playerId, K <= 10000`
/// - It's guaranteed that `K` is less than or equal to the current number of players.
/// - `1 <= score <= 100`
/// - There will be at most `1000` function calls.
///
/// https://leetcode.com/problems/design-a-leaderboard/
#[derive(Debug, Default)]
struct Leaderboard {
    scores: HashMap<i32, i32>,
    sorted: Option<Vec<i32>>,
}
impl Leaderboard {
    fn new() -> Self {
        Self::default()
    }
    fn add_score(&mut self, player_id: i32, score: i32) {
        *self.scores.entry(player_id).or_default() += score;
        self.sorted = None;
    }
    fn top(&mut self, k: i32) -> i32 {
        if let Some(sorted) = &self.sorted {
            sorted.iter().take(k as usize).sum()
        } else {
            let mut sorted = self.scores.values().map(|&v| v).collect::<Vec<_>>();
            sorted.sort_unstable_by(|a, b| b.cmp(a));
            let result = sorted.iter().take(k as usize).sum();
            self.sorted = Some(sorted);
            result
        }
    }
    fn reset(&mut self, player_id: i32) {
        self.scores.remove(&player_id);
        self.sorted = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut leaderboard = Leaderboard::new();
        leaderboard.add_score(1, 73); // leaderboard = [[1,73]];
        leaderboard.add_score(2, 56); // leaderboard = [[1,73],[2,56]];
        leaderboard.add_score(3, 39); // leaderboard = [[1,73],[2,56],[3,39]];
        leaderboard.add_score(4, 51); // leaderboard = [[1,73],[2,56],[3,39],[4,51]];
        leaderboard.add_score(5, 4); // leaderboard = [[1,73],[2,56],[3,39],[4,51],[5,4]];
        assert_eq!(leaderboard.top(1), 73);
        assert_eq!(leaderboard.top(3), 180);
        leaderboard.reset(1); // leaderboard = [[2,56],[3,39],[4,51],[5,4]];
        leaderboard.reset(2); // leaderboard = [[3,39],[4,51],[5,4]];
        leaderboard.add_score(2, 51); // leaderboard = [[2,51],[3,39],[4,51],[5,4]];
        assert_eq!(leaderboard.top(3), 141); // returns 141 = 51 + 51 + 39;
    }
}
