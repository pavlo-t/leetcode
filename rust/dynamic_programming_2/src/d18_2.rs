#![allow(dead_code)]
/// 1130. Minimum Cost Tree From Leaf Values
/// ========================================
///
/// Given an array `arr` of positive integers, consider all binary trees such that:
///
/// - Each node has either `0` or `2` children;
/// - The values of `arr` correspond to the values of each __leaf__ in an in-order traversal of the tree.
/// - The value of each non-leaf node is equal to the product
///   of the largest leaf value in its left and right subtree, respectively.
///
/// Among all possible binary trees considered,
/// return _the smallest possible sum of the values of each non-leaf node_.
/// It is guaranteed this sum fits into a __32-bit__ integer.
///
/// A node is a __leaf__ if and only if it has zero children.
///
/// __Constraints:__
///
/// - `2 <= arr.length <= 40`
/// - `1 <= arr[i] <= 15`
/// - It is guaranteed that the answer fits into a __32-bit__ signed integer (i.e., it is less than `2**31`).
///
/// https://leetcode.com/problems/minimum-cost-tree-from-leaf-values/
struct Solution;
impl Solution {
    pub fn mct_from_leaf_values_rec(arr: Vec<i32>) -> i32 {
        println!("mct_from_leaf_values({:?})", arr);
        fn rec(l: usize, r: usize, a: &[i32]) -> i32 {
            if r == l {
                0
            } else if r == l + 1 {
                a[l] * a[r]
            } else {
                let mut result = i32::MAX;
                for m in l..r {
                    let &ml = a[l..=m].iter().max().unwrap();
                    let &mr = a[m + 1..=r].iter().max().unwrap();
                    result = result.min(ml * mr + rec(l, m, a) + rec(m + 1, r, a));
                }
                result
            }
        }
        rec(0, arr.len() - 1, &arr)
    }
    pub fn mct_from_leaf_values_rec_with_memo(arr: Vec<i32>) -> i32 {
        println!("mct_from_leaf_values({:?})", arr);
        fn rec(l: usize, r: usize, a: &[i32], memo: &mut Vec<Vec<i32>>) -> i32 {
            if memo[l][r] != -1 {
                memo[l][r]
            } else {
                memo[l][r] = if r == l {
                    0
                } else if r == l + 1 {
                    a[l] * a[r]
                } else {
                    let mut result = i32::MAX;
                    for m in l..r {
                        let &ml = a[l..=m].iter().max().unwrap();
                        let &mr = a[m + 1..=r].iter().max().unwrap();
                        result = result.min(ml * mr + rec(l, m, a, memo) + rec(m + 1, r, a, memo));
                    }
                    result
                };
                memo[l][r]
            }
        }
        let n = arr.len();
        let mut memo = vec![vec![-1; n]; n];
        rec(0, n - 1, &arr, &mut memo)
    }
    pub fn mct_from_leaf_values_dp(arr: Vec<i32>) -> i32 {
        println!("mct_from_leaf_values({:?})", arr);
        let n = arr.len();
        let mut dp = vec![vec![i32::MAX; n]; n];
        for l in 0..n - 1 {
            dp[l][l] = 0;
            dp[l][l + 1] = arr[l] * arr[l + 1];
        }
        dp[n - 1][n - 1] = 0;
        for len in 3..=n {
            for l in 0..=n - len {
                let r = l + len - 1;
                for m in l..r {
                    let &ml = arr[l..=m].iter().max().unwrap();
                    let &mr = arr[m + 1..=r].iter().max().unwrap();
                    dp[l][r] = dp[l][r].min(ml * mr + dp[l][m] + dp[m + 1][r]);
                }
            }
        }
        dp[0][n - 1]
    }

    /// from other submissions <https://leetcode.com/submissions/detail/591200285/>
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        use std::cmp::min;
        let mut stack = vec![];
        let mut cost = 0;
        for n in arr {
            let left = loop {
                if let Some((top, left)) = stack.last() {
                    if *top <= n {
                        cost += min(n, *left) * top;
                        stack.pop();
                    } else {
                        break *top;
                    }
                } else {
                    break i32::MAX;
                }
            };
            stack.push((n, left));
        }
        while stack.len() > 1 {
            let (top, left) = stack.pop().unwrap();
            cost += top * left;
        }
        cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_4_11() {
        //   44
        //  /  \
        // 4   11
        let a = vec![4, 11];
        assert_eq!(Solution::mct_from_leaf_values(a), 44);
    }
    #[test]
    fn a624() {
        let a = vec![6, 2, 4];
        assert_eq!(Solution::mct_from_leaf_values(a), 32);
        //     24       24
        //    /  \     /  \
        //   12   4   6    8
        //  /  \          / \
        // 6    2        2   4
        // Explanation: There are two possible trees shown.
        // The first has a non-leaf node sum 36, and the second has non-leaf node sum 32.
    }
    #[test]
    fn a1234() {
        let a = vec![1, 2, 3, 4];
        assert_eq!(Solution::mct_from_leaf_values(a), 20);
        //  4      4        8       12    12
        // / \    / \      / \      / \   / \
        // 1 12   1 8     2  12     6 4   3 4
        //   / \   / \   / \ / \   / \   / \
        //   6 4   2 12  1 2 3 4   2 3   1 6
        //  / \     / \           / \     / \
        //  2 3     3 4           1 2     2 3
        //------------------------------------
        //  22     24       22      20    21
    }
    #[test]
    fn a12345() {
        let a = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::mct_from_leaf_values(a), 40);
    }
    #[test]
    fn a1111() {
        let a = vec![1, 1, 1, 1];
        assert_eq!(Solution::mct_from_leaf_values(a), 3);
    }
    #[test]
    fn a11111() {
        let a = vec![1, 1, 1, 1, 1];
        assert_eq!(Solution::mct_from_leaf_values(a), 4);
    }

    //#[ignore]
    #[test]
    fn a1_repeat_40() {
        let a = vec![1; 40];
        assert_eq!(Solution::mct_from_leaf_values(a), 39);
    }
}
