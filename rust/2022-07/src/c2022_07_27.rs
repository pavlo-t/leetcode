#![allow(dead_code)]
//! \#114. Flatten Binary Tree to Linked List
//! =========================================
//!
//! <https://leetcode.com/problems/flatten-binary-tree-to-linked-list>
//!
//! Given the `root` of a binary tree, flatten the tree into a "linked list":
//!
//! - The "linked list" should use the same `TreeNode` class
//!   where the `right` child pointer points to the next node in the list
//!   and the `left` child pointer is always `null`.
//! - The "linked list" should be in the same order as a [pre-order traversal] of the binary tree.
//!
//! __Constraints:__
//!
//! - The number of nodes in the tree is in the range `[0, 2000]`.
//! - `-100 <= Node.val <= 100`
//!
//! __Follow up:__ Can you flatten the tree in-place (with `O(1)` extra space)?
//!
//! [pre-order traversal]:https://en.wikipedia.org/wiki/Tree_traversal#Pre-order,_NL

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

type T = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    /// Using stacks
    ///
    /// - time: `O(n)`
    /// - memory: `O(n)`
    pub fn flatten_v1(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut preorder = vec![];
        let mut stack = vec![root.take()];
        while let Some(Some(n)) = stack.pop() {
            let mut n = n.borrow_mut();
            preorder.push(n.val);
            if n.right.is_some() {
                stack.push(n.right.take());
            }
            if n.left.is_some() {
                stack.push(n.left.take());
            }
        }

        let mut right = None;
        while let Some(val) = preorder.pop() {
            let left = None;
            right = Some(Rc::new(RefCell::new(TreeNode { val, left, right })));
        }

        if let Some(head) = right {
            root.replace(head);
        }
    }

    /// In place
    ///
    /// - time: `O(n)`
    /// - memory: `O(1)`
    pub fn flatten(root: &mut T) {
        fn append(root: T, tail: T) -> T {
            if root.is_some() {
                let mut curr = root.clone().unwrap();
                while curr.borrow().right.is_some() {
                    let tmp = curr.borrow().right.clone().unwrap();
                    curr = tmp;
                }
                curr.borrow_mut().right = tail;

                root
            } else {
                tail
            }
        }

        if root.is_some() {
            let mut curr = root.clone();
            while let Some(n) = curr {
                if n.borrow().left.is_some() {
                    let mut n = n.borrow_mut();
                    n.right = append(n.left.take(), n.right.take());
                }
                curr = n.borrow().right.clone();
            }
        }
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq)]
pub struct TreeNode { pub val: i32, pub left: T, pub right: T }

#[rustfmt::skip]
impl TreeNode {
    #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
}

impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const START: char = '[';
        const SEPARATOR: char = ',';
        const LEVEL: char = '|';
        const EMPTY: char = '_';
        const END: char = ']';

        let mut result = String::new();
        result.push(START);
        result.push_str(&self.val.to_string());
        let mut curr = vec![];
        let mut next = vec![];
        if self.left.is_some() || self.right.is_some() {
            result.push(LEVEL);
            curr.push(self.left.clone());
            curr.push(self.right.clone());
        }
        while !curr.is_empty() {
            let mut has_next = false;
            for i in 0..curr.len() {
                if let Some(t) = &curr[i] {
                    let t = t.borrow();
                    result.push_str(&t.val.to_string());
                    next.push(t.left.clone());
                    next.push(t.right.clone());
                    if t.left.is_some() || t.right.is_some() {
                        has_next = true;
                    }
                } else {
                    result.push(EMPTY);
                    next.push(None);
                    next.push(None);
                }
                result.push(SEPARATOR);
            }
            curr.clear();
            result.pop();
            if has_next {
                result.push(LEVEL);
                std::mem::swap(&mut curr, &mut next);
            }
        }
        result.push(END);
        f.write_str(&result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: T = None;
    #[rustfmt::skip] fn l(val: i32)                    -> T { wrap(TreeNode::new(val)) }
    #[rustfmt::skip] fn t(val: i32, left: T, right: T) -> T { wrap(TreeNode { val, left, right }) }
    #[rustfmt::skip] fn r(val: i32,          right: T) -> T { wrap(TreeNode { val, left: N, right }) }

    #[rustfmt::skip] fn wrap(t: TreeNode) -> T { Some(Rc::new(RefCell::new(t))) }

    #[test]
    fn empty() {
        let mut root = N;
        let expected = N;
        Solution::flatten(&mut root);
        assert_eq!(root, expected);
    }
    #[test]
    fn r_0() {
        let mut root = l(0);
        let expected = l(0);
        Solution::flatten(&mut root);
        assert_eq!(root, expected);
    }
    #[test]
    fn r_1_2_5_3_4_n_6() {
        //    1
        //  2   5
        // 3 4   6
        let mut root = t(1, t(2, l(3), l(4)), t(5, N, l(6)));
        let expected = r(1, r(2, r(3, r(4, r(5, l(6))))));
        Solution::flatten(&mut root);
        assert_eq!(root, expected);
    }
    #[test]
    fn r_1_2_5_3_4_6() {
        //    1
        //  2   5
        // 3 4 6
        let mut root = t(1, t(2, l(3), l(4)), t(5, l(6), N));
        let expected = r(1, r(2, r(3, r(4, r(5, l(6))))));
        Solution::flatten(&mut root);
        assert_eq!(root, expected);
    }
    #[test]
    fn r_1_2_5_3_4_6_7() {
        //    1
        //  2   5
        // 3 4 6 7
        let mut root = t(1, t(2, l(3), l(4)), t(5, l(6), l(7)));
        let expected = r(1, r(2, r(3, r(4, r(5, r(6, l(7)))))));
        Solution::flatten(&mut root);
        assert_eq!(root, expected);
    }
}
