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

/// ### Balanced Binary Tree
///
/// Given a binary tree, determine if it is height-balanced.
///
/// For this problem, a height-balanced binary tree is defined as:
///
/// > a binary tree in which the left and right subtrees of every node differ in height by no more than 1.
///
/// **Constraints:**
///  - The number of nodes in the tree is in the range `[0, 5000]`.
///  - `-10000 <= Node.val <= 10000`
///
/// https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3577/
struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(n) = node {
                let ld = depth(&n.as_ref().borrow().left);
                if ld < 0 {
                    return -1;
                }
                let rd = depth(&n.as_ref().borrow().right);
                if rd < 0 || (rd - ld).abs() > 1 {
                    -1
                } else {
                    ld.max(rd) + 1
                }
            } else {
                0
            }
        }
        depth(&root) >= 0
    }

    pub fn is_balanced_clean(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(n) = node {
                let ld = depth(&n.as_ref().borrow().left);
                let rd = depth(&n.as_ref().borrow().right);
                if ld < 0 || rd < 0 || (rd - ld).abs() > 1 {
                    -1
                } else {
                    ld.max(rd) + 1
                }
            } else {
                0
            }
        }
        depth(&root) >= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // @formatter:off
    type Node = Option<Rc<RefCell<TreeNode>>>;
    fn wrap(n: TreeNode) -> Node { Some(Rc::new(RefCell::new(n))) }
    fn n(v: i32) -> Node { wrap(TreeNode { val: v, left: None, right: None }) }
    fn nl(v: i32, l: Node) -> Node { wrap(TreeNode { val: v, left: l, right: None }) }
    fn nr(v: i32, r: Node) -> Node { wrap(TreeNode { val: v, left: None, right: r }) }
    fn nlr(v: i32, l: Node, r: Node) -> Node { wrap(TreeNode { val: v, left: l, right: r }) }
    // @formatter:on

    #[test]
    fn example1_r3_9_20_n_n_15_7_is_balanced() {
        let root = nlr(3, n(9), nlr(20, n(15), n(7)));
        assert!(Solution::is_balanced(root));
    }

    #[test]
    fn example2_r1_2_2_3_3_n_n_4_4_is_not_balanced() {
        let root =
            nlr(1,
                nlr(2,
                    nlr(3, n(4), n(4)),
                    n(3)),
                n(2));
        assert!(!Solution::is_balanced(root));
    }

    #[test]
    fn example3_rn_is_balanced() {
        let root = None;
        assert!(Solution::is_balanced(root));
    }


    #[test]
    fn r1_2_2_3_3_3_3_4_4_n_n_n_n_4_4_is_balanced() {
        let root =
            nlr(1,
                nlr(2, nlr(3, n(4), n(4)), n(3)),
                nlr(2, n(3), nlr(3, n(4), n(4))),
            );
        assert!(Solution::is_balanced(root));
    }

    #[test]
    fn r1_2_2_3_n_n_3_4_n_n_4_is_not_balanced() {
        let root =
            nlr(1,
                nl(2, nl(3, n(4))),
                nr(2, nr(3, n(4))));
        assert!(!Solution::is_balanced(root));
    }

    fn balanced_tree(l: i32, r: i32) -> Node {
        if l > r { None } else {
            let mid = l + (r - l) / 2;
            nlr(mid, balanced_tree(l, mid - 1), balanced_tree(mid + 1, r))
        }
    }

    #[test]
    fn r10000_balanced_nodes_is_balanced() {
        let root = balanced_tree(1, 10000);
        assert!(Solution::is_balanced(root));
    }

    fn left_tree(v: i32) -> Node {
        if v == 0 { None } else { nl(v, left_tree(v - 1)) }
    }

    #[test]
    fn r_left_unbalanced_right_balanced_is_not_balanced() {
        let root = nlr(0, left_tree(5000), balanced_tree(1, 10000));
        assert!(!Solution::is_balanced(root));
    }

    #[test]
    fn r_left_balanced_right_unbalanced_is_not_balanced() {
        let root = nlr(0, balanced_tree(1, 10000), left_tree(5000));
        assert!(!Solution::is_balanced(root));
    }
}
