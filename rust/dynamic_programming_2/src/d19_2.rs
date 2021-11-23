#![allow(dead_code)]
/// 518. Coin Change 2
/// ==================
///
/// You are given an integer array `coins` representing coins of different denominations
/// and an integer `amount` representing a total amount of money.
///
/// Return _the number of combinations that make up that amount_.
/// If that amount of money cannot be made up by any combination of the coins, return `0`.
///
/// You may assume that you have an infinite number of each kind of coin.
///
/// The answer is __guaranteed__ to fit into a signed __32-bit__ integer.
///
/// __Constraints:__
///
/// - `1 <= coins.length <= 300`
/// - `1 <= coins[i] <= 5000`
/// - All the values of `coins` are __unique__.
/// - `0 <= amount <= 5000`
///
/// https://leetcode.com/problems/coin-change-2/
struct Solution;
impl Solution {
    /// 00:20-00:26
    pub fn change_rec(amount: i32, coins: Vec<i32>) -> i32 {
        println!("change({}, {:?})", amount, coins);
        fn rec(a: i32, i: usize, c: &[i32]) -> i32 {
            if a == 0 {
                1
            } else if a < 0 || i == c.len() {
                0
            } else {
                rec(a, i + 1, c) + rec(a - c[i], i, c)
            }
        }
        rec(amount, 0, &coins)
    }
    /// 00:26-00:30
    pub fn change_rec_with_memo(amount: i32, coins: Vec<i32>) -> i32 {
        println!("change({}, {:?})", amount, coins);
        fn rec(a: i32, i: usize, c: &[i32], memo: &mut Vec<Vec<i32>>) -> i32 {
            if a == 0 {
                1
            } else if a < 0 || i == c.len() {
                0
            } else if memo[a as usize][i] != -1 {
                memo[a as usize][i]
            } else {
                memo[a as usize][i] = rec(a, i + 1, c, memo) + rec(a - c[i], i, c, memo);
                memo[a as usize][i]
            }
        }
        let n = coins.len();
        let mut memo = vec![vec![-1; n]; amount as usize + 1];
        rec(amount, 0, &coins, &mut memo)
    }
    /// 00:30-00:38 WRONG!!!
    pub fn change_dp_vec_vec_wrong(amount: i32, coins: Vec<i32>) -> i32 {
        println!("change({}, {:?})", amount, coins);
        let n = coins.len();
        let t = amount as usize;
        let mut dp = vec![vec![0; n + 1]; t + 1];
        for c in 0..n {
            dp[0][c] = 1;
        }
        for a in 1..=t {
            for (i, c) in coins.iter().map(|&c| c as usize).enumerate().rev() {
                if c <= a {
                    dp[a][i] = dp[a][i + 1] + dp[a - c][i];
                }
            }
        }
        dp[t][0]
    }
    /// Failed to get it again; looked at the other submissions
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        println!("change({}, {:?})", amount, coins);
        let t = amount as usize;
        let mut dp = vec![0; t + 1];
        dp[0] = 1;
        for c in coins.iter().map(|&c| c as usize) {
            for a in c..=t {
                dp[a] += dp[a - c];
            }
        }
        dp[t]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a5_c125() {
        let a = 5;
        let c = vec![1, 2, 5];
        assert_eq!(Solution::change(a, c), 4);
        // Explanation: there are four ways to make up the amount:
        // 5=5
        // 5=2+2+1
        // 5=2+1+1+1
        // 5=1+1+1+1+1
    }
    #[test]
    fn a3_c2() {
        let a = 3;
        let c = vec![2];
        assert_eq!(Solution::change(a, c), 0);
        // Explanation: the amount of 3 cannot be made up just with coins of 2.
    }
    #[test]
    fn a10_c10() {
        let a = 10;
        let c = vec![10];
        assert_eq!(Solution::change(a, c), 1);
    }

    #[test]
    fn a100_c99_1() {
        let a = 100;
        let c = vec![99, 1];
        assert_eq!(Solution::change(a, c), 2);
    }

    #[test]
    fn a5000_c4701to5000() {
        let a = 5000;
        let c = (4701..=5000).collect();
        assert_eq!(Solution::change(a, c), 1);
    }
    #[test]
    fn a5000_c1to300() {
        let a = 120;
        let c = (1..=300).collect();
        assert_eq!(Solution::change(a, c), 1_844_349_560);
    }
}
