#![allow(dead_code)]
/// Sum of Distances in Tree
/// ========================
///
/// There is an undirected connected tree with `n` nodes labeled from `0` to `n - 1` and `n - 1` edges.
///
/// You are given the integer `n` and the array `edges` where `edges[i] = [ai, bi]`
/// indicates that there is an edge between nodes `ai` and `bi` in the tree.
///
/// Return an array `answer` of length `n` where `answer[i]` is the sum of the distances
/// between the `ith` node in the tree and all other nodes.
///
/// __Constraints:__
///
/// - `1 <= n <= 30_000`
/// - `edges.length == n - 1`
/// - `edges[i].length == 2`
/// - `0 <= ai, bi < n`
/// - `ai != bi`
/// - The given input represents a valid tree.
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/636/week-1-september-1st-september-7th/3963/
struct Solution;
impl Solution {
    /// https://leetcode.com/problems/sum-of-distances-in-tree/solution/
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for e in &edges {
            let (u, v) = (e[0] as usize, e[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
        }
        let mut count = vec![1; n];
        let mut ans = vec![0; n];

        fn dfs(
            node: usize,
            parent: usize,
            graph: &[Vec<usize>],
            count: &mut Vec<i32>,
            ans: &mut Vec<i32>,
        ) {
            for &child in graph[node].iter() {
                if child != parent {
                    dfs(child, node, graph, count, ans);
                    count[node] += count[child];
                    ans[node] += ans[child] + count[child];
                }
            }
        }
        fn dfs2(
            node: usize,
            parent: usize,
            graph: &[Vec<usize>],
            count: &mut Vec<i32>,
            ans: &mut Vec<i32>,
        ) {
            for &child in graph[node].iter() {
                if child != parent {
                    ans[child] = ans[node] - count[child] + ans.len() as i32 - count[child];
                    dfs2(child, node, graph, count, ans);
                }
            }
        }

        dfs(0, 0, &graph, &mut count, &mut ans);
        dfs2(0, 0, &graph, &mut count, &mut ans);

        ans
    }
    pub fn sum_of_distances_in_tree_my(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        fn build_tree(
            n: usize,
            edges_map: &[Vec<usize>],
            tree: &mut Vec<Vec<usize>>,
            seen: &mut Vec<bool>,
        ) {
            if !seen[n] {
                seen[n] = true;
                edges_map[n].iter().for_each(|&c| {
                    if !seen[c] {
                        tree[n].push(c);
                        build_tree(c, edges_map, tree, seen);
                    }
                });
            }
        }
        fn count_children(n: usize, tree: &[Vec<usize>], children: &mut Vec<i32>) -> i32 {
            if children[n] != -1 {
                children[n]
            } else {
                let r = tree[n]
                    .iter()
                    .map(|&c| count_children(c, tree, children))
                    .sum();
                children[n] = r;
                r + 1
            }
        }
        fn get_distance_sum(n: usize, tree: &[Vec<usize>], depth: i32) -> i32 {
            depth
                + tree[n]
                    .iter()
                    .map(|&c| get_distance_sum(c, tree, depth + 1))
                    .sum::<i32>()
        }
        fn set_distance_sums(
            n: usize,
            parent_distance_sum: i32,
            results: &mut Vec<i32>,
            tree: &[Vec<usize>],
            children: &[i32],
        ) {
            let r = parent_distance_sum - children[n] - 1 + children.len() as i32 - children[n] - 1;
            results[n] = r;
            tree[n]
                .iter()
                .for_each(|&c| set_distance_sums(c, r, results, tree, children));
        }

        let n = n as usize;
        let mut edges_map = vec![vec![]; n];
        for e in &edges {
            let (f, t) = (e[0] as usize, e[1] as usize);
            edges_map[f].push(t);
            edges_map[t].push(f);
        }
        let mut tree = vec![vec![]; n];
        build_tree(0, &edges_map, &mut tree, &mut vec![false; n]);
        let mut children = vec![-1; n];
        count_children(0, &tree, &mut children);
        let mut result = vec![-1; n];
        result[0] = get_distance_sum(0, &tree, 0);
        //println!("emu:      {:?}", emu);
        //println!("tree:     {:?}", tree);
        //println!("children: {:?}", children);
        //println!("root_ds:  {:?}", result[0]);
        tree[0]
            .iter()
            .for_each(|&c| set_distance_sums(c, result[0], &mut result, &tree, &children));

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn example1() {
        //   0
        //  / \
        // 1   2
        //    /|\
        //   3 4 5
        let n = 6;
        let edges = vv![[0, 1], [0, 2], [2, 3], [2, 4], [2, 5]];
        let e = vec![8, 12, 6, 10, 10, 10];
        assert_eq!(Solution::sum_of_distances_in_tree(n, edges), e);
        // Explanation: The tree is shown above.
        // We can see that dist(0,1) + dist(0,2) + dist(0,3) + dist(0,4) + dist(0,5)
        // equals 1 + 1 + 2 + 2 + 2 = 8.
        // Hence, answer[0] = 8, and so on.
    }
    #[test]
    fn example2() {
        // 0
        let n = 1;
        let edges = vec![];
        let e = vec![0];
        assert_eq!(Solution::sum_of_distances_in_tree(n, edges), e);
    }
    #[test]
    fn example3() {
        //   0
        //  /
        // 1
        let n = 2;
        let edges = vv![[1, 0]];
        let e = vec![1, 1];
        assert_eq!(Solution::sum_of_distances_in_tree(n, edges), e);
    }

    #[test]
    fn test12() {
        let n = 3;
        let edges = vv![[2, 1], [0, 2]];
        let e = vec![3, 3, 2];
        assert_eq!(Solution::sum_of_distances_in_tree(n, edges), e);
    }
}
