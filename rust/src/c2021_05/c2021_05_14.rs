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

/// Flatten Binary Tree to Linked List
/// ==================================
///
/// Given the `root` of a binary tree, flatten the tree into a "linked list":
///
/// - The "linked list" should use the same `TreeNode` class where
///   the `right` child pointer points to the next node in the list and
///   the `left` child pointer is always null.
/// - The "linked list" should be in the same order as a [pre-order] traversal of the binary tree.
///
/// [pre-order]:https://en.wikipedia.org/wiki/Tree_traversal#Pre-order,_NLR
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[0, 2000]`.
/// - `-100 <= Node.val <= 100`
///
/// __Follow up:__ Can you flatten the tree in-place (with `O(1)` extra space)?
///
/// https://leetcode.com/explore/featured/card/may-leetcoding-challenge-2021/599/week-2-may-8th-may-14th/3742/
struct Solution;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_some() {
            let mut curr = Rc::clone(root.as_ref().unwrap());
            while curr.borrow().right.is_some() || curr.borrow().left.is_some() {
                let right = curr.borrow_mut().right.take();
                let left = curr.borrow_mut().left.take();
                curr.borrow_mut().right = left;
                let mut rmax = Rc::clone(&curr);
                while rmax.borrow().right.is_some() {
                    let x = Rc::clone(rmax.borrow().right.as_ref().unwrap());
                    rmax = x;
                }
                rmax.borrow_mut().right = right;
                let next = Rc::clone(curr.borrow().right.as_ref().unwrap());
                curr = next;
            }
        }
    }

    pub fn flatten_my_with_vec(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_some() {
            fn to_vec_postorder(n: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
                let mut result = vec![];
                if let Some(n) = n {
                    result.push(n.borrow().val);
                    result.append(&mut to_vec_postorder(&n.borrow().left));
                    result.append(&mut to_vec_postorder(&n.borrow().right));
                }
                result
            }
            fn from_vec(xs: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
                if xs.is_empty() {
                    None
                } else {
                    Some(Rc::new(RefCell::new(TreeNode {
                        val: xs[0],
                        left: None,
                        right: from_vec(&xs[1..]),
                    })))
                }
            }
            *root = from_vec(&to_vec_postorder(&root));
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
    fn example1() {
        let mut root = nlr(1, nlr(2, n(3), n(4)), nr(5, n(6)));
        let e = nr(1, nr(2, nr(3, nr(4, nr(5, n(6))))));

        Solution::flatten(&mut root);

        assert_eq!(root, e);
    }
    #[test]
    fn example2() {
        let mut root = None;
        Solution::flatten(&mut root);
        assert_eq!(root, None);
    }
    #[test]
    fn example3() {
        let mut root = n(0);
        Solution::flatten(&mut root);
        assert_eq!(root, n(0));
    }
}
