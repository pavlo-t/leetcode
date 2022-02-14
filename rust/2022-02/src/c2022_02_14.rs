#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 104. Maximum Depth of Binary Tree
/// =================================
///
/// Given the `root` of a binary tree, return _its maximum depth_.
///
/// A binary tree's __maximum depth__ is the number of nodes along the longest path
/// from the root node down to the farthest leaf node.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[0, 10_000]`.
/// - `-100 <= Node.val <= 100`
///
/// https://leetcode.com/problems/maximum-depth-of-binary-tree/
struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn max_depth(root: T) -> i32 {
        println!("max_depth({root:?})");
        let mut result = 0;
        let mut curr = vec![root];
        let mut next = vec![];
        while !curr.is_empty() {
            let mut seen_non_empty_node = false;
            while let Some(n) = curr.pop() {
                if let Some(n) = n {
                    seen_non_empty_node = true;
                    let mut n = n.borrow_mut();
                    next.push(n.left.take());
                    next.push(n.right.take());
                }
            }
            if seen_non_empty_node {
                result += 1;
            }
            std::mem::swap(&mut curr, &mut next);
        }
        result
    }
}

#[rustfmt::skip] #[derive(Debug, PartialEq, Eq)] pub struct TreeNode { pub val: i32, pub left: T, pub right: T }
#[rustfmt::skip] impl TreeNode {
    #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
    #[inline] fn wrap(self) -> T { Some(Rc::new(RefCell::new(self))) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: T = None;

    #[rustfmt::skip] fn t(val: i32, l: T, r: T) -> T { TreeNode { val, left: l, right: r }.wrap() }
    #[rustfmt::skip] fn l(val: i32            ) -> T { TreeNode::new(val).wrap() }

    #[test]
    fn r_n() {
        assert_eq!(Solution::max_depth(N), 0);
    }
    #[test]
    fn r_1() {
        assert_eq!(Solution::max_depth(l(1)), 1);
    }
    #[test]
    fn r_1_n_2() {
        assert_eq!(Solution::max_depth(t(1, N, l(2))), 2);
    }
    #[test]
    fn r_3_9_20_n_n_15_7() {
        let r = t(3, l(9), t(20, l(15), l(7)));
        assert_eq!(Solution::max_depth(r), 3);
    }

    /// Got stack overflow with default stack
    #[test]
    fn r_10000_nodes_in_left() {
        let child = std::thread::Builder::new()
            .stack_size(2usize.pow(31))
            .spawn(|| {
                let mut r = None;
                for _ in 0..10000 {
                    r = t(1, r, N);
                }
                assert_eq!(Solution::max_depth(r), 10000);
            })
            .unwrap();
        child.join().unwrap();
    }
}
