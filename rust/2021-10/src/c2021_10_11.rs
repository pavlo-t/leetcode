#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// 543. Diameter of Binary Tree
/// ============================
///
/// Given the `root` of a binary tree, return _the length of the __diameter__ of the tree_.
///
/// The __diameter__ of a binary tree is the __length__ of the longest path between any two nodes in a tree.
/// This path may or may not pass through the `root`.
///
/// The __length__ of a path between two nodes is represented by the number of edges between them.
///
/// __Constraints:__
///
/// -The number of nodes in the tree is in the range `[1, 10_000]`.
/// - `-100 <= Node.val <= 100`
///
/// https://leetcode.com/problems/diameter-of-binary-tree/
struct Solution;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //println!("diameter_of_binary_tree({:?})", root);
        fn rec(n: Option<&Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            if let Some(n) = n {
                let n = n.borrow();
                let (l_diam, l_depth) = rec(n.left.as_ref());
                let (r_diam, r_depth) = rec(n.right.as_ref());
                let diam = l_diam.max(r_diam).max(l_depth + r_depth);
                (diam, 1 + l_depth.max(r_depth))
            } else {
                (0, 0)
            }
        }
        rec(root.as_ref()).0
    }
    pub fn diameter_of_binary_tree_rec_with_take(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //println!("diameter_of_binary_tree({:?})", root);
        fn rec(n: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            if let Some(n) = n {
                let mut n = n.borrow_mut();
                let (l_diam, l_depth) = rec(n.left.take());
                let (r_diam, r_depth) = rec(n.right.take());
                let diam = l_diam.max(r_diam).max(l_depth + r_depth);
                (diam, 1 + l_depth.max(r_depth))
            } else {
                (0, 0)
            }
        }
        rec(root).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type N = Option<Rc<RefCell<TreeNode>>>;
    #[rustfmt::skip] fn wrap(n: TreeNode)       -> N { Some(Rc::new(RefCell::new(n))) }
    #[rustfmt::skip] fn nlr(v: i32, l: N, r: N) -> N { wrap(TreeNode{val: v, left: l,    right: r}) }
    #[rustfmt::skip] fn  nl(v: i32, l: N)       -> N { wrap(TreeNode{val: v, left: l,    right: None}) }
    #[rustfmt::skip] fn  nr(v: i32, r: N)       -> N { wrap(TreeNode{val: v, left: None, right: r}) }
    #[rustfmt::skip] fn   n(v: i32)             -> N { wrap(TreeNode{val: v, left: None, right: None}) }

    #[test]
    fn r_1_2_3_4_5() {
        let r = nlr(1, nlr(2, n(4), n(5)), n(3));
        assert_eq!(Solution::diameter_of_binary_tree(r), 3);
        // Explanation: 3 is the length of the path [4,2,1,3] or [5,2,1,3].
    }
    #[test]
    fn r_1_2() {
        let r = nl(1, n(2));
        assert_eq!(Solution::diameter_of_binary_tree(r), 1);
    }
    #[test]
    fn r_1() {
        let r = n(1);
        assert_eq!(Solution::diameter_of_binary_tree(r), 0);
    }

    /// If getting stack overflow:
    ///
    /// ```sh
    /// thread 'c2021_10_11::tests::r_10000_nodes_in_left' has overflowed its stack
    /// fatal runtime error: stack overflow
    /// ```
    ///
    /// Add `RUST_MIN_STACK=67108864` to env:
    ///
    /// ```sh
    /// RUST_MIN_STACK=67108864 cargo test --lib c2021_10_11
    /// ```
    #[test]
    fn r_10000_nodes_in_left() {
        let mut r = None;
        for _ in 0..10000 {
            r = nl(1, r);
        }
        assert_eq!(Solution::diameter_of_binary_tree(r), 9999);
    }
}
