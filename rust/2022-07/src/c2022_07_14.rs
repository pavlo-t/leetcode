#![allow(dead_code)]
//! \#105. Construct Binary Tree from Preorder and Inorder Traversal
//! ================================================================
//!
//! <https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal>
//!
//! Given two integer arrays `preorder` and `inorder`
//! where `preorder` is the preorder traversal of a binary tree
//! and `inorder` is the inorder traversal of the same tree,
//! construct and return _the binary tree_.
//!
//! __Constraints:__
//!
//! - `1 <= preorder.length <= 3000`
//! - `inorder.length == preorder.length`
//! - `-3000 <= preorder[i], inorder[i] <= 3000`
//! - `preorder` and `inorder` consist of __unique__ values.
//! - Each value of `inorder` also appears in `preorder`.
//! - `preorder` is __guaranteed__ to be the preorder traversal of the tree.
//! - `inorder` is __guaranteed__ to be the inorder traversal of the tree.

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn wrap(t: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
            Some(Rc::new(RefCell::new(t)))
        }
        fn rec(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            match preorder.len() {
                0 => None,
                1 => wrap(TreeNode::new(preorder[0])),
                _ => {
                    let val = preorder[0];
                    let len = inorder
                        .iter()
                        .enumerate()
                        .find(|&(_, &v)| v == val)
                        .map(|(i, _)| i)
                        .unwrap();
                    let left = rec(&preorder[1..len + 1], &inorder[..len]);
                    let right = rec(&preorder[len + 1..], &inorder[len + 1..]);

                    wrap(TreeNode { val, left, right })
                }
            }
        }
        rec(&preorder, &inorder)
    }
}

type T = Option<Rc<RefCell<TreeNode>>>;

#[rustfmt::skip]
#[derive(PartialEq, Eq)]
pub struct TreeNode { pub val: i32, pub left: T, pub right: T }

#[rustfmt::skip]
impl TreeNode {
  #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
}

use std::fmt;

impl fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::collections::VecDeque;

        let mut result = String::new();
        let mut curr = VecDeque::new();
        let mut next = VecDeque::new();
        result.push_str(&self.val.to_string());
        if self.left.is_some() || self.right.is_some() {
            result.push('|');
            curr.push_back(self.left.clone());
            curr.push_back(self.right.clone());
        }
        while !curr.is_empty() {
            let mut seen_some = false;
            while let Some(opt) = curr.pop_front() {
                if let Some(node) = opt {
                    let node = node.borrow();
                    next.push_back(node.left.clone());
                    next.push_back(node.right.clone());
                    if node.left.is_some() || node.right.is_some() {
                        seen_some = true;
                    }
                    result.push_str(&node.val.to_string());
                    result.push(',');
                } else {
                    result.push_str("n,");
                }
            }
            result.pop();
            if seen_some {
                result.push('|');
                std::mem::swap(&mut curr, &mut next);
            }
        }
        f.write_str(&result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: T = None;
    #[rustfmt::skip] fn wrap(t: TreeNode) -> T { Some(Rc::new(RefCell::new(t))) }
    #[rustfmt::skip] fn l(v: i32) -> T { wrap(TreeNode::new(v)) }
    #[rustfmt::skip] fn n(v: i32, l: T, r: T) -> T { wrap(TreeNode { val: v, left: l, right: r }) }

    #[test]
    fn p_m1_i_m1() {
        let p = vec![-1];
        let i = vec![-1];
        let e = l(-1);
        assert_eq!(Solution::build_tree(p, i), e);
    }
    #[test]
    fn p_3_9_20_15_7_i_9_3_15_20_7() {
        //   3
        // 9  20
        //   15 7
        let p = vec![3, 9, 20, 15, 7];
        let i = vec![9, 3, 15, 20, 7];
        let e = n(3, l(9), n(20, l(15), l(7)));
        assert_eq!(Solution::build_tree(p, i), e);
    }

    #[test]
    fn p_4_2_1_3_6_5_7_i_1_2_3_4_5_6_7() {
        //    4
        //  2   6
        // 1 3 5 7
        let p = vec![4, 2, 1, 3, 6, 5, 7];
        let i = vec![1, 2, 3, 4, 5, 6, 7];
        let e = n(4, n(2, l(1), l(3)), n(6, l(5), l(7)));
        assert_eq!(Solution::build_tree(p, i), e);
    }
}
