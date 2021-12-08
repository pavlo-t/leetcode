#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 563. Binary Tree Tilt
/// =====================
///
/// Given the `root` of a binary tree, return _the sum of every tree node's __tilt___.
///
/// The __tilt__ of a tree node is the __absolute difference__ between the sum of all left subtree node __values__
/// and all right subtree node __values__.
/// If a node does not have a left child, then the sum of the left subtree node __values__ is treated as `0`.
/// The rule is similar if there the node does not have a right child.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[0, 10_000]`.
/// - `-1000 <= Node.val <= 1000`
///
/// https://leetcode.com/problems/binary-tree-tilt/
struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn find_tilt(root: T) -> i32 {
        println!("find_tilt({:?})", root);
        fn rec(n: T) -> (i32, i32) {
            n.map(|n| {
                let b = n.borrow();
                let (lt, ls) = rec(b.left.clone());
                let (rt, rs) = rec(b.right.clone());
                ((ls - rs).abs() + lt + rt, ls + rs + b.val)
            })
            .unwrap_or((0, 0))
        }
        rec(root).0
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq)]
pub struct TreeNode { pub val: i32, pub left: T, pub right: T }
impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::collections::VecDeque;
        let mut q = VecDeque::new();
        let mut results = vec![self.val.to_string()];
        if self.left.is_some() || self.right.is_some() {
            q.push_back(self.left.clone());
            q.push_back(self.right.clone());
        }
        while let Some(n) = q.pop_front() {
            if let Some(n) = n {
                let b = n.borrow();
                results.push(b.val.to_string());
                if b.left.is_some() || b.right.is_some() {
                    q.push_back(b.left.clone());
                    q.push_back(b.right.clone());
                }
            } else {
                results.push("n".to_string());
            }
        }
        f.debug_list().entries(results.iter()).finish()
    }
}
#[rustfmt::skip]
impl TreeNode {
    #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
    fn wrap(self) -> T { Some(Rc::new(RefCell::new(self))) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: T = None;
    #[rustfmt::skip] fn n(v: i32, l: T, r: T) -> T { TreeNode { val: v, left: l, right: r }.wrap() }
    #[rustfmt::skip] fn l(v: i32            ) -> T { TreeNode::new(v).wrap() }

    #[test]
    fn r_123() {
        assert_eq!(Solution::find_tilt(n(1, l(2), l(3))), 1);
        //   1      1
        //  / \ => / \
        // 2   3  0   0
        // Explanation:
        // Tilt of node 2 : |0-0| = 0 (no children)
        // Tilt of node 3 : |0-0| = 0 (no children)
        // Tilt of node 1 : |2-3| = 1 (left subtree is just left child, so sum is 2;
        //                             right subtree is just right child, so sum is 3)
        // Sum of every tilt : 0 + 0 + 1 = 1
    }
    #[test]
    fn r_42935n7() {
        let r = n(4, n(2, l(3), l(5)), n(9, N, l(7)));
        assert_eq!(Solution::find_tilt(r), 15);
        //     4          6
        //    / \        / \
        //   2   9  =>  2   7
        //  / \   \    / \   \
        // 3   5   7  0   0   0
        // Explanation:
        // Tilt of node 3 : |0-0| = 0 (no children)
        // Tilt of node 5 : |0-0| = 0 (no children)
        // Tilt of node 7 : |0-0| = 0 (no children)
        // Tilt of node 2 : |3-5| = 2 (left subtree is just left child, so sum is 3;
        //                             right subtree is just right child, so sum is 5)
        // Tilt of node 9 : |0-7| = 7 (no left child, so sum is 0; right subtree is just right child, so sum is 7)
        // Tilt of node 4 : |(3+5+2)-(9+7)| = |10-16| = 6 (left subtree values are 3, 5, and 2, which sums to 10;
        //                                                 right subtree values are 9 and 7, which sums to 16)
        // Sum of every tilt : 0 + 0 + 0 + 2 + 7 + 6 = 15
    }
    #[test]
    fn r_21_7_14_1_1_2_2_3_3() {
        let r = n(21, n(7, n(1, l(3), l(3)), l(1)), n(14, l(2), l(2)));
        assert_eq!(Solution::find_tilt(r), 9);
        //       21              3
        //      /  \           /   \
        //     7    14        6     0
        //    / \   / \ =>   / \   / \
        //   1   1 2   2    0   0 0   0
        //  / \            / \
        // 3   3          0   0
    }

    #[test]
    fn r_16383_nodes_in_a_balanced_tree() {
        fn build_tree(depth: i32) -> T {
            if depth <= 0 {
                N
            } else {
                n(depth, build_tree(depth - 1), build_tree(depth - 1))
            }
        }
        let child = std::thread::Builder::new()
            .stack_size(2usize.pow(27))
            .spawn(move || assert_eq!(Solution::find_tilt(build_tree(14)), 0))
            .unwrap();
        child.join().unwrap();
    }
    #[test]
    fn r_10000_nodes_in_left_tree() {
        fn build_tree(mut depth: i32) -> T {
            let mut root = N;
            while depth > 0 {
                root = n(1, root, N);
                depth -= 1;
            }
            root
        }
        let child = std::thread::Builder::new()
            .stack_size(2usize.pow(27))
            .spawn(move || assert_eq!(Solution::find_tilt(build_tree(10000)), 49_995_000))
            .unwrap();
        child.join().unwrap();
    }
    #[test]
    fn r_10000_nodes_in_right_tree() {
        fn build_tree(mut depth: i32) -> T {
            let mut root = N;
            while depth > 0 {
                root = n(1, N, root);
                depth -= 1;
            }
            root
        }
        let child = std::thread::Builder::new()
            .stack_size(2usize.pow(27))
            .spawn(move || assert_eq!(Solution::find_tilt(build_tree(10000)), 49_995_000))
            .unwrap();
        child.join().unwrap();
    }
}
