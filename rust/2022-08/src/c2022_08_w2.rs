#![allow(dead_code)]
//! \#314. Binary Tree Vertical Order Traversal
//! ===========================================
//!
//! <https://leetcode.com/problems/binary-tree-vertical-order-traversal>
//!
//! Given the `root` of a binary tree, return ___the vertical order traversal__ of its nodes' values_.
//! (i.e., from top to bottom, column by column).
//!
//! If two nodes are in the same row and column, the order should be from __left to right__.
//!
//! ##### Examples
//!
//! ###### Example 1:
//!
//! ```text
//!  | 3 |   |
//!  |/ \|   |
//! 9|   |20 |
//!  |   |/ \|
//!  | 15|   |7
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_w2::*;
//! # use c2022_08::tree_node::*;
//! # use c2022_08::vv;
//! let root = t(3, l(9), t(20, l(15), l(7)));
//! assert_eq!(Solution::vertical_order(root), vv![[9], [3, 15], [20], [7]]);
//! ```
//!
//! ###### Example 2:
//!
//! ```text
//!  |   | 3 |   |
//!  |   |/ \|   |
//!  | 9 |   | 8 |
//!  |/ \|   |/ \|
//! 4|   |0 1|   |7
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_w2::*;
//! # use c2022_08::tree_node::*;
//! # use c2022_08::vv;
//! let root = t(3, t(9, l(4), l(0)), t(8, l(1), l(7)));
//! assert_eq!(Solution::vertical_order(root), vv![[4], [9], [3, 0, 1], [8], [7]]);
//! ```
//!
//! ###### Example 3:
//!
//! ```text
//!  |   | 3 |   |
//!  |   |/ \|   |
//!  | 9 |   | 8 |
//!  |/ \|   |/ \|
//! 4|   |0 1|   |7
//!  |  /|   |\  |
//!  | 5 |   | 2 |
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_w2::*;
//! # use c2022_08::tree_node::*;
//! # use c2022_08::vv;
//! let root = t(3, t(9, l(4), t(0, l(5), N)), t(8, t(1, N, l(2)), l(7)));
//! assert_eq!(Solution::vertical_order(root), vv![[4], [9, 5], [3, 0, 1], [8, 2], [7]]);
//! ```
//!
//! ##### Constraints
//!
//! - The number of nodes in the tree is in the range `[0, 100]`.
//! - `-100 <= Node.val <= 100`

use crate::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type T = Option<Rc<RefCell<TreeNode>>>;

pub struct Solution;
impl Solution {
    pub fn vertical_order_v1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;

        let mut queue = VecDeque::from(vec![(root, 100)]);
        let mut result = vec![vec![]; 200];

        while let Some((node_opt, col)) = queue.pop_front() {
            if let Some(node) = node_opt {
                let node = node.borrow();
                result[col].push(node.val);
                queue.push_back((node.left.clone(), col - 1));
                queue.push_back((node.right.clone(), col + 1));
            }
        }

        result.into_iter().filter(|vec| !vec.is_empty()).collect()
    }

    pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;

        if root.is_none() {
            vec![]
        } else {
            let mut queue = VecDeque::from(vec![(root.unwrap(), 0)]);
            let mut col_min = 0;
            let mut col_max = 0;

            let mut vals_with_cols = vec![];

            while let Some((node, col)) = queue.pop_front() {
                col_min = col_min.min(col);
                col_max = col_max.max(col);

                let node = node.borrow();

                vals_with_cols.push((node.val, col));

                if let Some(left) = node.left.clone() {
                    queue.push_back((left, col - 1));
                }
                if let Some(right) = node.right.clone() {
                    queue.push_back((right, col + 1));
                }
            }

            vals_with_cols
                .into_iter()
                .map(|(val, col)| (val, (col - col_min) as usize))
                .fold(
                    vec![vec![]; (col_max - col_min) as usize + 1],
                    |mut result, (val, col)| {
                        result[col].push(val);
                        result
                    },
                )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_node::*;
    use crate::vv;

    const EMPTY: Vec<Vec<i32>> = vec![];

    #[rustfmt::skip] #[test] fn r_empty() { assert_eq!(Solution::vertical_order(None), EMPTY); }
    #[rustfmt::skip] #[test] fn r_1() { assert_eq!(Solution::vertical_order(l(1)), vv![[1]]); }

    #[rustfmt::skip] #[test] fn r_1_2_n() { assert_eq!(Solution::vertical_order(t(1, l(2),    N)), vv![[2], [1]]); }
    #[rustfmt::skip] #[test] fn r_1_n_2() { assert_eq!(Solution::vertical_order(t(1,    N, l(2))), vv![[1], [2]]); }
    #[rustfmt::skip] #[test] fn r_1_2_3() { assert_eq!(Solution::vertical_order(t(1, l(2), l(3))), vv![[2], [1], [3]]); }

    /// ```text
    ///  | 3 |   |
    ///  |/ \|   |
    /// 9|   |20 |
    ///  |   |/ \|
    ///  | 15|   |7
    /// ```
    ///
    #[test]
    fn r_3_9_20_n_n_15_7() {
        let root = t(3, l(9), t(20, l(15), l(7)));
        assert_eq!(Solution::vertical_order(root), vv![[9], [3, 15], [20], [7]]);
    }

    /// ```text
    ///  |   | 3 |   |
    ///  |   |/ \|   |
    ///  | 9 |   | 8 |
    ///  |/ \|   |/ \|
    /// 4|   |0 1|   |7
    /// ```
    #[test]
    fn r_3_9_8_4_0_1_7() {
        let root = t(3, t(9, l(4), l(0)), t(8, l(1), l(7)));
        let expected = vv![[4], [9], [3, 0, 1], [8], [7]];
        assert_eq!(Solution::vertical_order(root), expected);
    }

    /// ```text
    ///  |   | 3 |   |
    ///  |   |/ \|   |
    ///  | 9 |   | 8 |
    ///  |/ \|   |/ \|
    /// 4|   |0 1|   |7
    ///  |  /|   |\  |
    ///  | 5 |   | 2 |
    /// ```
    #[test]
    fn r_3_9_8_4_0_1_7_n_n_n_2_5() {
        let root = t(3, t(9, l(4), t(0, l(5), N)), t(8, t(1, N, l(2)), l(7)));
        let expected = vv![[4], [9, 5], [3, 0, 1], [8, 2], [7]];
        assert_eq!(Solution::vertical_order(root), expected);
    }

    #[test]
    fn r_1_to_100_in_left_branch() {
        let root = {
            let mut root = N;
            for val in (1..=100).rev() {
                root = t(val, root, N);
            }
            root
        };
        let expected: Vec<Vec<i32>> = (1..=100).rev().map(|v| vec![v]).collect();
        assert_eq!(Solution::vertical_order(root), expected);
    }
    #[test]
    fn r_1_to_100_in_right_branch() {
        let root = {
            let mut root = N;
            for val in (1..=100).rev() {
                root = t(val, N, root);
            }
            root
        };
        let expected: Vec<Vec<i32>> = (1..=100).map(|v| vec![v]).collect();
        assert_eq!(Solution::vertical_order(root), expected);
    }
}
