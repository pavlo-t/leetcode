#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 1026. Maximum Difference Between Node and Ancestor
/// ==================================================
///
/// Given the `root` of a binary tree, find the maximum value `v`
/// for which there exist __different__ nodes `a` and `b`
/// where `v = |a.val - b.val|` and `a` is an ancestor of `b`.
///
/// A node `a` is an ancestor of `b` if either:
/// any child of `a` is equal to `b` or any child of `a` is an ancestor of `b`.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[2, 5000]`.
/// - `0 <= Node.val <= 100_000`
///
/// https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/
struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn max_ancestor_diff(root: T) -> i32 {
        println!("max_ancestor_diff({:?})", root);
        fn rec(root: &T) -> (i32, i32, i32) {
            if let Some(n) = root {
                let b = n.borrow();
                let (l_diff, l_min, l_max) = rec(&b.left);
                let (r_diff, r_min, r_max) = rec(&b.right);
                let min = b.val.min(l_min).min(r_min);
                let max = b.val.max(l_max).max(r_max);
                let diff = l_diff.max(r_diff).max(max - b.val).max(b.val - min);
                (diff, min, max)
            } else {
                (0, i32::MAX, i32::MIN)
            }
        }
        rec(&root).0
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq)]
pub struct TreeNode { pub val: i32, pub left: T, pub right: T }
impl TreeNode {
    #[rustfmt::skip] #[inline]    pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
    #[rustfmt::skip] #[inline]    pub fn wrap(self) -> T { Some(Rc::new(RefCell::new(self))) }
}
impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::collections::VecDeque;

        let mut data = vec![self.val.to_string()];
        let mut q = VecDeque::new();
        q.push_back(self.left.clone());
        q.push_back(self.right.clone());
        while let Some(n) = q.pop_front() {
            if let Some(n) = n {
                let b = n.borrow();
                data.push(b.val.to_string());
                q.push_back(b.left.clone());
                q.push_back(b.right.clone());
            } else {
                data.push("n".into());
            }
        }
        f.debug_list().entries(data.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: T = None;
    #[rustfmt::skip] fn t(v: i32, l: T, r: T) -> T { TreeNode { val: v, left: l, right: r }.wrap() }
    #[rustfmt::skip] fn l(v: i32            ) -> T { TreeNode::new(v).wrap() }

    #[test]
    fn r_0_0_0() {
        let r = t(0, l(0), l(0));
        assert_eq!(Solution::max_ancestor_diff(r), 0);
    }
    #[test]
    fn r_100000_100000_100000() {
        let r = t(100000, l(100000), l(100000));
        assert_eq!(Solution::max_ancestor_diff(r), 0);
    }
    #[test]
    fn r_1_2_n() {
        let r = t(1, l(2), N);
        assert_eq!(Solution::max_ancestor_diff(r), 1);
    }
    #[test]
    fn r_1_2_3() {
        let r = t(1, l(2), l(3));
        assert_eq!(Solution::max_ancestor_diff(r), 2);
    }
    #[test]
    fn r_8_3_10_1_6_n_14_n_n_4_7_13() {
        let r = t(8, t(3, l(1), t(6, l(4), l(7))), t(10, N, t(14, l(13), N)));
        assert_eq!(Solution::max_ancestor_diff(r), 7);
        //     8
        //    / \
        //   3   10
        //  / \    \
        // 1   6    14
        //    / \   /
        //   4   7 13
        // Explanation: We have various ancestor-node differences, some of which are given below :
        // |8 - 3| = 5
        // |3 - 7| = 4
        // |8 - 1| = 7
        // |10 - 13| = 3
        // Among all possible differences, the maximum value of 7 is obtained by |8 - 1| = 7.
    }
    #[test]
    fn r_1_n_2_n_0_3() {
        let r = t(1, N, t(2, N, t(0, l(3), N)));
        assert_eq!(Solution::max_ancestor_diff(r), 3);
    }
}
