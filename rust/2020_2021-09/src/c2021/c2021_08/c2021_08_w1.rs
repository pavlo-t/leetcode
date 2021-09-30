#![allow(dead_code)]
/// Optimize Water Distribution in a Village
/// ========================================
///
/// There are `n` houses in a village.
/// We want to supply water for all the houses by building wells and laying pipes.
///
/// For each house `i`, we can either build a well inside it directly with cost `wells[i - 1]`
/// (note the `-1` due to __0-indexing__), or pipe in water from another well to it.
/// The costs to lay pipes between houses are given by the array `pipes`,
/// where each `pipes[j] = [house1j, house2j, costj]` represents
/// the cost to connect `house1j` and `house2j` together using a pipe.
/// Connections are bidirectional.
///
/// Return _the minimum total cost to supply water to all houses_.
///
/// Constraints:
///
/// - `1 <= wells.length <= 10_000`
/// - `0 <= wells[i] <= 100_000`
/// - `1 <= pipes.length <= 10_000`
/// - `pipes[j].length == 3`
/// - `1 <= house1j, house2j <= wells.length`
/// - `0 <= costj <= 100_000`
/// - `house1j != house2j`
///
/// https://leetcode.com/explore/featured/card/august-leetcoding-challenge-2021/613/week-1-august-1st-august-7th/3834/
struct Solution;
impl Solution {
    pub fn min_cost_to_supply_water(n: i32, wells: Vec<i32>, pipes: Vec<Vec<i32>>) -> i32 {
        fn find(x: usize, parent: &mut Vec<usize>) -> usize {
            if parent[x] != x {
                let y = find(parent[x], parent);
                parent[x] = y;
            }
            parent[x]
        }
        fn union(x: usize, y: usize, parent: &mut Vec<usize>, rank: &mut Vec<usize>) -> bool {
            let mut x = find(x, parent);
            let mut y = find(y, parent);
            if x == y {
                false
            } else {
                if rank[x] < rank[y] {
                    std::mem::swap(&mut x, &mut y);
                }
                parent[y] = x;
                if rank[x] == rank[y] {
                    rank[x] += 1;
                }
                true
            }
        }

        let n = n as usize;
        let mut parent = (0..=n).collect::<Vec<usize>>();
        let mut rank = vec![0usize; n + 1];

        let mut es = pipes
            .into_iter()
            .map(|v| (v[2], v[0].min(v[1]) as usize, v[0].max(v[1]) as usize))
            .chain(wells.into_iter().enumerate().map(|(i, c)| (c, 0, i + 1)))
            .collect::<Vec<_>>();
        es.sort_unstable();

        let mut result = 0;
        for (c, n1, n2) in es {
            if union(n1, n2, &mut parent, &mut rank) {
                result += c;
            }
        }
        result
    }

    pub fn min_cost_to_supply_water_ds(n: i32, wells: Vec<i32>, pipes: Vec<Vec<i32>>) -> i32 {
        struct DisjointSet {
            parent: Vec<usize>,
            rank: Vec<usize>,
        }
        impl DisjointSet {
            fn new(n: usize) -> Self {
                Self {
                    parent: (0..n).collect(),
                    rank: vec![0; n],
                }
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

        let n = n as usize;
        let mut es = pipes
            .into_iter()
            .map(|v| (v[2], v[0].min(v[1]) as usize, v[0].max(v[1]) as usize))
            .chain(wells.into_iter().enumerate().map(|(i, c)| (c, 0, i + 1)))
            .collect::<Vec<_>>();
        es.sort_unstable();

        let mut ds = DisjointSet::new(n + 1);
        let mut result = 0;
        for (c, n1, n2) in es {
            if ds.find(n1) != ds.find(n2) {
                result += c;
                ds.union(n1, n2);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn n_3_w_1_2_2_p_1t2i1_2t3i1_produces_3() {
        let n = 3;
        let wells = vec![1, 2, 2];
        let pipes = vv![[1, 2, 1], [2, 3, 1]];
        assert_eq!(Solution::min_cost_to_supply_water(n, wells, pipes), 3);
        // Explanation:
        // The best strategy is to build a well in the first house with cost 1
        // and connect the other houses to it with cost 2 so the total cost is 3.
    }
    #[test]
    fn n_3_w_0_0_0_p_1t2i1_2t3i1_produces_0() {
        let n = 3;
        let wells = vec![0, 0, 0];
        let pipes = vv![[1, 2, 1], [2, 3, 1]];
        assert_eq!(Solution::min_cost_to_supply_water(n, wells, pipes), 0);
    }
    #[test]
    fn n_5_w_1_2_2_3_2_p_1t2c1_2t3c1_4t5c7_produces_8() {
        let n = 5;
        let wells = vec![1, 2, 2, 3, 2];
        let pipes = vv![[1, 2, 1], [2, 3, 1], [4, 5, 7]];
        assert_eq!(Solution::min_cost_to_supply_water(n, wells, pipes), 8);
    }
    #[test]
    fn n_5_w_46012_72474_64965_751_33304_p_2t1c6719_3t2c75312_5t3c44918_test_4_produces_131_704() {
        let n = 5;
        let wells = vec![46012, 72474, 64965, 751, 33304];
        let pipes = vv![[2, 1, 6719], [3, 2, 75312], [5, 3, 44918]];
        assert_eq!(Solution::min_cost_to_supply_water(n, wells, pipes), 131_704);
    }
}
