#![allow(dead_code)]
//! \#637. Average of Levels in Binary Tree
//! =======================================
//!
//! <https://leetcode.com/problems/average-of-levels-in-binary-tree>
//!
//! Given the `root` of a binary tree, return _the average value of the nodes on each level in the form of an array_.
//! Answers within `0.00001` of the actual answer will be accepted.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```text
//!   3
//!  / \
//! 9  20
//!   /  \
//!  15   7
//! ```
//!
//! ```
//! # use c2022_09::c2022_09_02::*;
//! # use c2022_09::tree_node::*;
//! let root = t(3, l(9), t(20, l(15), l(7)));
//! assert_eq!(Solution::average_of_levels(root), [3.0, 14.5, 11.0]);
//! ```
//!
//! __Explanation:__ The average value of nodes on level `0` is `3`, on level `1` is `14.5`, and on level `2` is `11`.
//! Hence return `[3, 14.5, 11]`.
//!
//! ###### Example 2
//!
//! ```text
//!     3
//!    / \
//!   9  20
//!  / \
//! 15  7
//! ```
//!
//! ```
//! # use c2022_09::c2022_09_02::*;
//! # use c2022_09::tree_node::*;
//! let root = t(3, t(9, l(15), l(7)), l(20));
//! assert_eq!(Solution::average_of_levels(root), [3.0, 14.5, 11.0]);
//! ```
//!
//! ##### Constraints
//!
//! - The number of nodes in the tree is in the range `[1, 10_000]`.
//! - `-2**31 <= Node.val <= 2**31 - 1`

use crate::tree_node::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        println!("average_of_levels({root:?})");
        let mut curr = vec![root];
        let mut result = vec![];
        while !curr.is_empty() {
            let mut next = vec![];
            let mut sum = 0;
            let mut count = 0;
            while let Some(Some(node)) = curr.pop() {
                let node = node.borrow();
                count += 1;
                sum += node.val as i64;
                if node.left.is_some() {
                    next.push(node.left.clone());
                }
                if node.right.is_some() {
                    next.push(node.right.clone());
                }
            }
            if count > 0 {
                result.push(sum as f64 / count as f64);
            }
            curr = next;
        }
        result
    }
}
