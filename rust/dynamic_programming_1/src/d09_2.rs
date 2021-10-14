#![allow(dead_code)]
/// 42. Trapping Rain Water
/// =======================
///
/// Given `n` non-negative integers representing an elevation map where the width of each bar is `1`,
/// compute how much water it can trap after raining.
///
/// __Constraints:__
///
/// - `1 <= height.length <= 20_000`
/// - `0 <= height[i] <= 100_000`
///
/// https://leetcode.com/problems/trapping-rain-water/
struct Solution;
impl Solution {
    /// Approach 4: Using 2 pointers
    /// https://leetcode.com/problems/trapping-rain-water/solution/
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut max_l, mut max_r) = (0, 0);
        let (mut l, mut r) = (0, height.len() - 1);
        let mut result = 0;
        while l < r {
            let (lh, rh) = (height[l], height[r]);
            if lh <= rh {
                if lh >= max_l {
                    max_l = lh;
                } else {
                    result += max_l - lh;
                }
                l += 1;
            } else {
                if rh >= max_r {
                    max_r = rh;
                } else {
                    result += max_r - rh;
                }
                r -= 1;
            }
        }
        result
    }

    pub fn trap_dp_2vecs(height: Vec<i32>) -> i32 {
        println!("trap({:?})", height);
        let n = height.len();

        let max_ls = height.iter().fold(Vec::with_capacity(n), |mut acc, h| {
            acc.push(*acc.last().unwrap_or(&0).max(h));
            acc
        });
        #[rustfmt::skip]
        let mut max_rs = height.iter().rev().fold(Vec::with_capacity(n), |mut acc, h| {
            acc.push(*acc.last().unwrap_or(&0).max(h));
            acc
        });
        max_rs.reverse();

        let mut result = 0;
        for i in 1..height.len() - 1 {
            result += 0.max(max_ls[i].min(max_rs[i]) - height[i]);
        }
        result
    }

    pub fn trap_iter_2(height: Vec<i32>) -> i32 {
        println!("trap({:?})", height);
        let mut result = 0;
        for i in 1..height.len() - 1 {
            let mut max_l = 0;
            for l in 0..i {
                max_l = max_l.max(height[l]);
            }
            let mut max_r = 0;
            for r in i + 1..height.len() {
                max_r = max_r.max(height[r]);
            }
            result += (max_l.min(max_r) - height[i]).max(0);
        }
        result
    }
    pub fn trap_iter_1(height: Vec<i32>) -> i32 {
        println!("trap({:?})", height);
        let mut result = 0;
        for i in 1..height.len() - 1 {
            let &max_l = height[..i].iter().max().unwrap_or(&0);
            let &max_r = height[i + 1..].iter().max().unwrap_or(&0);
            result += (max_l.min(max_r) - height[i]).max(0);
        }
        result
    }
    pub fn trap_rec(height: Vec<i32>) -> i32 {
        println!("trap({:?})", height);
        fn rec(i: usize, h: &[i32]) -> i32 {
            if i >= h.len() {
                0
            } else {
                let &max_l = h[..i].iter().max().unwrap_or(&0);
                let &max_r = h[i + 1..].iter().max().unwrap_or(&0);
                (max_l.min(max_r) - h[i]).max(0) + rec(i + 1, h)
            }
        }
        rec(0, &height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h12_0_1_0_2_1_0_1_3_2_1_2_1() {
        let h = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(Solution::trap(h), 6);
        //        X
        //    X---XX-X
        // _X-XX_XXXXXX
        // Explanation: The above elevation map (`X` section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1].
        // In this case, 6 units of rain water (`-` section) are being trapped.
    }
    #[test]
    fn h06_4_2_0_3_2_5() {
        let h = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(Solution::trap(h), 9);
    }

    #[rustfmt::skip] #[test] fn h01_1() { assert_eq!(Solution::trap(vec![1]), 0); }
    #[rustfmt::skip] #[test] fn h02_1_0() { assert_eq!(Solution::trap(vec![1, 0]), 0); }

    #[rustfmt::skip] #[test] fn h03_0_0_0() { assert_eq!(Solution::trap(vec![0, 0, 0]), 0); }
    #[rustfmt::skip] #[test] fn h03_0_0_1() { assert_eq!(Solution::trap(vec![0, 0, 1]), 0); }
    #[rustfmt::skip] #[test] fn h03_0_1_0() { assert_eq!(Solution::trap(vec![0, 1, 0]), 0); }
    #[rustfmt::skip] #[test] fn h03_0_1_1() { assert_eq!(Solution::trap(vec![0, 1, 1]), 0); }
    #[rustfmt::skip] #[test] fn h03_1_0_0() { assert_eq!(Solution::trap(vec![1, 0, 0]), 0); }
    #[rustfmt::skip] #[test] fn h03_1_0_1() { assert_eq!(Solution::trap(vec![1, 0, 1]), 1); }
    #[rustfmt::skip] #[test] fn h03_1_1_0() { assert_eq!(Solution::trap(vec![1, 1, 0]), 0); }
    #[rustfmt::skip] #[test] fn h03_1_1_1() { assert_eq!(Solution::trap(vec![1, 1, 1]), 0); }

    #[test]
    fn h20000_1() {
        let h = vec![1; 20000];
        assert_eq!(Solution::trap(h), 0);
    }
}
