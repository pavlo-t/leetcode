#![allow(dead_code)]
/// 1035. Uncrossed Lines
/// =====================
///
/// You are given two integer arrays `nums1` and `nums2`.
/// We write the integers of `nums1` and `nums2` (in the order they are given) on two separate horizontal lines.
///
/// We may draw connecting lines: a straight line connecting two numbers `nums1[i]` and `nums2[j]` such that:
///
/// - `nums1[i] == nums2[j]`
/// - the line we draw does not intersect any other connecting (non-horizontal) line
///
/// Note that a connecting line cannot intersect even at the endpoints
/// (i.e., each number can only belong to one connecting line).
///
/// Return _the maximum number of connecting lines we can draw in this way_.
///
/// __Constraints:__
///
/// - `1 <= nums1.length, nums2.length <= 500`
/// - `1 <= nums1[i], nums2[j] <= 2000`
///
/// https://leetcode.com/problems/uncrossed-lines/
struct Solution;
impl Solution {
    /// 17:41-17:49
    pub fn max_uncrossed_lines_rec(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        println!("max_uncrossed_lines({:?}, {:?})", nums1, nums2);
        fn rec(i: usize, j: usize, is: &[i32], js: &[i32]) -> i32 {
            if i == is.len() || j == js.len() {
                0
            } else {
                if is[i] == js[j] {
                    1 + rec(i + 1, j + 1, is, js)
                } else {
                    rec(i + 1, j, is, js).max(rec(i, j + 1, is, js))
                }
            }
        }
        rec(0, 0, &nums1, &nums2)
    }
    /// 17:49-17:54
    pub fn max_uncrossed_lines_dp_vec_vec(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        println!("max_uncrossed_lines({:?}, {:?})", nums1, nums2);
        let (m, n) = (nums1.len(), nums2.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                dp[i + 1][j + 1] = if nums1[i] == nums2[j] {
                    1 + dp[i][j]
                } else {
                    dp[i][j + 1].max(dp[i + 1][j])
                };
            }
        }
        dp[m][n]
    }
    /// 17:54-18:03
    pub fn max_uncrossed_lines(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        println!("max_uncrossed_lines({:?}, {:?})", nums1, nums2);
        if nums1.len() < nums2.len() {
            std::mem::swap(&mut nums1, &mut nums2);
        }
        let (m, n) = (nums1.len(), nums2.len());
        let mut dp = vec![0; n + 1];
        for i in 0..m {
            let mut prev = dp[0];
            for j in 0..n {
                let tmp = dp[j + 1];
                dp[j + 1] = if nums1[i] == nums2[j] {
                    1 + prev
                } else {
                    dp[j + 1].max(dp[j])
                };
                prev = tmp;
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1_n_1() { assert_eq!(Solution::max_uncrossed_lines(vec![1], vec![1]), 1); }
    #[rustfmt::skip] #[test] fn n_1_n_2() { assert_eq!(Solution::max_uncrossed_lines(vec![1], vec![2]), 0); }
    #[test]
    fn n_142_n_124() {
        let n1 = vec![1, 4, 2];
        let n2 = vec![1, 2, 4];
        assert_eq!(Solution::max_uncrossed_lines(n1, n2), 2);
        // Explanation: We can draw 2 uncrossed lines as in the diagram.
        // 1 4 2
        // |  \
        // 1 2 4
        // We cannot draw 3 uncrossed lines, because the line from `nums1[1]=4` to `nums2[2]=4`
        // will intersect the line from `nums1[2]=2` to `nums2[1]=2`.
    }
    #[test]
    fn n_25125_n_10_5_2_1_5_2() {
        let n1 = vec![2, 5, 1, 2, 5];
        let n2 = vec![10, 5, 2, 1, 5, 2];
        assert_eq!(Solution::max_uncrossed_lines(n1, n2), 3);
    }
    #[test]
    fn n_137175_n_19251() {
        let n1 = vec![1, 3, 7, 1, 7, 5];
        let n2 = vec![1, 9, 2, 5, 1];
        assert_eq!(Solution::max_uncrossed_lines(n1, n2), 2);
    }

    #[test]
    fn n_1_repeat_500_n_1_repeat_500() {
        let n1 = vec![1; 500];
        let n2 = vec![1; 500];
        assert_eq!(Solution::max_uncrossed_lines(n1, n2), 500);
    }
    #[test]
    fn n_1_repeat_500_n_2_repeat_500() {
        let n1 = vec![1; 500];
        let n2 = vec![2; 500];
        assert_eq!(Solution::max_uncrossed_lines(n1, n2), 0);
    }
}
