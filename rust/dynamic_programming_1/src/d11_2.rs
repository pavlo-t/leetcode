#![allow(dead_code)]
/// 96. Unique Binary Search Trees
/// ==============================
///
/// Given an integer `n`,
/// return _the number of structurally unique __BST__'s (binary search trees)
/// which has exactly `n` nodes of unique values from `1` to `n`_.
///
/// __Constraints:__
///
/// - `1 <= n <= 19`
///
/// https://leetcode.com/problems/unique-binary-search-trees/
struct Solution;
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        println!("num_trees({})", n);
        let n = n as usize;
        let mut dp = vec![1; n + 1];
        for i in 2..=n {
            dp[i] = (0..i).map(|j| dp[j] * dp[i - j - 1]).sum();
        }
        dp[n]
    }
    pub fn num_trees_rec_with_memo(n: i32) -> i32 {
        println!("num_trees({})", n);
        fn rec(n: usize, memo: &mut Vec<i32>) -> i32 {
            if n < 2 {
                1
            } else if memo[n] > 0 {
                memo[n]
            } else {
                memo[n] = (0..n).map(|i| rec(i, memo) * rec(n - i - 1, memo)).sum();
                memo[n]
            }
        }
        rec(n as usize, &mut vec![0; n as usize + 1])
    }
    pub fn num_trees_rec(n: i32) -> i32 {
        println!("num_trees({})", n);
        fn rec(n: i32) -> i32 {
            if n < 2 {
                1
            } else {
                (0..n).map(|i| rec(i) * rec(n - i - 1)).sum()
            }
        }
        rec(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_01() { assert_eq!(Solution::num_trees(1),  1); }
    #[rustfmt::skip] #[test] fn n_02() { assert_eq!(Solution::num_trees(2),  2); }
    #[rustfmt::skip] #[test] fn n_03() { assert_eq!(Solution::num_trees(3),  5); }
    /// n(0)*n(3) + n(1)*n(2) + n(2)*n(1) + n(3)*n(0)
    #[rustfmt::skip] #[test] fn n_04() { assert_eq!(Solution::num_trees(4), 14); }
    /// n(0)*n(4) + n(1)*n(3) + n(2)*n(2) + n(3)*n(1) + n(4)*n(0)
    #[rustfmt::skip] #[test] fn n_05() { assert_eq!(Solution::num_trees(5), 42); }

    #[rustfmt::skip] #[test] fn n_19() { assert_eq!(Solution::num_trees(19), 1_767_263_190); }
}
