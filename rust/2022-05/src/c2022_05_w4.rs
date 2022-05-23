#![allow(dead_code)]
/// \#277. Find the Celebrity
/// =========================
///
/// Suppose you are at a party with `n` people labeled from `0` to `n - 1` and among them,
/// there may exist one celebrity.
/// The definition of a celebrity is that all the other `n - 1` people know the celebrity,
/// but the celebrity does not know any of them.
///
/// Now you want to find out who the celebrity is or verify that there is not one.
/// The only thing you are allowed to do is ask questions like:
/// "Hi, A. Do you know B?" to get information about whether A knows B.
/// You need to find out the celebrity (or verify there is not one)
/// by asking as few questions as possible (in the asymptotic sense).
///
/// You are given a helper function `bool knows(a, b)` that tells you whether A knows B.
/// Implement a function `int findCelebrity(n)`.
/// There will be exactly one celebrity if they are at the party.
///
/// Return _the celebrity's label if there is a celebrity at the party_.
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
/// __Follow up:__ If the maximum number of allowed calls to the API `knows` is `3 * n`,
/// could you find a solution without exceeding the maximum number of calls?
///
/// https://leetcode.com/problems/find-the-celebrity/
#[derive(Debug)]
struct Solution {
    network: Vec<Vec<i32>>,
}
impl Solution {
    fn new(network: Vec<Vec<i32>>) -> Self {
        Self { network }
    }
    fn knows(&self, a: i32, b: i32) -> bool {
        self.network[a as usize][b as usize] == 1
    }

    pub fn find_celebrity(&self, n: i32) -> i32 {
        use std::collections::HashSet;

        let mut candidates = (0..n).collect::<HashSet<_>>();
        for a in 0..(n - 1) {
            for b in a + 1..n {
                if candidates.is_empty() {
                    return -1;
                }
                if self.knows(a, b) {
                    candidates.remove(&a);
                } else {
                    candidates.remove(&b);
                }
            }
        }
        let a = n - 1;
        for b in 0..(n - 1) {
            if candidates.is_empty() {
                return - 1;
            }
            if self.knows(a, b) {
                candidates.remove(&a);
            } else {
                candidates.remove(&b);
            }
        }

        candidates.into_iter().next().unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn g_110_010_111() {
        let s = Solution::new(vv![[1, 1, 0], [0, 1, 0], [1, 1, 1]]);
        assert_eq!(s.find_celebrity(3), 1);
        // Explanation: There are three persons labeled with 0, 1 and 2.
        // graph[i][j] = 1 means person i knows person j,
        // otherwise graph[i][j] = 0 means person i does not know person j.
        // The celebrity is the person labeled as 1 because both 0 and 2 know him but 1 does not know anybody.
    }
    #[test]
    fn g_101_110_011() {
        let s = Solution::new(vv![[1, 0, 1], [1, 1, 0], [0, 1, 1]]);
        assert_eq!(s.find_celebrity(3), -1);
        // Explanation: There is no celebrity.
    }

    #[test]
    fn g_100x100x1() {
        let s = Solution::new(vec![vec![1; 100]; 100]);
        assert_eq!(s.find_celebrity(100), -1);
    }
}
