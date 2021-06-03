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

/// Binary Tree Cameras
/// ===================
///
/// Given a binary tree, we install cameras on the nodes of the tree.
///
/// Each camera at a node can monitor __its parent, itself, and its immediate children__.
///
/// Calculate the minimum number of cameras needed to monitor all nodes of the tree.
///
/// __Note:__
///
/// 1. The number of nodes in the given tree will be in the range `[1, 1000]`.
/// 2. __Every__ node has value 0.
///
/// https://leetcode.com/explore/featured/card/may-leetcoding-challenge-2021/600/week-3-may-15th-may-21st/3745/
struct Solution;
impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        type Node = Option<Rc<RefCell<TreeNode>>>;

        // Monitoring state:
        // 0 - needs to be monitored from parent
        // 1 - is monitored from children
        // 2 - can monitor the parent, has camera
        fn rec(n: &Node) -> (i32, u8) {
            if n.is_none() {
                (0, 1)
            } else {
                let n = n.as_ref().unwrap().borrow();
                if n.left.is_none() && n.right.is_none() {
                    (0, 0)
                } else {
                    let (lcs, lm) = rec(&n.left);
                    let (rcs, rm) = rec(&n.right);

                    if lm.min(rm) == 0 {
                        (1 + lcs + rcs, 2)
                    } else if lm.max(rm) == 2 {
                        (lcs + rcs, 1)
                    } else {
                        (lcs + rcs, 0)
                    }
                }
            }
        }

        match rec(&root) {
            (cs, 0) => cs + 1,
            (cs, _) => cs,
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
    fn nlr(l: Node, r: Node) -> Node {
        wrap(TreeNode {
            val: 0,
            left: l,
            right: r,
        })
    }
    fn nl(l: Node) -> Node {
        wrap(TreeNode {
            val: 0,
            left: l,
            right: None,
        })
    }
    fn nr(r: Node) -> Node {
        wrap(TreeNode {
            val: 0,
            left: None,
            right: r,
        })
    }
    fn n() -> Node {
        wrap(TreeNode::new(0))
    }

    #[test]
    fn example1_1_camera() {
        let root = nl(nlr(n(), n()));
        assert_eq!(Solution::min_camera_cover(root), 1);
        //     0
        //    /
        //   C
        //  / \
        // 0   0
        //
        // Explanation: One camera is enough to monitor all nodes if placed as shown.
    }
    #[test]
    fn example2_2_cameras() {
        let root = nl(nl(nl(nr(n()))));
        assert_eq!(Solution::min_camera_cover(root), 2);
        //       0
        //      /
        //     C
        //    /
        //   0
        //  /
        // C
        //  \
        //   0
        //
        // Explanation: At least two cameras are needed to monitor all nodes of the tree.
        // The above image shows one of the valid configurations of camera placement.
    }

    #[test]
    fn my_test1_1_camera_in_root() {
        let root = n();
        assert_eq!(Solution::min_camera_cover(root), 1);
        // C
    }
    #[test]
    fn my_test2_1_camera_in_root() {
        let root = nlr(n(), n());
        assert_eq!(Solution::min_camera_cover(root), 1);
        //   C
        //  / \
        // 0   0
    }
    #[test]
    fn my_test3_2_cameras() {
        let root = nlr(nlr(n(), n()), n());
        assert_eq!(Solution::min_camera_cover(root), 2);
        //     C
        //    / \
        //   C   0
        //  / \
        // 0   0
    }
    #[test]
    fn my_test4_2_cameras() {
        let root = nlr(n(), nlr(n(), n()));
        assert_eq!(Solution::min_camera_cover(root), 2);
        //   C
        //  / \
        // 0   C
        //    / \
        //   0   0
    }
    #[test]
    fn my_test5_2_cameras() {
        let root = nlr(nlr(n(), n()), nlr(n(), n()));
        assert_eq!(Solution::min_camera_cover(root), 2);
        //      0
        //    /   \
        //   C     C
        //  / \   / \
        // 0   0 0   0
    }
    #[test]
    fn my_test5_3_cameras() {
        let root = nlr(nlr(nlr(n(), n()), n()), n());
        assert_eq!(Solution::min_camera_cover(root), 3);
        //       C
        //      / \
        //     C   0
        //    / \
        //   C   0
        //  / \
        // 0   0
    }
    #[test]
    fn my_test6_3_cameras() {
        let root = nlr(nlr(nlr(n(), n()), nlr(n(), n())), n());
        assert_eq!(Solution::min_camera_cover(root), 3);
        //        C
        //       / \
        //      0   0
        //    /   \
        //   C     C
        //  / \   / \
        // 0   0 0   0
    }
    #[test]
    fn my_test7_3_cameras() {
        let root = nlr(nl(nlr(nlr(n(), n()), nlr(n(), n()))), n());
        assert_eq!(Solution::min_camera_cover(root), 3);
        //          C
        //         / \
        //        0   0
        //       /
        //      0
        //    /   \
        //   C     C
        //  / \   / \
        // 0   0 0   0
    }
}
