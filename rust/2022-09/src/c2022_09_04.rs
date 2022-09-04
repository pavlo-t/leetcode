#![allow(dead_code)]
//! \#987. Vertical Order Traversal of a Binary Tree
//! ================================================
//!
//! <https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree>
//!
//! Given the `root` of a binary tree, calculate the __vertical order traversal__ of the binary tree.
//!
//! For each node at position `(row, col)`, its left and right children will be at positions
//! `(row + 1, col - 1)` and `(row + 1, col + 1)` respectively.
//! The root of the tree is at `(0, 0)`.
//!
//! The __vertical order traversal__ of a binary tree is a list of top-to-bottom orderings for each column
//! index starting from the leftmost column and ending on the rightmost column.
//! There may be multiple nodes in the same row and same column.
//! In such a case, sort these nodes by their values.
//!
//! Return _the __vertical order traversal__ of the binary tree_.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```text
//!          3 (0,0)
//!         / \
//! (1,-1) 9  20 (1,1)
//!          /  \
//!   (2,0) 15   7 (2,2)
//! ```
//!
//! ```
//! # use c2022_09::c2022_09_04::*;
//! # use c2022_09::tree_node::*;
//! # use c2022_09::vv;
//! let root = t(3, l(9), t(20, l(15), l(7)));
//! assert_eq!(Solution::vertical_traversal(root), vv![[9], [3, 15], [20], [7]]);
//! ```
//!
//! __Explanation:__
//!
//! - Column `-1`: Only node `9` is in this column.
//! - Column `0`: Nodes `3` and `15` are in this column in that order from top to bottom.
//! - Column `1`: Only node `20` is in this column.
//! - Column `2`: Only node `7` is in this column.
//!
//! ###### Example 2:
//!
//! ```text
//!                1 (0,0)
//!             /     \
//!   (1,-1) 2           3 (1,1)
//!         / \         / \
//! (2,-2) 4   5 (2,0) 6   7 (2,2)
//! ```
//!
//! ```
//! # use c2022_09::c2022_09_04::*;
//! # use c2022_09::tree_node::*;
//! # use c2022_09::vv;
//! let root = t(1, t(2, l(4), l(5)), t(3, l(6), l(7)));
//! assert_eq!(Solution::vertical_traversal(root), vv![[4], [2], [1, 5, 6], [3], [7]]);
//! ```
//!
//! __Explanation:__
//!
//! - Column `-2`: Only node `4` is in this column.
//! - Column `-1`: Only node `2` is in this column.
//! - Column `0`: Nodes `1, `5`, and `6` are in this column.
//!   - `1` is at the top, so it comes first.
//!   - `5` and `6` are at the same position `(2, 0)`, so we order them by their value, `5` before `6`.
//! - Column `1`: Only node `3` is in this column.
//! - Column `2`: Only node `7` is in this column.
//!
//! ###### Example 3:
//!
//! ```text
//!                1 (0,0)
//!             /     \
//!   (1,-1) 2           3 (1,1)
//!         / \         / \
//! (2,-2) 4   6 (2,0) 5   7 (2,2)
//! ```
//!
//! ```
//! # use c2022_09::c2022_09_04::*;
//! # use c2022_09::tree_node::*;
//! # use c2022_09::vv;
//! let root = t(1, t(2, l(4), l(6)), t(3, l(5), l(7)));
//! assert_eq!(Solution::vertical_traversal(root), vv![[4], [2], [1, 5, 6], [3], [7]]);
//! ```
//!
//! __Explanation:__
//! This case is the exact same as _example 2_, but with nodes `5` and `6` swapped.
//! Note that the solution remains the same since `5` and `6` are in the same location
//! and should be ordered by their values.
//!
//! ##### Constraints
//!
//! - The number of nodes in the tree is in the range `[1, 1000]`.
//! - `0 <= Node.val <= 1000`

use crate::tree_node::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn vertical_traversal_v1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;

        let mut rows = 0;
        let mut cols = 0;
        let mut left_offset = 0;
        let mut queue = VecDeque::new();
        queue.push_back((root.clone().unwrap(), (0usize, 0i32)));
        while let Some((node, (row, col))) = queue.pop_front() {
            rows = rows.max(row + 1);
            if col < 0 {
                left_offset = left_offset.max(-col as usize);
            }
            if col >= 0 {
                cols = cols.max(col as usize + 1);
            }

            let node = node.borrow();
            if let Some(node) = node.left.clone() {
                queue.push_back((node, (row + 1, col - 1)));
            }
            if let Some(node) = node.right.clone() {
                queue.push_back((node, (row + 1, col + 1)));
            }
        }
        cols += left_offset;

        let mut matrix = vec![vec![vec![]; cols]; rows];
        let mut queue = VecDeque::new();
        queue.push_back((root.clone().unwrap(), (0usize, left_offset)));
        while let Some((node, (row, col))) = queue.pop_front() {
            let node = node.borrow();
            matrix[row][col].push(node.val);
            if let Some(node) = node.left.clone() {
                queue.push_back((node, (row + 1, col - 1)));
            }
            if let Some(node) = node.right.clone() {
                queue.push_back((node, (row + 1, col + 1)));
            }
        }
        for row in 0..rows {
            for col in 0..cols {
                matrix[row][col].sort_unstable();
            }
        }

        let mut result = vec![vec![]; cols];
        for col in 0..cols {
            for row in 0..rows {
                result[col].append(&mut matrix[row][col]);
            }
        }
        result
    }

    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut data = vec![];
        let mut stack = vec![(root.unwrap(), 0, 0)];
        while let Some((node, row, col)) = stack.pop() {
            let node = node.borrow();
            data.push((col, row, node.val));
            if let Some(node) = node.left.clone() {
                stack.push((node, row + 1, col - 1));
            }
            if let Some(node) = node.right.clone() {
                stack.push((node, row + 1, col + 1));
            }
        }
        data.sort_unstable();

        let left_offset = -data[0].0;
        let mut result = vec![vec![]; (left_offset + data.last().unwrap().0) as usize + 1];
        for (col, _, val) in data {
            result[(col + left_offset) as usize].push(val);
        }

        result
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vv;

    #[test]
    fn r_3_9_20_n_n_15_7() {
        let root = t(3, l(9), t(20, l(15), l(7)));
        assert_eq!(Solution::vertical_traversal(root), vv![[9], [3, 15], [20], [7]]);
    }

    #[test]
    fn r_1_2_3_4_5_6_7() {
        let root = t(1, t(2, l(4), l(5)), t(3, l(6), l(7)));
        assert_eq!(Solution::vertical_traversal(root), vv![[4], [2], [1, 5, 6], [3], [7]]);
    }

    #[test]
    fn r_1_2_3_4_6_5_7() {
        let root = t(1, t(2, l(4), l(6)), t(3, l(5), l(7)));
        assert_eq!(Solution::vertical_traversal(root), vv![[4], [2], [1, 5, 6], [3], [7]]);
    }

    #[test]
    fn r_3_1_4_0_2_2() {
        let root = t(3, t(1, l(0), l(2)), t(4, l(2), N));
        assert_eq!(Solution::vertical_traversal(root), vv![[0], [1], [3, 2, 2], [4]]);
    }
}
