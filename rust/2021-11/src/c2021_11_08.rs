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
    /// 04:27-04:29
    pub fn num_trees(n: i32) -> i32 {
        println!("num_trees({})", n);
        let n = n as usize;
        let mut dp = vec![1; n + 1];
        for i in 2..=n {
            dp[i] = 0;
            let (mut l, mut r) = (0, i - 1);
            while l < i {
                dp[i] += dp[l] * dp[r];
                l += 1;
                r = r.wrapping_sub(1);
            }
        }
        dp[n]
    }
    /// 04:23-04:27
    pub fn num_trees_rec_with_memo(n: i32) -> i32 {
        println!("num_trees({})", n);
        let n = n as usize;
        let mut memo = vec![0; n + 1];
        fn rec(n: usize, memo: &mut Vec<i32>) -> i32 {
            if n < 2 {
                1
            } else if memo[n] > 0 {
                memo[n]
            } else {
                let mut result = 0;
                let (mut l, mut r) = (0, n - 1);
                while l < n {
                    result += rec(l, memo) * rec(r, memo);
                    l += 1;
                    r = r.wrapping_sub(1);
                }
                memo[n] = result;
                memo[n]
            }
        }
        rec(n, &mut memo)
    }
    /// 04:16-04:23
    pub fn num_trees_rec(n: i32) -> i32 {
        println!("num_trees({})", n);
        fn rec(n: i32) -> i32 {
            if n < 2 {
                1
            } else {
                let mut result = 0;
                let (mut l, mut r) = (0, n - 1);
                while l < n {
                    result += rec(l) * rec(r);
                    l += 1;
                    r -= 1;
                }
                result
            }
        }
        rec(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1() {assert_eq!(Solution::num_trees( 1),  1);}
    #[rustfmt::skip] #[test] fn n_2() {assert_eq!(Solution::num_trees( 2),  2);}
    #[rustfmt::skip] #[test] fn n_3() {assert_eq!(Solution::num_trees( 3),  5);}
    #[rustfmt::skip] #[test] fn n_4() {assert_eq!(Solution::num_trees( 4), 14);}

    #[rustfmt::skip] #[test] fn n19() {assert_eq!(Solution::num_trees(19), 1_767_263_190);}
}
