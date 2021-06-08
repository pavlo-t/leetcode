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

/// Construct Binary Tree from Preorder and Inorder Traversal
/// =========================================================
///
/// Given two integer arrays `preorder` and `inorder`
/// where `preorder` is the preorder traversal of a binary tree
/// and `inorder` is the inorder traversal of the same tree,
/// construct and return _the binary tree_.
///
/// __Constraints:__
///
/// - `1 <= preorder.length <= 3000`
/// - `inorder.length == preorder.length`
/// - `-3000 <= preorder[i], inorder[i] <= 3000`
/// - `preorder` and `inorder` consist of __unique__ values.
/// - Each value of `inorder` also appears in `preorder`.
/// - `preorder` is guaranteed to be the __preorder__ traversal of the tree.
/// - `inorder` is guaranteed to be the __inorder__ traversal of the tree.
///
/// https://leetcode.com/explore/featured/card/june-leetcoding-challenge-2021/604/week-2-june-8th-june-14th/3772/
struct Solution;
type Tree = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Tree {
        fn bt(p: &[i32], i: &[i32]) -> Tree {
            if p.is_empty() {
                None
            } else {
                let val = p[0];
                let mut n = TreeNode::new(val);
                let idx = i.iter().enumerate().find(|(_, &v)| v == val).unwrap().0;

                n.left = bt(&p[1..=idx], &i[..idx]);
                n.right = bt(&p[idx + 1..], &i[idx + 1..]);

                Some(Rc::new(RefCell::new(n)))
            }
        }

        bt(&preorder, &inorder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Node = Option<Rc<RefCell<TreeNode>>>;
    fn wrap(n: TreeNode) -> Node {
        Some(Rc::new(RefCell::new(n)))
    }
    fn nlr(v: i32, l: Node, r: Node) -> Node {
        wrap(TreeNode {
            val: v,
            left: l,
            right: r,
        })
    }
    fn nl(v: i32, l: Node) -> Node {
        wrap(TreeNode {
            val: v,
            left: l,
            right: None,
        })
    }
    fn nr(v: i32, r: Node) -> Node {
        wrap(TreeNode {
            val: v,
            left: None,
            right: r,
        })
    }
    fn n(v: i32) -> Node {
        wrap(TreeNode {
            val: v,
            left: None,
            right: None,
        })
    }

    #[test]
    fn preorder_3_9_20_15_7_inorder_9_3_15_20_7_produces_3_9_20_n_n_15_7() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let e = nlr(3, n(9), nlr(20, n(15), n(7)));
        assert_eq!(Solution::build_tree(preorder, inorder), e);
        //   3
        //  / \
        // 9  20
        //   /  \
        //  15   7
    }
    #[test]
    fn preorder_m1_inorder_m1_produces_m1() {
        assert_eq!(Solution::build_tree(vec![-1], vec![-1]), n(-1));
    }

    #[test]
    fn preorder_1_2_3_4_5_6_7_inorder_3_2_4_1_6_5_7_produces_1_2_5_3_4_6_7() {
        let preorder = vec![1, 2, 3, 4, 5, 6, 7];
        let inorder = vec![3, 2, 4, 1, 6, 5, 7];
        let e = nlr(1, nlr(2, n(3), n(4)), nlr(5, n(6), n(7)));
        assert_eq!(Solution::build_tree(preorder, inorder), e);
        //      1
        //    /   \
        //   2     5
        //  / \   / \
        // 3   4 6   7
    }
    #[test]
    fn preorder_1_2_3_inorder_3_2_1_produces_1_2_n_3_n() {
        let preorder = vec![1, 2, 3];
        let inorder = vec![3, 2, 1];
        let e = nl(1, nl(2, n(3)));
        assert_eq!(Solution::build_tree(preorder, inorder), e);
        //     1
        //    /
        //   2
        //  /
        // 3
    }
}
