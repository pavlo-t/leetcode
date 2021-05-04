#![allow(dead_code)]
/// Number of Connected Components in an Undirected Graph
/// =====================================================
///
/// You have a graph of `n` nodes.
/// You are given an integer `n` and an array `edges` where `edges[i] = [ai, bi]` indicates that
/// there is an edge between `ai` and `bi` in the graph.
///
/// Return _the number of connected components in the graph_.
///
/// __Constraints:__
///
/// - `1 <= n <= 2000`
/// - `1 <= edges.length <= 5000`
/// - `edges[i].length == 2`
/// - `0 <= ai <= bi < n`
/// - `ai != bi`
/// - There are no repeated edges.
///
/// https://leetcode.com/explore/featured/card/may-leetcoding-challenge-2021/598/week-1-may-1st-may-7th/3727/
struct Solution;
impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        fn dfs(i: usize, tree: &[Vec<usize>], seen: &mut [bool]) {
            if !seen[i] {
                seen[i] = true;
                tree[i].iter().for_each(|&j| {
                    dfs(j, tree, seen);
                });
            }
        }

        let mut tree = vec![Vec::new(); n as usize];
        for e in edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            tree[a].push(b);
            tree[b].push(a);
        }
        let mut seen = vec![false; n as usize];

        (0..n as usize)
            .filter_map(|i| {
                if seen[i] {
                    None
                } else {
                    Some(dfs(i, &tree, &mut seen))
                }
            })
            .count() as i32
    }

    pub fn count_components_my_v1(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;

        let mut tree = HashMap::new();
        for e in edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            tree.entry(a).or_insert(Vec::new()).push(b);
            tree.entry(b).or_insert(Vec::new()).push(a);
        }
        let mut seen = vec![false; n as usize];

        fn dfs(i: usize, tree: &HashMap<usize, Vec<usize>>, seen: &mut [bool]) -> bool {
            if seen[i] {
                false
            } else {
                seen[i] = true;
                tree.get(&i).unwrap_or(&Vec::new()).iter().for_each(|&j| {
                    dfs(j, tree, seen);
                });
                true
            }
        }

        (0..n as usize)
            .map(|i| dfs(i, &tree, &mut seen))
            .filter(|&r| r)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {
        ($x:tt; $n:tt) => { vec![vec!$x; $n] };
        ($($x:tt),*) => { vec![$(vec!$x),*] };
    }

    #[test]
    fn example1() {
        let n = 5;
        let edges = vv![[0, 1], [1, 2], [3, 4]];
        assert_eq!(Solution::count_components(n, edges), 2);
        // Explanation:
        // 0 - 1   3
        //     |   |
        //     2   4
    }
    #[test]
    fn example2() {
        let n = 5;
        let edges = vv![[0, 1], [1, 2], [2, 3], [3, 4]];
        assert_eq!(Solution::count_components(n, edges), 1);
        // Explanation:
        // 0 - 1   3
        //     | / |
        //     2   4
    }
}
