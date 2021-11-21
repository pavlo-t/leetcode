#![allow(dead_code)]
/// 576. Out of Boundary Paths
/// ==========================
///
/// There is an `m x n` grid with a ball.
/// The ball is initially at the position `[startRow, startColumn]`.
/// You are allowed to move the ball to one of the four adjacent cells in the grid
/// (possibly out of the grid crossing the grid boundary).
/// You can apply __at most__ `maxMove` moves to the ball.
///
/// Given the five integers `m`, `n`, `maxMove`, `startRow`, `startColumn`,
/// return the number of paths to move the ball out of the grid boundary.
/// Since the answer can be very large, return it __modulo__ `10**9 + 7`.
///
/// __Constraints:__
///
/// - `1 <= m, n <= 50`
/// - `0 <= maxMove <= 50`
/// - `0 <= startRow < m`
/// - `0 <= startColumn < n`
///
/// https://leetcode.com/problems/out-of-boundary-paths/
struct Solution;
impl Solution {
    /// 16:50-17:07
    pub fn find_paths_rec(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        #[rustfmt::skip] println!("find_paths({}, {}, {}, {}, {})", m, n, max_move, start_row, start_column);
        use std::iter::once;

        const MOD: i32 = 1_000_000_007;

        fn rec(r: i32, c: i32, mv: i32, m: i32, n: i32) -> i32 {
            if r == 0 || r == m + 1 || c == 0 || c == n + 1 {
                1
            } else if mv == 0 {
                0
            } else {
                let mut result = 0;
                once((r - 1, c))
                    .chain(once((r + 1, c)))
                    .chain(once((r, c - 1)))
                    .chain(once((r, c + 1)))
                    .for_each(|(r, c)| {
                        result = (result + rec(r, c, mv - 1, m, n)) % MOD;
                    });
                result
            }
        }
        rec(start_row + 1, start_column + 1, max_move, m, n)
    }
    /// 17:07-17:17
    pub fn find_paths_rec_with_memo(
        m: i32,
        n: i32,
        max_move: i32,
        start_row: i32,
        start_column: i32,
    ) -> i32 {
        #[rustfmt::skip] println!("find_paths({}, {}, {}, {}, {})", m, n, max_move, start_row, start_column);
        use std::iter::once;

        const MOD: i32 = 1_000_000_007;

        fn rec(
            r: usize,
            c: usize,
            mv: usize,
            mr: usize,
            mc: usize,
            memo: &mut Vec<Vec<Vec<i32>>>,
        ) -> i32 {
            if memo[r][c][mv] != -1 {
                memo[r][c][mv]
            } else {
                memo[r][c][mv] = if r == 0 || r == mr || c == 0 || c == mc {
                    1
                } else if mv == 0 {
                    0
                } else {
                    let mut result = 0;
                    once((r - 1, c))
                        .chain(once((r + 1, c)))
                        .chain(once((r, c - 1)))
                        .chain(once((r, c + 1)))
                        .for_each(|(r, c)| {
                            result = (result + rec(r, c, mv - 1, mr, mc, memo)) % MOD;
                        });
                    result
                };
                memo[r][c][mv]
            }
        }
        let (r, c) = (start_row as usize + 1, start_column as usize + 1);
        let (mv, mr, mc) = (max_move as usize, m as usize + 1, n as usize + 1);
        let mut memo = vec![vec![vec![-1; mv + 1]; mc + 1]; mr + 1];
        rec(r, c, mv, mr, mc, &mut memo)
    }
    /// 17:17-17:43
    pub fn find_paths_dp_vec_vec_vec(
        m: i32,
        n: i32,
        max_move: i32,
        start_row: i32,
        start_column: i32,
    ) -> i32 {
        #[rustfmt::skip] println!("find_paths({}, {}, {}, {}, {})", m, n, max_move, start_row, start_column);
        const MOD: i32 = 1_000_000_007;
        let (mv, mr, mc) = (max_move as usize, m as usize, n as usize);
        let mut dp = vec![vec![vec![0; mc + 2]; mr + 2]; mv + 1];
        for c in 1..=mc {
            dp[0][0][c] = 1;
            dp[0][mr + 1][c] = 1;
        }
        for r in 1..=mr {
            dp[0][r][0] = 1;
            dp[0][r][mc + 1] = 1;
        }
        for m in 1..=mv {
            for c in 1..=mc {
                dp[m][0][c] = 1;
                dp[m][mr + 1][c] = 1;
            }
            for r in 1..=mr {
                dp[m][r][0] = 1;
                dp[m][r][mc + 1] = 1;
            }
            for r in 1..=mr {
                for c in 1..=mc {
                    dp[m][r][c] = (dp[m][r][c] + dp[m - 1][r - 1][c]) % MOD;
                    dp[m][r][c] = (dp[m][r][c] + dp[m - 1][r + 1][c]) % MOD;
                    dp[m][r][c] = (dp[m][r][c] + dp[m - 1][r][c - 1]) % MOD;
                    dp[m][r][c] = (dp[m][r][c] + dp[m - 1][r][c + 1]) % MOD;
                }
            }
        }
        println!("dp: {:?}", dp);
        dp[mv][start_row as usize + 1][start_column as usize + 1]
    }
    /// 17:43-17:47
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        #[rustfmt::skip] println!("find_paths({}, {}, {}, {}, {})", m, n, max_move, start_row, start_column);
        const MOD: i32 = 1_000_000_007;
        let (mv, mr, mc) = (max_move as usize, m as usize, n as usize);
        let mut curr = vec![vec![0; mc + 2]; mr + 2];
        for c in 1..=mc {
            curr[0][c] = 1;
            curr[mr + 1][c] = 1;
        }
        for r in 1..=mr {
            curr[r][0] = 1;
            curr[r][mc + 1] = 1;
        }
        let mut prev = curr.clone();

        for _ in 0..mv {
            std::mem::swap(&mut curr, &mut prev);
            for r in 1..=mr {
                for c in 1..=mc {
                    let mut result = 0;
                    result = (result + prev[r - 1][c]) % MOD;
                    result = (result + prev[r + 1][c]) % MOD;
                    result = (result + prev[r][c - 1]) % MOD;
                    result = (result + prev[r][c + 1]) % MOD;
                    curr[r][c] = result;
                }
            }
        }
        curr[start_row as usize + 1][start_column as usize + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn m1n1mm0r0c0() { assert_eq!(Solution::find_paths(1, 1, 0, 0, 0), 0); }
    #[rustfmt::skip] #[test] fn m1n1mm1r0c0() { assert_eq!(Solution::find_paths(1, 1, 1, 0, 0), 4); }
    #[rustfmt::skip] #[test] fn m1n1mm2r0c0() { assert_eq!(Solution::find_paths(1, 1, 2, 0, 0), 4); }

    #[test]
    fn m2n2mm2r0c0() {
        assert_eq!(Solution::find_paths(2, 2, 2, 0, 0), 6);
    }
    #[test]
    fn m1n3mm3r0c1() {
        assert_eq!(Solution::find_paths(1, 3, 3, 0, 1), 12);
    }

    #[rustfmt::skip] #[test] fn m9n9mm9r0c0() { assert_eq!(Solution::find_paths(9, 9, 9, 0, 0), 5022); }

    #[rustfmt::skip] #[test] fn m9n9mm50r0c0()   { assert_eq!(Solution::find_paths( 9,  9, 50, 0, 0),  88_573_477); }
    #[rustfmt::skip] #[test] fn m50n50mm50r0c0() { assert_eq!(Solution::find_paths(50, 50, 50, 0, 0), 678_188_903); }
}
