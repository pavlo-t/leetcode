#![allow(dead_code)]
//! \#235. Lowest Common Ancestor of a Binary Search Tree
//! =====================================================
//!
//! <https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree>
//!
//! Given a binary search tree (BST), find the lowest common ancestor (LCA) node of two given nodes in the BST.
//!
//! According to the [definition of LCA on Wikipedia]: "The lowest common ancestor is defined
//! between two nodes `p` and `q` as the lowest node in `T` that has both `p` and `q` as descendants
//! (where we allow __a node to be a descendant of itself__).”
//!
//! [definition of LCA on Wikipedia]:https://en.wikipedia.org/wiki/Lowest_common_ancestor
//!
//! ##### Constraints:
//!
//! - The number of nodes in the tree is in the range `[2, 100_000]`.
//! - `-1_000_000_000 <= Node.val <= 1_000_000_000`
//! - All `Node.val` are __unique__.
//! - `p != q`
//! - `p` and `q` will exist in the BST.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```text
//!      6
//!    /   \
//!  p2    q8
//!  / \   / \
//! 0   4 7   9
//!    / \
//!   3   5
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_12::*;
//! let q = t(8, l(7), l(9));
//! let p = t(2, l(0), t(4, l(3), l(5)));
//! let root = t(6, p.clone(), q.clone());
//! assert_eq!(Solution::lowest_common_ancestor(root.clone(), p.clone(), q.clone()), root);
//! ```
//!
//! __Explanation:__ The LCA of nodes `2` and `8` is `6`.
//!
//! ###### Example 2
//!
//! ```text
//!      6
//!    /   \
//!  p2     8
//!  / \   / \
//! 0  q4 7   9
//!    / \
//!   3   5
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_12::*;
//! let q = t(4, l(3), l(5));
//! let p = t(2, l(0), q.clone());
//! let root = t(6, p.clone(), t(8, l(7), l(9)));
//! assert_eq!(Solution::lowest_common_ancestor(root.clone(), p.clone(), q.clone()), p);
//! ```
//!
//! __Explanation:__ The LCA of nodes `2` and `4` is `2`,
//! since a node can be a descendant of itself according to the LCA definition.
//!
//! ###### Example 3
//!
//! ```text
//!   p2
//!   /
//! q1
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_12::*;
//! let q = l(1);
//! let p = t(2, q.clone(), N);
//! let root = p.clone();
//! assert_eq!(Solution::lowest_common_ancestor(root.clone(), p.clone(), q.clone()), p);
//! ```

use std::cell::RefCell;
use std::rc::Rc;

type T = Option<Rc<RefCell<TreeNode>>>;

pub struct Solution;
impl Solution {
    pub fn lowest_common_ancestor(mut root: T, p: T, q: T) -> T {
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;

        loop {
            if let Some(node) = root.clone() {
                let node = node.borrow();
                let val = node.val;
                if val < p_val && val < q_val {
                    root = node.right.clone();
                } else if val > p_val && val > q_val {
                    root = node.left.clone();
                } else {
                    return root;
                }
            }
        }
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
