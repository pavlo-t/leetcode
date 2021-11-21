#![allow(dead_code)]
/// 361. Bomb Enemy
/// ===============
///
/// Given an `m x n` matrix `grid` where each cell is either a wall `'W'`, an enemy `'E'` or empty `'0'`,
/// return _the maximum enemies you can kill using one bomb_.
/// You can only place the bomb in an empty cell.
///
/// The bomb kills all the enemies in the same row and column from the planted point
/// until it hits the wall since it is too strong to be destroyed.
///
/// __Constraints:__
///
/// - `1 <= grid.length, grid[i].length <= 500`
/// - `grid[i][j]` is either `'W'`, `'E'`, or `'0'`.
///
/// https://leetcode.com/problems/bomb-enemy/
struct Solution;
impl Solution {
    pub fn max_killed_enemies_rec(grid: Vec<Vec<char>>) -> i32 {
        println!("max_killed_enemies({:?})", grid);
        /// d: direction; 0 - left, 1 - righ, 2 - up, 3 - down
        fn kills(r: usize, c: usize, d: usize, g: &[Vec<char>]) -> i32 {
            let (m, n) = (g.len(), g[0].len());
            match g[r][c] {
                'W' => None,
                'E' => Some(1),
                _ => Some(0),
            }
            .map(|k| {
                let (r, c) = match d {
                    0 => (r.wrapping_sub(1), c),
                    1 => (r + 1, c),
                    2 => (r, c.wrapping_sub(1)),
                    _ => (r, c + 1),
                };
                k + if r < m && c < n { kills(r, c, d, g) } else { 0 }
            })
            .unwrap_or(0)
        }
        let (m, n) = (grid.len(), grid[0].len());
        let mut result = 0;
        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == '0' {
                    result = result.max((0..4).map(|d| kills(r, c, d, &grid)).sum());
                }
            }
        }
        result
    }
    pub fn max_killed_enemies_rec_with_memo(grid: Vec<Vec<char>>) -> i32 {
        println!("max_killed_enemies({:?})", grid);
        /// d: direction; 0 - left, 1 - righ, 2 - up, 3 - down
        #[rustfmt::skip]
        fn kills(r: usize, c: usize, d: usize, g: &[Vec<char>], memo: &mut Vec<Vec<[i32; 4]>>) -> i32 {
            let (m, n) = (g.len(), g[0].len());
            if memo[r][c][d] != -1 {
                memo[r][c][d]
            } else {
                memo[r][c][d] = match g[r][c] {
                    'W' => None,
                    'E' => Some(1),
                    _ => Some(0),
                }
                .map(|k| {
                    let (r, c) = match d {
                        0 => (r.wrapping_sub(1), c),
                        1 => (r + 1, c),
                        2 => (r, c.wrapping_sub(1)),
                        _ => (r, c + 1),
                    };
                    k + if r < m && c < n { kills(r, c, d, g, memo) } else { 0 }
                })
                .unwrap_or(0);
                memo[r][c][d]
            }
        }
        let (m, n) = (grid.len(), grid[0].len());
        let mut memo = vec![vec![[-1; 4]; n]; m];
        let mut result = 0;
        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == '0' {
                    result = result.max((0..4).map(|d| kills(r, c, d, &grid, &mut memo)).sum());
                }
            }
        }
        result
    }

    /// Approach 2: Dynamic Programming
    /// https://leetcode.com/problems/bomb-enemy/solution/
    pub fn max_killed_enemies(grid: Vec<Vec<char>>) -> i32 {
        println!("max_killed_enemies({:?})", grid);
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut max_count = 0;
        let mut row_hits = 0;
        let mut col_hits = vec![0; cols];

        for row in 0..rows {
            for col in 0..cols {
                // reset the hits on the row, if necessary.
                if col == 0 || grid[row][col - 1] == 'W' {
                    row_hits = 0;
                    for k in col..cols {
                        if grid[row][k] == 'W' {
                            // stop the scan when we hit the wall.
                            break;
                        } else if grid[row][k] == 'E' {
                            row_hits += 1;
                        }
                    }
                }
                // reset the hits on the column, if necessary.
                if row == 0 || grid[row - 1][col] == 'W' {
                    col_hits[col] = 0;
                    for k in row..rows {
                        if grid[k][col] == 'W' {
                            break;
                        } else if grid[k][col] == 'E' {
                            col_hits[col] += 1;
                        }
                    }
                }
                // run the calculation for the empty cell.
                if grid[row][col] == '0' {
                    max_count = max_count.max(row_hits + col_hits[col]);
                }
            }
        }

        max_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn oeoo_eowe_oeoo() {
        let g = vv![
            ['0', 'E', '0', '0'],
            ['E', '0', 'W', 'E'],
            ['0', 'E', '0', '0']
        ];
        assert_eq!(Solution::max_killed_enemies(g), 3);
    }
    #[test]
    fn www_ooo_eee() {
        let g = vv![['W', 'W', 'W'], ['0', '0', '0'], ['E', 'E', 'E']];
        assert_eq!(Solution::max_killed_enemies(g), 1);
    }

    #[test]
    fn g_0x500x500() {
        let g = vec![vec!['0'; 500]; 500];
        assert_eq!(Solution::max_killed_enemies(g), 0);
    }
}
