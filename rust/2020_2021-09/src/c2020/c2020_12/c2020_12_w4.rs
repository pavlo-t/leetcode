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

/// ### Find Nearest Right Node in Binary Tree
///
/// https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3576/
struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn find_nearest_right_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        u: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::VecDeque;

        let u = u.expect("Illegal input");
        let u = u.borrow();
        let mut q = VecDeque::new();
        q.push_back(root);
        q.push_back(None);

        while q.len() > 1 {
            match q.pop_front().unwrap().as_ref() {
                None => q.push_back(None),
                Some(n) => {
                    let n = n.borrow();
                    if n.val == u.val {
                        return q.pop_front().unwrap().clone();
                    }
                    if n.left.is_some() { q.push_back(n.left.clone()) }
                    if n.right.is_some() { q.push_back(n.right.clone()) }
                }
            };
        }

        None
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
    fn example1_r123n456_u4_is_5() {
        let root =
            nlr(1,
                nlr(2,
                    None,
                    n(4)),
                nlr(3,
                    n(5),
                    n(6)));
        assert_eq!(Solution::find_nearest_right_node(root, n(4)), n(5));
        // Explanation: The nearest node on the same level to the right of node 4 is node 5.
    }

    #[test]
    fn example2_r3n42_u2_is_n() {
        let root = nr(3, nl(4, n(2)));
        assert_eq!(Solution::find_nearest_right_node(root, n(2)), None);
        // Explanation: There are no nodes to the right of 2.
    }

    #[test]
    fn example3_r1_u1_is_n() {
        assert_eq!(Solution::find_nearest_right_node(n(1), n(1)), None);
    }

    #[test]
    fn example4_r342nnn1_u4_is_2() {
        let root = nlr(3, n(4), nr(2, n(1)));
        assert_eq!(Solution::find_nearest_right_node(root, n(4)), nr(2, n(1)));
    }

    fn build_bst(from: i32, to: i32) -> Node {
        if from > to {
            None
        } else {
            let mid = from + (to - from) / 2;
            nlr(mid, build_bst(from, mid - 1), build_bst(mid + 1, to))
        }
    }

    #[test]
    fn r100000_nodes_in_a_bst_u99998_is_100000() {
        let root = build_bst(1, 100_000);
        assert_eq!(Solution::find_nearest_right_node(root, n(99998)), n(100000));
    }

    #[test]
    fn r100000_nodes_in_a_bst_u99999_is_n() {
        let root = build_bst(1, 100_000);
        assert_eq!(Solution::find_nearest_right_node(root, n(99999)), None);
    }

    #[test]
    fn r100000_nodes_in_a_bst_u50000_is_n() {
        let root = build_bst(1, 100_000);
        assert_eq!(Solution::find_nearest_right_node(root, n(50000)), None);
    }
}
