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

/// Path Sum II
/// ===========
///
/// Given the `root` of a binary tree and an integer `targetSum`,
/// return _all __root-to-leaf__ paths where each path's sum equals `targetSum`_.
///
/// A __leaf__ is a node with no children.
///
/// Constraints:
///
/// - The number of nodes in the tree is in the range `[0, 5000]`.
/// - `-1000 <= Node.val <= 1000`
/// - `-1000 <= targetSum <= 1000`
///
/// https://leetcode.com/explore/featured/card/august-leetcoding-challenge-2021/613/week-1-august-1st-august-7th/3838/
struct Solution;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        type Node = Option<Rc<RefCell<TreeNode>>>;
        fn bt(n: &Node, s: i32, p: &mut Vec<i32>, rsf: &mut Vec<Vec<i32>>) {
            if let Some(n) = n.as_ref() {
                let n = n.borrow();
                p.push(n.val);
                if n.left.is_none() && n.right.is_none() && n.val == s {
                    rsf.push(p.clone());
                } else {
                    bt(&n.left, s - n.val, p, rsf);
                    bt(&n.right, s - n.val, p, rsf);
                }
                p.pop();
            }
        }

        let mut result = vec![];
        bt(&root, target_sum, &mut vec![], &mut result);
        result
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
        wrap(TreeNode::new(v))
    }

    #[test]
    fn r_5_4_8_11_n_13_4_7_2_n_n_5_1_t_22_produces_p5p4p11p2_p5p8p4p5() {
        let root = nlr(
            5,
            nl(4, nlr(11, n(7), n(2))),
            nlr(8, n(13), nlr(4, n(5), n(1))),
        );
        let e = vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]];
        assert_eq!(Solution::path_sum(root, 22), e);
        //       5
        //      / \
        //     4   8
        //    /   / \
        //   11  13  4
        //  /  \    / \
        // 7    2  5   1
    }
    #[test]
    fn r_1_2_3_t_5_produces_empty() {
        let root = nlr(1, n(2), n(3));
        let e: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::path_sum(root, 5), e);
    }
    #[test]
    fn r_1_2_t_0_produces_empty() {
        let root = nl(1, n(2));
        let e: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::path_sum(root, 0), e);
    }
    #[test]
    fn r_empty_t_0_produces_empty() {
        let root = None;
        let e: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::path_sum(root, 0), e);
    }
    #[test]
    fn r_0_t_0_produces_0() {
        let root = n(0);
        let e = vec![vec![0]];
        assert_eq!(Solution::path_sum(root, 0), e);
    }
}
