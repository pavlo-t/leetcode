#![allow(dead_code)]
/// ### 5625. Count of Matches in Tournament
///
/// You are given an integer `n`, the number of teams in a tournament that has strange rules:
///
/// - If the current number of teams is **even**, each team gets paired with another team.
///   A total of `n / 2` matches are played, and `n / 2` teams advance to the next round.
/// - If the current number of teams is **odd**, one team randomly advances in the tournament,
///   and the rest gets paired.
///   A total of `(n - 1) / 2` matches are played,
///   and `(n - 1) / 2 + 1` teams advance to the next round.
///
/// Return _the number of matches played in the tournament until a winner is decided_.
///
///
/// **Constraints:**
///
/// - `1 <= n <= 200`
///
/// https://leetcode.com/contest/weekly-contest-219/problems/count-of-matches-in-tournament/
struct Solution;
impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        if n == 2 {
            1
        } else if n == 1 {
            0
        } else if n % 2 == 1 {
            let m = (n - 1) / 2;
            m + Self::number_of_matches(m + 1)
        } else {
            n / 2 + Self::number_of_matches(n / 2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::number_of_matches(7), 6);
        //Example 1:
        //
        // Input: n = 7
        // Output: 6
        // Explanation: Details of the tournament:
        // - 1st Round: Teams = 7, Matches = 3, and 4 teams advance.
        // - 2nd Round: Teams = 4, Matches = 2, and 2 teams advance.
        // - 3rd Round: Teams = 2, Matches = 1, and 1 team is declared the winner.
        // Total number of matches = 3 + 2 + 1 = 6.
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::number_of_matches(14), 13);
        // Example 2:
        //
        // Input: n = 14
        // Output: 13
        // Explanation: Details of the tournament:
        // - 1st Round: Teams = 14, Matches = 7, and 7 teams advance.
        // - 2nd Round: Teams = 7, Matches = 3, and 4 teams advance.
        // - 3rd Round: Teams = 4, Matches = 2, and 2 teams advance.
        // - 4th Round: Teams = 2, Matches = 1, and 1 team is declared the winner.
        // Total number of matches = 7 + 3 + 2 + 1 = 13.
    }
}
