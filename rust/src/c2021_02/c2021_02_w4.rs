#![allow(dead_code)]

/// # Find the Celebrity
///
/// Suppose you are at a party with `n` people (labeled from `0` to `n - 1`), and among them,
/// there may exist one celebrity.
/// The definition of a celebrity is that all the other `n - 1` people know him/her,
/// but he/she does not know any of them.
///
/// Now you want to find out who the celebrity is or verify that there is not one.
/// The only thing you are allowed to do is to ask questions like: "Hi, A. Do you know B?"
/// to get information about whether A knows B.
/// You need to find out the celebrity (or verify there is not one)
/// by asking as few questions as possible (in the asymptotic sense).
///
/// You are given a helper function `bool knows(a, b)` which tells you whether A knows B.
/// Implement a function `int findCelebrity(n)`.
/// There will be exactly one celebrity if he/she is in the party.
/// Return the celebrity's label if there is a celebrity in the party.
/// If there is no celebrity, return `-1`.
///
/// __Constraints:__
///
/// - `n == graph.length`
/// - `n == graph[i].length`
/// - `2 <= n <= 100`
/// - `graph[i][j]` is `0` or `1`.
/// - `graph[i][i] == 1`
///
///
/// __Follow up:__ If the maximum number of allowed calls to the API `knows` is `3 * n`,
/// could you find a solution without exceeding the maximum number of calls?
///
/// https://leetcode.com/explore/featured/card/february-leetcoding-challenge-2021/587/week-4-february-22nd-february-28th/3648/
struct Solution {
    graph: Vec<Vec<i8>>,
}

impl Solution {
    fn knows(&self, a: i32, b: i32) -> bool {
        self.graph[a as usize][b as usize] == 1
    }

    fn is_celebrity(&self, i: i32, n: i32) -> bool {
        for j in 0..n {
            if i == j {
                continue;
            }
            if self.knows(i, j) || !self.knows(j, i) {
                return false;
            }
        }
        true
    }

    pub fn find_celebrity(&self, n: i32) -> i32 {
        let mut candidate = 0;

        for i in 1..n {
            if self.knows(candidate, i) {
                candidate = i;
            }
        }
        if self.is_celebrity(candidate, n) {
            candidate
        } else {
            -1
        }
    }

    /// O(N^2) time
    pub fn find_celebrity_brute_force(&self, n: i32) -> i32 {
        for i in 0..n {
            if self.is_celebrity(i, n) {
                return i;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let graph = vec![vec![1, 1, 0], vec![0, 1, 0], vec![1, 1, 1]];
        let s = Solution { graph };
        assert_eq!(s.find_celebrity(3), 1);
        // Explanation:
        // There are three persons labeled with 0, 1 and 2.
        // graph[i][j] = 1 means person i knows person j,
        // otherwise graph[i][j] = 0 means person i does not know person j.
        // The celebrity is the person labeled as 1 because both 0 and 2 know him but 1 does not know anybody.
    }

    #[test]
    fn example2() {
        let graph = vec![vec![1, 0, 1], vec![1, 1, 0], vec![0, 1, 1]];
        let s = Solution { graph };
        assert_eq!(s.find_celebrity(3), -1);
        // Explanation: There is no celebrity.
    }
}
