#![allow(dead_code)]

/// # Coin Change
///
/// You are given coins of different denominations and a total amount of money _amount_.
/// Write a function to compute the fewest number of coins that you need to make up that amount.
/// If that amount of money cannot be made up by any combination of the coins, return `-1`.
///
/// You may assume that you have an infinite number of each kind of coin.
///
/// __Constraints:__
///
/// - `1 <= coins.length <= 12`
/// - `1 <= coins[i] <= 2^31 - 1`
/// - `0 <= amount <= 10_000`
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/589/week-2-march-8th-march-14th/3668/
struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![-1; amount as usize + 1];
        dp[0] = 0;
        for i in 1..=amount {
            for &c in &coins {
                if c <= i {
                    let j = (i - c) as usize;
                    let i = i as usize;
                    if dp[j] != -1 {
                        if dp[i] == -1 {
                            dp[i] = dp[j] + 1;
                        } else {
                            dp[i] = dp[i].min(dp[j] + 1)
                        }
                    }
                }
            }
        }
        dp[amount as usize]
    }

    pub fn coin_change_brute_force(coins: Vec<i32>, amount: i32) -> i32 {
        let mut coins = coins;
        coins.sort_unstable_by(|a, b| b.cmp(a));
        use std::collections::VecDeque;
        let mut q = VecDeque::new();
        q.push_back((0, 0));

        while let Some((ca, steps)) = q.pop_front() {
            if ca == amount {
                return steps;
            } else if ca < amount {
                for c in &coins {
                    q.push_back((ca + c, steps + 1));
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_c125a11_produces_3() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
        // Explanation: 11 = 5 + 5 + 1
    }
    #[test]
    fn example2_c2a3_produces_m1() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }
    #[test]
    fn example3_c1a0_produces_0() {
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
    }
    #[test]
    fn example4_c1a1_produces_1() {
        assert_eq!(Solution::coin_change(vec![1], 1), 1);
    }
    #[test]
    fn example5_c1a2_produces_2() {
        assert_eq!(Solution::coin_change(vec![1], 2), 2);
    }

    #[test]
    fn c1a10000_produces_10000() {
        assert_eq!(Solution::coin_change(vec![1], 10000), 10000);
    }
    #[test]
    fn c1to12a10000_produces_834() {
        let coins = (1..=12).collect();
        assert_eq!(Solution::coin_change(coins, 10000), 834);
    }
}
