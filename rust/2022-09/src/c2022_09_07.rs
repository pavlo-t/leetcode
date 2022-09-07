#![allow(dead_code)]
//! \#606. Construct String from Binary Tree
//! ========================================
//!
//! Given the `root` of a binary tree, construct a string consisting of parenthesis
//! and integers from a binary tree with the preorder traversal way, and return it.
//!
//! Omit all the empty parenthesis pairs that do not affect the one-to-one mapping
//! relationship between the string and the original binary tree.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```text
//!     1
//!    / \
//!   2   3
//!  /
//! 4
//! ```
//!
//! ```
//! # use c2022_09::c2022_09_07::*;
//! # use c2022_09::tree_node::*;
//! let root = t(1, t(2, l(4), N), l(3));
//! assert_eq!(Solution::tree2str(root), "1(2(4))(3)");
//! ```
//!
//! __Explanation:__ Originally, it needs to be `"1(2(4)())(3()())"`,
//! but you need to omit all the unnecessary empty parenthesis pairs.
//! And it will be `"1(2(4))(3)"`
//!
//! ###### Example 2:
//!
//! ```text
//!    1
//!   / \
//!  2   3
//!   \
//!    4
//! ```
//!
//! ```
//! # use c2022_09::c2022_09_07::*;
//! # use c2022_09::tree_node::*;
//! let root = t(1, t(2, N, l(4)), l(3));
//! assert_eq!(Solution::tree2str(root), "1(2()(4))(3)");
//! ```
//!
//! __Explanation:__ Almost the same as the first example,
//! except we cannot omit the first parenthesis pair to break the one-to-one
//! mapping relationship between the input and the output.
//!
//! ##### Constraints
//!
//! - The number of nodes in the tree is in the range `[1, 10_000]`.
//! - `-1000 <= Node.val <= 1000`

use crate::tree_node::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    /// Recursion
    pub fn tree2str_v1(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = String::new();

        if let Some(node) = root {
            let node = node.borrow();
            result.push_str(&node.val.to_string());

            if node.left.is_some() || node.right.is_some() {
                result.push('(');
                result.push_str(&Self::tree2str(node.left.clone()));
                result.push(')');

                if node.right.is_some() {
                    result.push('(');
                    result.push_str(&Self::tree2str(node.right.clone()));
                    result.push(')');
                }
            }
        }

        result
    }

    /// Using a stack
    pub fn tree2str_v2(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        enum Action {
            AddNode(Rc<RefCell<TreeNode>>),
            OpenParen,
            CloseParen,
        }
        use Action::*;

        let mut result = String::new();

        let mut stack = vec![AddNode(root.unwrap())];
        while let Some(action) = stack.pop() {
            match action {
                AddNode(node) => {
                    let node = node.borrow();
                    result.push_str(&node.val.to_string());

                    if node.left.is_some() || node.right.is_some() {
                        if let Some(right) = node.right.clone() {
                            stack.push(CloseParen);
                            stack.push(AddNode(right));
                            stack.push(OpenParen);
                        }

                        stack.push(CloseParen);
                        if let Some(left) = node.left.clone() {
                            stack.push(AddNode(left));
                        }
                        stack.push(OpenParen);
                    }
                }
                OpenParen => result.push('('),
                CloseParen => result.push(')'),
            }
        }

        result
    }

    /// Recursion with a mutable string
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn rec(node: &TreeNode, result: &mut String) {
            result.push_str(&node.val.to_string());

            if node.left.is_some() || node.right.is_some() {
                result.push('(');
                if let Some(left) = &node.left {
                    rec(&left.borrow(), result);
                }
                result.push(')');

                if let Some(right) = &node.right {
                    result.push('(');
                    rec(&right.borrow(), result);
                    result.push(')');
                }
            }
        }

        let mut result = String::new();

        rec(&root.unwrap().borrow(), &mut result);

        result
    }
}
