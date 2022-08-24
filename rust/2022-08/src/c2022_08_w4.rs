#![allow(dead_code)]
//! \#549. Binary Tree Longest Consecutive Sequence II
//! ==================================================
//!
//! <https://leetcode.com/problems/binary-tree-longest-consecutive-sequence-ii>
//!
//! Given the `root` of a binary tree, return _the length of the longest consecutive path in the tree_.
//!
//! A consecutive path is a path where the values of the consecutive nodes in the path differ by one.
//! This path can be either increasing or decreasing.
//!
//! For example, `[1,2,3,4]` and `[4,3,2,1]` are both considered valid,
//! but the path `[1,2,4,3]` is not valid.
//!
//! On the other hand, the path can be in the child-Parent-child order,
//! where not necessarily be parent-child order.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```text
//!   1
//!  / \
//! 2   3
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_w4::*;
//! # use c2022_08::tree_node::*;
//! let root = t(1, l(2), l(3));
//! assert_eq!(Solution::longest_consecutive(root), 2);
//! ```
//!
//! __Explanation:__ The longest consecutive path is `[1, 2]` or `[2, 1]`.
//!
//! ###### Example 2
//!
//! ```text
//!   2
//!  / \
//! 1   3
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_w4::*;
//! # use c2022_08::tree_node::*;
//! let root = t(2, l(1), l(3));
//! assert_eq!(Solution::longest_consecutive(root), 3);
//! ```
//!
//! __Explanation:__ The longest consecutive path is `[1, 2, 3]` or `[3, 2, 1]`.
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
        /// `-> (longest, Option<val>, longest_raising, longest_falling)`
        fn rec(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<i32>, i32, i32) {
            if let Some(node) = root {
                let node = node.borrow();

                let val = node.val;
                let (l_longest, l_opt, l_raising, l_falling) = rec(node.left.clone());
                let (r_longest, r_opt, r_raising, r_falling) = rec(node.right.clone());

                let is_prev = |v: &i32| v + 1 == val;
                let is_next = |v: &i32| v - 1 == val;

                let longest_raising = {
                    let l = l_opt.filter(is_prev).map(|_| l_raising).unwrap_or(0);
                    let r = r_opt.filter(is_prev).map(|_| r_raising).unwrap_or(0);
                    1 + l.max(r)
                };

                let longest_falling = {
                    let l = l_opt.filter(is_next).map(|_| l_falling).unwrap_or(0);
                    let r = r_opt.filter(is_next).map(|_| r_falling).unwrap_or(0);
                    1 + l.max(r)
                };

                let longest = match (l_opt, r_opt) {
                    (Some(l), Some(r)) if is_prev(&l) && is_next(&r) => 1 + l_raising + r_falling,
                    (Some(l), Some(r)) if is_next(&l) && is_prev(&r) => 1 + l_falling + r_raising,
                    _ => longest_raising.max(longest_falling),
                }
                .max(l_longest)
                .max(r_longest);

                (longest, Some(val), longest_raising, longest_falling)
            } else {
                (0, None, 0, 0)
            }
        }

        rec(root).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_1() {
        assert_eq!(Solution::longest_consecutive(l(1)), 1);
    }
    #[test]
    fn t_1_3() {
        assert_eq!(Solution::longest_consecutive(t(1, l(3), N)), 1);
    }
    #[test]
    fn t_1_n_3() {
        assert_eq!(Solution::longest_consecutive(t(1, N, l(3))), 1);
    }
    #[test]
    fn t_1_2() {
        assert_eq!(Solution::longest_consecutive(t(1, l(2), N)), 2);
    }
    #[test]
    fn t_1_n_2() {
        assert_eq!(Solution::longest_consecutive(t(1, N, l(2))), 2);
    }

    #[test]
    fn t_1_2_3() {
        let root = t(1, l(2), l(3));
        assert_eq!(Solution::longest_consecutive(root), 2);
    }

    #[test]
    fn t_2_1_3() {
        let root = t(2, l(1), l(3));
        assert_eq!(Solution::longest_consecutive(root), 3);
    }

    /// ```text
    ///        3
    ///      /   \
    ///     2     4
    ///    / \   / \
    ///   1   3 3   5
    /// ```
    #[test]
    fn t_3_2_4_1_3_3_5() {
        let root = t(3, t(2, l(1), l(3)), t(4, l(3), l(5)));
        assert_eq!(Solution::longest_consecutive(root), 5);
    }

    /// ```text
    ///        4
    ///      /   \
    ///     2     4
    ///    / \   / \
    ///   1   3 3   5
    /// ```
    #[test]
    fn t_4_2_4_1_3_3_5() {
        let root = t(4, t(2, l(1), l(3)), t(4, l(3), l(5)));
        assert_eq!(Solution::longest_consecutive(root), 3);
    }

    /// Got stack overflow with default stack
    #[test]
    fn t_1_to_30000_in_left_branch() {
        let child = std::thread::Builder::new()
            .stack_size(2usize.pow(31))
            .spawn(|| {
                let root = {
                    let mut root = N;
                    for i in 1..=30_000 {
                        root = t(i, root, N);
                    }
                    root
                };
                assert_eq!(Solution::longest_consecutive(root), 30_000);
            })
            .unwrap();
        child.join().unwrap();
    }
}
