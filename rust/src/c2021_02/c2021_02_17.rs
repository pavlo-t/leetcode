#![allow(dead_code)]

/// # Container With Most Water
///
/// Given `n` non-negative integers `a1, a2, ..., an`,
/// where each represents a point at coordinate `(i, ai)`.
/// `n` vertical lines are drawn such that the two endpoints of the line `i` is at
/// `(i, ai)` and `(i, 0)`.
/// Find two lines, which, together with the x-axis forms a container,
/// such that the container contains the most water.
///
/// __Notice__ that you may not slant the container.
///
/// __Constraints:__
///
/// - `n == height.length`
/// - `2 <= n <= 30_000`
/// - `0 <= height[i] <= 30_000`
///
/// https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/586/week-3-february-15th-february-21st/3643/
struct Solution;

impl Solution {
    pub fn max_area(hs: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = hs.len() - 1;
        fn get_volume(l: usize, r: usize, hs: &[i32]) -> i32 {
            (hs[l].min(hs[r])) * ((r - l) as i32)
        }
        let mut rsf = get_volume(l, r, &hs);

        while l < r {
            if hs[l] <= hs[r] {
                let lower = hs[l];
                while l < r && hs[l] <= lower {
                    l += 1;
                }
            } else {
                let lower = hs[r];
                while l < r && hs[r] <= lower {
                    r -= 1;
                }
            }
            rsf = rsf.max(get_volume(l, r, &hs));
        }

        rsf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_h1_8_6_2_5_4_8_3_7_should_produce_49() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(Solution::max_area(height), 49);
        // Explanation:
        // The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7].
        // In this case, the max area of water (blue section) the container can contain is 49.
    }

    #[test]
    fn example2_h1_1_should_produce_1() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }

    #[test]
    fn example3_h4_3_2_1_4_should_produce_16() {
        assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
    }

    #[test]
    fn example4_h1_2_1_should_produce_2() {
        assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
    }
}
