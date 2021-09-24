#![allow(dead_code)]
/// Shortest Distance from All Buildings
/// ====================================
///
/// You are given an `m x n` grid `grid` of values `0`, `1`, or `2`, where:
///
/// - each `0` marks __an empty land__ that you can pass by freely,
/// - each `1` marks __a building__ that you cannot pass through, and
/// - each `2` marks __an obstacle__ that you cannot pass through.
///
/// You want to build a house on an empty land that reaches all buildings in the __shortest total travel__ distance.
/// You can only move up, down, left, and right.
///
/// Return _the __shortest travel distance__ for such a house_.
/// If it is not possible to build such a house according to the above rules, return `-1`.
///
/// The __total travel distance__ is the sum of the distances between the houses of the friends and the meeting point.
///
/// The distance is calculated using [Manhattan Distance], where `distance(p1, p2) = |p2.x - p1.x| + |p2.y - p1.y|`.
///
/// [Manhattan Distance]:http://en.wikipedia.org/wiki/Taxicab_geometry
///
/// __Constraints:__
///
/// - `1 <= grid.length, grid[i].length <= 50`
/// - `grid[i][j]` is either `0`, `1`, or `2`.
/// - There will be __at least one__ building in the `grid`.
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/639/week-4-september-22nd-september-28th/3983/
struct Solution;
impl Solution {
    pub fn shortest_distance(grid: Vec<Vec<i32>>) -> i32 {
        //println!("shortest_distance({:?})", grid);
        use std::collections::VecDeque;

        fn append_q(
            q: &mut VecDeque<(usize, usize, i32)>,
            r: usize,
            c: usize,
            dist: i32,
            distances: &[Vec<i32>],
            visited: &mut [Vec<bool>],
            grid: &[Vec<i32>],
        ) {
            if distances[r][c] != i32::MAX && grid[r][c] == 0 && !visited[r][c] {
                q.push_back((r, c, dist));
                visited[r][c] = true;
            }
        }

        let m = grid.len();
        let n = grid[0].len();
        let mut distances = vec![vec![0; n]; m];
        let mut q = VecDeque::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == 2 {
                    distances[r][c] = i32::MAX;
                } else if grid[r][c] == 1 {
                    //println!("update_distances({:?}, {},{})", distances, r, c);
                    distances[r][c] = i32::MAX;
                    q.push_back((r, c, 0));
                    visited[r][c] = true;

                    while let Some((r, c, d)) = q.pop_front() {
                        //println!(" ds[{}][{}] = {} + {}", r, c, ds[r][c], d);
                        distances[r][c] += d;
                        let nd = d + 1;
                        if let Some(r) = r.checked_sub(1) {
                            append_q(&mut q, r, c, nd, &distances, &mut visited, &grid);
                        }
                        if r < m - 1 {
                            append_q(&mut q, r + 1, c, nd, &distances, &mut visited, &grid);
                        }
                        if let Some(c) = c.checked_sub(1) {
                            append_q(&mut q, r, c, nd, &distances, &mut visited, &grid);
                        }
                        if c < n - 1 {
                            append_q(&mut q, r, c + 1, nd, &distances, &mut visited, &grid);
                        }
                    }
                    for r in 0..m {
                        for c in 0..n {
                            if !visited[r][c] {
                                distances[r][c] = i32::MAX;
                            } else {
                                visited[r][c] = false;
                            }
                        }
                    }
                }
            }
        }
        //println!("distances: {:?}", distances);

        let mut result = i32::MAX;
        for r in 0..m {
            for c in 0..n {
                result = result.min(distances[r][c]);
            }
        }
        match result {
            i32::MAX => -1,
            r => r,
        }
    }
    pub fn shortest_distance_for_instead_of_iter(grid: Vec<Vec<i32>>) -> i32 {
        //println!("shortest_distance({:?})", grid);
        use std::collections::VecDeque;

        let m = grid.len();
        let n = grid[0].len();
        let mut distances = vec![vec![0; n]; m];
        let mut q = VecDeque::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == 2 {
                    distances[r][c] = i32::MAX;
                } else if grid[r][c] == 1 {
                    //println!("update_distances({:?}, {},{})", distances, r, c);
                    distances[r][c] = i32::MAX;
                    visited[r][c] = true;
                    q.push_back((r, c, 0));

                    while let Some((r, c, d)) = q.pop_front() {
                        //println!(" ds[{}][{}] = {} + {}", r, c, ds[r][c], d);
                        distances[r][c] += d;
                        let nd = d + 1;
                        let nrcs = [
                            (r.checked_sub(1), Some(c)),
                            (Some(r + 1).filter(|&r| r < m), Some(c)),
                            (Some(r), c.checked_sub(1)),
                            (Some(r), Some(c + 1).filter(|&c| c < n)),
                        ];
                        for &rc in &nrcs {
                            if let (Some(r), Some(c)) = rc {
                                if distances[r][c] < i32::MAX && !visited[r][c] && grid[r][c] == 0 {
                                    q.push_back((r, c, nd));
                                    visited[r][c] = true;
                                }
                            }
                        }
                    }
                    for r in 0..m {
                        for c in 0..n {
                            if !visited[r][c] {
                                distances[r][c] = i32::MAX;
                            } else {
                                visited[r][c] = false;
                            }
                        }
                    }
                }
            }
        }
        //println!("distances: {:?}", distances);

        let mut result = i32::MAX;
        for r in 0..m {
            for c in 0..n {
                result = result.min(distances[r][c]);
            }
        }
        match result {
            i32::MAX => -1,
            r => r,
        }
    }
    pub fn shortest_distance_reuse_q_and_visited(grid: Vec<Vec<i32>>) -> i32 {
        //println!("shortest_distance({:?})", grid);
        use std::collections::VecDeque;

        let m = grid.len();
        let n = grid[0].len();
        let mut distances = vec![vec![0; n]; m];
        let mut q = VecDeque::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        for (r, c) in (0..m).flat_map(|r| (0..n).map(move |c| (r, c))) {
            if grid[r][c] == 2 {
                distances[r][c] = i32::MAX;
            } else if grid[r][c] == 1 {
                //println!("update_distances({:?}, {},{})", distances, r, c);
                distances[r][c] = i32::MAX;
                visited[r][c] = true;
                q.push_back((r, c, 0));

                while let Some((r, c, d)) = q.pop_front() {
                    //println!(" ds[{}][{}] = {} + {}", r, c, ds[r][c], d);
                    distances[r][c] += d;
                    let nd = d + 1;
                    let nrcs = [
                        (r.checked_sub(1), Some(c)),
                        (Some(r + 1).filter(|&r| r < m), Some(c)),
                        (Some(r), c.checked_sub(1)),
                        (Some(r), Some(c + 1).filter(|&c| c < n)),
                    ];
                    for &rc in &nrcs {
                        if let (Some(r), Some(c)) = rc {
                            if distances[r][c] < i32::MAX && !visited[r][c] && grid[r][c] == 0 {
                                q.push_back((r, c, nd));
                                visited[r][c] = true;
                            }
                        }
                    }
                }
                for (r, c) in (0..m).flat_map(|r| (0..n).map(move |c| (r, c))) {
                    if !visited[r][c] {
                        distances[r][c] = i32::MAX;
                    } else {
                        visited[r][c] = false;
                    }
                }
            }
        }
        //println!("distances: {:?}", distances);

        let mut result = i32::MAX;
        for (r, c) in (0..m).flat_map(|r| (0..n).map(move |c| (r, c))) {
            result = result.min(distances[r][c]);
        }
        match result {
            i32::MAX => -1,
            r => r,
        }
    }
    pub fn shortest_distance_my_init(grid: Vec<Vec<i32>>) -> i32 {
        //println!("shortest_distance({:?})", grid);
        use std::collections::VecDeque;

        fn update_distances(ds: &mut Vec<Vec<i32>>, r: usize, c: usize, grid: &[Vec<i32>]) {
            //println!("update_distances({:?}, {},{})", ds, r, c);
            let m = grid.len();
            let n = grid[0].len();

            let mut q = VecDeque::new();
            q.push_back((r, c, 0));
            let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
            ds[r][c] = i32::MAX;
            visited[r][c] = true;

            while let Some((r, c, d)) = q.pop_front() {
                //println!(" ds[{}][{}] = {} + {}", r, c, ds[r][c], d);
                ds[r][c] += d;
                let nd = d + 1;
                let nrcs = [
                    (r.checked_sub(1), Some(c)),
                    (Some(r + 1).filter(|&r| r < m), Some(c)),
                    (Some(r), c.checked_sub(1)),
                    (Some(r), Some(c + 1).filter(|&c| c < n)),
                ];
                for &rc in &nrcs {
                    if let (Some(r), Some(c)) = rc {
                        if ds[r][c] < i32::MAX && !visited[r][c] && grid[r][c] == 0 {
                            q.push_back((r, c, nd));
                            visited[r][c] = true;
                        }
                    }
                }
            }
            for (r, c) in (0..m).flat_map(|r| (0..n).map(move |c| (r, c))) {
                if !visited[r][c] {
                    ds[r][c] = i32::MAX;
                }
            }
        }

        let m = grid.len();
        let n = grid[0].len();
        let mut distances = vec![vec![0; n]; m];
        for (r, c) in (0..m).flat_map(|r| (0..n).map(move |c| (r, c))) {
            match grid[r][c] {
                1 => update_distances(&mut distances, r, c, &grid),
                2 => distances[r][c] = i32::MAX,
                _ => (),
            }
        }
        //println!("distances: {:?}", distances);

        let mut result = i32::MAX;
        for (r, c) in (0..m).flat_map(|r| (0..n).map(move |c| (r, c))) {
            result = result.min(distances[r][c]);
        }
        match result {
            i32::MAX => -1,
            r => r,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]}}

    #[test]
    fn g_10201_00000_00100() {
        let g = vv![[1, 0, 2, 0, 1], [0, 0, 0, 0, 0], [0, 0, 1, 0, 0]];
        assert_eq!(Solution::shortest_distance(g), 7);
        // Explanation: Given three buildings at (0,0), (0,4), (2,2), and an obstacle at (0,2).
        // The point (1,2) is an ideal empty land to build a house, as the total travel distance of 3+3+1=7 is minimal.
        // So return 7.
    }
    #[test]
    fn g_11201_10000_00100() {
        let g = vv![[1, 1, 2, 0, 1], [1, 0, 0, 0, 0], [0, 0, 1, 0, 0]];
        assert_eq!(Solution::shortest_distance(g), -1);
    }
    #[test]
    fn g_01201_10000_00100() {
        let g = vv![[0, 1, 2, 0, 1], [1, 0, 0, 0, 0], [0, 0, 1, 0, 0]];
        assert_eq!(Solution::shortest_distance(g), 8);
    }
    #[test]
    fn g_10201_10000_10100() {
        let g = vv![[1, 0, 2, 0, 1], [1, 0, 0, 0, 0], [1, 0, 1, 0, 0]];
        assert_eq!(Solution::shortest_distance(g), 11);
    }
    #[test]
    fn g_10() {
        let g = vv![[1, 0]];
        assert_eq!(Solution::shortest_distance(g), 1);
    }
    #[test]
    fn g_1() {
        let g = vv![[1]];
        assert_eq!(Solution::shortest_distance(g), -1);
    }

    mod performance {
        use super::*;

        #[test]
        fn g_50x50_1() {
            let g = vec![vec![1; 50]; 50];
            assert_eq!(Solution::shortest_distance(g), -1);
        }
        #[test]
        fn g_outer_1s_iner_0s() {
            let mut g = vec![vec![0; 50]; 50];
            for (r, c) in (0..50).flat_map(|r| (0..50).map(move |c| (r, c))) {
                if r == 0 || r == 49 || c == 0 || c == 49 {
                    g[r][c] = 1;
                }
            }
            assert_eq!(Solution::shortest_distance(g), -1);
        }
        #[test]
        fn g_outer_1s_iner_0s_except_for_corners() {
            let mut g = vec![vec![0; 50]; 50];
            for (r, c) in (0..50).flat_map(|r| (0..50).map(move |c| (r, c))) {
                if r == 0 || r == 49 || c == 0 || c == 49 {
                    g[r][c] = 1;
                }
            }
            g[0][0] = 2;
            g[0][49] = 2;
            g[49][0] = 2;
            g[49][49] = 2;
            //println!("Big Grid:\n{:?}", g);
            assert_eq!(Solution::shortest_distance(g), 7008);
        }
    }
}
