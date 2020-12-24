#![allow(dead_code)]

/// ### 1697. Checking Existence of Edge Length Limited Paths
///
/// https://leetcode.com/contest/weekly-contest-220/problems/checking-existence-of-edge-length-limited-paths/
struct Solution;

/// https://leetcode.com/problems/checking-existence-of-edge-length-limited-paths/discuss/978450/C%2B%2B-DSU-%2B-Two-Pointers
/// https://en.wikipedia.org/wiki/Disjoint-set_data_structure
struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self { parent: (0..n).collect(), rank: vec![0; n] }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x])
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x != y {
            if self.rank[x] < self.rank[y] {
                let tmp = x;
                x = y;
                y = tmp;
            }
            self.parent[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }
}

impl Solution {
    pub fn distance_limited_paths_exist(n: i32, edge_list: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut queries: Vec<_> =
            queries.into_iter()
                .enumerate()
                .map(|(i, q)| (q[2], q[0] as usize, q[1] as usize, i))
                .collect();
        queries.sort_unstable();
        let mut edge_list: Vec<_> =
            edge_list.into_iter()
                .map(|e| (e[2], e[0] as usize, e[1] as usize))
                .collect();
        edge_list.sort_unstable();

        let mut ds = DisjointSet::new(n as usize);
        let mut results = vec![false; queries.len()];
        let mut i = 0;

        for (lim, p, q, j) in queries {
            while i < edge_list.len() && edge_list[i].0 < lim {
                let (_, u, v) = edge_list[i];
                ds.union(u, v);
                i += 1;
            }
            results[j] = ds.find(p) == ds.find(q);
        }

        results
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let n = 3;
        let edge_list = vec![vec![0, 1, 2], vec![1, 2, 4], vec![2, 0, 8], vec![1, 0, 16]];
        let queries = vec![vec![0, 1, 2], vec![0, 2, 5]];

        let expected = vec![false, true];

        assert_eq!(Solution::distance_limited_paths_exist(n, edge_list, queries), expected);
        // Explanation:
        // Note that there are two overlapping edges between 0 and 1 with distances 2 and 16.
        //
        // For the first query,
        // between 0 and 1 there is no path where each distance is less than 2,
        // thus we return false for this query.
        //
        // For the second query,
        // there is a path (0 -> 1 -> 2) of two edges with distances less than 5,
        // thus we return true for this query.
    }

    #[test]
    fn example2() {
        let n = 5;
        let edge_list = vec![vec![0, 1, 10], vec![1, 2, 5], vec![2, 3, 9], vec![3, 4, 13]];
        let queries = vec![vec![0, 4, 14], vec![1, 4, 13]];

        let expected = vec![true, false];

        assert_eq!(Solution::distance_limited_paths_exist(n, edge_list, queries), expected);
        // Explanation:
        // 1. There is a path (0 -> 1 -> 2 -> 3 -> 4) where all distances are less than 14.
        // 2. Edge [3,4,13] has distance of 13 which is not strictly less than 13.
    }
}