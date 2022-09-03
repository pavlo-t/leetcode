#![allow(dead_code)]
//! \#298. Binary Tree Longest Consecutive Sequence
//! ===============================================
//!
//! <https://leetcode.com/problems/binary-tree-longest-consecutive-sequence>
//!
//! Given the `root` of a binary tree, return _the length of the longest __consecutive sequence path___.
//!
//! A __consecutive sequence path__ is a path where the values __increase by one__ along the path.
//!
//! Note that the path can start __at any node__ in the tree, and you cannot go from a node to its parent in the path.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```text
//! 1
//!  \
//!   3 start
//!  / \
//! 2   4
//!      \
//!       5 end
//! ```
//!
//! ```
//! # use c2022_09::tree_node::*;
//! # use c2022_09::c2022_09_w1::*;
//! let root = t(1, N, t(3, l(2), t(4, N, l(5))));
//! assert_eq!(Solution::longest_consecutive(root), 3);
//! ```
//!
//! __Explanation:__ Longest consecutive sequence path is `3-4-5`, so return `3`.
//!
//! ###### Example 2
//!
//! ```text
//!   2 start
//!    \
//!     3 end
//!    /
//!   2
//!  /
//! 1
//! ```
//!
//! ```
//! # use c2022_09::tree_node::*;
//! # use c2022_09::c2022_09_w1::*;
//! let root = t(2, N, t(3, t(2, l(1), N), N));
//! assert_eq!(Solution::longest_consecutive(root), 2);
//! ```
//!
//! __Explanation:__ Longest consecutive sequence path is `2-3`, not `3-2-1`, so return `2`.
//!
//! ##### Constraints
//!
//! - The number of nodes in the tree is in the range `[1, 30_000]`.
//! - `-30_000 <= Node.val <= 30_000`

use crate::tree_node::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn rec(root: Option<Rc<RefCell<TreeNode>>>, prev: i32, len: i32) -> i32 {
            if let Some(node) = root {
                let node = node.borrow();
                if node.val - 1 == prev {
                    let len_n = len + 1;
                    let len_l = rec(node.left.clone(), node.val, len_n);
                    let len_r = rec(node.right.clone(), node.val, len_n);

                    len_n.max(len_l).max(len_r)
                } else {
                    let len_l = rec(node.left.clone(), node.val, 1);
                    let len_r = rec(node.right.clone(), node.val, 1);

                    len.max(len_l).max(len_r)
                }
            } else {
                len
            }
        }

        rec(root, i32::MAX, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r_empty() {
        assert_eq!(Solution::longest_consecutive(N), 0);
    }
    #[test]
    fn r_1() {
        assert_eq!(Solution::longest_consecutive(l(1)), 1);
    }

    #[test]
    fn r_1_n_3_2_4_n_n_n_5() {
        let root = t(1, N, t(3, l(2), t(4, N, l(5))));
        assert_eq!(Solution::longest_consecutive(root), 3);
    }

    #[test]
    fn r_2_n_3_2_n_1() {
        let root = t(2, N, t(3, t(2, l(1), N), N));
        assert_eq!(Solution::longest_consecutive(root), 2);
    }
}
