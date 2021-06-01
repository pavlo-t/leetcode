#![allow(dead_code)]
/// Paint House
/// ===========
///
/// There is a row of `n` houses, where each house can be painted one of three colors:
/// red (`0`), blue, (`1`), or green (`2`).
/// The cost of painting each house with a certain color is different.
/// You have to paint all the houses such that no two adjacent houses have the same color.
///
/// The cost of painting each house with a certain color is represented by an `n x 3` cost matrix `costs`.
/// For example:
///
/// - `costs[0][0]` is the cost of painting house `0` with the color red;
/// - `costs[1][2]` is the cost of painting house `1` with color green, and so on...
///
/// Return _the minimum cost to paint all houses_.
///
/// __Constraints:__
///
/// - `costs[i].length == 3`
/// - `1 <= costs.length <= 100`
/// - `1 <= costs[i][j] <= 20`
///
/// https://leetcode.com/explore/featured/card/june-leetcoding-challenge-2021/603/week-1-june-1st-june-7th/3763/
struct Solution;
impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let (mut r0, mut r1, mut r2) = (0, 0, 0);
        for c in costs {
            let (c0, c1, c2) = (c[0], c[1], c[2]);
            let (p0, p1, p2) = (r0, r1, r2);
            r0 = c0 + p1.min(p2);
            r1 = c1 + p0.min(p2);
            r2 = c2 + p0.min(p1);
        }
        r0.min(r1).min(r2)
    }

    pub fn min_cost_my_1(mut costs: Vec<Vec<i32>>) -> i32 {
        for h in 1..costs.len() {
            let p = &costs[h - 1];
            let (p0, p1, p2) = (p[0], p[1], p[2]);
            let c = costs.get_mut(h).unwrap();
            c[0] += p1.min(p2);
            c[1] += p0.min(p2);
            c[2] += p0.min(p1);
        }
        costs.last().unwrap().into_iter().min().unwrap().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn example1() {
        let costs = vv![[17, 2, 17], [16, 16, 5], [14, 3, 19]];
        assert_eq!(Solution::min_cost(costs), 10);
        // Explanation: Paint house 0 into blue, paint house 1 into green, paint house 2 into blue.
        // Minimum cost: 2 + 5 + 3 = 10.
    }
    #[test]
    fn example2() {
        let costs = vv![[7, 6, 2]];
        assert_eq!(Solution::min_cost(costs), 2);
    }

    #[test]
    fn c_123_123_123_produces_4() {
        let costs = vv![[1, 2, 3], [1, 2, 3], [1, 2, 3]];
        assert_eq!(Solution::min_cost(costs), 4);
    }
    #[test]
    fn c_123_repeat_100_produces_150() {
        let costs = vec![vec![1, 2, 3]; 100];
        assert_eq!(Solution::min_cost(costs), 150);
    }
}
