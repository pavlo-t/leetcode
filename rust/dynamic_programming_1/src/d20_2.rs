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
    /// Approach 1: Dynamic Programming
    /// https://leetcode.com/problems/coin-change-2/solution/
    pub fn change_leetcode_dp(amount: i32, coins: Vec<i32>) -> i32 {
        println!("change({}, {:?})", amount, coins);
        let amount = amount as usize;
        let mut dp = vec![0; amount + 1];
        dp[0] = 1;
        for coin in coins.into_iter().map(|c| c as usize) {
            for a in coin..=amount {
                dp[a] += dp[a - coin];
            }
        }
        dp[amount]
    }

    /// 19:47-19:50
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        println!("change({}, {:?})", amount, coins);
        let coins = coins.into_iter().map(|c| c as usize).collect::<Vec<_>>();
        let amount = amount as usize;
        let mut dp = vec![0; amount + 1];
        dp[0] = 1;
        for i in (0..coins.len()).rev() {
            for a in coins[i]..=amount {
                dp[a] += dp[a - coins[i]];
            }
        }
        dp[amount]
    }
    /// 19:39-19:47
    pub fn change_dp(amount: i32, coins: Vec<i32>) -> i32 {
        println!("change({}, {:?})", amount, coins);
        let coins = coins.into_iter().map(|c| c as usize).collect::<Vec<_>>();
        let amount = amount as usize;
        let mut dp = vec![vec![1; amount + 1]; coins.len() + 1];
        for a in 1..=amount {
            dp[coins.len()][a] = 0;
        }
        for i in (0..coins.len()).rev() {
            for a in 1..=amount {
                dp[i][a] = if coins[i] <= a {
                    dp[i + 1][a] + dp[i][a - coins[i]]
                } else {
                    dp[i + 1][a]
                };
            }
        }
        dp[0][amount]
    }
    /// 19:35-19:39
    pub fn change_rec_2_with_memo(amount: i32, coins: Vec<i32>) -> i32 {
        println!("change({}, {:?})", amount, coins);
        fn rec(i: usize, a: usize, cs: &[usize], memo: &mut Vec<Vec<i32>>) -> i32 {
            if a == 0 {
                1
            } else if i == cs.len() {
                0
            } else if memo[i][a] >= 0 {
                memo[i][a]
            } else {
                memo[i][a] = if cs[i] <= a {
                    rec(i + 1, a, cs, memo) + rec(i, a - cs[i], cs, memo)
                } else {
                    rec(i + 1, a, cs, memo)
                };
                memo[i][a]
            }
        }
        let coins = coins.into_iter().map(|c| c as usize).collect::<Vec<_>>();
        let amount = amount as usize;
        let mut memo = vec![vec![-1; amount + 1]; coins.len()];
        rec(0, amount, &coins, &mut memo)
    }
    /// 18:41-19:35
    pub fn change_rec_2(amount: i32, coins: Vec<i32>) -> i32 {
        println!("change({}, {:?})", amount, coins);
        fn rec(i: usize, a: usize, cs: &[usize]) -> i32 {
            if a == 0 {
                1
            } else if i == cs.len() {
                0
            } else if cs[i] <= a {
                rec(i + 1, a, cs) + rec(i, a - cs[i], cs)
            } else {
                rec(i + 1, a, cs)
            }
        }
        let coins = coins.into_iter().map(|c| c as usize).collect::<Vec<_>>();
        let amount = amount as usize;
        rec(0, amount, &coins)
    }

    /// 22:02-22:30
    pub fn change_dp_1_vec_vec(amount: i32, coins: Vec<i32>) -> i32 {
        println!("change({}, {:?})", amount, coins);
        let coins = coins.into_iter().map(|c| c as usize).collect::<Vec<_>>();
        let amount = amount as usize;
        let mut dp = vec![vec![0; coins.len()]; amount + 1];
        for i in 0..coins.len() {
            dp[0][i] = 1;
        }
        for a in 1..=amount {
            for i in 0..coins.len() {
                for j in i..coins.len() {
                    if coins[j] <= a {
                        dp[a][i] += dp[a - coins[j]][j];
                    }
                }
            }
        }
        dp[amount][0]
    }
    /// 22:02-22:08
    pub fn change_rec_1_with_memo(amount: i32, coins: Vec<i32>) -> i32 {
        println!("change({}, {:?})", amount, coins);
        fn rec(a: usize, i: usize, cs: &[usize], memo: &mut Vec<Vec<i32>>) -> i32 {
            if a == 0 {
                1
            } else if i == cs.len() {
                0
            } else if memo[a][i] >= 0 {
                memo[a][i]
            } else {
                let mut result = 0;
                for i in i..cs.len() {
                    if cs[i] <= a {
                        result += rec(a - cs[i], i, cs, memo);
                    }
                }
                memo[a][i] = result;
                memo[a][i]
            }
        }
        let coins = coins.into_iter().map(|i| i as usize).collect::<Vec<_>>();
        let amount = amount as usize;
        let mut memo = vec![vec![-1; coins.len()]; amount + 1];
        rec(amount, 0, &coins, &mut memo)
    }
    /// 21:46-22:02
    pub fn change_rec_1(amount: i32, coins: Vec<i32>) -> i32 {
        println!("change({}, {:?})", amount, coins);
        fn rec(a: usize, i: usize, cs: &[usize]) -> i32 {
            if a == 0 {
                1
            } else if i == cs.len() {
                0
            } else {
                let mut result = 0;
                for i in i..cs.len() {
                    if cs[i] <= a {
                        result += rec(a - cs[i], i, cs);
                    }
                }
                result
            }
        }
        let coins = coins.into_iter().map(|i| i as usize).collect::<Vec<_>>();
        rec(amount as usize, 0, &coins)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn a_10_c_1() { assert_eq!(Solution::change(10, vec![1]), 1); }
    #[test]
    fn a_5_c_1_2_5() {
        assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
        // Explanation: there are four ways to make up the amount:
        // 5=5
        // 5=2+2+1
        // 5=2+1+1+1
        // 5=1+1+1+1+1
    }
    #[test]
    fn a_10_c_10() {
        assert_eq!(Solution::change(10, vec![10]), 1);
    }
    #[rustfmt::skip] #[test] fn a_0_c_1_2()   { assert_eq!(Solution::change(0, vec![1, 2   ]), 1); }
    #[rustfmt::skip] #[test] fn a_0_c_1_2_3() { assert_eq!(Solution::change(0, vec![1, 2, 3]), 1); }

    #[rustfmt::skip] #[test] fn a_1_c_1()     { assert_eq!(Solution::change(1, vec![1      ]), 1); }
    #[rustfmt::skip] #[test] fn a_1_c_2()     { assert_eq!(Solution::change(1, vec![2      ]), 0); }
    #[rustfmt::skip] #[test] fn a_1_c_1_2()   { assert_eq!(Solution::change(1, vec![1, 2   ]), 1); } // 1
    #[rustfmt::skip] #[test] fn a_1_c_1_2_3() { assert_eq!(Solution::change(1, vec![1, 2, 3]), 1); }

    #[rustfmt::skip] #[test] fn a_2_c_1()     { assert_eq!(Solution::change(2, vec![1      ]), 1); }
    #[rustfmt::skip] #[test] fn a_2_c_2()     { assert_eq!(Solution::change(2, vec![2      ]), 1); }
    #[rustfmt::skip] #[test] fn a_2_c_1_2()   { assert_eq!(Solution::change(2, vec![1, 2   ]), 2); } // 11 2
    #[rustfmt::skip] #[test] fn a_2_c_1_2_3() { assert_eq!(Solution::change(2, vec![1, 2, 3]), 2); } // 11 2

    #[rustfmt::skip] #[test] fn a_3_c_1()     { assert_eq!(Solution::change(3, vec![1      ]), 1); }
    #[rustfmt::skip] #[test] fn a_3_c_2()     { assert_eq!(Solution::change(3, vec![2      ]), 0); }
    #[rustfmt::skip] #[test] fn a_3_c_1_2()   { assert_eq!(Solution::change(3, vec![1, 2   ]), 2); } // 111 12
    #[rustfmt::skip] #[test] fn a_3_c_1_2_3() { assert_eq!(Solution::change(3, vec![1, 2, 3]), 3); } // 111 12 3

    #[rustfmt::skip] #[test] fn a_4_c_1()     { assert_eq!(Solution::change(4, vec![1      ]), 1); }
    #[rustfmt::skip] #[test] fn a_4_c_2()     { assert_eq!(Solution::change(4, vec![2      ]), 1); }
    #[rustfmt::skip] #[test] fn a_4_c_1_2()   { assert_eq!(Solution::change(4, vec![1, 2   ]), 3); } // 1111 112 22
    #[rustfmt::skip] #[test] fn a_4_c_1_2_3() { assert_eq!(Solution::change(4, vec![1, 2, 3]), 4); } // 1111 112 13 22

    #[rustfmt::skip] #[test] fn a_5_c_1()     { assert_eq!(Solution::change(5, vec![1      ]), 1); }
    #[rustfmt::skip] #[test] fn a_5_c_2()     { assert_eq!(Solution::change(5, vec![2      ]), 0); }
    #[rustfmt::skip] #[test] fn a_5_c_1_2()   { assert_eq!(Solution::change(5, vec![1, 2   ]), 3); } // 11111 1112 122
    #[rustfmt::skip] #[test] fn a_5_c_1_2_3() { assert_eq!(Solution::change(5, vec![1, 2, 3]), 5); } // 11111 1112 113 122 23

    #[rustfmt::skip] #[test] fn a_6_c_1()     { assert_eq!(Solution::change(6, vec![1      ]), 1); }
    #[rustfmt::skip] #[test] fn a_6_c_2()     { assert_eq!(Solution::change(6, vec![2      ]), 1); }
    #[rustfmt::skip] #[test] fn a_6_c_1_2()   { assert_eq!(Solution::change(6, vec![1, 2   ]), 4); } // 111111 11112 1122 222
    #[rustfmt::skip] #[test] fn a_6_c_1_2_3() { assert_eq!(Solution::change(6, vec![1, 2, 3]), 7); }
    // c a 0 1 2 3 4 5 6
    // 1 | 1 1 1 1 1 1 1
    // 2 | 1 1 2 2 3 3 4
    // 3 | 1 1 2 3 4 5 7
    //
    // dp[ci][amount] = dp[ci - 1][amount] + if coins[ci] <= amount { dp[ci][amount - coins[ci] } else { 0 }
    // rec(i + 1, amount) + if cs[i] <= amount { rec(i, amount - cs[i]) } else { 0 }

    #[test]
    fn a_5000_c_1() {
        assert_eq!(Solution::change(5000, vec![1]), 1);
    }
    //#[ignore]
    #[test]
    fn a_5000_c_4701to5000() {
        assert_eq!(Solution::change(5000, (4701..=5000).collect()), 1);
    }
    //#[ignore]
    #[test]
    fn a_5000_c_1to4() {
        assert_eq!(Solution::change(5000, (1..=4).collect()), 870_662_223);
    }
}
