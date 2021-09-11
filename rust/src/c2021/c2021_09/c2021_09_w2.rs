#![allow(dead_code)]

/// Best Meeting Point
/// ==================
///
/// Given an `m x n` binary grid `grid` where each `1` marks the home of one friend,
/// return _the minimal __total travel distance___.
///
/// The __total travel distance__ is the sum of the distances between the houses of the friends and the meeting point.
///
/// The distance is calculated using [Manhattan Distance], where `distance(p1, p2) = |p2.x - p1.x| + |p2.y - p1.y|`.
///
/// [Manhattan Distance]:http://en.wikipedia.org/wiki/Taxicab_geometry
///
/// __Constraints:__
///
/// - `1 <= grid.length, grid[i].length <= 200`
/// - `grid[i][j]` is either `0` or `1`.
/// - There will be __at least two__ friends in the `grid`.
///
/// https://leetcode.com/explore/challenge/card/september-leetcoding-challenge-2021/637/week-2-september-8th-september-14th/3967/
struct Solution;
impl Solution {
    /// https://leetcode.com/problems/best-meeting-point/solution/
    pub fn min_total_distance(grid: Vec<Vec<i32>>) -> i32 {
        fn distance_1d(ps: Vec<usize>) -> i32 {
            let mut distance = 0;
            let mut l = 0;
            let mut r = ps.len() - 1;
            while l < r {
                distance += (ps[r] - ps[l]) as i32;
                l += 1;
                r -= 1;
            }
            distance
        }
        let m = grid.len();
        let n = grid[0].len();
        let mut rs = vec![];
        (0..m)
            .flat_map(|r| (0..n).map(move |c| (r, c)))
            .filter(|&(r, c)| grid[r][c] == 1)
            .for_each(|(r, _)| rs.push(r));
        let mut cs = vec![];
        (0..n)
            .flat_map(|c| (0..m).map(move |r| (r, c)))
            .filter(|&(r, c)| grid[r][c] == 1)
            .for_each(|(_, c)| cs.push(c));

        distance_1d(rs) + distance_1d(cs)
    }
    /// https://www.geeksforgeeks.org/best-meeting-point-2d-binary-array/
    pub fn min_total_distance_median(grid: Vec<Vec<i32>>) -> i32 {
        fn distance(r1: usize, c1: usize, r2: usize, c2: usize) -> i32 {
            (r2 as i32 - r1 as i32).abs() + (c2 as i32 - c1 as i32).abs()
        }
        let m = grid.len();
        let n = grid[0].len();
        let mut hs = vec![];
        let mut vs = vec![];
        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == 1 {
                    hs.push(r);
                    vs.push(c);
                }
            }
        }
        hs.sort_unstable();
        vs.sort_unstable();
        let mid = hs.len() / 2;
        let (r, c) = (hs[mid], vs[mid]);
        (0..m)
            .flat_map(|r| (0..n).map(move |c| (r, c)))
            .filter(|&(r, c)| grid[r][c] == 1)
            .map(|(r2, c2)| distance(r, c, r2, c2))
            .sum()
    }
    pub fn min_total_distance_brute_force(grid: Vec<Vec<i32>>) -> i32 {
        fn distance(r1: usize, c1: usize, r2: usize, c2: usize) -> i32 {
            (r2 as i32 - r1 as i32).abs() + (c2 as i32 - c1 as i32).abs()
        }
        let m = grid.len();
        let n = grid[0].len();
        let mut fs = vec![];
        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == 1 {
                    fs.push((r, c));
                }
            }
        }
        let mut result = i32::MAX;
        for r in 0..m {
            for c in 0..n {
                let d = fs.iter().map(|&(r2, c2)| distance(r, c, r2, c2)).sum();
                result = result.min(d);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]}}

    #[test]
    fn g10001_00000_00100_produces_6() {
        let grid = vv![[1, 0, 0, 0, 1], [0, 0, 0, 0, 0], [0, 0, 1, 0, 0]];
        assert_eq!(Solution::min_total_distance(grid), 6);
        // Explanation: Given three friends living at (0,0), (0,4), and (2,2).
        // The point (0,2) is an ideal meeting point, as the total travel distance of 2 + 2 + 2 = 6 is minimal.
        // So return 6.
    }
    #[test]
    fn g11_produces_1() {
        assert_eq!(Solution::min_total_distance(vv![[1, 1]]), 1);
    }
    #[test]
    fn g11111_produces_6() {
        assert_eq!(Solution::min_total_distance(vv![[1, 1, 1, 1, 1]]), 6);
    }

    mod performance {
        use super::*;

        #[test]
        fn g200x200_1s_produces_4_000_000() {
            let grid = vec![vec![1; 200]; 200];
            assert_eq!(Solution::min_total_distance(grid), 4_000_000);
        }
    }
}
