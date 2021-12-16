#![allow(dead_code)]
/// 310. Minimum Height Trees
/// =========================
///
/// A tree is an undirected graph in which any two vertices are connected by _exactly_ one path.
/// In other words, any connected graph without simple cycles is a tree.
///
/// Given a tree of `n` nodes labelled from `0` to `n - 1`,
/// and an array of `n - 1` `edges` where `edges[i] = [ai, bi]`
/// indicates that there is an undirected edge between the two nodes `ai` and `bi` in the tree,
/// you can choose any node of the tree as the root.
/// When you select a node `x` as the root, the result tree has height `h`.
/// Among all possible rooted trees,
/// those with minimum height (i.e. `min(h)`) are called __minimum height trees__ (MHTs).
///
/// Return _a list of all __MHTs'__ root labels_.
/// You can return the answer in __any order__.
///
/// The __height__ of a rooted tree is the number of edges on the longest downward path between the root and a leaf.
///
/// __Constraints:__
///
/// - `1 <= n <= 20_000`
/// - `edges.length == n - 1`
/// - `0 <= ai, bi < n`
/// - `ai != bi`
/// - All the pairs `(ai, bi)` are distinct.
/// - The given input is __guaranteed__ to be a tree and there will be __no repeated__ edges.
///
/// https://leetcode.com/problems/minimum-height-trees/
struct Solution;
impl Solution {
    pub fn find_min_height_trees_my_brute_force(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut tree = vec![vec![]; n];
        for e in &edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            tree[a].push(b);
            tree[b].push(a);
        }

        fn height(prev: usize, curr: usize, tree: &[Vec<usize>]) -> usize {
            let mut result = 1;
            for &next in &tree[curr] {
                if next != prev {
                    result = result.max(1 + height(curr, next, tree));
                }
            }
            result
        }

        let mut heights = vec![0; n];
        let mut min_height = usize::MAX;
        for root in 0..n {
            heights[root] = height(n, root, &tree);
            min_height = min_height.min(heights[root]);
        }
        heights
            .into_iter()
            .enumerate()
            .filter(|&(_, height)| height == min_height)
            .map(|(i, _)| i as i32)
            .collect()
    }

    /// /src/main/scala/challenge/c2020/c2020_11/d2020_11_04.scala
    pub fn find_min_height_trees_from_past(mut n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n < 3 {
            (0..n).collect()
        } else {
            use std::collections::{HashMap, HashSet};
            let mut tree: HashMap<i32, HashSet<i32>> = HashMap::new();
            for e in &edges {
                let (a, b) = (e[0], e[1]);
                tree.entry(a).or_default().insert(b);
                tree.entry(b).or_default().insert(a);
            }
            let mut leaves: Vec<i32> = tree
                .iter()
                .filter(|(_, nexts)| nexts.len() == 1)
                .map(|(&i, _)| i)
                .collect();

            while n > 2 {
                let mut new_leaves = vec![];
                for leaf in &leaves {
                    let nexts = tree.remove(leaf).unwrap();
                    for n in nexts {
                        if let Some(nexts) = tree.get_mut(&n) {
                            nexts.remove(leaf);
                            if nexts.len() == 1 {
                                new_leaves.push(n);
                            }
                        }
                    }
                }
                n -= leaves.len() as i32;
                leaves = new_leaves;
            }

            leaves
        }
    }
    /// https://leetcode.com/problems/minimum-height-trees/solution/
    pub fn find_min_height_trees(mut n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n < 3 {
            (0..n).collect()
        } else {
            use std::collections::{HashMap, HashSet};
            let mut tree: HashMap<i32, HashSet<i32>> = HashMap::new();
            for e in &edges {
                let (a, b) = (e[0], e[1]);
                tree.entry(a).or_default().insert(b);
                tree.entry(b).or_default().insert(a);
            }
            let mut leaves: Vec<i32> = tree
                .iter()
                .filter(|(_, nexts)| nexts.len() == 1)
                .map(|(&i, _)| i)
                .collect();

            while n > 2 {
                let mut new_leaves = vec![];
                for leaf in &leaves {
                    let neighbor = tree.remove(leaf).unwrap().into_iter().next().unwrap();
                    if let Some(nexts) = tree.get_mut(&neighbor) {
                        nexts.remove(leaf);
                        if nexts.len() == 1 {
                            new_leaves.push(neighbor);
                        }
                    }
                }
                n -= leaves.len() as i32;
                leaves = new_leaves;
            }

            leaves
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    /// ```text
    ///   0     1     2     3
    ///   |    /|\    |     |
    ///   1   0 2 3   1     1
    ///  / \         / \   / \
    /// 2   3       0   3 0   2
    /// ```
    ///
    /// Explanation: As shown,
    /// the height of the tree is 1 when the root is the node with label 1 which is the only MHT.
    #[test]
    fn n4_e10_12_13() {
        let e = vv![[1, 0], [1, 2], [1, 3]];
        let mut result = Solution::find_min_height_trees(4, e);
        result.sort_unstable();
        assert_eq!(result, vec![1]);
    }
    /// ```text
    ///    3       4
    ///  // \\    / \
    /// 0 1 2 4  5   3
    ///       |     /|\
    ///       5    0 1 2
    /// ```
    #[test]
    fn n6_e30_31_32_34_54() {
        let e = vv![[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]];
        let mut result = Solution::find_min_height_trees(6, e);
        result.sort_unstable();
        assert_eq!(result, vec![3, 4]);
    }
    #[test]
    fn n1_en() {
        let mut result = Solution::find_min_height_trees(1, vv![]);
        result.sort_unstable();
        assert_eq!(result, vec![0]);
    }
    #[test]
    fn n_2_e01() {
        let mut result = Solution::find_min_height_trees(2, vv![[0, 1]]);
        result.sort_unstable();
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_21() {
        let n = 7;
        let e = vv![[0, 1], [1, 2], [1, 3], [2, 4], [3, 5], [4, 6]];
        let expected = [1, 2];
        let mut result = Solution::find_min_height_trees(n, e);
        result.sort_unstable();
        assert_eq!(result, expected);
    }

    /// If getting stack overflow: (2 ** 27)
    /// RUST_MIN_STACK=134217728 cargo test --lib c2021_12_16
    //#[ignore]
    #[test]
    fn n_20000_e0_to_1to19999() {
        let mut e = vec![];
        for i in 1..=19999 {
            e.push(vec![0, i]);
        }
        let mut result = Solution::find_min_height_trees(20000, e);
        result.sort_unstable();
        assert_eq!(result, vec![0]);
    }
    //#[ignore]
    #[test]
    fn n_20000_e1to19999_to_0() {
        let mut e = vec![];
        for i in 1..=19999 {
            e.push(vec![i, 0]);
        }
        let mut result = Solution::find_min_height_trees(20000, e);
        result.sort_unstable();
        assert_eq!(result, vec![0]);
    }
    //#[ignore]
    #[test]
    fn n_20000_e0to19998_to_next() {
        let mut e = vec![];
        for i in 0..=19998 {
            e.push(vec![i, i + 1]);
        }
        let mut result = Solution::find_min_height_trees(20000, e);
        result.sort_unstable();
        assert_eq!(result, vec![9999, 10000]);
    }
    //#[ignore]
    #[test]
    fn n_20000_e1to19999_to_prev() {
        let mut e = vec![];
        for i in 1..=19999 {
            e.push(vec![i, i - 1]);
        }
        let mut result = Solution::find_min_height_trees(20000, e);
        result.sort_unstable();
        assert_eq!(result, vec![9999, 10000]);
    }
}
