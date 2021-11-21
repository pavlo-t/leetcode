#![allow(dead_code)]
/// 650. 2 Keys Keyboard
/// ====================
///
/// There is only one character `'A'` on the screen of a notepad.
/// You can perform two operations on this notepad for each step:
///
/// - Copy All: You can copy all the characters present on the screen (a partial copy is not allowed).
/// - Paste: You can paste the characters which are copied last time.
///
/// Given an integer `n`,
/// return _the minimum number of operations to get the character `'A'` exactly n times on the screen_.
///
/// __Constraints:__
///
/// - `1 <= n <= 1000`
///
/// https://leetcode.com/problems/2-keys-keyboard/
struct Solution;
impl Solution {
    /// 19:16-19:29
    pub fn min_steps(n: i32) -> i32 {
        println!("min_steps({})", n);
        fn rec(l: usize, c: usize, n: usize) -> i32 {
            if l == n {
                0
            } else if l > n {
                i32::MAX
            } else if l == c {
                rec(l + c, c, n).saturating_add(1)
            } else {
                rec(l + c, c, n).min(rec(l, l, n)).saturating_add(1)
            }
        }
        if n == 1 {
            0
        } else {
            1 + rec(1, 1, n as usize)
        }
    }
    /// 19:29-19:34
    pub fn min_steps_rec_with_memo(n: i32) -> i32 {
        println!("min_steps({})", n);
        fn rec(l: usize, c: usize, n: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if l == n {
                0
            } else if l + c > n {
                i32::MAX
            } else if memo[l][c] != -1 {
                memo[l][c]
            } else {
                memo[l][c] = if l == c {
                    rec(l + l, l, n, memo)
                } else {
                    rec(l + c, c, n, memo).min(rec(l, l, n, memo))
                }
                .saturating_add(1);
                memo[l][c]
            }
        }
        if n == 1 {
            0
        } else {
            let n = n as usize;
            let mut memo = vec![vec![-1; n]; n + 1];
            1 + rec(1, 1, n, &mut memo)
        }
    }
    /// 19:34-20:00
    pub fn min_steps_dp(n: i32) -> i32 {
        println!("min_steps({})", n);
        if n == 1 {
            0
        } else {
            let n = n as usize;
            let mut dp: Vec<Vec<i32>> = vec![vec![i32::MAX; n + 1]; n + 1];
            for c in 0..=n {
                dp[n][c] = 0;
            }
            for l in (0..=n).rev() {
                if l + l <= n {
                    dp[l][l] = dp[l + l][l].saturating_add(1);
                }
                for c in 1..=n {
                    if l + c <= n {
                        dp[l][c] = dp[l + c][c].min(dp[l][l]).saturating_add(1);
                    }
                }
            }
            dp[0][1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n1() { assert_eq!(Solution::min_steps(1), 0); }
    #[rustfmt::skip] #[test] fn n2() { assert_eq!(Solution::min_steps(2), 2); }
    // Explanation: Intitally, we have one character 'A'.
    // In step 1, we use Copy All operation.
    // In step 2, we use Paste operation to get 'AA'.
    // In step 3, we use Paste operation to get 'AAA'.
    #[rustfmt::skip] #[test] fn n3() { assert_eq!(Solution::min_steps(3), 3); }

    #[rustfmt::skip] #[test] fn  n100() { assert_eq!(Solution::min_steps( 100), 14); }
    #[rustfmt::skip] #[test] fn n1000() { assert_eq!(Solution::min_steps(1000), 21); }

    #[rustfmt::skip] #[test] fn n10000() { assert_eq!(Solution::min_steps(10000), 28); }

    #[rustfmt::skip] #[test] fn n2_to_1000() { for i in 2..=1000 { assert!(Solution::min_steps(i) > 0); } }
}
