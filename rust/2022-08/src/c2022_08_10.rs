#![allow(dead_code)]
//! \#108. Convert Sorted Array to Binary Search Tree
//! =================================================
//!
//! <https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree>
//!
//! Given an integer array `nums` where the elements are sorted in __ascending order__,
//! convert _it to a __height-balanced__ binary search tree_.
//!
//! A __height-balanced__ binary tree is a binary tree in which
//! the depth of the two subtrees of every node never differs by more than one.
//!
//! ##### Constraints:
//!
//! - `1 <= nums.length <= 10_000`
//! - `-10_000 <= nums[i] <= 10_000`
//! - `nums` is sorted in a __strictly increasing__ order.
//!
//! ##### Examples
//!
//! ###### Example 1:
//!
//! ```text
//!      0         0
//!     / \       / \
//!   -3   9    -10  5
//!   /   /       \   \
//! -10  5        -3   9
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_10::*;
//! let nums = vec![-10, -3, 0, 5, 9];
//! let expected = t(0, t(-3, l(-10), N), t(9, l(5), N));
//!
//! assert_eq!(Solution::sorted_array_to_bst(nums), expected);
//! ```
//!
//! ###### Example 2:
//!
//! ```text
//!   3   1
//!  /     \
//! 1       3
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_10::*;
//! let nums = vec![1, 3];
//! let expected = t(3, l(1), N);
//!
//! assert_eq!(Solution::sorted_array_to_bst(nums), expected);
//! ```

use std::cell::RefCell;
use std::rc::Rc;

type T = Option<Rc<RefCell<TreeNode>>>;

pub struct Solution;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn wrap(t: TreeNode) -> T {
            Some(Rc::new(RefCell::new(t)))
        }

        fn rec(nums: &[i32]) -> T {
            match nums.len() {
                0 => None,
                1 => wrap(TreeNode::new(nums[0])),
                n => {
                    let mid = n / 2;
                    let left = rec(&nums[..mid]);
                    let val = nums[mid];
                    let right = rec(&nums[mid + 1..]);

                    wrap(TreeNode { val, left, right })
                }
            }
        }

        rec(&nums)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_1() {
        assert_eq!(Solution::sorted_array_to_bst(vec![1]), l(1));
    }

    /// ```text
    ///      0         0
    ///     / \       / \
    ///   -3   9    -10  5
    ///   /   /       \   \
    /// -10  5        -3   9
    /// ```
    #[test]
    fn n_m10_m3_0_5_9() {
        let nums = vec![-10, -3, 0, 5, 9];
        let expected = t(0, t(-3, l(-10), N), t(9, l(5), N));

        assert_eq!(Solution::sorted_array_to_bst(nums), expected);
    }

    /// ```text
    ///   3   1
    ///  /     \
    /// 1       3
    /// ```
    #[test]
    fn n_1_3() {
        let nums = vec![1, 3];
        let expected = t(3, l(1), N);

        assert_eq!(Solution::sorted_array_to_bst(nums), expected);
    }

    /// ```text
    ///   2
    ///  / \
    /// 1   3
    /// ```
    #[test]
    fn n_1_2_3() {
        let nums = vec![1, 2, 3];
        let expected = t(2, l(1), l(3));

        assert_eq!(Solution::sorted_array_to_bst(nums), expected);
    }

    /// ```text
    ///     3       3       2         2
    ///    / \     / \     / \       / \
    ///   2   4   1   4   1   3     1   4
    ///  /         \           \       /
    /// 1           2           4     3
    /// ```
    #[test]
    fn n_1_2_3_4() {
        let nums = vec![1, 2, 3, 4];
        let expected = t(3, t(2, l(1), N), l(4));

        assert_eq!(Solution::sorted_array_to_bst(nums), expected);
    }

    /// ```text
    ///     3        4
    ///    / \      / \
    ///   2   5    2   5
    ///  /   /    / \
    /// 1   4    1   3
    /// ```
    #[test]
    fn n_1_2_3_4_5() {
        let nums = vec![1, 2, 3, 4, 5];
        let expected = t(3, t(2, l(1), N), t(5, l(4), N));

        assert_eq!(Solution::sorted_array_to_bst(nums), expected);
    }

    /// ```text
    ///      4
    ///    /   \
    ///   2     6
    ///  / \   /
    /// 1   3 5
    /// ```
    #[test]
    fn n_1_2_3_4_5_6() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let expected = t(4, t(2, l(1), l(3)), t(6, l(5), N));

        assert_eq!(Solution::sorted_array_to_bst(nums), expected);
    }

    /// ```text
    ///      4
    ///    /   \
    ///   2     6
    ///  / \   / \
    /// 1   3 5   7
    /// ```
    #[test]
    fn n_1_2_3_4_5_6_7() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7];
        let expected = t(4, t(2, l(1), l(3)), t(6, l(5), l(7)));

        assert_eq!(Solution::sorted_array_to_bst(nums), expected);
    }

    #[test]
    fn n_1_to_10000() {
        let nums = (1..=10000).collect();

        assert_eq!(Solution::sorted_array_to_bst(nums).unwrap().borrow().val, 5001);
    }
}
