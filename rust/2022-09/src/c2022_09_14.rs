#![allow(dead_code)]
//! \#1457. Pseudo-Palindromic Paths in a Binary Tree
//! =================================================
//!
//! <https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree>
//!
//! Given a binary tree where node values are digits from `1` to `9`.
//! A path in the binary tree is said to be __pseudo-palindromic__ if at least one permutation
//! of the node values in the path is a palindrome.
//!
//! _Return the number of __pseudo-palindromic__ paths going from the root node to leaf nodes_.
//!
//! ##### Examples
//!
//! ###### Example 1:
//!
//! ```text
//!     2
//!    / \
//!   3   1
//!  / \   \
//! 3   1   1
//! ```
//!
//! ```
//! # use c2022_09::tree_node::*;
//! # use c2022_09::c2022_09_14::*;
//! let root = t(2, t(3, l(3), l(1)), t(1, N, l(1)));
//! assert_eq!(Solution::pseudo_palindromic_paths(root), 2);
//! ```
//!
//! __Explanation:__ There are three paths going from the root node to leaf nodes: `[2,3,3]`, `[2,3,1]`, and `[2,1,1]`.
//! Among these paths only the first path and the last path are pseudo-palindromic paths,
//! since the path `[2,3,3]` can be rearranged in `[3,2,3]` (palindrome)
//! and the path `[2,1,1]` can be rearranged in `[1,2,1]` (palindrome).
//!
//! ###### Example 2:
//!
//! ```text
//!     2
//!    / \
//!   1   1
//!  / \
//! 1   3
//!      \
//!       1
//! ```
//!
//! ```
//! # use c2022_09::tree_node::*;
//! # use c2022_09::c2022_09_14::*;
//! let root = t(2, t(1, l(1), t(3, N, l(1))), l(1));
//! assert_eq!(Solution::pseudo_palindromic_paths(root), 1);
//! ```
//!
//! __Explanation:__ There are three paths going from the root node to leaf nodes: `[2,1,1]`, `[2,1,3,1]`, and `[2,1]`.
//! Among these paths only the first path is pseudo-palindromic, since `[2,1,1]` can be rearranged in `[1,2,1]`.
//!
//! ###### Example 3:
//!
//! ```
//! # use c2022_09::tree_node::*;
//! # use c2022_09::c2022_09_14::*;
//! let root = l(9);
//! assert_eq!(Solution::pseudo_palindromic_paths(root), 1);
//! ```
//!
//! ##### Constraints
//!
//! - The number of nodes in the tree is in the range `[1, 100_000]`.
//! - `1 <= Node.val <= 9`

use crate::tree_node::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn is_pseudo_palindrome(counts: &[usize; 9]) -> bool {
            counts.iter().filter(|&i| i % 2 == 1).count() <= 1
        }

        fn rec(node: Rc<RefCell<TreeNode>>, counts: &mut [usize; 9], result: &mut i32) {
            let node = node.borrow();
            let i = node.val as usize - 1;
            counts[i] += 1;

            if node.left.is_none() && node.right.is_none() {
                *result += is_pseudo_palindrome(counts) as i32
            } else {
                if let Some(left) = node.left.clone() {
                    rec(left, counts, result);
                }
                if let Some(right) = node.right.clone() {
                    rec(right, counts, result);
                }
            }

            counts[i] -= 1;
        }

        let mut result = 0;

        rec(root.unwrap(), &mut [0; 9], &mut result);

        result
    }
}
