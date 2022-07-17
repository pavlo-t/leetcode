#![allow(dead_code)]
//! \#629. K Inverse Pairs Array
//! ============================
//!
//! <https://leetcode.com/problems/k-inverse-pairs-array>
//!
//! For an integer array `nums`, an __inverse pair__ is a pair of integers `[i, j]`
//! where `0 <= i < j < nums.length` and `nums[i] > nums[j]`.
//!
//! Given two integers `n` and `k`, return _the number of different arrays consisting of numbers from `1` to `n`
//! such that there are exactly `k` __inverse pairs___.
//! Since the answer can be huge, return it __modulo__ `1_000_000_007`.
//!
//! __Constraints:__
//!
//! - `1 <= n <= 1000`
//! - `0 <= k <= 1000`

pub struct Solution;
impl Solution {
    /// Recursion
    pub fn k_inverse_pairs_v1(n: i32, k: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let max_pairs = n * (n - 1) / 2;

        if k > max_pairs || k < 0 {
            0
        } else if k == 0 {
            1
        } else {
            let mut result = Self::k_inverse_pairs(n - 1, k) + Self::k_inverse_pairs(n, k - 1);
            if k >= n {
                result -= Self::k_inverse_pairs(n - 1, k - n);
                if result < 0 {
                    result += MOD;
                }
            }
            result % MOD
        }
    }

    /// Recursion with `memo`
    pub fn k_inverse_pairs_v2(n: i32, k: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;

        fn rec(n: usize, k: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            let max_pairs = n * (n - 1) / 2;

            if k > max_pairs {
                0
            } else {
                if memo[n][k] == -1 {
                    memo[n][k] = if k == 0 {
                        1
                    } else {
                        let mut result = rec(n - 1, k, memo) + rec(n, k - 1, memo);
                        if k >= n {
                            result -= rec(n - 1, k - n, memo);
                            if result < 0 {
                                result += MOD;
                            }
                        }
                        result % MOD
                    };
                }
                memo[n][k]
            }
        }

        let (n, k) = (n as usize, k as usize);
        let mut memo = vec![vec![-1; k + 1]; n + 1];

        rec(n, k, &mut memo)
    }

    /// DP 2 dimensions
    pub fn k_inverse_pairs_v3(n: i32, k: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let (n, k) = (n as usize, k as usize);
        let mut dp = vec![vec![0; k + 1]; n + 1];

        for i in 1..=n {
            dp[i][0] = 1;
            for j in 1..=k.min(i * (i - 1) / 2) {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                if j >= i {
                    dp[i][j] -= dp[i - 1][j - i];
                    if dp[i][j] < 0 {
                        dp[i][j] += MOD;
                    }
                }
                dp[i][j] %= MOD;
            }
        }

        dp[n][k]
    }

    /// DP 1 dimension
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let (n, k) = (n as usize, k as usize);
        let mut curr = vec![0; k + 1];
        curr[0] = 1;
        let mut prev = curr.clone();

        for i in 1..=n {
            for j in 1..=k.min(i * (i - 1) / 2) {
                curr[j] = prev[j] + curr[j - 1];
                if j >= i {
                    curr[j] -= prev[j - i];
                    if curr[j] < 0 {
                        curr[j] += MOD;
                    }
                }
                curr[j] %= MOD;
            }
            std::mem::swap(&mut curr, &mut prev);
        }

        prev[k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1_k_0() { assert_eq!(Solution::k_inverse_pairs(1, 0), 1); }
    #[rustfmt::skip] #[test] fn n_1_k_1() { assert_eq!(Solution::k_inverse_pairs(1, 1), 0); }

    #[rustfmt::skip] #[test] fn n_2_k_0() { assert_eq!(Solution::k_inverse_pairs(2, 0), 1); } // 12
    #[rustfmt::skip] #[test] fn n_2_k_1() { assert_eq!(Solution::k_inverse_pairs(2, 1), 1); } // 21
    #[rustfmt::skip] #[test] fn n_2_k_2() { assert_eq!(Solution::k_inverse_pairs(2, 2), 0); }

    #[rustfmt::skip] #[test] fn n_3_k_0() { assert_eq!(Solution::k_inverse_pairs(3, 0), 1); } // 123
    #[rustfmt::skip] #[test] fn n_3_k_1() { assert_eq!(Solution::k_inverse_pairs(3, 1), 2); } // 213,132
    #[rustfmt::skip] #[test] fn n_3_k_2() { assert_eq!(Solution::k_inverse_pairs(3, 2), 2); } // 312,231
    #[rustfmt::skip] #[test] fn n_3_k_3() { assert_eq!(Solution::k_inverse_pairs(3, 3), 1); } // 321
    #[rustfmt::skip] #[test] fn n_3_k_4() { assert_eq!(Solution::k_inverse_pairs(3, 4), 0); }

    #[rustfmt::skip] #[test] fn n_4_k_0() { assert_eq!(Solution::k_inverse_pairs(4, 0), 1); } // 1234
    #[rustfmt::skip] #[test] fn n_4_k_1() { assert_eq!(Solution::k_inverse_pairs(4, 1), 3); } // 2134,1324,1243
    #[rustfmt::skip] #[test] fn n_4_k_2() { assert_eq!(Solution::k_inverse_pairs(4, 2), 5); } // 3124,2314,1423,1342,2143
                                                                                              // r(n-1,k) + r(n-2,
    #[rustfmt::skip] #[test] fn n_4_k_3() { assert_eq!(Solution::k_inverse_pairs(4, 3), 6); } // 1432,2341,2413,3142,3214,4123
    #[rustfmt::skip] #[test] fn n_4_k_4() { assert_eq!(Solution::k_inverse_pairs(4, 4), 5); } // 4213,4132,3241,2431,3412
    #[rustfmt::skip] #[test] fn n_4_k_5() { assert_eq!(Solution::k_inverse_pairs(4, 5), 3); } // 4312,4231,3421
    #[rustfmt::skip] #[test] fn n_4_k_6() { assert_eq!(Solution::k_inverse_pairs(4, 6), 1); } // 4321
    #[rustfmt::skip] #[test] fn n_4_k_7() { assert_eq!(Solution::k_inverse_pairs(4, 7), 0); }

    #[rustfmt::skip] #[test] fn n_1000_k_1000() { assert_eq!(Solution::k_inverse_pairs(1000, 1000), 663_677_020); }
}
