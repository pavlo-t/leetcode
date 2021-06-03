#![allow(dead_code)]
/// Critical Connections in a Network
/// =================================
///
/// There are `n` servers numbered from `0` to `n-1` connected by undirected server-to-server
/// `connections` forming a network where `connections[i] = [a, b]` represents a connection between
/// servers `a` and `b`.
/// Any server can reach any other server directly or indirectly through the network.
///
/// A _critical connection_ is a connection that, if removed,
/// will make some server unable to reach some other server.
///
/// Return all critical connections in the network in any order.
///
/// __Constraints:__
///
/// - `1 <= n <= 100_000`
/// - `n-1 <= connections.length <= 100_000`
/// - `connections[i][0] != connections[i][1]`
/// - There are no repeated connections.
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/596/week-4-april-22nd-april-28th/3719/
struct Solution;
impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        let n = n as usize;
        let mut tree = vec![Vec::new(); n];
        let mut bridges = HashSet::new();

        for c in connections {
            let (a, b) = (c[0] as usize, c[1] as usize);
            tree[a].push(b);
            tree[b].push(a);
            bridges.insert((a.min(b), a.max(b)));
        }

        fn dfs(
            node: usize,
            order: i32,
            tree: &[Vec<usize>],
            bridges: &mut HashSet<(usize, usize)>,
            seen: &mut [i32],
        ) -> i32 {
            if seen[node] >= 0 {
                seen[node]
            } else {
                seen[node] = order;
                let mut min_o = order + 1;
                for &c in &tree[node] {
                    if seen[c] != order - 1 {
                        let co = dfs(c as usize, order + 1, tree, bridges, seen);
                        min_o = min_o.min(co);
                        if co <= order {
                            bridges.remove(&(node.min(c), node.max(c)));
                        }
                    }
                }
                min_o
            }
        }

        dfs(0, 0, &tree, &mut bridges, &mut vec![-2; n]);

        bridges
            .into_iter()
            .map(|(a, b)| vec![a as i32, b as i32])
            .collect()
    }

    pub fn critical_connections_ranks_option_int(
        n: i32,
        connections: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        let n = n as usize;
        let mut tree = vec![Vec::new(); n];
        let mut bridges = HashSet::new();

        for c in connections {
            let (a, b) = (c[0] as usize, c[1] as usize);
            tree[a].push(b);
            tree[b].push(a);
            bridges.insert((a.min(b), a.max(b)));
        }

        fn dfs(
            n: usize,
            r: i32,
            tree: &[Vec<usize>],
            bridges: &mut HashSet<(usize, usize)>,
            seen: &mut [Option<i32>],
        ) -> i32 {
            if let Some(r) = seen[n] {
                r
            } else {
                seen[n] = Some(r);
                let mut min_r = r + 1;
                for &c in &tree[n] {
                    if seen[c].is_none() || seen[c].unwrap() != r - 1 {
                        let cr = dfs(c as usize, r + 1, tree, bridges, seen);
                        min_r = min_r.min(cr);
                        if cr <= r {
                            bridges.remove(&(n.min(c), n.max(c)));
                        }
                    }
                }
                min_r
            }
        }

        dfs(0, 0, &tree, &mut bridges, &mut vec![None; n]);

        bridges
            .into_iter()
            .map(|(a, b)| vec![a as i32, b as i32])
            .collect()
    }

    /// https://leetcode.com/problems/critical-connections-in-a-network/solution/
    pub fn critical_connections_leetcode(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;

        let mut graph = HashMap::new();
        let mut rank = HashMap::new();
        let mut conn_dict = HashMap::new();

        // Default rank for unvisited nodes is "None"
        for i in 0..n {
            graph.insert(i, Vec::new());
            rank.insert(i, None);
        }
        for edge in connections {
            // Bidirectional edges
            let (u, v) = (edge[0], edge[1]);
            graph.get_mut(&u).unwrap().push(v);
            graph.get_mut(&v).unwrap().push(u);
            let uv_sorted = (u.min(v), u.max(v));
            conn_dict.insert(uv_sorted, true);
        }

        fn dfs(
            node: i32,
            discovery_rank: i32,
            graph: &HashMap<i32, Vec<i32>>,
            rank: &mut HashMap<i32, Option<i32>>,
            conn_dict: &mut HashMap<(i32, i32), bool>,
        ) -> i32 {
            // That means this node is already visited. We simply return the rank.
            if let Some(&Some(r)) = rank.get(&node) {
                return r;
            }

            // Update the rank of this node.
            rank.insert(node, Some(discovery_rank));

            // This is the max we have seen till now. So we start with this instead of INT_MAX or something.
            let mut min_rank = discovery_rank + 1;

            for &neighbor in graph.get(&node).unwrap().iter() {
                let &neigh_rank = rank.get(&neighbor).unwrap();
                if neigh_rank.is_some() && neigh_rank.unwrap() == discovery_rank - 1 {
                    continue;
                }
                // Recurse on the neighbor.
                let recursive_rank = dfs(neighbor, discovery_rank + 1, graph, rank, conn_dict);

                // Step 1, check if this edge needs to be discarded.
                if recursive_rank <= discovery_rank {
                    let uv_sorted = (node.min(neighbor), node.max(neighbor));
                    conn_dict.remove(&uv_sorted);
                }

                // Step 2, update the minRank if needed.
                min_rank = min_rank.min(recursive_rank);
            }

            min_rank
        }

        dfs(0, 0, &graph, &mut rank, &mut conn_dict);

        let mut result = Vec::new();
        for &(u, v) in conn_dict.keys() {
            result.push(vec![u, v]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let n = 4;
        let connections = vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]];
        assert_eq!(Solution::critical_connections(n, connections), [[1, 3]]);
        // Explanation: both [[1,3]] and [[3,1]] are accepted.
    }

    #[test]
    fn test6() {
        let n = 6;
        let connections = vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 0],
            vec![1, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 3],
        ];
        assert_eq!(Solution::critical_connections(n, connections), [[1, 3]]);
    }

    mod performance {
        use std::collections::HashSet;

        use super::*;

        #[test]
        /// If getting stack overflow:
        ///
        /// ```sh
        /// thread 'c2021::c2021_04::c2021_04_24::tests::performance::n100000cs100000_produces_' has overflowed its stack
        /// fatal runtime error: stack overflow
        /// ```
        ///
        /// Add `RUST_MIN_STACK=33554432` to env
        fn n100000connections99999_produces_connections() {
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
}
