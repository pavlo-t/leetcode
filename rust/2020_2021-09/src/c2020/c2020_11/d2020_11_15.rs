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

struct Solution {}

struct SolutionWithClosureStruct {}

struct SolutionMy {}

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        match root {
            None => 0,
            Some(tn) => {
                let n = Rc::try_unwrap(tn).unwrap().into_inner();
                if n.val < low {
                    Self::range_sum_bst(n.right, low, high)
                } else if n.val > high {
                    Self::range_sum_bst(n.left, low, high)
                } else {
                    n.val + Self::range_sum_bst(n.left, low, high) + Self::range_sum_bst(n.right, low, high)
                }
            }
        }
    }
}

impl SolutionWithClosureStruct {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        struct DFS<'s> { apply: &'s dyn Fn(&DFS, &Option<Rc<RefCell<TreeNode>>>) -> i32 }
        let dfs = DFS {
            apply: &|dfs, n| {
                if n.is_none() { 0 } else {
                    let n = n.as_ref().unwrap().borrow();
                    if n.val < low {
                        (dfs.apply)(dfs, &n.right)
                    } else if n.val > high {
                        (dfs.apply)(dfs, &n.left)
                    } else {
                        n.val + (dfs.apply)(dfs, &n.left) + (dfs.apply)(dfs, &n.right)
                    }
                }
            }
        };
        (dfs.apply)(&dfs, &root)
    }
}

impl SolutionMy {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        Self::_range_sum_bst(&root, low, high)
    }
    fn _range_sum_bst(root: &Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        match root {
            None => 0,
            Some(tn) => {
                let n = tn.borrow();
                if n.val < low {
                    Self::_range_sum_bst(&n.right, low, high)
                } else if n.val > high {
                    Self::_range_sum_bst(&n.left, low, high)
                } else {
                    n.val + Self::_range_sum_bst(&n.left, low, high) + Self::_range_sum_bst(&n.right, low, high)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // impl From<TreeNode> for
    impl From<TreeNode> for TN {
        fn from(t: TreeNode) -> Self {
            Some(Rc::new(RefCell::new(t)))
        }
    }

    type TN = Option<Rc<RefCell<TreeNode>>>;

    fn tlr(val: i32, l: TreeNode, r: TreeNode) -> TreeNode {
        TreeNode { val, left: l.into(), right: r.into() }
    }

    fn tl(val: i32, l: TreeNode) -> TreeNode {
        TreeNode { val, left: l.into(), right: None }
    }

    fn tr(val: i32, r: TreeNode) -> TreeNode {
        TreeNode { val, left: None, right: r.into() }
    }

    fn t(val: i32) -> TreeNode {
        TreeNode { val, left: None, right: None }
    }

    // Example 1: (root = [10,5,15,3,7,null,18], low = 7, high = 15) -> 32
    #[test]
    fn example_1() {
        let root = tlr(10, tlr(5, t(3), t(7)), tr(15, t(18)));
        assert_eq!(Solution::range_sum_bst(root.into(), 7, 15), 32)
    }

    // Example 2: (root = [10,5,15,3,7,13,18,1,null,6], low = 6, high = 10) -> 23
    #[test]
    fn example_2() {
        let root = tlr(10,
                       tlr(5, tl(3, t(1)), tl(7, t(6))),
                       tlr(15, t(13), t(18)));
        assert_eq!(Solution::range_sum_bst(root.into(), 6, 10), 23)
    }

    // Test 13: (root = [18,9,27,6,15,24,30,3,null,12,null,21], low = 18, high = 24) -> 63
    #[test]
    fn test_13() {
        let root = tlr(18,
                       tlr(9, tl(6, t(3)), tl(15, t(12))),
                       tlr(27, tl(24, t(21)), t(30)));
        assert_eq!(Solution::range_sum_bst(root.into(), 18, 24), 63)
    }
    // (root = [tree max size+], low = 1, high = 100_000) -> 286912
    // (root = [chain tree 19_577], low = 1, high = 100_000) -> 191_639_253
}
