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

/// Binary Tree Pruning
/// ===================
///
/// Given the `root` of a binary tree, return
/// _the same tree where every subtree (of the given tree) not containing a `1` has been removed_.
///
/// A subtree of a node `node` is `node` plus every node that is a descendant of `node`.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[1, 200]`.
/// - `Node.val` is either `0` or `1`.
///
/// https://leetcode.com/explore/challenge/card/july-leetcoding-challenge-2021/611/week-4-july-22nd-july-28th/3824/
struct Solution;
impl Solution {
    pub fn prune_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn prune(root: &mut Option<Rc<RefCell<TreeNode>>>) {
            if let Some(n) = root {
                prune(&mut n.borrow_mut().left);
                prune(&mut n.borrow_mut().right);
                if n.borrow().val == 0 && n.borrow().left.is_none() && n.borrow().right.is_none() {
                    *root = None;
                }
            }
        }
        prune(&mut root);
        root
    }

    pub fn prune_tree_my(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn contains_1(n: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            n.as_ref()
                .map(|n| {
                    let nb = n.borrow();
                    nb.val == 1 || contains_1(&nb.left) || contains_1(&nb.right)
                })
                .unwrap_or(false)
        }
        if contains_1(&root) {
            let root = root.unwrap();
            let mut n = root.borrow_mut();
            let mut result = TreeNode::new(n.val);
            result.left = Self::prune_tree(n.left.take());
            result.right = Self::prune_tree(n.right.take());

            Some(Rc::new(RefCell::new(result)))
        } else {
            None
        }
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
    fn r_1n001_produces_1n0n1() {
        let root = nr(1, nlr(0, n(0), n(1)));
        let e = nr(1, nr(0, n(1)));
        assert_eq!(Solution::prune_tree(root), e);
        // Explanation:
        // 1        1
        //  \        \
        //   0   =>   0
        //  / \        \
        // 0   1        1
        // Only the red nodes satisfy the property "every subtree not containing a 1".
        // The diagram on the right represents the answer.
    }
    #[test]
    fn r_1010001_produces_1n1n1() {
        let root = nlr(1, nlr(0, n(0), n(0)), nlr(1, n(0), n(1)));
        let e = nr(1, nr(1, n(1)));
        assert_eq!(Solution::prune_tree(root), e);
        //      1          1
        //    /   \         \
        //   0     1    =>   1
        //  / \   / \         \
        // 0   0 0   1         1
    }
    #[test]
    fn r_11011010_produces_11011n1() {
        let root = nlr(1, nlr(1, nl(1, n(0)), n(1)), nlr(0, n(0), n(1)));
        let e = nlr(1, nlr(1, n(1), n(1)), nr(0, n(1)));
        assert_eq!(Solution::prune_tree(root), e);
        //        1             1
        //      /   \          / \
        //     1     0   =>   1   0
        //    / \   / \      / \   \
        //   1   1 0   1    1   1   1
        //  /
        // 0
    }
    #[test]
    fn r_0000000_produces_n() {
        let root = nlr(0, nlr(0, n(0), n(0)), nlr(0, n(0), n(0)));
        let e = None;
        assert_eq!(Solution::prune_tree(root), e);
        //      0
        //    /   \
        //   0     0
        //  / \   / \
        // 0   0 0   0
    }

    mod performance {
        use super::*;

        #[test]
        fn r_200x1_in_left_produces_itself() {
            let mut root = n(1);
            let mut e = n(1);
            for _ in 0..200 {
                root = nl(1, root);
                e = nl(1, e);
            }
            assert_eq!(Solution::prune_tree(root), e);
        }
    }
}
