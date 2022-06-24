#![allow(dead_code)]
//! \#968. Binary Tree Cameras
//! ==========================
//!
//! You are given the `root` of a binary tree.
//! We install cameras on the tree nodes where each camera at a node can monitor its parent,
//! itself, and its immediate children.
//!
//! Return _the minimum number of cameras needed to monitor all nodes of the tree_.
//!
//! __Constraints:__
//!
//! - The number of nodes in the tree is in the range `[1, 1000]`.
//! - `Node.val == 0`
//!
//! <https://leetcode.com/problems/binary-tree-cameras>

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn min_camera_cover(root: T) -> i32 {
        enum Monitoring {
            NotCovered,
            Covered,
            HasCamera,
        }
        use Monitoring::*;
        fn min_cameras(n: T) -> (i32, Monitoring) {
            match n {
                None => (0, Covered),
                Some(n) => {
                    let mut n = n.borrow_mut();
                    if n.left.is_none() && n.right.is_none() {
                        (0, NotCovered)
                    } else {
                        let (l_cameras, l_need) = min_cameras(n.left.take());
                        let (r_cameras, r_need) = min_cameras(n.right.take());
                        let cameras = l_cameras + r_cameras;
                        match (l_need, r_need) {
                            (NotCovered, _) | (_, NotCovered) => (cameras + 1, HasCamera),
                            (HasCamera, _) | (_, HasCamera) => (cameras, Covered),
                            _ => (cameras, NotCovered),
                        }
                    }
                }
            }
        }
        match min_cameras(root) {
            (cameras, NotCovered) => cameras + 1,
            (cameras, _) => cameras,
        }
    }
    /// 12:30‥13:05
    pub fn min_camera_cover_i32(root: T) -> i32 {
        // -> (total_cameras, monitoring_needed)
        // monitoring_needed: 0 - need a camera, 1 - no need for camera, 2 - has camera
        fn min_cameras(n: T) -> (i32, i32) {
            match n {
                None => (0, 1),
                Some(n) => {
                    let mut n = n.borrow_mut();
                    if n.left.is_none() && n.right.is_none() {
                        (0, 0)
                    } else {
                        let (l_cameras, l_need) = min_cameras(n.left.take());
                        let (r_cameras, r_need) = min_cameras(n.right.take());
                        let cameras = l_cameras + r_cameras;
                        match (l_need, r_need) {
                            (0, _) | (_, 0) => (cameras + 1, 2),
                            (2, _) | (_, 2) => (cameras, 1),
                            _ => (cameras, 0),
                        }
                    }
                }
            }
        }
        let (cameras, need_a_camera) = min_cameras(root);
        cameras + (need_a_camera == 0) as i32
    }
}

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode { pub val: i32, pub left: T, pub right: T }

#[rustfmt::skip]
impl TreeNode {
    #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
    #[inline] pub fn wrap(self) -> T { Some(Rc::new(RefCell::new(self))) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: T = None;
    #[rustfmt::skip] fn t(l: T, r: T) -> T { TreeNode { val: 0, left: l, right: r }.wrap() }
    #[rustfmt::skip] fn l(          ) -> T { TreeNode::new(0).wrap() }

    #[test]
    fn r_0() {
        assert_eq!(Solution::min_camera_cover(l()), 1);
    }
    #[test]
    fn r_0_0_0() {
        let r = t(l(), l());
        assert_eq!(Solution::min_camera_cover(r), 1);
        //  c
        // 0 0
    }
    #[test]
    fn r_0_0_n_0_0() {
        let r = t(t(l(), l()), N);
        assert_eq!(Solution::min_camera_cover(r), 1);
        // Explanation: One camera is enough to monitor all nodes if placed as shown.
        //   0
        //  c
        // 0 0
    }
    #[test]
    fn r_0_0_n_0_n_0_n_n_0() {
        let r = t(t(t(t(N, l()), N), N), N);
        assert_eq!(Solution::min_camera_cover(r), 2);
        // Explanation: At least two cameras are needed to monitor all nodes of the tree.
        //    0
        //   c
        //  0
        // c
        //  0
    }
    #[test]
    fn r_0_0_n_0_n_0_n_0_n_0_n() {
        let r = t(t(t(t(t(l(), N), N), N), N), N);
        assert_eq!(Solution::min_camera_cover(r), 2);
        //      0
        //     c
        //    0
        //   0
        //  c
        // 0
    }
    #[test]
    fn r_0_0_0_0_0_0_0() {
        let r = t(t(l(), l()), t(l(), l()));
        assert_eq!(Solution::min_camera_cover(r), 2);
        //    0
        //  c   c
        // 0 0 0 0
    }
    #[test]
    fn r_0_0_0_0_0_0_0_0_0_0_0_0_0_0_0() {
        let r = t(t(t(l(), l()), t(l(), l())), t(t(l(), l()), t(l(), l())));
        assert_eq!(Solution::min_camera_cover(r), 5);
        //        8c
        //    4       12
        //  2c  6c  10c  14c
        // 1 3 5 7 9 11 13 15
        //
        // (total_cameras, monitoring_needed)
        // monitoring_needed: 0 - need a camera, 1 - no need for camera, 2 - has camera
        //  1 -> (0, 0)
        //  3 -> (0, 0)
        //  2 -> (1, 2)
        //  5 -> (0, 0)
        //  7 -> (0, 0)
        //  6 -> (1, 2)
        //  4 -> (2, 1)
        //  9 -> (0, 0)
        // 11 -> (0, 0)
        // 10 -> (1, 2)
        // 13 -> (0, 0)
        // 15 -> (0, 0)
        // 14 -> (1, 2)
        // 12 -> (2, 1)
        //  8 -> (4, 0)
    }
}
