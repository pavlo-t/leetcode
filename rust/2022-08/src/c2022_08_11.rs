#![allow(dead_code)]
//! \#98. Validate Binary Search Tree
//! =================================
//!
//! <https://leetcode.com/problems/validate-binary-search-tree>
//!
//! Given the `root` of a binary tree, determine _if it is a valid binary search tree (BST)_.
//!
//! A __valid BST__ is defined as follows:
//!
//! - The left subtree of a node contains only nodes with keys __less than__ the node's key.
//! - The right subtree of a node contains only nodes with keys __greater than__ the node's key.
//! - Both the left and right subtrees must also be binary search trees.
//!
//! ##### Constraints
//!
//! - The number of nodes in the tree is in the range `[1, 10_000]`.
//! - `-2**31 <= Node.val <= 2**31 - 1`
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```text
//!   2
//!  / \
//! 1   3
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_11::*;
//! let root = t(2, l(1), l(3));
//! assert_eq!(Solution::is_valid_bst(root), true);
//! ```
//!
//! ###### Example 2
//!
//! ```text
//!   5
//!  / \
//! 1   4
//!    / \
//!   3   6
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_11::*;
//!
//! let root = t(5, l(1), t(4, l(3), l(6)));
//! assert_eq!(Solution::is_valid_bst(root), false);
//! ```
//!
//! __Explanation:__ The root node's value is `5` but its right child's value is `4`.

use std::cell::RefCell;
use std::rc::Rc;

type T = Option<Rc<RefCell<TreeNode>>>;

pub struct Solution;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        type MIN = i32;
        type MAX = i32;

        fn rec(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Option<(MIN, MAX)>> {
            if let Some(node) = root {
                let node = node.borrow();
                let val = node.val;

                match rec(node.left.clone()) {
                    None => None,
                    Some(Some((_, l_max))) if l_max >= val => None,
                    Some(l_opt) => match rec(node.right.clone()) {
                        None => None,
                        Some(Some((r_min, _))) if r_min <= val => None,
                        Some(r_opt) => {
                            let min = l_opt.map(|(min, _)| min).unwrap_or(val);
                            let max = r_opt.map(|(_, max)| max).unwrap_or(val);

                            Some(Some((min, max)))
                        }
                    },
                }
            } else {
                Some(None)
            }
        }

        rec(root).is_some()
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq)]
pub struct TreeNode { pub val: i32, pub left: T, pub right: T }
impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::new();
        result.push_str(&self.val.to_string());

        let mut curr = vec![];
        if self.left.is_some() || self.right.is_some() {
            curr.push(self.right.clone());
            curr.push(self.left.clone());
            result.push('|');
        }

        while !curr.is_empty() {
            let mut seen_some = false;
            let mut next = vec![];
            while let Some(opt) = curr.pop() {
                if let Some(node) = opt {
                    let node = node.borrow();
                    result.push_str(&node.val.to_string());
                    next.push(node.left.clone());
                    next.push(node.right.clone());
                    if node.left.is_some() || node.right.is_some() {
                        seen_some = true;
                    }
                } else {
                    result.push('_');
                }
                result.push(',');
            }
            result.pop();
            if seen_some {
                next.reverse();
                curr = next;
                result.push('|');
            }
        }

        f.write_str(&result)
    }
}

#[rustfmt::skip]
impl TreeNode {
  #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
}

#[rustfmt::skip] fn wrap(t: TreeNode) -> T { Some(Rc::new(RefCell::new(t))) }

/// Empty tree
pub const N: T = None;
/// Wrapped leaf node
#[rustfmt::skip] pub fn l(val: i32)                    -> T { wrap(TreeNode::new(val)) }
/// Wrapped tree node
#[rustfmt::skip] pub fn t(val: i32, left: T, right: T) -> T { wrap(TreeNode { val, left, right }) }
