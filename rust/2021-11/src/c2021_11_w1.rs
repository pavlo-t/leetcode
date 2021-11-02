#![allow(dead_code)]
/// 1231. Divide Chocolate
/// ======================
///
/// You have one chocolate bar that consists of some chunks.
/// Each chunk has its own sweetness given by the array `sweetness`.
///
/// You want to share the chocolate with your `k` friends
/// so you start cutting the chocolate bar into `k + 1` pieces using `k` cuts,
/// each piece consists of some __consecutive__ chunks.
///
/// Being generous,
/// you will eat the piece with the __minimum total sweetness__ and give the other pieces to your friends.
///
/// Find the __maximum total sweetness__ of the piece you can get by cutting the chocolate bar optimally.
///
/// __Constraints:__
///
/// - `0 <= k < sweetness.length <= 10_000`
/// - `1 <= sweetness[i] <= 100_000`
///
/// https://leetcode.com/problems/divide-chocolate/
struct Solution;
impl Solution {
    /// Approach 1: Binary Search + Greedy
    /// https://leetcode.com/problems/divide-chocolate/solution/
    ///
    /// 22:03; after reading the hints: 22:08-22:49, solved on my own, with 1 bug
    pub fn maximize_sweetness(sweetness: Vec<i32>, k: i32) -> i32 {
        fn can_cut(t: i32, ss: &[i32], mut k: i32) -> bool {
            let mut curr = 0;
            for &s in ss {
                curr += s;
                if curr >= t {
                    curr = 0;
                    k -= 1;
                    if k < 0 {
                        return true;
                    }
                }
            }
            false
        }
        let find_min_sum = |(min, sum): (i32, i32), &n| (min.min(n), sum + n);

        let (min, sum) = sweetness.iter().fold((i32::MAX, 0), find_min_sum);
        let (mut l, mut r) = (min, sum / (k + 1));
        while l < r {
            let m = l + (r - l + 1) / 2;
            if can_cut(m, &sweetness, k) {
                l = m;
            } else {
                r = m - 1;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s_1_2_3_4_5_6_7_8_9_k_5() {
        let s = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(Solution::maximize_sweetness(s, 5), 6);
        // Explanation: You can divide the chocolate to [1,2,3], [4,5], [6], [7], [8], [9]
    }
    #[test]
    fn s_5_6_7_8_9_1_2_3_4_k_8() {
        let s = vec![5, 6, 7, 8, 9, 1, 2, 3, 4];
        assert_eq!(Solution::maximize_sweetness(s, 8), 1);
        // Explanation: There is only one way to cut the bar into 9 pieces.
    }
    #[test]
    fn s_1_2_2_1_2_2_1_2_2_k_2() {
        let s = vec![1, 2, 2, 1, 2, 2, 1, 2, 2];
        assert_eq!(Solution::maximize_sweetness(s, 2), 5);
        // Explanation: You can divide the chocolate to [1,2,2], [1,2,2], [1,2,2]
    }
    #[rustfmt::skip] #[test] fn s_1_k_0()     { assert_eq!(Solution::maximize_sweetness(vec![1]    , 0), 1); }
    #[rustfmt::skip] #[test] fn s_1_1_k_1()   { assert_eq!(Solution::maximize_sweetness(vec![1,1]  , 1), 1); }
    #[rustfmt::skip] #[test] fn s_1_2_3_k_1() { assert_eq!(Solution::maximize_sweetness(vec![1,2,3], 1), 3); }
    #[rustfmt::skip] #[test] fn s_1_2_3_k_2() { assert_eq!(Solution::maximize_sweetness(vec![1,2,3], 2), 1); }
    #[rustfmt::skip] #[test] fn s_1_8_9_k_1() { assert_eq!(Solution::maximize_sweetness(vec![1,8,9], 1), 9); }
    #[rustfmt::skip] #[test] fn s_1_8_9_k_2() { assert_eq!(Solution::maximize_sweetness(vec![1,8,9], 2), 1); }

    #[rustfmt::skip] #[test]
    fn s_10000x1_k_1()          { assert_eq!(Solution::maximize_sweetness(vec![1; 10000]      , 1   ), 5000); }
    #[rustfmt::skip] #[test]
    fn s_9999x1_k_1()           { assert_eq!(Solution::maximize_sweetness(vec![1;  9999]      , 1   ), 4999); }
    #[rustfmt::skip] #[test]
    fn s_10000x100_000_k_9999() { assert_eq!(Solution::maximize_sweetness(vec![100_000; 10000], 9999), 100_000); }
    #[rustfmt::skip] #[test]
    fn s_10000x100_000_k_0()    { assert_eq!(Solution::maximize_sweetness(vec![100_000; 10000], 0   ), 1_000_000_000); }

    #[rustfmt::skip] #[test]
    fn test_14() {
        let s = vec![90670, 55382, 95298, 95795, 73204, 41464, 18675, 30104, 47442, 55307];
        assert_eq!(Solution::maximize_sweetness(s, 6), 55382);
    }
}
