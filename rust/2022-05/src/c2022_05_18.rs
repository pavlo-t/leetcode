#![allow(dead_code)]
/// \#1192. Critical Connections in a Network
/// =========================================
///
/// There are `n` servers numbered from `0` to `n - 1` connected by undirected server-to-server `connections`
/// forming a network where `connections[i] = [ai, bi]` represents a connection between servers `ai` and `bi`.
/// Any server can reach other servers directly or indirectly through the network.
///
/// A `critical connection` is a connection that, if removed, will make some servers unable to reach some other server.
///
/// Return all critical connections in the network in any order.
///
/// __Constraints:__
///
/// - `2 <= n <= 100_000`
/// - `n - 1 <= connections.length <= 100_000`
/// - `0 <= ai, bi <= n - 1`
/// - `ai != bi`
/// - There are no repeated connections.
///
/// https://leetcode.com/problems/critical-connections-in-a-network/
struct Solution;
impl Solution {
    /// Took from `rust/2020_2021-09/src/c2021/c2021_04/c2021_04_24.rs`
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        let n = n as usize;

        let mut graph = vec![vec![]; n];
        let mut ranks = vec![None; n];
        let mut result = HashSet::new();

        for connection in connections {
            let (a, b) = (connection[0] as usize, connection[1] as usize);
            graph[a].push(b);
            graph[b].push(a);
            result.insert((a.min(b), a.max(b)));
        }

        fn filter_result(
            node: usize,
            rank: i32,
            graph: &Vec<Vec<usize>>,
            ranks: &mut Vec<Option<i32>>,
            result: &mut HashSet<(usize, usize)>,
        ) -> i32 {
            if let Some(rank) = ranks[node] {
                rank
            } else {
                ranks[node] = Some(rank);
                let mut min_rank = rank + 1;
                for &neighbor in &graph[node] {
                    if ranks[neighbor].filter(|&nr| nr == rank - 1).is_none() {
                        let nr = filter_result(neighbor, rank + 1, graph, ranks, result);
                        if nr <= rank {
                            result.remove(&(node.min(neighbor), node.max(neighbor)));
                        }
                        min_rank = min_rank.min(nr);
                    }
                }
                min_rank
            }
        }

        filter_result(0, 0, &graph, &mut ranks, &mut result);

        result
            .into_iter()
            .map(|(a, b)| vec![a as i32, b as i32])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn n_4_c_01_12_20_13() {
        let c = vv![[0, 1], [1, 2], [2, 0], [1, 3]];
        assert_eq!(Solution::critical_connections(4, c), vv![[1, 3]]);
        // Explanation: [[3,1]] is also accepted.
    }
    #[test]
    fn n_2_c_01() {
        let c = vv![[0, 1]];
        assert_eq!(Solution::critical_connections(2, c), vv![[0, 1]]);
    }
    #[test]
    fn n_3_c_01_12() {
        let c = vv![[0, 1], [1, 2]];
        assert_eq!(Solution::critical_connections(3, c), vv![[0, 1], [1, 2]]);
    }
    #[test]
    fn n_5_c_01_12_20_13_43() {
        let c = vv![[0, 1], [1, 2], [2, 0], [1, 3], [4, 3]];
        assert_eq!(Solution::critical_connections(5, c), vv![[1, 3], [3, 4]]);
    }

    #[test]
    fn n100000connections99999_produces_connections() {
        let child = std::thread::Builder::new()
            .stack_size(2usize.pow(31))
            .spawn(|| {
                let n = 100_000;
                let connections = (0..99999).map(|i| vec![i, i + 1]).collect::<Vec<_>>();

                let e = connections
                    .iter()
                    .map(|c| c.clone())
                    .collect::<HashSet<_>>();
                let result = Solution::critical_connections(n, connections)
                    .iter()
                    .map(|c| c.clone())
                    .collect::<HashSet<_>>();
                assert_eq!(result, e);
            })
            .unwrap();
        child.join().unwrap();
    }

    #[test]
    fn n100000connections199997_produces_empty() {
        let n = 100_000;
        let connections = (0..99999)
            .flat_map(|i| {
                if i == 0 {
                    vec![vec![0, i + 1]]
                } else {
                    vec![vec![0, i + 1], vec![1, i + 1]]
                }
            })
            .collect::<Vec<_>>();

        let e: [Vec<i32>; 0] = [];
        assert_eq!(Solution::critical_connections(n, connections), e);
    }
}
