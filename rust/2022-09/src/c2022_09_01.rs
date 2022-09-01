#![allow(dead_code)]
//! \#1448. Count Good Nodes in Binary Tree
//! =======================================
//!
//! <https://leetcode.com/problems/count-good-nodes-in-binary-tree>
//!
//! Given a binary tree `root`, a node `X` in the tree is named __good__ if in the path from root to `X`
//! there are no nodes with a value _greater than `X`_.
//!
//! Return the number of __good__ nodes in the binary tree.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```text
//!     3
//!    / \
//!   1   4
//!  /   / \
//! 3   1   5
//! ```
//!
//! ```
//! # use c2022_09::c2022_09_01::*;
//! # use c2022_09::tree_node::*;
//! let root = t(3, t(1, l(3), N), t(4, l(1), l(5)));
//! assert_eq!(Solution::good_nodes(root), 4);
//! ```
//!
//! __Explanation:__ Nodes `3`, `3`, `4`, `5` are __good__.
//!
//! - Root Node `(3)` is always a good node.
//! - Node `4 -> (3,4)` is the maximum value in the path starting from the root.
//! - Node `5 -> (3,4,5)` is the maximum value in the path
//! - Node `3 -> (3,1,3)` is the maximum value in the path.
//!
//! ###### Example 2
//!
//! ```text
//!     3
//!    /
//!   3
//!  / \
//! 4   2
//! ```
//!
//! ```
//! # use c2022_09::c2022_09_01::*;
//! # use c2022_09::tree_node::*;
//! let root = t(3, t(3, l(3), l(2)), N);
//! assert_eq!(Solution::good_nodes(root), 3);
//! ```
//!
//! __Explanation:__ Node `2 -> (3, 3, 2)` is not good, because `"3"` is higher than it.
//!
//! ###### Example 3
//!
//! ```
//! # use c2022_09::c2022_09_01::*;
//! # use c2022_09::tree_node::*;
//! let root = l(1);
//! assert_eq!(Solution::good_nodes(root), 1);
//! ```
//!
//! __Explanation:__ Root is considered as good.
//!
//! ##### Constraints:
//!
//! - The number of nodes in the binary tree is in the range `[1, 100_000]`.
//! - Each node's value is between `[-10_000, 10_000]`.

use crate::tree_node::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn rec(root: Option<Rc<RefCell<TreeNode>>>, max_in_path: i32) -> i32 {
            if let Some(node) = root {
                let node = node.borrow();
                let max = node.val.max(max_in_path);
                (node.val == max) as i32
                    + rec(node.left.clone(), max)
                    + rec(node.right.clone(), max)
            } else {
                0
            }
        }

        rec(root, i32::MIN)
    }
}
