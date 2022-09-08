#![allow(dead_code)]
//! \#94. Binary Tree Inorder Traversal
//! ===================================
//!
//! <https://leetcode.com/problems/binary-tree-inorder-traversal>
//!
//! Given the `root` of a binary tree, return _the inorder traversal of its nodes' values_.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```text
//! 1
//!  \
//!   2
//!  /
//! 3
//! ```
//!
//! ```
//! # use c2022_09::tree_node::*;
//! # use c2022_09::c2022_09_08::*;
//! assert_eq!(Solution::inorder_traversal(t(1, N, t(2, l(3), N))), [1, 3, 2]);
//! ```
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_09::tree_node::*;
//! # use c2022_09::c2022_09_08::*;
//! let empty: Vec<i32> = vec![];
//! assert_eq!(Solution::inorder_traversal(N), empty);
//! ```
//!
//! ###### Example 3
//!
//! ```
//! # use c2022_09::tree_node::*;
//! # use c2022_09::c2022_09_08::*;
//! assert_eq!(Solution::inorder_traversal(l(1)), [1]);
//! ```
//!
//! ##### Constraints:
//!
//! - The number of nodes in the tree is in the range `[0, 100]`.
//! - `-100 <= Node.val <= 100`
//!
//! ##### Follow up
//!
//! Recursive solution is trivial, could you do it iteratively?

use crate::tree_node::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    /// Recursive
    pub fn inorder_traversal_v1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(node) = root {
            let node = node.borrow();

            let mut result = Self::inorder_traversal(node.left.clone());
            result.push(node.val);
            result.append(&mut Self::inorder_traversal(node.right.clone()));

            result
        } else {
            vec![]
        }
    }

    pub fn inorder_traversal_v2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        enum Action {
            PushVal(i32),
            ProcessNode(Rc<RefCell<TreeNode>>),
        }
        use Action::*;

        let mut result = vec![];
        if let Some(node) = root {
            let mut stack = vec![ProcessNode(node)];
            while let Some(action) = stack.pop() {
                match action {
                    PushVal(val) => result.push(val),
                    ProcessNode(node) => {
                        let node = node.borrow();
                        if let Some(right) = node.right.clone() {
                            stack.push(ProcessNode(right));
                        }
                        stack.push(PushVal(node.val));
                        if let Some(left) = node.left.clone() {
                            stack.push(ProcessNode(left));
                        }
                    }
                }
            }
        }
        result
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];

        let mut stack = vec![];
        let mut curr = root;
        while curr.is_some() || !stack.is_empty() {
            while curr.is_some() {
                stack.push(curr.clone());
                curr = curr.unwrap().borrow().left.clone();
            }
            if let Some(Some(node)) = stack.pop() {
                let node = node.borrow();
                result.push(node.val);
                curr = node.right.clone();
            }
        }

        result
    }
}
