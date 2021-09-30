// @formatter:off
#![allow(dead_code)]

use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
  #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
}

struct Solution;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() { return 0 }

        let mut max_depth = 0;
        let mut q = std::collections::VecDeque::new();
        q.push_back((root, 1));

        while let Some((n, d)) = q.pop_front() {
            if let Some(n) = n {
                let n = n.borrow();
                let nd = d + 1;
                if n.left.is_some() { q.push_back((n.left.to_owned(), nd)); }
                if n.right.is_some() { q.push_back((n.right.to_owned(), nd)); }

                max_depth = max_depth.max(d);
            }
        }

        max_depth
    }
}

struct SolutionRecursive;
impl SolutionRecursive {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let n = node.borrow();
            let ld = Self::max_depth(n.left.to_owned());
            let rd = Self::max_depth(n.right.to_owned());
            1 + ld.max(rd)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Node = Option<Rc<RefCell<TreeNode>>>;

    fn wrap(n: TreeNode) -> Node { Some(Rc::new(RefCell::new(n))) }

    fn n(v: i32) -> Node { wrap(TreeNode::new(v)) }
    fn nl(v: i32, l: Node) -> Node { wrap(TreeNode { val: v, left: l, right: None }) }
    fn nr(v: i32, r: Node) -> Node { wrap(TreeNode { val: v, left: None, right: r }) }
    fn nlr(v: i32, l: Node, r: Node) -> Node { wrap(TreeNode { val: v, left: l, right: r }) }

    fn balanced_tree(nodes: i32) -> Node {
        if nodes < 2 { None } else { nlr(nodes, balanced_tree(nodes / 2), balanced_tree(nodes / 2)) }
    }
    fn left_tree(nodes: i32) -> Node {
        let mut curr = n(1);
        for i in 2..=nodes { curr = nl(i, curr); }
        curr
    }
    fn right_tree(nodes: i32) -> Node {
        let mut curr = n(1);
        for i in 2..=nodes { curr = nr(i, curr); }
        curr
    }

    #[test]
    fn example1_tn3_9_20_n_n_15_7_is_3() {
        let root = nlr(3, n(9), nlr(20, n(15), n(7)));
        assert_eq!(Solution::max_depth(root), 3);
    }
    #[test]
    fn example2_tn1_n_2_is_2() {
        let root = nr(1, n(2));
        assert_eq!(Solution::max_depth(root), 2);
    }
    #[test]
    fn example3_tn_is_0() {
        assert_eq!(Solution::max_depth(None), 0);
    }
    #[test]
    fn example4_tn0_is_1() {
        assert_eq!(Solution::max_depth(n(0)), 1);
    }

    #[test]
    fn test_10000plus_nodes_in_a_balanced_tree_is_13() {
        let root = balanced_tree(10000);
        assert_eq!(Solution::max_depth(root), 13);
    }

    /// If getting stack overflow:
    ///
    /// ```sh
    /// thread 'c2020::c2020_12::d2020_12_01::tests::max_size' has overflowed its stack
    /// fatal runtime error: stack overflow
    /// ```
    ///
    /// Add `RUST_MIN_STACK=67108864` to env
    #[test]
    fn test_10000nodes_in_the_left_branch_is_10000() {
        let root = left_tree(10000);
        assert_eq!(Solution::max_depth(root), 10000)
    }
    #[test]
    fn test_10000nodes_in_the_right_branch_is_10000() {
        let root = right_tree(10000);
        assert_eq!(Solution::max_depth(root), 10000)
    }

    #[test]
    fn test_100000plus_nodes_in_a_balanced_tree_is_13() {
        let root = balanced_tree(100000);
        assert_eq!(Solution::max_depth(root), 16);
    }
    #[test]
    fn test_100000nodes_in_the_right_branch_is_100000() {
        let root = right_tree(100_000);
        assert_eq!(Solution::max_depth(root), 100_000)
    }
}
