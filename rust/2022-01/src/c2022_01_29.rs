#![allow(dead_code)]
/// 84. Largest Rectangle in Histogram
/// ==================================
///
/// Given an array of integers `heights` representing the histogram's bar height where the width of each bar is `1`,
/// return _the area of the largest rectangle in the histogram_.
///
/// __Constraints:__
///
/// - `1 <= heights.length <= 100_000`
/// - `0 <= heights[i] <= 10_000`
///
/// https://leetcode.com/problems/largest-rectangle-in-histogram/
struct Solution;
impl Solution {
    pub fn largest_rectangle_area_dp(heights: Vec<i32>) -> i32 {
        let max_height = *(heights.iter().max().unwrap()) as usize;
        let n = heights.len();
        let mut result = 0;
        let mut dp = vec![vec![0; max_height + 1]; n + 1];
        for c in (0..n).rev() {
            let max_h = heights[c] as usize;
            for r in 1..=max_h {
                let max_width = dp[c + 1][r] + 1;
                dp[c][r] = max_width;
                result = result.max(max_width * r);
            }
        }
        result as i32
    }

    /// from last year: `/rust/2020_2021-09/src/c2020/c2020_12/c2020_12_31.rs`
    pub fn largest_rectangle_area(hs: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut result = 0;
        for i in 0..hs.len() {
            while stack.last().map(|&j| hs[j] >= hs[i]).unwrap_or(false) {
                let height = hs[stack.pop().unwrap()];
                let width = stack.last().map(|&j| i - j - 1).unwrap_or(i) as i32;
                result = result.max(height * width);
            }
            stack.push(i);
        }
        let n = hs.len();
        while let Some(i) = stack.pop() {
            let height = hs[i];
            let width = stack.last().map(|&j| n - j - 1).unwrap_or(n) as i32;
            result = result.max(height * width);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h_2_1_5_6_2_3() {
        let h = vec![2, 1, 5, 6, 2, 3];
        assert_eq!(Solution::largest_rectangle_area(h), 10);
        //    o
        //   xx
        //   xx
        //   xx o
        // o xxoo
        // ooxxoo
        //
        // Explanation: The above is a histogram where width of each bar is 1.
        // The largest rectangle is shown with `x`, which has an area = 10 units.
    }
    #[test]
    fn h_2_4() {
        let h = vec![2, 4];
        assert_eq!(Solution::largest_rectangle_area(h), 4);
    }
    #[test]
    fn h_0_1_2_3_4_5_6_7() {
        let h = vec![0, 1, 2, 3, 4, 5, 6, 7];
        assert_eq!(Solution::largest_rectangle_area(h), 16);
        //        o
        //       oo
        //      ooo
        //     xxxx
        //    oxxxx
        //   ooxxxx
        //  oooxxxx
    }
    #[test]
    fn h_7_6_5_4_3_2_1_0() {
        let h = vec![7, 6, 5, 4, 3, 2, 1, 0];
        assert_eq!(Solution::largest_rectangle_area(h), 16);
        // o
        // oo
        // ooo
        // xxxx
        // xxxxo
        // xxxxoo
        // xxxxooo
    }

    //#[ignore]
    #[test]
    fn h_100000x10000() {
        let h = vec![10_000; 100_000];
        assert_eq!(Solution::largest_rectangle_area(h), 1_000_000_000);
    }
}
