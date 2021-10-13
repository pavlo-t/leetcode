#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 1008. Construct Binary Search Tree from Preorder Traversal
/// ==========================================================
///
/// Given an array of integers preorder,
/// which represents the __preorder traversal__ of a BST (i.e., __binary search tree__),
/// construct the tree and return _its root_.
///
/// It is __guaranteed__ that there is always possible to find a binary search tree
/// with the given requirements for the given test cases.
///
/// A __binary search tree__ is a binary tree where for every node,
/// any descendant of `Node.left` has a value __strictly less than__ `Node.val`,
/// and any descendant of `Node.right` has a value __strictly greater than__ `Node.val`.
///
/// A __preorder traversal__ of a binary tree displays the value of the node first,
/// then traverses `Node.left`, then traverses `Node.right`.
///
/// __Constraints:__
///
/// - `1 <= preorder.length <= 100`
/// - `1 <= preorder[i] <= 100_000_000`
/// - All the values of `preorder` are __unique__.
///
/// https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/
struct Solution;
impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn rec(p: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if p.is_empty() {
                None
            } else {
                let val = p[0];
                #[rustfmt::skip]
                let ri = p.iter().enumerate().find(|(_, &v)| v > val).map(|(i, _)| i).unwrap_or(p.len());
                let left = rec(&p[1..ri]);
                let right = rec(&p[ri..]);
                Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
            }
        }

        rec(&preorder)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    type N = Option<Rc<RefCell<TreeNode>>>;
    #[rustfmt::skip] fn wrap(n: TreeNode) -> N { Some(Rc::new(RefCell::new(n))) }
    #[rustfmt::skip] fn nlr(v: i32, l: N, r: N) -> N { wrap(TreeNode {val: v, left: l, right: r}) }
    #[rustfmt::skip] fn nl(v: i32, l: N) -> N { wrap(TreeNode {val: v, left: l, right: None}) }
    #[rustfmt::skip] fn nr(v: i32, r: N) -> N { wrap(TreeNode {val: v, left: None, right: r}) }
    #[rustfmt::skip] fn n(v: i32) -> N { wrap(TreeNode {val: v, left: None, right: None}) }

    #[test]
    fn p_8_5_1_7_10_12() {
        let p = vec![8, 5, 1, 7, 10, 12];
        let e = nlr(8, nlr(5, n(1), n(7)), nr(10, n(12)));
        assert_eq!(Solution::bst_from_preorder(p), e);
    }
    #[test]
    fn p_8_5_1_9_10_12() {
        let p = vec![8, 5, 1, 9, 10, 12];
        let e = nlr(8, nl(5, n(1)), nr(9, nr(10, n(12))));
        assert_eq!(Solution::bst_from_preorder(p), e);
    }
    #[test]
    fn p_1_3() {
        let p = vec![1, 3];
        let e = nr(1, n(3));
        assert_eq!(Solution::bst_from_preorder(p), e);
    }
    #[test]
    fn p_2_1() {
        let p = vec![2, 1];
        let e = nl(2, n(1));
        assert_eq!(Solution::bst_from_preorder(p), e);
    }
    #[test]
    fn p_1() {
        let p = vec![1];
        let e = n(1);
        assert_eq!(Solution::bst_from_preorder(p), e);
    }
}
