#![allow(dead_code)]
/// 96. Unique Binary Search Trees
/// ==============================
///
/// Given an integer `n`, return _the number of structurally unique __BST__'s (binary search trees)
/// which has exactly `n` nodes of unique values from `1` to `n`_.
///
/// __Constraints:__
///
/// - `1 <= n <= 19`
///
/// https://leetcode.com/problems/unique-binary-search-trees/
struct Solution;
impl Solution {
    pub fn num_trees_rec(n: i32) -> i32 {
        println!("num_trees({})", n);
        fn rec(n: i32) -> i32 {
            if n < 2 {
                1
            } else {
                let mut result = 0;
                for r in 0..n {
                    result += rec(r) * rec(n - r - 1);
                }
                result
            }
        }
        rec(n)
    }
    pub fn num_trees_rec_with_memo(n: i32) -> i32 {
        println!("num_trees({})", n);
        fn rec(n: usize, memo: &mut Vec<i32>) -> i32 {
            if memo[n] != -1 {
                memo[n]
            } else {
                memo[n] = if n < 2 {
                    1
                } else {
                    let mut result = 0;
                    for r in 0..n {
                        result += rec(r, memo) * rec(n - r - 1, memo);
                    }
                    result
                };
                memo[n]
            }
        }
        let n = n as usize;
        let mut memo = vec![-1; n + 1];
        rec(n, &mut memo)
    }
    pub fn num_trees(n: i32) -> i32 {
        println!("num_trees({})", n);
        let n = n as usize;
        let mut dp = vec![1; n + 1];
        for i in 2..=n {
            dp[i] = 0;
            for r in 0..i {
                dp[i] += dp[r] * dp[i - r - 1];
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n1() { assert_eq!(Solution::num_trees(1),  1); }
    #[rustfmt::skip] #[test] fn n2() { assert_eq!(Solution::num_trees(2),  2); }
    #[rustfmt::skip] #[test] fn n3() { assert_eq!(Solution::num_trees(3),  5); }
    #[rustfmt::skip] #[test] fn n4() { assert_eq!(Solution::num_trees(4), 14); }
    #[rustfmt::skip] #[test] fn n5() { assert_eq!(Solution::num_trees(5), 42); }

    #[rustfmt::skip] #[test] fn n19() { assert_eq!(Solution::num_trees(19), 1_767_263_190); }
}
