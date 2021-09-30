#![allow(dead_code)]
struct Solution;
impl Solution {
    /// https://titanwolf.org/Network/Articles/Article?AID=e02a67d6-c6e2-485e-a41d-7dbda101f3cb#gsc.tab=0
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        use std::collections::BinaryHeap;
        let n = n as usize;
        let mut res = 0;
        let mut graph = vec![vec![-1; n]; n];
        let mut visited = vec![false; n];
        let mut pq = BinaryHeap::new();
        pq.push((max_moves, 0));
        for e in edges {
            let (f, t, d) = (e[0] as usize, e[1] as usize, e[2]);
            graph[f][t] = d;
            graph[t][f] = d;
        }
        while let Some(t) = pq.pop() {
            let mut _move = t.0;
            let cur = t.1;
            if !visited[cur] {
                visited[cur] = true;
                res += 1;
                for i in 0..n {
                    if graph[cur][i] != -1 {
                        if _move > graph[cur][i] && !visited[i] {
                            pq.push((_move - graph[cur][i] - 1, i));
                        }
                        graph[i][cur] -= _move.min(graph[cur][i]);
                        res += _move.min(graph[cur][i]);
                    }
                }
            }
        }
        res
    }
    pub fn reachable_nodes_could_not_figure_it_out(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        println!("reachable_nodes({:?},{},{})", edges, max_moves, n);
        let mut es = vec![];
        let mut tc = n + 1;
        for e in edges {
            let (f, t, mut d) = (e[0], e[1], e[2]);
            let mut fc = f;
            while d > 0 {
                es.push((fc, tc));
                fc = tc;
                tc += 1;
                d -= 1;
            }
            es.push((fc, t));
        }
        println!("edges: {:?}", es);
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]}}

    #[test]
    fn e1_mm_6_n_3_produces_13() {
        let edges = vv![[0,1,10],[0,2,1],[1,2,2]];
        assert_eq!(Solution::reachable_nodes(edges, 6, 3), 13);
        // Output: 13
        // Explanation: The edge subdivisions are shown in the image above.
        // The nodes that are reachable are highlighted in yellow.
    }
    #[test]
    fn e2_10_4_produces_23() {
        let edges = vv![[0,1,4],[1,2,6],[0,2,8],[1,3,1]];
        assert_eq!(Solution::reachable_nodes(edges, 10, 4), 23);
    }
    #[test]
    fn e3_17_5_produces_1() {
        let edges = vv![[1,2,4],[1,4,5],[1,3,1],[2,3,4],[3,4,5]];
        assert_eq!(Solution::reachable_nodes(edges, 17, 5), 1);
        // Explanation: Node 0 is disconnected from the rest of the graph, so only node 0 is reachable.
    }
}
