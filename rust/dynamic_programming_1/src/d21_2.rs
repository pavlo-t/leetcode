#![allow(dead_code)]
/// 343. Integer Break
/// ==================
///
/// Given an integer `n`, break it into the sum of `k` __positive integers__,
/// where `k >= 2`, and maximize the product of those integers.
///
/// Return _the maximum product you can get_.
///
/// __Constraints:__
///
/// - `2 <= n <= 58`
///
/// https://leetcode.com/problems/integer-break/
struct Solution;
impl Solution {
    /// 22:57-23:15
    pub fn integer_break(n: i32) -> i32 {
        println!("integer_break({})", n);
        match n {
            2 => 1,
            3 => 2,
            n => {
                let (mut a, mut b, mut c) = (2, 3, 4);
                for _ in 4..n {
                    std::mem::swap(&mut a, &mut b);
                    std::mem::swap(&mut b, &mut c);
                    c *= 3;
                }
                c
            }
        }
    }
    /// 22:42-22:52
    pub fn integer_break_dp_vec_cubic_time(n: i32) -> i32 {
        println!("integer_break({})", n);
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        for n in 2..=n {
            dp[n - 1] = dp[n - 1].max(n as i32 - 1);
            for k in 1..n {
                for d in 1..=n - k {
                    dp[n] = dp[n].max(d as i32 * dp[n - d]);
                }
            }
        }
        println!("  dp: {:?}", dp);
        dp[n]
    }
    /// 22:33-22:42
    pub fn integer_break_dp_vec_vec_2(n: i32) -> i32 {
        println!("integer_break({})", n);
        if n == 2 {
            1
        } else {
            let n = n as usize;
            let mut dp = vec![vec![0; n + 1]; n + 1];
            for n in 2..=n {
                for k in 2..n {
                    dp[n][k] = dp[n][k - 1];
                    for d in 1..=n - k {
                        dp[n][k] = dp[n][k].max(d as i32 * dp[n - d][k - 1]);
                    }
                }
                dp[n][1] = n as i32;
            }
            println!("  dp: [");
            dp.iter().for_each(|v| println!("    {:?}", v));
            dp[n][n - 1]
        }
    }
    /// 21:47-22:33
    pub fn integer_break_dp_vec_vec_1(n: i32) -> i32 {
        println!("integer_break({})", n);
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; n + 1];
        for n in 2..=n {
            dp[n][n] = 1;
            for k in (2..n).rev() {
                dp[n][k] = dp[n][k + 1];
                for d in 1..=n - k {
                    dp[n][k] = dp[n][k].max(d as i32 * dp[n - d][k - 1]);
                }
            }
            dp[n][1] = n as i32;
        }
        println!("  dp: [");
        dp.iter().for_each(|v| println!("    {:?}", v));
        dp[n][2]
    }
    /// 21:43-21:47
    pub fn integer_break_(n: i32) -> i32 {
        println!("integer_break({})", n);
        fn rec(n: usize, k: usize, memo: &mut Vec<Vec<usize>>) -> usize {
            if k >= n {
                1
            } else if k == 1 {
                n
            } else if memo[n][k] > 0 {
                memo[n][k]
            } else {
                let mut result = 0;
                for d in 1..=n - k {
                    result = result.max(d * rec(n - d, k - 1, memo));
                }
                result = result.max(rec(n, k + 1, memo));
                memo[n][k] = result;
                memo[n][k]
            }
        }
        let n = n as usize;
        let mut memo = vec![vec![0; n + 1]; n + 1];
        let result = rec(n, 2, &mut memo) as i32;
        println!("  dp: [");
        memo.iter().for_each(|v| println!("    {:?}", v));
        result + 1
    }
    pub fn integer_break_rec(n: i32) -> i32 {
        println!("integer_break({})", n);
        fn rec(n: i32, k: i32) -> i32 {
            if k >= n {
                1
            } else if k == 1 {
                n
            } else {
                let mut result = 0;
                for d in 1..=n - k {
                    result = result.max(d * rec(n - d, k - 1));
                }
                result = result.max(rec(n, k + 1));
                result
            }
        }
        rec(n, 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn  n2() {assert_eq!(Solution::integer_break( 2),  1);} // 1x1
    #[rustfmt::skip] #[test] fn  n3() {assert_eq!(Solution::integer_break( 3),  2);} // 1x2
    #[rustfmt::skip] #[test] fn  n4() {assert_eq!(Solution::integer_break( 4),  4);} // 2x2
    #[rustfmt::skip] #[test] fn  n5() {assert_eq!(Solution::integer_break( 5),  6);} // 2x3
    #[rustfmt::skip] #[test] fn  n6() {assert_eq!(Solution::integer_break( 6),  9);} // 3x3
    #[rustfmt::skip] #[test] fn  n7() {assert_eq!(Solution::integer_break( 7), 12);} // 3x4        2x 6 = 3x 4 > 4x 2
    #[rustfmt::skip] #[test] fn  n8() {assert_eq!(Solution::integer_break( 8), 18);} // 2x3x3      2x 9 = 3x 6 > 4x 4
    #[rustfmt::skip] #[test] fn  n9() {assert_eq!(Solution::integer_break( 9), 27);} // 3x3x3      2x12 < 3x 9 > 4x 6
    #[rustfmt::skip] #[test] fn n10() {assert_eq!(Solution::integer_break(10), 36);} // 3x3x4      2x18 = 3x12 = 4x 9
    #[rustfmt::skip] #[test] fn n11() {assert_eq!(Solution::integer_break(11), 54);} // 2x3x3x3    2x27 = 3x18 > 4x12
    #[rustfmt::skip] #[test] fn n12() {assert_eq!(Solution::integer_break(12), 81);} // 3x3x3x3    2x36 < 3x27 > 4x18
    #[rustfmt::skip] #[test] fn n13() {assert_eq!(Solution::integer_break(13),108);} // 3x3x3x4    2x54 = 3x36 = 4x27
    #[rustfmt::skip] #[test] fn n14() {assert_eq!(Solution::integer_break(14),162);} // 2x3x3x3x3  2x81 = 3x54 > 4x36

    //#[ignore]
    #[rustfmt::skip] #[test] fn n58() {assert_eq!(Solution::integer_break(58),1_549_681_956);}
}
