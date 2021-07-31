#![allow(dead_code)]
/// Trapping Rain Water
/// ===================
///
/// Given `n` non-negative integers representing an elevation map where the width of each bar is `1`,
/// compute how much water it can trap after raining.
///
/// __Constraints:__
///
/// - `0 <= height.length <= 30_000`
/// - `0 <= height[i] <= 100_000`
///
/// https://leetcode.com/explore/challenge/card/july-leetcoding-challenge-2021/612/week-5-july-29th-july-31st/3833/
struct Solution;
impl Solution {
    /// Approach 4: Using 2 pointers
    ///
    /// https://leetcode.com/problems/trapping-rain-water/solution/
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            0
        } else {
            let (mut l, mut r) = (0, height.len() - 1);
            let (mut ml, mut mr) = (0, 0);
            let mut result = 0;
            while l < r {
                if height[l] < height[r] {
                    if height[l] >= ml {
                        ml = height[l];
                    } else {
                        result += ml - height[l];
                    }
                    l += 1;
                } else {
                    if height[r] >= mr {
                        mr = height[r];
                    } else {
                        result += mr - height[r];
                    }
                    r -= 1;
                }
            }
            result
        }
    }

    pub fn trap_my_dp(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            0
        } else {
            let n = height.len();
            let (max_l, _) = height
                .iter()
                .fold((Vec::with_capacity(n), 0), |(mut v, m), &c| {
                    let m = m.max(c);
                    v.push(m);
                    (v, m)
                });
            let (mut max_r, _) =
                height
                    .iter()
                    .rev()
                    .fold((Vec::with_capacity(n), 0), |(mut v, m), &c| {
                        let m = m.max(c);
                        v.push(m);
                        (v, m)
                    });
            max_r.reverse();
            let mut result = 0;
            for i in 0..n {
                result += max_l[i].min(max_r[i]) - height[i];
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h_010210132121_produces_6() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(Solution::trap(height), 6);
    }
    #[test]
    fn h_420325_produces_9() {
        let height = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(Solution::trap(height), 9);
    }

    #[test]
    fn h_2_1x29998_2_produces_29998() {
        let mut height = vec![1; 30000];
        height[0] = 2;
        height[29999] = 2;
        assert_eq!(Solution::trap(height), 29998);
    }
}
