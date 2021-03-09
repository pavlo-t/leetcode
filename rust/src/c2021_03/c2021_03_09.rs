#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TreeNode {
    pub val: i32,
    pub left: TreeLink,
    pub right: TreeLink,
}

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

macro_rules! tree {
    ($e:expr) => {
        TreeLink::leaf($e)
    };
    ($e:expr, $l:expr, $r:expr) => {
        TreeLink::branch($e, $l, $r)
    };
}

trait TreeMaker {
    fn branch(val: i32, left: TreeLink, right: TreeLink) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
    fn leaf(val: i32) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }
}

impl TreeMaker for TreeLink {}

/// # Add One Row to Tree
///
/// Given the `root` of a binary tree, then value `v` and depth `d`,
/// you need to add a row of nodes with value `v` at the given depth `d`.
/// The `root` node is at depth `1`.
///
/// The adding rule is: given a positive integer depth `d`,
/// for each NOT null tree nodes `N` in depth `d-1`,
/// create two tree nodes with value `v` as `N's` left subtree root and right subtree root.
/// And `N's` __original left subtree__ should be the left subtree of the new left subtree root,
/// its __original right subtree__ should be the right subtree of the new right subtree root.
/// If depth `d` is `1` that means there is no depth `d-1` at all,
/// then create a tree node with value `v` as the new root of the whole original tree,
/// and the original tree is the new root's left subtree.
///
/// __Note:__
///
/// 1. The given `d` is in range `[1, maximum depth of the given tree + 1]`.
/// 2. The given binary tree has at least one tree node.
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/589/week-2-march-8th-march-14th/3666/
struct Solution;

trait Postorder {
    fn postorder(self, depth: i32, v: i32, d: i32) -> TreeLink;
}

impl Postorder for TreeLink {
    fn postorder(self, depth: i32, v: i32, d: i32) -> TreeLink {
        if let Some(node) = self {
            let mut node = node.borrow_mut();
            let val = node.val;
            let left = node.left.take();
            let right = node.right.take();
            let mut left = left.postorder(depth + 1, v, d);
            let mut right = right.postorder(depth + 1, v, d);
            if depth + 1 == d {
                left = tree!(v, left, None);
                right = tree!(v, None, right);
            }
            tree!(val, left, right)
        } else {
            None
        }
    }
}

type Node = Option<Rc<RefCell<TreeNode>>>;

fn wrap_node(n: TreeNode) -> Node { Some(Rc::new(RefCell::new(n))) }

impl Solution {
    pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, v: i32, d: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if d == 1 {
            wrap_node(TreeNode { val: v, left: root, right: None })
        } else {
            fn rec(n: Node, cd: i32, tv: i32, td: i32) -> Node {
                if let Some(n) = n {
                    let mut n = n.borrow_mut();
                    let nv = n.val;
                    let mut left = rec(n.left.take(), cd + 1, tv, td);
                    let mut right = rec(n.right.take(), cd + 1, tv, td);
                    if cd + 1 == td {
                        left = wrap_node(TreeNode { val: tv, left, right: None });
                        right = wrap_node(TreeNode { val: tv, left: None, right });
                    }
                    wrap_node(TreeNode { val: nv, left, right })
                } else {
                    None
                }
            }
            rec(root, 1, v, d)
        }
    }

    /// https://rustgym.com/leetcode/623
    /// https://github.com/warycat/rustgym/blob/f0874c1b23fbc5aaaeb75e26edd978cfb2a2862d/leetcode/src/d6/_623_add_one_row_to_tree.rs
    pub fn add_one_row_rust_gym(root: TreeLink, val: i32, d: i32) -> TreeLink {
        if d == 1 {
            Some(Rc::new(RefCell::new(TreeNode { val, left: root, right: None })))
        } else {
            root.postorder(1, val, d)
        }
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
    fn example1() {
        let root = nlr(4, nlr(2, n(3), n(1)), nl(6, n(5)));
        let e = nlr(4, nl(1, nlr(2, n(3), n(1))), nr(1, nl(6, n(5))));
        assert_eq!(Solution::add_one_row(root, 1, 2), e);
        //        4            4
        //      /   \         / \
        //     2     6       1   1
        //    / \   /       /     \
        //   3   1 5       2       6
        //                / \     /
        //               3   1   5
    }

    #[test]
    fn example2() {
        let root = nl(4, nlr(2, n(3), n(1)));
        let e = nl(4, nlr(2, nl(1, n(3)), nr(1, n(1))));
        assert_eq!(Solution::add_one_row(root, 1, 3), e);
        //       4          4
        //      /          /
        //     2          2
        //    / \        / \
        //   3   1      1   1
        //             /     \
        //            3       1
    }

    #[test]
    fn depth_1() {
        let root = nl(4, nlr(2, n(3), n(1)));
        let e = nl(1, nl(4, nlr(2, n(3), n(1))));
        assert_eq!(Solution::add_one_row(root, 1, 1), e);
        //       4          1
        //      /          /
        //     2          4
        //    / \        /
        //   3   1      2
        //             / \
        //            3   1
    }

    #[test]
    fn depth_3() {
        let root = nlr(1, nlr(2, n(3), n(3)), nlr(2, n(3), n(3)));
        let e = nlr(1,
                    nlr(2, nl(4, n(3)), nr(4, n(3))),
                    nlr(2, nl(4, n(3)), nr(4, n(3))));
        assert_eq!(Solution::add_one_row(root, 4, 3), e);
        //        1              1
        //      /   \          /   \
        //     2     2        2     2
        //    / \   / \      / \   / \
        //   3   3 3   3    4  4   4  4
        //                 /   \   /   \
        //                3     3 3     3
    }
}
