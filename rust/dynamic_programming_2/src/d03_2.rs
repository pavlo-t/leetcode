#![allow(dead_code)]
/// 265. Paint House II
/// ===================
///
/// There are a row of `n` houses, each house can be painted with one of the `k` colors.
/// The cost of painting each house with a certain color is different.
/// You have to paint all the houses such that no two adjacent houses have the same color.
///
/// The cost of painting each house with a certain color is represented by an `n x k` cost matrix costs.
///
/// For example, `costs[0][0]` is the cost of painting house `0` with color `0`;
/// `costs[1][2]` is the cost of painting house `1` with color `2`, and so on...
///
/// Return _the minimum cost to paint all houses_.
///
/// __Constraints:__
///
/// - `1 <= costs.length <= 100`
/// - `2 <= costs[i].length <= 20`
/// - `1 <= costs[i][j] <= 20
///
/// https://leetcode.com/problems/paint-house-ii/
struct Solution;
impl Solution {
    /// Approach 5: Dynamic programming with Optimized Time and Space
    /// https://leetcode.com/problems/paint-house-ii/solution/
    pub fn min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
        println!("min_cost_ii({:?})", costs);
        let (n, k) = (costs.len(), costs[0].len());
        let (mut min1, mut c1, mut min2) = (0, k, 0);
        for i in 0..n {
            let mins = |(nm1, nc1, nm2): (i32, usize, i32), c: usize| {
                let cost = costs[i][c] + if c == c1 { min2 } else { min1 };
                if cost < nm1 {
                    (cost, c, nm1)
                } else if cost < nm2 {
                    (nm1, nc1, cost)
                } else {
                    (nm1, nc1, nm2)
                }
            };
            let (nm1, nc1, nm2) = (0..k).fold((i32::MAX, k, i32::MAX), mins);
            min1 = nm1;
            c1 = nc1;
            min2 = nm2;
        }
        min1
    }
    /// Approach 4: Dynamic programming with Optimized Time
    /// https://leetcode.com/problems/paint-house-ii/solution/
    pub fn min_cost_ii_leetcode_approach_4(costs: Vec<Vec<i32>>) -> i32 {
        println!("min_cost_ii({:?})", costs);
        let (n, k) = (costs.len(), costs[0].len());
        let mut curr = costs[0].clone();
        let mut prev = curr.clone();
        for i in 1..n {
            std::mem::swap(&mut curr, &mut prev);
            let mins2 = |(m1, m2): (usize, usize), c: usize| {
                if m1 >= k || prev[c] <= prev[m1] {
                    (c, m1)
                } else if m2 >= k || prev[c] < prev[m2] {
                    (m1, c)
                } else {
                    (m1, m2)
                }
            };
            let (min1, min2) = (0..k).fold((usize::MAX, usize::MAX), mins2);
            for c in 0..k {
                curr[c] = costs[i][c] + if c == min1 { prev[min2] } else { prev[min1] };
            }
        }
        curr.into_iter().min().unwrap()
    }

    pub fn min_cost_ii_my_dp(costs: Vec<Vec<i32>>) -> i32 {
        println!("min_cost_ii({:?})", costs);
        let (n, k) = (costs.len(), costs[0].len());
        let mut curr = costs[0].clone();
        let mut prev = curr.clone();
        for i in 1..n {
            std::mem::swap(&mut curr, &mut prev);
            for c in 0..k {
                let next = (0..k).filter(|&p| p != c).map(|p| prev[p]).min().unwrap();
                curr[c] = costs[i][c] + next;
            }
        }
        curr.into_iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn c_12() {
        let c = vv![[1, 2]];
        assert_eq!(Solution::min_cost_ii(c), 1);
    }
    #[test]
    fn c_153_294() {
        let c = vv![[1, 5, 3], [2, 9, 4]];
        assert_eq!(Solution::min_cost_ii(c), 5);
        // Explanation:
        // Paint house 0 into color 0, paint house 1 into color 2. Minimum cost: 1 + 4 = 5;
        // Or paint house 0 into color 2, paint house 1 into color 0. Minimum cost: 3 + 2 = 5.
    }
    #[test]
    fn c_13_24() {
        let c = vv![[1, 3], [2, 4]];
        assert_eq!(Solution::min_cost_ii(c), 5);
    }
}
