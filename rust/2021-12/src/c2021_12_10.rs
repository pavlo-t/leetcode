#![allow(dead_code)]
/// 790. Domino and Tromino Tiling
/// ==============================
///
/// You have two types of tiles: a `2 x 1` domino shape and a tromino shape.
/// You may rotate these shapes.
///
/// ```text
///                                    0
/// Domino tile: 00     Tromino tile: 00
/// ```
///
/// Given an integer `n`, return _the number of ways to tile an `2 x n` board_.
/// Since the answer may be very large, return it __modulo__ `10**9 + 7`.
///
/// In a tiling, every square must be covered by a tile.
/// Two tilings are different if and only if
/// there are two 4-directionally adjacent cells on the board
/// such that exactly one of the tilings has both squares occupied by a tile.
///
/// __Constraints:__
///
/// - `1 <= n <= 1000`
///
/// https://leetcode.com/problems/domino-and-tromino-tiling/
struct Solution;
impl Solution {
    pub fn num_tilings_my_dp(n: i32) -> i32 {
        println!("num_tilings({})", n);
        const MOD: i32 = 1_000_000_007;

        let n = n as usize;
        let mut dp = vec![1; n + 1];
        for i in 2..=n {
            dp[i] = dp[i - 1] + dp[i - 2];
            dp[i] %= MOD;
            for j in 0..i - 2 {
                dp[i] += (dp[j] * 2) % MOD;
                dp[i] %= MOD;
            }
        }
        dp[n]
    }
    pub fn num_tilings_my_dp_optimized_time(n: i32) -> i32 {
        println!("num_tilings({})", n);
        const MOD: i32 = 1_000_000_007;

        let n = n as usize;
        let mut results = vec![1; n + 1];
        let mut dp = vec![2; n + 1];
        for i in 2..=n {
            results[i] = results[i - 1] + results[i - 2];
            results[i] %= MOD;
            if i > 2 {
                results[i] += dp[i - 3];
                results[i] %= MOD;
                dp[i - 2] = (dp[i - 3] + (results[i - 2] * 2) % MOD) % MOD;
            }
        }
        results[n]
    }
    pub fn num_tilings_my_dp_optimized_space(n: i32) -> i32 {
        println!("num_tilings({})", n);
        const MOD: i32 = 1_000_000_007;

        let (mut prev, mut curr, mut other) = (1, 1, 2);
        for i in 2..=n {
            let prev_prev = prev;
            std::mem::swap(&mut curr, &mut prev);
            curr = (curr + prev) % MOD;
            if i > 2 {
                curr = (curr + other) % MOD;
                other = (other + (prev_prev * 2) % MOD) % MOD;
            }
        }
        curr
    }
    /// after final optimizations have almost the same stuff as `leetcode`
    pub fn num_tilings(n: i32) -> i32 {
        println!("num_tilings({})", n);
        if n <= 2 {
            n
        } else {
            const MOD: i32 = 1_000_000_007;

            let (mut prev, mut curr, mut other) = (1, 2, 2);
            for _ in 3..=n {
                let prev_prev = prev;
                std::mem::swap(&mut curr, &mut prev);
                curr = ((curr + prev) % MOD + other) % MOD;
                other = (other + (prev_prev * 2) % MOD) % MOD;
            }
            curr
        }
    }

    /// Approach 3: Dynamic Programming (Bottom-up, space optimization)
    /// https://leetcode.com/problems/domino-and-tromino-tiling/solution/
    pub fn num_tilings_leetcode(n: i32) -> i32 {
        println!("num_tilings({})", n);
        if n <= 2 {
            n
        } else {
            const MOD: i32 = 1_000_000_007;

            let mut full_prev = 1;
            let mut full_curr = 2;
            let mut part_curr = 1;
            for _ in 3..=n {
                let tmp = full_curr;
                full_curr = ((full_curr + full_prev) % MOD + (2 * part_curr) % MOD) % MOD;
                part_curr = (part_curr + full_prev) % MOD;
                full_prev = tmp;
            }
            full_curr
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n1() { assert_eq!(Solution::num_tilings(1), 1); }
    // 11 12
    // 22 12
    #[rustfmt::skip] #[test] fn n2() { assert_eq!(Solution::num_tilings(2), 2); }
    // 122 123 113 112 122
    // 133 123 223 122 112
    #[rustfmt::skip] #[test] fn n3() { assert_eq!(Solution::num_tilings(3), 5); }
    // 1233 1224 1234 1223 1233 | 1133 1134 | 1123 1223 | 1133 1223
    // 1244 1334 1234 1233 1223 | 2244 2234 | 1223 1123 | 1223 1133
    // f(n-1) + f(n-2) + 2*f(n-3) + 2*f(n-4) = 5+2+2*1+2*1
    #[rustfmt::skip] #[test] fn n4() { assert_eq!(Solution::num_tilings(4), 11); }
    // f(n-1) + f(n-2) + 2*f(n-3) + 2*f(n-4) + 2*f(n-5) = 11+5+2*2+2*1+2*1
    #[rustfmt::skip] #[test] fn n5() { assert_eq!(Solution::num_tilings(5), 24); }
    // f(n-1) + f(n-2) + 2*f(n-3) + 2*f(n-4) + 2*f(n-5) + 2*f(n-6) = 24+11+2*5+2*2+2*1+2*1
    #[rustfmt::skip] #[test] fn n6() { assert_eq!(Solution::num_tilings(6), 53); }

    #[rustfmt::skip] #[test] fn n7() { assert_eq!(Solution::num_tilings(7), 117); }
    #[rustfmt::skip] #[test] fn n8() { assert_eq!(Solution::num_tilings(8), 258); }
    #[rustfmt::skip] #[test] fn n9() { assert_eq!(Solution::num_tilings(9), 569); }

    //#[ignore]
    #[rustfmt::skip] #[test] fn n1000() { assert_eq!(Solution::num_tilings(1000), 979_232_805); }
}
