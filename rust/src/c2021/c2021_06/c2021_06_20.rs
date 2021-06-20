#![allow(dead_code)]
/// Swim in Rising Water
/// ====================
///
/// On an N x N `grid`, each square `grid[i][j]` represents the elevation at that point `(i,j)`.
///
/// Now rain starts to fall.
/// At time `t`, the depth of the water everywhere is `t`.
/// You can swim from a square to another 4-directionally adjacent square
/// if and only if the elevation of both squares individually are at most `t`.
/// You can swim infinite distance in zero time.
/// Of course, you must stay within the boundaries of the grid during your swim.
///
/// You start at the top left square `(0, 0)`.
/// What is the least time until you can reach the bottom right square `(N-1, N-1)`?
///
/// __Note:__
///
/// - `2 <= N <= 50`.
/// - `grid[i][j]` is a permutation of `[0, ..., N*N - 1]`.
///
/// https://leetcode.com/explore/challenge/card/june-leetcoding-challenge-2021/605/week-3-june-15th-june-21st/3785/
struct Solution;
impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        fn can_reach_right_bottom(t: i32, g: &[Vec<i32>]) -> bool {
            use std::collections::HashSet;

            fn rec(
                r: usize,
                c: usize,
                s: &mut HashSet<(usize, usize)>,
                t: i32,
                g: &[Vec<i32>],
            ) -> bool {
                let li = g.len() - 1;
                if g[r][c] > t || s.contains(&(r, c)) {
                    false
                } else if r == li && c == li {
                    true
                } else {
                    s.insert((r, c));
                    (c > 0 && rec(r, c - 1, s, t, g))
                        || (c < li && rec(r, c + 1, s, t, g))
                        || (r > 0 && rec(r - 1, c, s, t, g))
                        || (r < li && rec(r + 1, c, s, t, g))
                }
            }
            rec(0, 0, &mut HashSet::new(), t, g)
        }

        let n = grid.len();

        let mut l = 0;
        let mut r = (n * n) as i32 - 1;

        while l < r {
            let m = l + (r - l) / 2;
            if can_reach_right_bottom(m, &grid) {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn example1() {
        let grid = vv![[0, 2], [1, 3]];
        assert_eq!(Solution::swim_in_water(grid), 3);
        // Explanation:
        // At time 0, you are in grid location (0, 0).
        // You cannot go anywhere else because 4-directionally adjacent neighbors have a higher elevation than t = 0.
        //
        // You cannot reach point (1, 1) until time 3.
        // When the depth of water is 3, we can swim anywhere inside the grid.
    }
    #[test]
    fn example2() {
        let grid = vv![
            [0, 1, 2, 3, 4],
            [24, 23, 22, 21, 5],
            [12, 13, 14, 15, 16],
            [11, 17, 18, 19, 20],
            [10, 9, 8, 7, 6]
        ];
        assert_eq!(Solution::swim_in_water(grid), 16);
        // Explanation:
        //  0 >  1 >  2 >  3 >  4
        //                      v
        // 24   23   22   21    5
        //                      v
        // 12 < 13 < 14 < 15 < 16
        //  v
        // 11   17   18   19   20
        //  v
        // 10 >  9 >  8 >  7 >  6
        //
        // We need to wait until time 16 so that (0, 0) and (4, 4) are connected.
    }

    mod performance {
        use super::*;

        #[test]
        fn g50x50_produces_2499() {
            let n = 50;
            let grid = (0..n).flat_map(|r| (0..n).map(move |c| (r, c))).fold(
                vec![vec![-1; n]; n],
                |mut acc, (r, c)| {
                    acc[r][c] = (r * n + c) as i32;
                    acc
                },
            );
            assert_eq!(Solution::swim_in_water(grid), 2499);
        }
    }
}
