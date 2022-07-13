#![allow(dead_code)]
//! \#102. Binary Tree Level Order Traversal
//! ========================================
//!
//! Given the `root` of a binary tree, return _the level order traversal of its nodes' values_.
//! (i.e., from left to right, level by level).
//!
//! __Constraints:__
//!
//! - The number of nodes in the tree is in the range `[0, 2000]`.
//! - `-1000 <= Node.val <= 1000`
//!
//! <https://leetcode.com/problems/binary-tree-level-order-traversal>

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;

        let mut result = vec![];
        let mut q = VecDeque::new();
        q.push_back(root.clone());
        loop {
            let mut level = vec![];
            for _ in 0..q.len() {
                if let Some(Some(node)) = q.pop_front() {
                    let node = node.borrow();
                    level.push(node.val);
                    q.push_back(node.left.clone());
                    q.push_back(node.right.clone());
                }
            }
            if level.is_empty() {
                return result;
            }
            result.push(level);
        }
    }
}

type T = Option<Rc<RefCell<TreeNode>>>;

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode { pub val: i32, pub left: T, pub right: T }
#[rustfmt::skip]
impl TreeNode {
    #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
    #[inline] pub fn wrap(self) -> T { Some(Rc::new(RefCell::new(self))) }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    const N: T = None;
    #[rustfmt::skip] fn l(v: i32) -> T { TreeNode::new(v).wrap() }
    #[rustfmt::skip] fn n(v: i32, l: T, r: T) -> T { TreeNode { val: v, left: l, right: r }.wrap() }

    #[test]
    fn t_3_9_20_n_n_15_7() {
        //    3
        //  9   20
        //     15 7
        let r = n(3, l(9), n(20, l(15), l(7)));
        let e = vv![[3], [9, 20], [15, 7]];
        assert_eq!(Solution::level_order(r), e);
    }
    #[test]
    fn t_1() {
        assert_eq!(Solution::level_order(l(1)), [[1]]);
    }
    #[test]
    fn t_n() {
        let e: [[i32; 0]; 0] = [];
        assert_eq!(Solution::level_order(None), e);
    }
}
