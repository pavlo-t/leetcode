#![allow(dead_code)]
//! \#814. Binary Tree Pruning
//! ==========================
//!
//! <https://leetcode.com/problems/binary-tree-pruning>
//!
//! Given the `root` of a binary tree, return _the same tree where every subtree (of the given tree)
//! not containing a `1` has been removed_.
//!
//! A subtree of a node `node` is `node` plus every node that is a descendant of `node`.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```text
//! 1        1
//!  \        \
//!   0   =>   0
//!  / \        \
//! 0   1        1
//! ```
//!
//! ```
//! # use c2022_09::tree_node::*;
//! # use c2022_09::c2022_09_06::*;
//! let root = t(1, N, t(0, l(0), l(1)));
//! let expected = t(1, N, t(0, N, l(1)));
//!
//! assert_eq!(Solution::prune_tree(root), expected);
//! ```
//!
//! ###### Example 2
//!
//! ```text
//!      1         1
//!    /   \        \
//!   0     1   =>   1
//!  / \   / \        \
//! 0   0 0   1        1
//! ```
//!
//! ```
//! # use c2022_09::tree_node::*;
//! # use c2022_09::c2022_09_06::*;
//! let root = t(1, t(0, l(0), l(0)), t(1, l(0), l(1)));
//! let expected = t(1, N, t(1, N, l(1)));
//!
//! assert_eq!(Solution::prune_tree(root), expected);
//! ```
//!
//! ###### Example 3
//!
//! ```text
//!        1              1
//!      /   \          /   \
//!     1     0   =>   1     0
//!    / \   / \      / \     \
//!   1   1 0   1    1   1     1
//!  /
//! 0
//! ```
//!
//! ```
//! # use c2022_09::tree_node::*;
//! # use c2022_09::c2022_09_06::*;
//! let root = t(1, t(1, t(1, l(0), N), l(1)), t(0, l(0), l(1)));
//! let expected = t(1, t(1, l(1), l(1)), t(0, N, l(1)));
//!
//! assert_eq!(Solution::prune_tree(root), expected);
//! ```
//!
//! ##### Constraints
//!
//! - The number of nodes in the tree is in the range `[1, 200]`.
//! - `Node.val` is either `0` or `1`.

use crate::tree_node::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let node = node.borrow();

            let val = node.val;
            let left = Self::prune_tree(node.left.clone());
            let right = Self::prune_tree(node.right.clone());

            if val == 0 && left.is_none() && right.is_none() {
                None
            } else {
                Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
            }
        } else {
            None
        }
    }
}
