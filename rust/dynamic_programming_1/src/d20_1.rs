#![allow(dead_code)]
/// 322. Coin Change
/// ================
///
/// You are given an integer array `coins` representing coins of different denominations
/// and an integer `amount` representing a total amount of money.
///
/// Return _the fewest number of coins that you need to make up that amount_.
/// If that amount of money cannot be made up by any combination of the coins, return `-1`.
///
/// You may assume that you have an infinite number of each kind of coin.
///
/// __Constraints:__
///
/// - `1 <= coins.length <= 12`
/// - `1 <= coins[i] <= 2**31 - 1`
/// - `0 <= amount <= 10_000`
///
/// https://leetcode.com/problems/coin-change/
struct Solution;
impl Solution {
    /// 20:25-20:39
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        println!("coin_change({:?}, {})", coins, amount);
        use std::collections::HashSet;
        let target = amount as usize;
        let coins = coins
            .into_iter()
            .map(|coin| coin as usize)
            .filter(|&coin| coin <= target)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let mut dp = vec![i32::MAX; target + 1];
        dp[target] = 0;
        for amount in (0..target).rev() {
            for &coin in &coins {
                if amount + coin <= target {
                    dp[amount] = dp[amount].min(dp[amount + coin].saturating_add(1));
                }
            }
        }
        match dp[0] {
            i32::MAX => -1,
            result => result,
        }
    }
    /// 20:11-20:25
    pub fn coin_change_rec_1_with_memo(coins: Vec<i32>, amount: i32) -> i32 {
        println!("coin_change({:?}, {})", coins, amount);
        fn rec(a: usize, cs: &[usize], memo: &mut Vec<i32>) -> i32 {
            if a == 0 {
                0
            } else if memo[a] >= 0 {
                memo[a]
            } else {
                memo[a] = cs
                    .iter()
                    .filter_map(|&c| a.checked_sub(c))
                    .map(|na| rec(na, cs, memo))
                    .filter(|&r| r >= 0)
                    .min()
                    .map(|r| r + 1)
                    .unwrap_or(-1);
                memo[a]
            }
        }
        let a = amount as usize;
        let cs = coins
            .into_iter()
            .map(|c| c as usize)
            .filter(|&c| c <= a)
            .collect::<Vec<_>>();
        let mut memo = vec![-1; a + 1];
        rec(a, &cs, &mut memo)
    }
    /// 19:57-20:11
    pub fn coin_change_rec_1(coins: Vec<i32>, amount: i32) -> i32 {
        println!("coin_change({:?}, {})", coins, amount);
        fn rec(a: i32, cs: &[i32]) -> i32 {
            if a == 0 {
                0
            } else {
                cs.iter()
                    .map(|&c| a - c)
                    .filter(|&na| na >= 0)
                    .map(|na| rec(na, cs))
                    .filter(|&r| r >= 0)
                    .min()
                    .map(|r| r + 1)
                    .unwrap_or(-1)
            }
        }
        rec(amount, &coins)
    }
    pub fn coin_change_bfs_2(coins: Vec<i32>, amount: i32) -> i32 {
        println!("coin_change({:?}, {})", coins, amount);
        use std::collections::{HashSet, VecDeque};
        let mut coins = coins
            .into_iter()
            .filter(|&coin| coin <= amount)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        coins.sort_unstable_by(|a, b| b.cmp(a));
        let mut q = VecDeque::new();
        let mut seen = HashSet::new();
        q.push_back((amount, 0));
        seen.insert(amount);
        while let Some((amount, steps)) = q.pop_front() {
            if amount == 0 {
                return steps;
            } else {
                for &coin in &coins {
                    let new_amount = amount - coin;
                    if new_amount >= 0 && !seen.contains(&new_amount) {
                        q.push_back((new_amount, steps + 1));
                        seen.insert(new_amount);
                    }
                }
            }
        }
        -1
    }
    pub fn coin_change_bfs_1(coins: Vec<i32>, amount: i32) -> i32 {
        println!("coin_change({:?}, {})", coins, amount);
        use std::collections::{HashSet, VecDeque};
        let mut coins = coins
            .into_iter()
            .filter(|&coin| coin <= amount)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        coins.sort_unstable_by(|a, b| b.cmp(a));
        let mut q = VecDeque::new();
        let mut seen = HashSet::new();
        let mut steps = 0;
        q.push_back(amount);
        seen.insert(amount);
        while !q.is_empty() {
            let mut size = q.len();
            while size > 0 {
                size -= 1;
                let a = q.pop_front().unwrap();
                if a == 0 {
                    return steps;
                }
                for &c in &coins {
                    if c <= a && !seen.contains(&(a - c)) {
                        q.push_back(a - c);
                        seen.insert(a - c);
                    }
                }
            }
            steps += 1
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_1_2_5_a_11() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
        // Explanation: 11 = 5 + 5 + 1
    }
    #[rustfmt::skip] #[test] fn c_1_a_0() { assert_eq!(Solution::coin_change(vec![1], 0),  0); }
    #[rustfmt::skip] #[test] fn c_1_a_1() { assert_eq!(Solution::coin_change(vec![1], 1),  1); }
    #[rustfmt::skip] #[test] fn c_1_a_2() { assert_eq!(Solution::coin_change(vec![1], 2),  2); }
    #[rustfmt::skip] #[test] fn c_2_a_3() { assert_eq!(Solution::coin_change(vec![2], 3), -1); }

    /// If getting stack overflow:
    /// Add `RUST_MIN_STACK=67108864` to env:
    /// RUST_MIN_STACK=67108864 cargo test --lib d20_1
    #[test]
    fn c_12x1_a_10000() {
        assert_eq!(Solution::coin_change(vec![1; 12], 10000), 10000);
    }
    #[test]
    fn c_1to12_a_10000() {
        assert_eq!(Solution::coin_change((1..=12).collect(), 10000), 834);
    }
}
