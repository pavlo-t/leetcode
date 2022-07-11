#![allow(dead_code)]
//! \#199. Binary Tree Right Side View
//! ==================================
//!
//! Given the `root` of a binary tree, imagine yourself standing on the __right side__ of it,
//! return _the values of the nodes you can see ordered from top to bottom_.
//!
//! __Constraints:__
//!
//! - The number of nodes in the tree is in the range `[0, 100]`.
//! - `-100 <= Node.val <= 100`
//!
//! <https://leetcode.com/problems/binary-tree-right-side-view>

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut curr = VecDeque::new();
        let mut next = VecDeque::new();
        let mut result = vec![];
        if root.is_some() {
            curr.push_back(root.clone().unwrap());
        }
        while let Some(last) = curr.back() {
            result.push(last.borrow().val);
            while let Some(node) = curr.pop_front() {
                let borrowed = node.borrow();
                for child in [borrowed.left.clone(), borrowed.right.clone()] {
                    if let Some(node) = child {
                        next.push_back(node);
                    }
                }
            }
            std::mem::swap(&mut curr, &mut next);
        }
        result
    }
}

type T = Option<Rc<RefCell<TreeNode>>>;

#[rustfmt::skip]
#[derive(PartialEq, Eq)]
pub struct TreeNode { pub val: i32, pub left: T, pub right: T }
impl TreeNode {
    #[rustfmt::skip] #[inline]    pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
    #[rustfmt::skip] #[inline]    pub fn wrap(self) -> T { Some(Rc::new(RefCell::new(self))) }
}
impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result: String = String::new();
        result.push('\n');
        result.push_str(&self.val.to_string());
        let mut curr = vec![];
        let mut next = vec![];
        if self.left.is_some() || self.right.is_some() {
            result.push('\n');
            curr.push(self.left.clone());
            curr.push(self.right.clone());
        }
        while !curr.is_empty() {
            let mut seen_some = false;
            curr.reverse();
            let mut curr_row = String::new();
            while let Some(t) = curr.pop() {
                if let Some(n) = t {
                    seen_some = true;
                    let n = n.borrow();
                    let l = n.left.clone();
                    let r = n.right.clone();
                    curr_row.push_str(&n.val.to_string());
                    curr_row.push(',');
                    next.push(l);
                    next.push(r);
                } else {
                    curr_row.push_str("_,");
                    next.push(None);
                    next.push(None);
                }
            }
            curr_row.pop();
            if seen_some {
                result.push_str(&curr_row);
                result.push('\n');
                std::mem::swap(&mut curr, &mut next);
            }
        }
        f.write_str(&result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: T = None;
    #[rustfmt::skip] fn n(v: i32, l: T, r: T) -> T { TreeNode { val: v, left: l, right: r }.wrap() }
    #[rustfmt::skip] fn l(v: i32            ) -> T { TreeNode::new(v).wrap() }

    #[test]
    fn r_n() {
        assert_eq!(Solution::right_side_view(N), []);
    }
    #[test]
    fn r_1_n_3() {
        // 1
        //  3
        let r = n(1, N, l(3));
        assert_eq!(Solution::right_side_view(r), [1, 3]);
    }
    #[test]
    fn r_1_2() {
        //  1
        // 2
        let r = n(1, l(2), N);
        assert_eq!(Solution::right_side_view(r), [1, 2]);
    }
    #[test]
    fn r_1_2_3_n_5_n_4() {
        //   1
        // 2   3
        //  5   4
        let r = n(1, n(2, N, l(5)), n(3, N, l(4)));
        assert_eq!(Solution::right_side_view(r), [1, 3, 4]);
    }

    #[test]
    fn r_4_2_6_1_3_5_7() {
        //    4
        //  2   6
        // 1 3 5 7
        let r = n(4, n(2, l(1), l(3)), n(6, l(5), l(7)));
        assert_eq!(Solution::right_side_view(r), [4, 6, 7]);
    }
    #[test]
    fn r_4_2_6_1_3_5() {
        //    4
        //  2   6
        // 1 3 5
        let r = n(4, n(2, l(1), l(3)), n(6, l(5), N));
        assert_eq!(Solution::right_side_view(r), [4, 6, 5]);
    }
    #[test]
    fn r_4_2_6_1_3() {
        //    4
        //  2   6
        // 1 3
        let r = n(4, n(2, l(1), l(3)), l(6));
        assert_eq!(Solution::right_side_view(r), [4, 6, 3]);
    }
    #[test]
    fn r_4_2_6_1() {
        //    4
        //  2   6
        // 1
        let r = n(4, n(2, l(1), N), l(6));
        assert_eq!(Solution::right_side_view(r), [4, 6, 1]);
    }
}
