#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 226. Invert Binary Tree
/// =======================
///
/// Given the `root` of a binary tree, invert the tree, and return _its_ root.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[0, 100]`.
/// - `-100 <= Node.val <= 100`
///
/// https://leetcode.com/problems/invert-binary-tree/
struct Solution;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        println!("invert_tree({:?})", root);
        root.map(|t| {
            let mut tb = t.borrow_mut();
            let (l, r) = (tb.left.take(), tb.right.take());
            tb.left = Self::invert_tree(r);
            tb.right = Self::invert_tree(l);
            drop(tb);
            t
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
#[rustfmt::skip]
impl TreeNode {
    #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
}

#[cfg(test)]
mod tests {
    use super::*;

    type T = Option<Rc<RefCell<TreeNode>>>;
    #[rustfmt::skip] fn wrap(t: TreeNode) -> T { Some(Rc::new(RefCell::new(t))) }
    #[rustfmt::skip] fn t(v: i32, l: T, r: T) -> T { wrap(TreeNode { val: v, left: l, right: r }) }
    #[rustfmt::skip] fn l(v: i32,           ) -> T { wrap(TreeNode::new(v)) }

    #[rustfmt::skip] #[test] fn r_n()     { assert_eq!(Solution::invert_tree(None            ), None            ); }
    #[rustfmt::skip] #[test] fn r_1()     { assert_eq!(Solution::invert_tree(l(1)            ), l(1)            ); }
    #[rustfmt::skip] #[test] fn r_2_1_3() { assert_eq!(Solution::invert_tree(t(2, l(1), l(3))), t(2, l(3), l(1))); }
    #[test]
    fn r_4_2_7_1_3_6_9() {
        let r = t(4, t(2, l(1), l(3)), t(7, l(6), l(9)));
        let e = t(4, t(7, l(9), l(6)), t(2, l(3), l(1)));
        assert_eq!(Solution::invert_tree(r), e);
    }
}
