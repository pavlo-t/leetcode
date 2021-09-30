#![allow(dead_code)]
/// Spiral Matrix
/// =============
///
/// Given an `m x n` `matrix`, return _all elements of the `matrix` in spiral order_.
///
/// __Constraints:__
///
/// - `1 <= matrix.length, matrix[i].length <= 10`
/// - `-100 <= matrix[i][j] <= 100`
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/638/week-3-september-15th-september-21st/3977/
struct Solution;
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        struct SpiralIter {
            m: Vec<Vec<i32>>,
            r: usize,
            c: usize,
            // direction: 0 = right, 1 = down, 2 = left, 3 = up
            d: u8,
            outer: [usize; 4],
            size: usize,
        }
        impl SpiralIter {
            fn new(matrix: Vec<Vec<i32>>) -> Self {
                let (m, n) = (matrix.len(), matrix[0].len());
                Self {
                    m: matrix,
                    r: 0,
                    c: 0,
                    d: 0,
                    outer: [0, m - 1, 0, n - 1],
                    size: m * n,
                }
            }
        }
        impl Iterator for SpiralIter {
            type Item = i32;
            fn next(&mut self) -> Option<Self::Item> {
                let [ot, ob, ol, or] = &mut self.outer;
                let result = if *ol <= self.c && self.c <= *or && *ot <= self.r && self.r <= *ob {
                    Some(self.m[self.r.clone()][self.c.clone()])
                } else {
                    None
                };
                match self.d {
                    0 if self.c < *or => self.c += 1,
                    0 => {
                        self.d = 1;
                        self.r += 1;
                        *ot += 1;
                    }
                    1 if self.r < *ob => self.r += 1,
                    1 if self.c == 0 => {
                        *or = 0;
                        *ol = 1;
                    }
                    1 => {
                        self.d = 2;
                        self.c -= 1;
                        *or -= 1;
                    }
                    2 if self.c > *ol => self.c -= 1,
                    2 => {
                        self.d = 3;
                        self.r -= 1;
                        *ob -= 1;
                    }
                    3 if self.r > *ot => self.r -= 1,
                    3 => {
                        self.d = 0;
                        self.c += 1;
                        *ol += 1;
                    }
                    _ => unreachable!(),
                }
                result
            }
        }
        let mut mi = SpiralIter::new(matrix);
        let mut result = Vec::with_capacity(mi.size);
        while let Some(i) = mi.next() {
            result.push(i);
        }
        result
    }
    pub fn spiral_order_my_1(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len(), matrix[0].len());
        if m == 1 {
            matrix[0].to_owned()
        } else if n == 1 {
            matrix.into_iter().map(|r| r[0]).collect()
        } else {
            // d - direction: 0 = right, 1 = down, 2 = left, 3 = up
            fn next(
                r: &mut usize,
                c: &mut usize,
                d: &mut u8,
                m: &[Vec<i32>],
                outer: &mut [usize; 4],
            ) -> Option<i32> {
                let [ot, ob, ol, or] = outer;
                match d {
                    0 if c < or => *c += 1,
                    0 => {
                        *d = 1;
                        *r += 1;
                        *ot += 1;
                    }
                    1 if r < ob => *r += 1,
                    1 => {
                        *d = 2;
                        *c -= 1;
                        *or -= 1;
                    }
                    2 if c > ol => *c -= 1,
                    2 => {
                        *d = 3;
                        *r -= 1;
                        *ob -= 1;
                    }
                    3 if r > ot => *r -= 1,
                    3 => {
                        *d = 0;
                        *c += 1;
                        *ol += 1;
                    }
                    _ => unreachable!(),
                }
                if ol <= c && c <= or && ot <= r && r <= ob {
                    Some(m[r.clone()][c.clone()])
                } else {
                    None
                }
            }
            let mut result = Vec::with_capacity(m * n);
            let (mut r, mut c, mut d) = (0, 0, 0);
            let mut outer = [0, m - 1, 0, n - 1];
            result.push(matrix[r][c]);
            while let Some(i) = next(&mut r, &mut c, &mut d, &matrix, &mut outer) {
                result.push(i);
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]}}

    #[test]
    fn m_1c2c3_4c5c6_7c8c9_produces_1_2_3_6_9_8_7_4_5() {
        let matrix = vv![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let e = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(Solution::spiral_order(matrix), e);
    }
    #[test]
    fn m_1c2c3c4_5c6c7c8_9c10c11c12_produces_1_2_3_4_8_12_11_10_9_5_6_7() {
        let matrix = vv![[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
        let e = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
        assert_eq!(Solution::spiral_order(matrix), e);
    }
    #[test]
    fn mat_1c2_produces_1_2() {
        let matrix = vv![[1, 2]];
        let e = vec![1, 2];
        assert_eq!(Solution::spiral_order(matrix), e);
    }
    #[test]
    fn mat_1_2_produces_1_2() {
        let matrix = vv![[1], [2]];
        let e = vec![1, 2];
        assert_eq!(Solution::spiral_order(matrix), e);
    }
}
