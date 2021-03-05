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

/// # Average of Levels in Binary Tree
///
/// Given a non-empty binary tree, return
/// _the average value of the nodes on each level in the form of an array_.
///
/// __Note:__
///
/// - The range of node's value is in the range of 32-bit signed integer.
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/588/week-1-march-1st-march-7th/3661/
struct Solution;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        if root.is_none() {
            return Vec::new()
        }
        let mut s = vec![(0, root.unwrap())];
        let mut result: Vec<(f64, f64)> = Vec::new();

        while let Some((l, n)) = s.pop() {
            if let Some((os, oc)) = result.get_mut(l) {
                *os += n.borrow().val as f64;
                *oc += 1.0;
            } else {
                result.push((n.borrow().val as f64, 1 as f64))
            }

            if let Some(n) = &n.borrow().right {
                s.push((l + 1, n.clone()))
            }
            if let Some(n) = &n.borrow().left {
                s.push((l + 1, n.clone()))
            }
        }

        result.into_iter().map(|(s, c)| s / c).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // @formatter:off
    type Node = Option<Rc<RefCell<TreeNode>>>;
    fn wrap(n: TreeNode) -> Node { Some(Rc::new(RefCell::new(n))) }
    fn n(v: i32) -> Node { wrap(TreeNode::new(v)) }
    fn nl(v: i32, l: Node) -> Node { wrap(TreeNode { val: v, left: l, right: None }) }
    fn nr(v: i32, r: Node) -> Node { wrap(TreeNode { val: v, left: None, right: r }) }
    fn nlr(v: i32, l: Node, r: Node) -> Node { wrap(TreeNode { val: v, left: l, right: r }) }
    // @formatter:on

    #[test]
    fn example() {
        let root = nlr(3, n(9), nlr(20, n(15), n(7)));
        assert_eq!(Solution::average_of_levels(root), [3.0, 14.5, 11.0]);
        // Input:
        //     3
        //    / \
        //   9  20
        //     /  \
        //    15   7
        // Explanation:
        // The average value of nodes on level 0 is 3,  on level 1 is 14.5, and on level 2 is 11.
        // Hence return [3, 14.5, 11].
    }

    #[test]
    fn test65() {
        let root = nlr(2147483647, n(2147483647), n(2147483647));
        assert_eq!(Solution::average_of_levels(root), [2147483647.0, 2147483647.0]);
    }
}
