#![allow(dead_code)]
/// 198. House Robber
/// =================
///
/// https://leetcode.com/problems/house-robber/
struct Solution;
impl Solution {
    /// 20:52-21:12
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut prev, mut curr) = (0, 0);
        for n in nums {
            std::mem::swap(&mut curr, &mut prev);
            curr = prev.max(curr + n);
        }
        curr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n0()  { assert_eq!(Solution::rob(vec![0  ]), 0); }
    #[rustfmt::skip] #[test] fn n1()  { assert_eq!(Solution::rob(vec![1  ]), 1); }
    #[rustfmt::skip] #[test] fn n12() { assert_eq!(Solution::rob(vec![1,2]), 2); }
    #[rustfmt::skip] #[test] fn n21() { assert_eq!(Solution::rob(vec![2,1]), 2); }
    #[test]
    fn n1231() {
        let n = vec![1, 2, 3, 1];
        assert_eq!(Solution::rob(n), 4);
        // Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
        // Total amount you can rob = 1 + 3 = 4.
    }
    #[test]
    fn n27931() {
        let n = vec![2, 7, 9, 3, 1];
        assert_eq!(Solution::rob(n), 12);
        // Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
        // Total amount you can rob = 2 + 9 + 1 = 12.
    }

    #[test]
    fn n1to100() {
        let n = (1..=100).collect();
        assert_eq!(Solution::rob(n), 2550);
    }
    #[test]
    fn n400_repeat_100() {
        let n = vec![400; 100];
        assert_eq!(Solution::rob(n), 20_000);
    }
}
