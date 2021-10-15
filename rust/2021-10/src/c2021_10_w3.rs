#![allow(dead_code)]
/// 302. Smallest Rectangle Enclosing Black Pixels
/// ==============================================
///
/// You are given an `m x n` binary matrix `image` where `0` represents a white pixel and `1` represents a black pixel.
///
/// The black pixels are connected (i.e., there is only one black region).
/// Pixels are connected horizontally and vertically.
///
/// Given two integers `x` and `y` that represents the location of one of the black pixels,
/// return _the area of the smallest (axis-aligned) rectangle that encloses all black pixels_.
///
/// You must write an algorithm with less than `O(mn)` runtime complexity.
///
/// __Constraints:__
///
/// - `m == image.length`
/// - `n == image[i].length`
/// - `1 <= m, n <= 100`
/// - `image[i][j]` is either `'0'` or `'1'`
/// - `0 <= x < m`
/// - `0 <= y < n`
/// - `image[x][y] == '1'`
/// - The black pixels in the image only form one component
///
/// https://leetcode.com/problems/smallest-rectangle-enclosing-black-pixels/
struct Solution;
impl Solution {
    /// Approach 3: Binary Search
    /// https://leetcode.com/problems/smallest-rectangle-enclosing-black-pixels/solution/
    pub fn min_area(image: Vec<Vec<char>>, x: i32, y: i32) -> i32 {
        println!("min_area({:?}, {}, {})", image, x, y);
        fn search_columns(
            image: &[Vec<char>],
            mut i: usize,
            mut j: usize,
            top: usize,
            bottom: usize,
            white_to_black: bool,
        ) -> usize {
            while i != j {
                let mut k = top;
                let mid = (i + j) / 2;
                while k < bottom && image[k][mid] == '0' {
                    k += 1;
                }
                // k < bottom means the column mid has black pixel
                if (k < bottom) == white_to_black {
                    j = mid; // search the boundary in the smaller half
                } else {
                    i = mid + 1; // search the boundary in the greater half
                }
            }
            i
        }

        fn search_rows(
            image: &[Vec<char>],
            mut i: usize,
            mut j: usize,
            left: usize,
            right: usize,
            white_to_black: bool,
        ) -> usize {
            while i != j {
                let mut k = left;
                let mid = (i + j) / 2;
                while k < right && image[mid][k] == '0' {
                    k += 1;
                }
                // k < right means the row mid has black pixel
                if (k < right) == white_to_black {
                    j = mid;
                } else {
                    i = mid + 1;
                }
            }
            i
        }

        let (m, n) = (image.len(), image[0].len());
        let (x, y) = (x as usize, y as usize);

        let left = search_columns(&image, 0, y, 0, m, true);
        let right = search_columns(&image, y + 1, n, 0, m, false);
        let top = search_rows(&image, 0, x, left, right, true);
        let bottom = search_rows(&image, x + 1, m, left, right, false);

        ((right - left) * (bottom - top)) as i32
    }

    pub fn min_area_rec(image: Vec<Vec<char>>, x: i32, y: i32) -> i32 {
        println!("min_area({:?}, {}, {})", image, x, y);
        #[rustfmt::skip]
        fn limits(x: usize, y: usize, img: &[Vec<char>], seen: &mut Vec<Vec<bool>>) -> (usize, usize, usize, usize) {
            let (m, n) = (img.len(), img[0].len());
            let mut results = (x, x, y, y);

            for &(x, y) in [(x + 1, y), (x.wrapping_sub(1), y),
                            (x, y + 1), (x, y.wrapping_sub(1))
            ].iter().filter(|&&(x, y)| x < m && y < n) {
                if img[x][y] == '1' && !seen[x][y] {
                    seen[x][y] = true;
                    let (x_min, x_max, y_min, y_max) = limits(x, y, img, seen);
                    results.0 = results.0.min(x_min);
                    results.1 = results.1.max(x_max);
                    results.2 = results.2.min(y_min);
                    results.3 = results.3.max(y_max);
                }
            }
            results
        }

        let (m, n) = (image.len(), image[0].len());
        let (x, y) = (x as usize, y as usize);
        let mut seen = vec![vec![false; n]; m];

        let (x_min, x_max, y_min, y_max) = limits(x, y, &image, &mut seen);

        ((x_max - x_min + 1) * (y_max - y_min + 1)) as i32
    }
    pub fn min_area_rec_mut_results(image: Vec<Vec<char>>, x: i32, y: i32) -> i32 {
        println!("min_area({:?}, {}, {})", image, x, y);
        #[rustfmt::skip]
        fn limits(x: usize, y: usize, img: &[Vec<char>],
            seen: &mut Vec<Vec<bool>>,
            results: &mut (usize, usize, usize, usize),
        ) {
            let (m, n) = (img.len(), img[0].len());
            results.0 = results.0.min(x);
            results.1 = results.1.max(x);
            results.2 = results.2.min(y);
            results.3 = results.3.max(y);

            for &(x, y) in [(x + 1, y), (x.wrapping_sub(1), y),
                            (x, y + 1), (x, y.wrapping_sub(1))
            ].iter().filter(|&&(x, y)| x < m && y < n) {
                if img[x][y] == '1' && !seen[x][y] {
                    seen[x][y] = true;
                    limits(x, y, img, seen, results);
                }
            }
        }

        let (m, n) = (image.len(), image[0].len());
        let (x, y) = (x as usize, y as usize);
        let mut seen = vec![vec![false; n]; m];
        let mut results = (x, x, y, y);

        limits(x, y, &image, &mut seen, &mut results);

        let (x_min, x_max, y_min, y_max) = results;
        ((x_max - x_min + 1) * (y_max - y_min + 1)) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn i_0010_0110_0100_x_0_y_2() {
        let i = vv![
            ['0', '0', '1', '0'],
            ['0', '1', '1', '0'],
            ['0', '1', '0', '0']
        ];
        assert_eq!(Solution::min_area(i, 0, 2), 6);
    }
    #[test]
    fn i_1_x_0_y_0() {
        let i = vv![['1']];
        assert_eq!(Solution::min_area(i, 0, 0), 1);
    }

    #[test]
    fn i_100x100x1_x_0_y_0() {
        let i = vec![vec!['1'; 100]; 100];
        assert_eq!(Solution::min_area(i, 0, 0), 10_000);
    }
}
