#![allow(dead_code)]
//! \#1059. All Paths from Source Lead to Destination
//! =================================================
//!
//! <https://leetcode.com/problems/all-paths-from-source-lead-to-destination>
//!
//! Given the edges of a directed graph where `edges[i] = [ai, bi]`
//! indicates there is an edge between nodes `ai` and `bi`,
//! and two nodes `source` and `destination` of this graph,
//! determine whether or not all paths starting from `source` eventually,
//! end at `destination`, that is:
//!
//! - At least one path exists from the `source` node to the `destination` node
//! - If a path exists from the `source` node to a node with no outgoing edges,
//!   then that node is equal to `destination`.
//! - The number of possible paths from `source` to `destination` is a finite number.
//!
//! Return `true` if and only if all roads from `source` lead to `destination`.
//!
//! __Constraints:__
//!
//! - `1 <= n <= 10_000`
//! - `0 <= edges.length <= 10_000`
//! - `edges[i].length == 2`
//! - `0 <= ai, bi < n`
//! - `0 <= source < n`
//! - `0 <= destination < n`
//! - The given graph may have self-loops and parallel edges.

pub struct Solution;
impl Solution {
    pub fn leads_to_destination(
        n: i32,
        edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
    ) -> bool {
        let (n, source, destination) = (n as usize, source as usize, destination as usize);

        let tree = edges
            .into_iter()
            .map(|e| (e[0] as usize, e[1] as usize))
            .fold(vec![vec![]; n], |mut tree, (a, b)| {
                tree[a].push(b);
                tree
            });

        fn bts(
            curr: usize,
            destination: usize,
            tree: &Vec<Vec<usize>>,
            seen: &mut Vec<bool>,
        ) -> bool {
            if tree[curr].is_empty() {
                curr == destination
            } else {
                for &next in &tree[curr] {
                    if seen[next] {
                        return false;
                    }
                    seen[next] = true;
                    if !bts(next, destination, tree, seen) {
                        return false;
                    }
                    seen[next] = false;
                }
                true
            }
        }

        let mut seen = vec![false; n];
        seen[source] = true;

        bts(source, destination, &tree, &mut seen)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::iter::once;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn n_3_e_01_02_s_0_d_2() {
        let e = vv![[0, 1], [0, 2]];
        assert_eq!(Solution::leads_to_destination(3, e, 0, 2), false);
        // Explanation: It is possible to reach and get stuck on both node 1 and node 2.
    }
    #[test]
    fn n_4_e_01_03_12_21_s_0_d_3() {
        let e = vv![[0, 1], [0, 3], [1, 2], [2, 1]];
        assert_eq!(Solution::leads_to_destination(4, e, 0, 3), false);
        // Explanation: We have two possibilities: to end at node 3, or to loop over node 1 and node 2 indefinitely.
    }
    #[test]
    fn n_4_e_01_03_13_31_s_0_d_3() {
        let e = vv![[0, 1], [0, 3], [1, 3], [3, 1]];
        assert_eq!(Solution::leads_to_destination(4, e, 0, 3), false);
    }
    #[test]
    fn n_4_e_01_02_13_23_s_0_d_3() {
        let e = vv![[0, 1], [0, 2], [1, 3], [2, 3]];
        assert_eq!(Solution::leads_to_destination(4, e, 0, 3), true);
    }
    #[test]
    fn n_5_e_01_02_13_23_s_0_d_4() {
        let e = vv![[0, 1], [0, 2], [1, 3], [2, 3], [3, 4]];
        assert_eq!(Solution::leads_to_destination(5, e, 0, 4), true);
    }
    #[test]
    fn n_10000_e_0t1_0t2_to_9998t9999_0t9999_s_0_d_9999() {
        let e = (0..9999)
            .map(|a| vec![a, a + 1])
            .chain(once(vec![0, 9999]))
            .collect();
        assert_eq!(Solution::leads_to_destination(10000, e, 0, 9999), true);
    }
    #[test]
    fn n_10000_e_0t9999_repeat_10000_s_0_d_9999() {
        let e = vec![vec![0, 9999]; 10000];
        assert_eq!(Solution::leads_to_destination(10000, e, 0, 9999), true);
    }
}
