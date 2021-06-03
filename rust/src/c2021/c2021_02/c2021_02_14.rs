#![allow(dead_code)]

/// ### Is Graph Bipartite?
///
/// Given an undirected `graph`, return `true` if and only if it is bipartite.
///
/// Recall that a graph is _bipartite_ if we can split its set of nodes into two independent
/// subsets A and B, such that every edge in the graph has one node in A and another node in B.
///
/// The graph is given in the following form: `graph[i]` is a list of indexes `j` for which
/// the edge between nodes `i` and `j` exists.
/// Each node is an integer between `0` and `graph.length - 1`.
/// There are no self edges or parallel edges:
/// `graph[i]` does not contain `i`, and it doesn't contain any element twice.
///
/// __Constraints:__
///
/// - `1 <= graph.length <= 100`
/// - `0 <= graph[i].length < 100`
/// - `0 <= graph[i][j] <= graph.length - 1`
/// - `graph[i][j] != i`
/// - All the values of `graph[i]` are __unique__.
/// - The graph is __guaranteed__ to be __undirected__.
///
/// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/585/week-2-february-8th-february-14th/3639/
struct Solution;


impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        #[derive(Copy, Clone, Eq, PartialEq)]
        enum Subset { NA, A, B }
        use Subset::*;

        let mut subsets = vec![NA; graph.len()];
        let mut work = vec![];

        while let Some((i, _)) = subsets.iter().enumerate().find(|(_, &s)| s == NA) {
            subsets[i] = A;
            work.push(i);
            while let Some(i) = work.pop() {
                let i_sub = subsets[i];
                for &j in graph[i].iter() {
                    let j_sub = subsets[j as usize];
                    if j_sub == NA {
                        subsets[j as usize] = if i_sub == A { B } else { A };
                        work.push(j as usize);
                    } else if j_sub == i_sub {
                        return false;
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_13_02_13_02_is_bipartite() {
        let g = [[1, 3], [0, 2], [1, 3], [0, 2]]
            .iter().map(|v| v.to_vec()).collect();
        assert!(Solution::is_bipartite(g));
        // Explanation: We can divide the vertices into two groups: {0, 2} and {1, 3}.
    }

    #[test]
    fn example2_123_02_013_02_is_not_bipartite() {
        let g = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
        assert!(!Solution::is_bipartite(g));
        // Explanation: We cannot find a way to divide the set of nodes into two independent subsets.
    }

    #[test]
    fn g_with_one_node_is_bipartite() {
        assert!(Solution::is_bipartite(vec![vec![]]));
    }

    #[test]
    fn g_1_0_is_bipartite() {
        let g = [[1], [0]]
            .iter().map(|v| v.to_vec()).collect();
        assert!(Solution::is_bipartite(g));
    }

    #[test]
    fn g_1_02_1_is_bipartite() {
        let g = vec![vec![1], vec![0, 2], vec![1]];
        assert!(Solution::is_bipartite(g));
    }

    #[test]
    fn g_12_02_01_is_not_bipartite() {
        let g = [[1, 2], [0, 2], [0, 1]]
            .iter().map(|v| v.to_vec()).collect();
        assert!(!Solution::is_bipartite(g));
    }

    #[test]
    fn test66_g_is_bipartite() {
        let g = vec![vec![4], vec![], vec![4], vec![4], vec![0, 2, 3]];
        assert!(Solution::is_bipartite(g));
    }

    #[test]
    fn test75_g_is_not_bipartite() {
        let g = vec![
            vec![], vec![2, 4, 6], vec![1, 4, 8, 9], vec![7, 8], vec![1, 2, 8, 9], vec![6, 9],
            vec![1, 5, 7, 8, 9], vec![3, 6, 9], vec![2, 3, 4, 6, 9], vec![2, 4, 5, 6, 7, 8]];
        assert!(!Solution::is_bipartite(g));
    }
}