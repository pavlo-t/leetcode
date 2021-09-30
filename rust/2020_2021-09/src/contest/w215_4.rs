#![allow(dead_code)]

struct Solution;
struct Solution1;

impl Solution {
    fn get_max_grid_happiness(n: i32, m: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
        let mut res = 0;
        let m = m as usize;
        let n = n as usize;
        let mut grid: Vec<Vec<i32>> = vec![vec![0; m]; n];
        let mut cur = 0;
        Self::dfs(
            0,
            introverts_count,
            extroverts_count,
            &mut grid,
            &mut cur,
            &mut res,
            n,
            m,
        );
        res
    }
    fn dfs(
        start: usize,
        a: i32,
        b: i32,
        grid: &mut Vec<Vec<i32>>,
        cur: &mut i32,
        max: &mut i32,
        n: usize,
        m: usize,
    ) {
        if a == 0 && b == 0 || start == n * m {
            *max = (*max).max(*cur);
            return;
        }
        if *cur + a * 120 + b * 120 <= *max { return; }
        let i = start / m;
        let j = start % m;
        if a > 0 {
            grid[i][j] = 1;
            let mut base = 120;
            if i > 0 && grid[i - 1][j] == 1 { base -= 60; }
            if i > 0 && grid[i - 1][j] == 2 { base -= 10; }
            if j > 0 && grid[i][j - 1] == 1 { base -= 60; }
            if j > 0 && grid[i][j - 1] == 2 { base -= 10; }
            *cur += base;
            Self::dfs(start + 1, a - 1, b, grid, cur, max, n, m);
            *cur -= base;
            grid[i][j] = 0;
        }
        if b > 0 {
            grid[i][j] = 2;
            let mut base = 40;
            if i > 0 && grid[i - 1][j] == 1 { base -= 10; }
            if i > 0 && grid[i - 1][j] == 2 { base += 40; }
            if j > 0 && grid[i][j - 1] == 1 { base -= 10; }
            if j > 0 && grid[i][j - 1] == 2 { base += 40; }
            *cur += base;
            Self::dfs(start + 1, a, b - 1, grid, cur, max, n, m);
            *cur -= base;
            grid[i][j] = 0;
        }
        if (i > 0 && grid[i - 1][j] != 0) || (j > 0 && grid[i][j - 1] != 0) {
            Self::dfs(start + 1, a, b, grid, cur, max, n, m);
        }
    }
}

impl Solution1 {
    pub fn get_max_grid_happiness(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
        struct Helpers {
            m: usize,
            n: usize,
            mem_size: usize,
            cache: Vec<Vec<Vec<Vec<Vec<i32>>>>>,
        }

        fn i_hap(n: usize) -> i32 { if n == 0 { 0 } else if n == 1 { -60 } else { -10 } }
        fn e_hap(n: usize) -> i32 { if n == 0 { 0 } else if n == 1 { -10 } else { 40 } }

        impl Helpers {
            fn new(m: usize, n: usize, ic: usize, ec: usize) -> Helpers {
                let mem_size = 3usize.pow(n as u32);
                let cache = vec![vec![vec![vec![vec![-1; mem_size]; ec + 1]; ic + 1]; n]; m];
                Helpers { m, n, mem_size, cache }
            }

            fn add_to_mem(&self, mem: usize, v: usize) -> usize { (mem * 3 + v) % self.mem_size }
            fn get_left(&self, mem: usize) -> usize { mem % 3 }
            fn get_up(&self, mem: usize) -> usize { mem / (self.mem_size / 3) }

            fn find_max(&mut self, row: usize, col: usize, ic: usize, ec: usize, mem: usize) -> i32 {
                if ic == 0 && ec == 0 {
                    0
                } else if row == self.m {
                    0
                } else if col == self.n {
                    self.find_max(row + 1, 0, ic, ec, mem)
                } else if self.cache[row][col][ic][ec][mem] != -1 {
                    self.cache[row][col][ic][ec][mem]
                } else {
                    let n_col = col + 1;
                    let up = if row == 0 { 0 } else { self.get_up(mem) };
                    let left = if col == 0 { 0 } else { self.get_left(mem) };

                    let hap0 = self.find_max(row, n_col, ic, ec, self.add_to_mem(mem, 0));
                    let hap1 = if ic == 0 { 0 } else {
                        let local_hap = 120 + i_hap(up) + i_hap(left);
                        let hap = self.find_max(row, n_col, ic - 1, ec, self.add_to_mem(mem, 1));
                        hap + local_hap
                    };
                    let hap2 = if ec == 0 { 0 } else {
                        let local_hap = 40 + e_hap(up) + e_hap(left);
                        let hap = self.find_max(row, n_col, ic, ec - 1, self.add_to_mem(mem, 2));
                        hap + local_hap
                    };

                    let hap = hap0.max(hap1).max(hap2);
                    self.cache[row as usize][col as usize][ic as usize][ec as usize][mem as usize] = hap;
                    hap
                }
            }
        }

        let mut h = Helpers::new(m as usize, n as usize, introverts_count as usize, extroverts_count as usize);

        h.find_max(0, 0, introverts_count as usize, extroverts_count as usize, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_3_1_2() {
        assert_eq!(Solution::get_max_grid_happiness(2, 3, 1, 2), 240);
    }

    #[test]
    fn test_3_1_2_1() {
        assert_eq!(Solution::get_max_grid_happiness(3, 1, 2, 1), 260);
    }

    #[test]
    fn test_2_2_4_0() {
        assert_eq!(Solution::get_max_grid_happiness(2, 2, 4, 0), 240);
    }

    #[test]
    fn test_5_5_6_6() {
        assert_eq!(Solution::get_max_grid_happiness(5, 5, 6, 6), 1240);
    }
}