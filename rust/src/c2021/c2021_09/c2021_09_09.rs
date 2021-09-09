#![allow(dead_code)]
/// Largest Plus Sign
/// =================
///
/// You are given an integer `n`.
/// You have an `n x n` binary grid `grid` with all values initially `1`'s
/// except for some indices given in the array `mines`.
/// The `i`th element of the array `mines` is defined as `mines[i] = [xi, yi]` where `grid[xi][yi] == 0`.
///
/// Return _the order of the largest __axis-aligned__ plus sign of `1`'s contained in_ `grid`.
/// If there is none, return `0`.
///
/// An __axis-aligned plus sign__ of `1`'s of order `k` has some center `grid[r][c] == 1`
/// along with four arms of length `k - 1` going up, down, left, and right, and made of `1`'s.
/// Note that there could be `0`'s or `1`'s beyond the arms of the plus sign,
/// only the relevant area of the plus sign is checked for `1`'s.
///
/// __Constraints:__
///
/// - `1 <= n <= 500`
/// - `1 <= mines.length <= 5000`
/// - `0 <= xi, yi < n`
/// - All the pairs `(xi, yi)` are __unique__.
///
/// https://leetcode.com/explore/challenge/card/september-leetcoding-challenge-2021/637/week-2-september-8th-september-14th/3969/
struct Solution;
impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut board = vec![vec![1; n]; n];
        for m in mines {
            let (r, c) = (m[0] as usize, m[1] as usize);
            board[r][c] = 0;
        }

        let mut pr = vec![0; n];
        for r in 0..n {
            let mut pc = 0;
            for c in 0..n {
                if board[r][c] != 0 {
                    pc += 1;
                    pr[c] += 1;
                    board[r][c] = pc.min(pr[c]);
                } else {
                    pc = 0;
                    pr[c] = 0;
                }
            }
        }
        let mut result = 0;
        let mut pr = vec![0; n];
        for r in (0..n).rev() {
            let mut pc = 0;
            for c in (0..n).rev() {
                if board[r][c] != 0 {
                    pc += 1;
                    pr[c] += 1;
                    result = result.max(board[r][c].min(pc).min(pr[c]));
                } else {
                    pc = 0;
                    pr[c] = 0;
                }
            }
        }
        result
    }
    pub fn order_of_largest_plus_sign_my_1(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        println!("order_of_largest_plus_sign({}, {:?})", n, mines);
        let n = n as usize;

        let mut board = vec![vec![true; n]; n];
        for m in mines {
            let (r, c) = (m[0] as usize, m[1] as usize);
            board[r][c] = false;
        }

        let mut udp = vec![vec![-1; n]; n];
        let mut ldp = vec![vec![-1; n]; n];
        for r in 0..n {
            for c in 0..n {
                if board[r][c] {
                    udp[r][c] = if r > 0 { udp[r - 1][c] + 1 } else { 1 };
                    ldp[r][c] = if c > 0 { ldp[r][c - 1] + 1 } else { 1 };
                } else {
                    udp[r][c] = 0;
                    ldp[r][c] = 0;
                }
            }
        }
        let mut ddp = vec![vec![-1; n]; n];
        let mut rdp = vec![vec![-1; n]; n];
        for r in (0..n).rev() {
            for c in (0..n).rev() {
                if board[r][c] {
                    ddp[r][c] = if r < n - 1 { ddp[r + 1][c] + 1 } else { 1 };
                    rdp[r][c] = if c < n - 1 { rdp[r][c + 1] + 1 } else { 1 };
                } else {
                    ddp[r][c] = 0;
                    rdp[r][c] = 0;
                }
            }
        }

        (0..n)
            .flat_map(|r| (0..n).map(move |c| (r, c)))
            .fold(0, |rsf, (r, c)| {
                rsf.max(udp[r][c].min(ddp[r][c]).min(ldp[r][c]).min(rdp[r][c]))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]}}

    #[test]
    fn n5_m_x4y2_produces_2() {
        assert_eq!(Solution::order_of_largest_plus_sign(5, vv![[4, 2]]), 2);
        // 1 1 1 1 1
        // 1 1 1 1 1
        // 1 * 1 1 1
        // * * * 1 1
        // 1 * 0 1 1
        // Output: 2
        // Explanation: In the above grid, the largest plus sign can only be of order 2.
        // One of them is shown with *'s.
    }
    #[test]
    fn n1_m_x0y0_produces_0() {
        assert_eq!(Solution::order_of_largest_plus_sign(1, vv![[0, 0]]), 0);
        // Explanation: There is no plus sign, so return 0.
    }
    #[test]
    fn n2_m_x0y0_produces_1() {
        assert_eq!(Solution::order_of_largest_plus_sign(2, vv![[0, 0]]), 1);
    }
    #[test]
    fn n5_m_x2y4_produces_2() {
        assert_eq!(Solution::order_of_largest_plus_sign(5, vv![[2, 4]]), 2);
    }
    #[test]
    fn n5_m_x0y2_produces_2() {
        assert_eq!(Solution::order_of_largest_plus_sign(5, vv![[0, 2]]), 2);
    }
    #[test]
    fn n5_m_x2y0_produces_2() {
        assert_eq!(Solution::order_of_largest_plus_sign(5, vv![[2, 0]]), 2);
    }
    #[test]
    fn n5_m_x2y2_produces_2() {
        assert_eq!(Solution::order_of_largest_plus_sign(5, vv![[2, 2]]), 2);
    }
    #[test]
    fn n5_m_x1y1_produces_3() {
        assert_eq!(Solution::order_of_largest_plus_sign(5, vv![[1, 1]]), 3);
    }

    mod performance {
        use super::*;

        #[test]
        fn n500_m_x1y1_produces_250() {
            assert_eq!(Solution::order_of_largest_plus_sign(500, vv![[1, 1]]), 250);
        }
    }
}
