#![allow(dead_code)]

/// ### 5627. Stone Game VII
///
/// Alice and Bob take turns playing a game, with __Alice starting first__.
///
/// There are `n` stones arranged in a row.
/// On each player's turn, they can __remove__ either the leftmost stone or the rightmost stone
/// from the row and receive points equal to the __sum__ of the remaining stones' values in the row.
/// The winner is the one with the higher score when there are no stones left to remove.
///
/// Bob found that he will always lose this game (poor Bob, he always loses),
/// so he decided to __minimize the score's difference__.
/// Alice's goal is to __maximize the difference__ in the score.
///
/// Given an array of integers `stones` where `stones[i]` represents the value of the `i`th stone __from the left__,
/// return _the __difference__ in Alice and Bob's score if they both play __optimally___.
///
/// __Constraints:__
///
/// - `2 <= stones.length <= 1000`
/// - `1 <= stones[i] <= 1000`
///
/// https://leetcode.com/contest/weekly-contest-219/problems/stone-game-vii/
struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        fn dfs(s: &[i32], l: usize, r: usize, sum: i32, dp: &mut Vec<Vec<i32>>) -> i32 {
            if l == r {
                0
            } else if dp[l][r] != 0 {
                dp[l][r]
            } else {
                let l_res = sum - s[l] - dfs(&s, l + 1, r, sum - s[l], dp);
                let r_res = sum - s[r] - dfs(&s, l, r - 1, sum - s[r], dp);
                dp[l][r] = l_res.max(r_res);

                dp[l][r]
            }
        }

        let n = stones.len();
        dfs(&stones, 0, n - 1, stones.iter().sum(), &mut vec![vec![0; n]; n])
    }

    pub fn stone_game_vii_brute_force_2(stones: Vec<i32>) -> i32 {
        fn dfs(s: &[i32], l: usize, r: usize, sum: i32) -> i32 {
            if l == r {
                0
            } else {
                let l_res = sum - s[l] - dfs(&s, l + 1, r, sum - s[l]);
                let r_res = sum - s[r] - dfs(&s, l, r - 1, sum - s[r]);

                l_res.max(r_res)
            }
        }

        dfs(&stones, 0, stones.len() - 1, stones.iter().sum())
    }

    pub fn stone_game_vii_brute_force(stones: Vec<i32>) -> i32 {
        fn dfs(s: &[i32], sum: i32) -> i32 {
            if s.is_empty() {
                0
            } else {
                let l_res = sum - s[0] - dfs(&s[1..], sum - s[0]);
                let r = s.len() - 1;
                let r_res = sum - s[r] - dfs(&s[..r], sum - s[r]);

                l_res.max(r_res)
            }
        }

        dfs(&stones, stones.iter().sum())
    }

    /// https://leetcode.com/problems/stone-game-vii/discuss/970363/Python-Top-Down-and-Bottom-Up-DP-explained
    pub fn stone_game_vii_dp(stones: Vec<i32>) -> i32 {
        let mut presum = vec![0];
        presum.extend(&stones);
        for i in 1..presum.len() { presum[i] += presum[i - 1]; }
        let score = |i, j| presum[j + 1] - presum[i];

        let mut dp = vec![vec![0; stones.len()]; stones.len()];

        for i in (0..stones.len()).rev() {
            for j in (i + 1)..stones.len() {
                dp[i][j] = (score(i + 1, j) - dp[i + 1][j]).max(score(i, j - 1) - dp[i][j - 1]);
            }
        }

        dp[0][stones.len() - 1]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::stone_game_vii(vec![5, 3, 1, 4, 2]), 6);
        // Example 1:
        //
        // Input: stones = [5,3,1,4,2]
        // Output: 6
        // Explanation:
        // - Alice removes 2 and gets 5 + 3 + 1 + 4 = 13 points. Alice = 13, Bob = 0, stones = [5,3,1,4].
        // - Bob removes 5 and gets 3 + 1 + 4 = 8 points. Alice = 13, Bob = 8, stones = [3,1,4].
        // - Alice removes 3 and gets 1 + 4 = 5 points. Alice = 18, Bob = 8, stones = [1,4].
        // - Bob removes 1 and gets 4 points. Alice = 18, Bob = 12, stones = [4].
        // - Alice removes 4 and gets 0 points. Alice = 18, Bob = 12, stones = [].
        // The score difference is 18 - 12 = 6.
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::stone_game_vii(vec![7, 90, 5, 1, 100, 10, 10, 2]), 122);
        // Example 2:
        //
        // Input: stones = [7,90,5,1,100,10,10,2]
        // Output: 122
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
