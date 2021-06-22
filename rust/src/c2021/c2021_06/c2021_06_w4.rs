#![allow(dead_code)]
/// All Paths from Source Lead to Destination
/// =========================================
///
/// Given the `edges` of a directed graph where `edges[i] = [ai, bi]` indicates there is an edge
/// between nodes `ai` and `bi`, and two nodes `source` and `destination` of this graph,
/// determine whether or not all paths starting from `source` eventually, end at `destination`,
/// that is:
///
/// - At least one path exists from the `source` node to the `destination` node
/// - If a path exists from the `source` node to a node with no outgoing edges,
///   then that node is equal to `destination`.
/// - The number of possible paths from `source` to `destination` is a finite number.
///
/// Return `true` if and only if all roads from `source` lead to `destination`.
///
/// __Constraints:__
///
/// - `1 <= n <= 10_000`
/// - `0 <= edges.length <= 10_000`
/// - `edges.length == 2`
/// - `0 <= ai, bi <= n - 1`
/// - `0 <= source <= n - 1`
/// - `0 <= destination <= n - 1`
/// - The given graph may have self-loops and parallel edges.
///
/// https://leetcode.com/explore/challenge/card/june-leetcoding-challenge-2021/606/week-4-june-22nd-june-28th/3787/
struct Solution;
impl Solution {
    pub fn leads_to_destination(n: i32, es: Vec<Vec<i32>>, s: i32, d: i32) -> bool {
        let (n, s, d) = (n as usize, s as usize, d as usize);
        let es = es.into_iter().fold(vec![vec![]; n as usize], |mut acc, e| {
            let (a, b) = (e[0] as usize, e[1] as usize);
            acc[a].push(b);
            acc
        });

        fn bts(c: usize, d: usize, es: &[Vec<usize>], seen: &mut Vec<bool>) -> bool {
            if seen[c] {
                false
            } else if es[c].is_empty() {
                c == d
            } else {
                seen[c] = true;
                let r = es[c].iter().all(|&n| bts(n, d, es, seen));
                seen[c] = false;
                r
            }
        }

        bts(s, d, &es, &mut vec![false; n])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn example1() {
        let edges = vv![[0, 1], [0, 2]];
        assert!(!Solution::leads_to_destination(3, edges, 0, 2));
        // Explanation: It is possible to reach and get stuck on both node 1 and node 2.
    }
    #[test]
    fn example2() {
        let edges = vv![[0, 1], [0, 3], [1, 2], [2, 1]];
        assert!(!Solution::leads_to_destination(4, edges, 0, 3));
        // Explanation: We have two possibilities: to end at node 3, or to loop over node 1 and node 2 indefinitely.
    }
    #[test]
    fn example3() {
        let edges = vv![[0, 1], [0, 2], [1, 3], [2, 3]];
        assert!(Solution::leads_to_destination(4, edges, 0, 3));
    }
    #[test]
    fn example4() {
        let edges = vv![[0, 1], [1, 1], [1, 2]];
        assert!(!Solution::leads_to_destination(3, edges, 0, 2));
        // Explanation:
        // All paths from the source node end at the destination node,
        // but there are an infinite number of paths,
        // such as 0-1-2, 0-1-1-2, 0-1-1-1-2, 0-1-1-1-1-2, and so on.
    }
    #[test]
    fn example5() {
        let edges = vv![[0, 1], [1, 1]];
        assert!(!Solution::leads_to_destination(2, edges, 0, 1));
        // Explanation: There is infinite self-loop at destination node.
    }

    mod performance {
        use super::*;

        #[test]
        fn n10000_es10000x0t1_s0_d1_produces_true() {
            let edges = vec![vec![0, 1]; 10000];
            assert!(Solution::leads_to_destination(10000, edges, 0, 1));
        }
        /// If getting stack overflow:
        ///
        /// ```sh
        /// thread 'c2021::c2021_06::c2021_06_w4::tests::performance::n10000_es0t1_1t2_and_so_on_s0_d9999_produces_true' has overflowed its stack
        /// fatal runtime error: stack overflow
        /// ```
        ///
        /// Add `RUST_MIN_STACK=33554432` to env
        #[test]
        fn n10000_es0t1_1t2_and_so_on_s0_d9999_produces_true() {
            let edges = (0..9999).map(|i| vec![i, i + 1]).collect();
            assert!(Solution::leads_to_destination(10000, edges, 0, 9999));
        }
    }
}
