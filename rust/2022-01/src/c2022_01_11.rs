#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 1022. Sum of Root To Leaf Binary Numbers
/// ========================================
///
/// You are given the `root` of a binary tree where each node has a value `0` or `1`.
/// Each root-to-leaf path represents a binary number starting with the most significant bit.
///
/// For example, if the path is `0 -> 1 -> 1 -> 0 -> 1`, then this could represent `01101` in binary, which is `13`.
///
/// For all leaves in the tree, consider the numbers represented by the path from the root to that leaf.
/// Return _the sum of these numbers_.
///
/// The test cases are generated so that the answer fits in a __32-bits__ integer.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[1, 1000]`.
/// - `Node.val` is `0` or `1`.
///
/// https://leetcode.com/problems/sum-of-root-to-leaf-binary-numbers/
struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn sum_root_to_leaf_my_v1_parse_string(root: T) -> i32 {
        println!("root: {:?}", root);
        fn bts(root: T, path: &mut String) -> i32 {
            if let Some(n) = root {
                let b = n.borrow();
                path.push(char::from_digit(b.val as u32, 2).unwrap());
                let result = if b.left.is_none() && b.right.is_none() {
                    i32::from_str_radix(path, 2).unwrap()
                } else {
                    bts(b.left.clone(), path) + bts(b.right.clone(), path)
                };
                path.pop();
                result
            } else {
                0
            }
        }

        bts(root, &mut String::new())
    }

    pub fn sum_root_to_leaf(root: T) -> i32 {
        println!("root: {:?}", root);
        fn rec(root: T, mut rsf: i32) -> i32 {
            if let Some(n) = root {
                let b = n.borrow();
                rsf <<= 1;
                rsf |= b.val;
                if b.left.is_none() && b.right.is_none() {
                    rsf
                } else {
                    rec(b.left.clone(), rsf) + rec(b.right.clone(), rsf)
                }
            } else {
                0
            }
        }

        rec(root, 0)
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq)]
pub struct TreeNode { pub val: i32, pub left: T, pub right: T }
#[rustfmt::skip]
impl TreeNode {
    #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
    #[inline] pub fn wrap(self) -> T { Some(Rc::new(RefCell::new(self))) }
}
impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const LEVEL: char = '_';
        const NULL: char = 'n';
        let mut data = vec![char::from_digit(self.val as u32, 2).unwrap()];
        let mut has_more = self.left.is_some() | self.right.is_some();
        let mut curr = vec![];
        let mut next = vec![];
        if has_more {
            data.push(LEVEL);
            curr.push(self.right.clone());
            curr.push(self.left.clone());
        }
        while !curr.is_empty() {
            has_more = false;
            while let Some(n) = curr.pop() {
                if let Some(n) = n {
                    let b = n.borrow();
                    data.push(char::from_digit(b.val as u32, 2).unwrap());
                    next.push(b.left.clone());
                    next.push(b.right.clone());
                    has_more |= b.left.is_some() | b.right.is_some();
                } else {
                    data.push(NULL);
                    next.push(None);
                    next.push(None);
                }
            }
            if has_more {
                data.push(LEVEL);
                std::mem::swap(&mut curr, &mut next);
                curr.reverse();
                next.clear();
            } else {
                break;
            }
        }
        f.debug_list().entries(data.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: T = None;

    #[rustfmt::skip] fn t(v: i32, l: T, r: T) -> T { TreeNode { val: v, left: l, right: r }.wrap() }
    #[rustfmt::skip] fn l(v: i32            ) -> T { TreeNode::new(v).wrap() }

    #[rustfmt::skip] #[test] fn r0() { assert_eq!(Solution::sum_root_to_leaf(l(0)), 0); }
    #[rustfmt::skip] #[test] fn r1() { assert_eq!(Solution::sum_root_to_leaf(l(1)), 1); }

    #[rustfmt::skip] #[test] fn r000() { assert_eq!(Solution::sum_root_to_leaf(t(0, l(0), l(0))), 0); }
    #[rustfmt::skip] #[test] fn r01n() { assert_eq!(Solution::sum_root_to_leaf(t(0, l(1), N)), 1); }
    #[rustfmt::skip] #[test] fn r0n1() { assert_eq!(Solution::sum_root_to_leaf(t(0, N, l(1))), 1); }
    #[rustfmt::skip] #[test] fn r111() { assert_eq!(Solution::sum_root_to_leaf(t(1, l(1), l(1))), 6); }
    #[test]
    fn r1010101() {
        let r = t(1, t(0, l(0), l(1)), t(1, l(0), l(1)));
        assert_eq!(Solution::sum_root_to_leaf(r), 22);
        //    1
        //   / \
        //  0   1
        // / \ / \
        // 0 1 0 1
        // Explanation: (100) + (101) + (110) + (111) = 4 + 5 + 6 + 7 = 22
    }
    #[test]
    fn r101010() {
        let r = t(1, t(0, l(0), l(1)), t(1, l(0), N));
        assert_eq!(Solution::sum_root_to_leaf(r), 15);
        //    1
        //   / \
        //  0   1
        // / \ /
        // 0 1 0
        // Explanation: (100) + (101) + (110) = 4 + 5 + 6 = 15
    }
    #[test]
    fn r1010111() {
        let r = t(1, t(0, l(0), l(1)), t(1, l(1), l(1)));
        assert_eq!(Solution::sum_root_to_leaf(r), 23);
        //    1
        //   / \
        //  0   1
        // / \ / \
        // 0 1 1 1
        // Explanation: (100) + (101) + (111) + (111) = 4 + 5 + 7 + 7 = 23
    }
}
