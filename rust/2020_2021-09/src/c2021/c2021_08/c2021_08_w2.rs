#![allow(dead_code)]
/// Paint Fence
/// ===========
///
/// You are painting a fence of `n` posts with `k` different colors.
/// You must paint the posts following these rules:
///
/// - Every post must be painted __exactly one__ color.
/// - There __cannot__ be three or more __consecutive__ posts with the same color.
///
/// Given the two integers `n` and `k`, return _the __number of ways__ you can paint the fence_.
///
/// Constraints:
///
/// - `1 <= n <= 50`
/// - `1 <= k <= 100_000`
/// - The testcases are generated such that the answer is in the range `[0, 2^31 - 1]` for the given `n` and `k`.
///
/// https://leetcode.com/explore/featured/card/august-leetcoding-challenge-2021/614/week-2-august-8th-august-14th/3873/
struct Solution;
impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        if n == 1 {
            k
        } else {
            use std::mem::swap;
            let mut pp = k;
            let mut p = k * k;
            for _ in 2..n {
                pp = (pp + p) * (k - 1);
                swap(&mut p, &mut pp);
            }
            p
        }
    }
    pub fn num_ways_rec(n: i32, k: i32) -> i32 {
        if n == 1 {
            k
        } else if n == 2 {
            k * k
        } else {
            (Self::num_ways(n - 1, k) + Self::num_ways(n - 2, k)) * (k - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn n_1_k_1_produces_1()  { assert_eq!(Solution::num_ways(1, 1),  1); }
    #[test] fn n_2_k_1_produces_1()  { assert_eq!(Solution::num_ways(2, 1),  1); }
    #[test] fn n_3_k_1_produces_0()  { assert_eq!(Solution::num_ways(3, 1),  0); }

    #[test] fn n_1_k_2_produces_2()  { assert_eq!(Solution::num_ways(1, 2),  2); }
    #[test] fn n_2_k_2_produces_4()  { assert_eq!(Solution::num_ways(2, 2),  4); }
    #[test] fn n_3_k_2_produces_6()  { assert_eq!(Solution::num_ways(3, 2),  6); }
    #[test] fn n_4_k_2_produces_10() { assert_eq!(Solution::num_ways(4, 2), 10); }
    #[test] fn n_5_k_2_produces_16() { assert_eq!(Solution::num_ways(5, 2), 16); }
    #[test] fn n_6_k_2_produces_26() { assert_eq!(Solution::num_ways(6, 2), 26); }
    #[test] fn n_7_k_2_produces_42() { assert_eq!(Solution::num_ways(7, 2), 42); }

    #[test] fn n_1_k_3_produces_3()    { assert_eq!(Solution::num_ways(1, 3),    3); }
    #[test] fn n_2_k_3_produces_9()    { assert_eq!(Solution::num_ways(2, 3),    9); }
    #[test] fn n_3_k_3_produces_24()   { assert_eq!(Solution::num_ways(3, 3),   24); }
    #[test] fn n_4_k_3_produces_66()   { assert_eq!(Solution::num_ways(4, 3),   66); }
    #[test] fn n_5_k_3_produces_180()  { assert_eq!(Solution::num_ways(5, 3),  180); }
    #[test] fn n_6_k_3_produces_492()  { assert_eq!(Solution::num_ways(6, 3),  492); }
    #[test] fn n_7_k_3_produces_1344() { assert_eq!(Solution::num_ways(7, 3), 1344); }

    #[test] fn n_1_k_4_produces_4()     { assert_eq!(Solution::num_ways(1, 4),     4); }
    #[test] fn n_2_k_4_produces_16()    { assert_eq!(Solution::num_ways(2, 4),    16); }
    #[test] fn n_3_k_4_produces_60()    { assert_eq!(Solution::num_ways(3, 4),    60); }
    #[test] fn n_4_k_4_produces_228()   { assert_eq!(Solution::num_ways(4, 4),   228); }
    #[test] fn n_5_k_4_produces_864()   { assert_eq!(Solution::num_ways(5, 4),   864); }
    #[test] fn n_6_k_4_produces_3276()  { assert_eq!(Solution::num_ways(6, 4),  3276); }
    #[test] fn n_7_k_4_produces_12420() { assert_eq!(Solution::num_ways(7, 4), 12420); }
}
