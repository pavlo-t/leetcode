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

/// Lowest Common Ancestor of a Binary Search Tree
/// ==============================================
///
/// Given a binary search tree (BST), find the lowest common ancestor (LCA) of two given nodes in the BST.
///
/// According to [the definition of LCA on Wikipedia](https://en.wikipedia.org/wiki/Lowest_common_ancestor):
/// "The lowest common ancestor is defined between two nodes `p` and `q` as the lowest node in `T`
/// that has both `p` and `q` as descendants (where we allow __a node to be a descendant of itself__)."
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[2, 100_000]`.
/// - `-1_000_000_000 <= Node.val <= 1_000_000_000`
/// - All `Node.val` are __unique__.
/// - `p != q`
/// - `p` and `q` will exist in the BST.
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/610/week-3-july-15th-july-21st/3819/
struct Solution;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        type Node = Option<Rc<RefCell<TreeNode>>>;
        fn find(node: &Node, p: i32, q: i32) -> Node {
            let n = node.as_ref().unwrap().borrow();
            if n.val > p && n.val > q {
                find(&n.left, p, q)
            } else if n.val < p && n.val < q {
                find(&n.right, p, q)
            } else {
                node.clone()
            }
        }
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;
        find(&root, p, q)
    }

    pub fn lowest_common_ancestor_my(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn path(n: &Option<Rc<RefCell<TreeNode>>>, v: i32) -> Vec<i32> {
            let n = n.as_ref().unwrap().borrow();
            let mut result = vec![n.val];
            if n.val > v {
                result.extend(path(&n.left, v));
            } else if n.val < v {
                result.extend(path(&n.right, v));
            }
            result
        }
        fn get(node: &Option<Rc<RefCell<TreeNode>>>, v: i32) -> Option<Rc<RefCell<TreeNode>>> {
            let n = node.as_ref().unwrap().borrow();
            if n.val > v {
                get(&n.left, v)
            } else if n.val < v {
                get(&n.right, v)
            } else {
                Some(node.as_ref().unwrap().clone())
            }
        }
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;
        let pp = path(&root, p);
        let qp = path(&root, q);
        let mut i = 0;
        while i + 1 < pp.len() && i + 1 < qp.len() && pp[i + 1] == qp[i + 1] {
            i += 1;
        }

        get(&root, pp[i])
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
    //Example 1:

    #[test]
    fn r_6_2_8_0_4_7_9_n_n_3_5_p_2_q_8_produces_6() {
        let root = nlr(6, nlr(2, n(0), nlr(4, n(3), n(5))), nlr(8, n(7), n(9)));
        let p = nlr(2, n(0), nlr(4, n(3), n(5)));
        let q = nlr(8, n(7), n(9));
        let e = nlr(6, nlr(2, n(0), nlr(4, n(3), n(5))), nlr(8, n(7), n(9)));
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), e);
        //       6
        //     /   \
        //   2       8
        //  / \     / \
        // 0   4   7   9
        //    / \
        //   3   5
        // Explanation: The LCA of nodes 2 and 8 is 6.
    }
    #[test]
    fn r_6_2_8_0_4_7_9_n_n_3_5_p_2_q_4_produces_2() {
        let root = nlr(6, nlr(2, n(0), nlr(4, n(3), n(5))), nlr(8, n(7), n(9)));
        let p = nlr(2, n(0), nlr(4, n(3), n(5)));
        let q = nlr(4, n(3), n(5));
        let e = nlr(2, n(0), nlr(4, n(3), n(5)));
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), e);
        //       6
        //     /   \
        //   2       8
        //  / \     / \
        // 0   4   7   9
        //    / \
        //   3   5
        // Explanation: The LCA of nodes 2 and 4 is 2,
        // since a node can be a descendant of itself according to the LCA definition.
    }
    #[test]
    fn r_2_1_p_2_q_1_produces_2() {
        let root = nl(2, n(1));
        let p = nl(2, n(1));
        let q = n(1);
        let e = nl(2, n(1));
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), e);
    }

    #[test]
    fn r_6_2_8_0_4_7_9_n_n_3_5_p_0_q_5_produces_2() {
        let root = nlr(6, nlr(2, n(0), nlr(4, n(3), n(5))), nlr(8, n(7), n(9)));
        let p = n(0);
        let q = n(5);
        let e = nlr(2, n(0), nlr(4, n(3), n(5)));
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), e);
        //       6
        //     /   \
        //   2       8
        //  / \     / \
        // 0   4   7   9
        //    / \
        //   3   5
    }
}
