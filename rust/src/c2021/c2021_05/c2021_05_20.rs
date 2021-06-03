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

/// Binary Tree Level Order Traversal
/// =================================
///
/// Given the `root` of a binary tree,
/// return _the level order traversal of its nodes'_ values
/// (i.e., from left to right, level by level).
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[0, 2000]`.
/// - `-1000 <= Node.val <= 1000`
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/600/week-3-may-15th-may-21st/3749/
struct Solution;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;

        let mut q = VecDeque::new();
        q.push_back(None);
        q.push_back(root);

        let mut result: Vec<Vec<i32>> = vec![];
        while let Some(maybe_n) = q.pop_front() {
            if let Some(n) = maybe_n {
                let mut n = n.borrow_mut();
                result.last_mut().unwrap().push(n.val);
                if let Some(l) = n.left.take() {
                    q.push_back(Some(l));
                }
                if let Some(r) = n.right.take() {
                    q.push_back(Some(r));
                }
            } else if q.front().unwrap_or(&None).is_none() {
                return result;
            } else {
                result.push(vec![]);
                q.push_back(None);
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type N = Option<Rc<RefCell<TreeNode>>>;
    fn wrap(n: TreeNode) -> N {
        Some(Rc::new(RefCell::new(n)))
    }
    fn nlr(v: i32, l: N, r: N) -> N {
        wrap(TreeNode {
            val: v,
            left: l,
            right: r,
        })
    }
    fn nl(v: i32, l: N) -> N {
        wrap(TreeNode {
            val: v,
            left: l,
            right: None,
        })
    }
    fn nr(v: i32, r: N) -> N {
        wrap(TreeNode {
            val: v,
            left: None,
            right: r,
        })
    }
    fn n(v: i32) -> N {
        wrap(TreeNode::new(v))
    }

    macro_rules! vv { ($($x:tt), *) => { vec!($(vec!$x), *) }; }

    #[test]
    fn example1() {
        let root = nlr(3, n(9), nlr(20, n(15), n(7)));
        let e = vv![[3], [9, 20], [15, 7]];
        assert_eq!(Solution::level_order(root), e);
    }
    #[test]
    fn example2() {
        let root = n(1);
        let e = vv![[1]];
        assert_eq!(Solution::level_order(root), e);
    }
    #[test]
    fn example3() {
        let e: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::level_order(None), e);
    }
}
