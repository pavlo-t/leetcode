#![allow(dead_code)]
/// 797. All Paths From Source to Target
/// ====================================
///
/// Given a directed acyclic graph (__DAG__) of `n` nodes labeled from `0` to `n - 1`,
/// find all possible paths from node `0` to node `n - 1` and return them in __any order__.
///
/// The graph is given as follows: `graph[i]` is a list of all nodes you can visit from node `i`
/// (i.e., there is a directed edge from node `i` to node `graph[i][j]`).
///
/// __Constraints:__
///
/// - `n == graph.length`
/// - `2 <= n <= 15`
/// - `0 <= graph[i][j] < n`
/// - `graph[i][j] != i` (i.e., there will be no self-loops).
/// - All the elements of `graph[i]` are __unique__.
/// - The input graph is __guaranteed__ to be a __DAG__.
///
/// https://leetcode.com/problems/all-paths-from-source-to-target/
struct Solution;
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        println!("all_paths_source_target({:?})", graph);
        fn bts(i: usize, g: &[Vec<i32>], curr: &mut Vec<i32>, rsf: &mut Vec<Vec<i32>>) {
            curr.push(i as i32);
            if i == g.len() - 1 {
                rsf.push(curr.clone());
            } else {
                for &j in g[i].iter() {
                    bts(j as usize, g, curr, rsf);
                }
            }
            curr.pop();
        }
        let mut result = vec![];
        bts(0, &graph, &mut vec![], &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn g_12_e_e_e() {
        let g = vv![[1, 2], [], [], []];
        let e: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::all_paths_source_target(g), e);
    }
    #[test]
    fn g_12_3_3_e() {
        let g = vv![[1, 2], [3], [3], []];
        let e = vv![[0, 1, 3], [0, 2, 3]];
        assert_eq!(Solution::all_paths_source_target(g), e);
        // Explanation: There are two paths: 0 -> 1 -> 3 and 0 -> 2 -> 3.
    }
    #[test]
    fn g_431_324_3_4_e() {
        let g = vv![[4, 3, 1], [3, 2, 4], [3], [4], []];
        let e = vv![[0, 4], [0, 3, 4], [0, 1, 3, 4], [0, 1, 2, 3, 4], [0, 1, 4]];
        assert_eq!(Solution::all_paths_source_target(g), e);
    }
    #[test]
    fn g_1_e() {
        let g = vv![[1], []];
        let e = vv![[0, 1]];
        assert_eq!(Solution::all_paths_source_target(g), e);
    }
    #[test]
    fn g_123_2_3_e() {
        let g = vv![[1, 2, 3], [2], [3], []];
        let e = vv![[0, 1, 2, 3], [0, 2, 3], [0, 3]];
        assert_eq!(Solution::all_paths_source_target(g), e);
    }
    #[test]
    fn g_13_2_3() {
        let g = vv![[1, 3], [2], [3], []];
        let e = vv![[0, 1, 2, 3], [0, 3]];
        assert_eq!(Solution::all_paths_source_target(g), e);
    }

    /// If getting stack overflow: (2 ** 27)
    /// RUST_MIN_STACK=134217728 cargo test --lib c2021_11_28
    #[test]
    fn g_1to14_2to14_and_so_on() {
        let mut g = vec![vec![]; 15];
        for i in 1..15 {
            for j in 0..i {
                g[j].push(i as i32);
            }
        }
        let r = Solution::all_paths_source_target(g);
        //let e = vv![[0, 1, 2, 3], [0, 3]];
        //assert_eq!(r, e);
        assert_eq!(r.len(), 8192);
    }
}
