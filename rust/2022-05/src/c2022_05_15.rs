#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 1302. Deepest Leaves Sum
/// ========================
///
/// Given the `root` of a binary tree, return _the sum of values of its deepest leaves_.
///
/// ### Constraints:
///
/// - The number of nodes in the tree is in the range `[1, 10_000]`.
/// - `1 <= Node.val <= 100`
///
/// https://leetcode.com/problems/deepest-leaves-sum/
struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut curr = vec![];
        let mut next = vec![];
        let mut non_empty = root.is_some();
        curr.push(root);
        while non_empty {
            non_empty = false;
            for n in &curr {
                if let Some(n) = n {
                    let mut n = n.borrow_mut();
                    non_empty = non_empty || n.left.is_some() || n.right.is_some();
                    next.push(n.left.take());
                    next.push(n.right.take());
                }
            }
            if non_empty {
                curr.clear();
                std::mem::swap(&mut curr, &mut next);
            }
        }

        curr.into_iter()
            .map(|n| n.map(|n| n.borrow().val).unwrap_or(0))
            .sum()
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq)]
pub struct TreeNode { pub val: i32, pub left: T, pub right: T }
impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("TreeNode\n{}", self.val))?;
        let mut curr = vec![];
        let mut next = vec![];
        if self.left.is_some() || self.right.is_some() {
            curr.push(self.right.clone());
            curr.push(self.left.clone());
        }
        while !curr.is_empty() {
            f.write_str("\n")?;
            let mut seen_some = false;
            while let Some(n) = curr.pop() {
                if let Some(n) = n {
                    let n = n.borrow();
                    f.write_fmt(format_args!("{} ", n.val))?;
                    next.push(n.left.clone());
                    next.push(n.right.clone());
                    if n.left.is_some() || n.right.is_some() {
                        seen_some = true;
                    }
                } else {
                    f.write_str("_ ")?;
                    next.push(None);
                    next.push(None);
                }
            }
            if seen_some {
                std::mem::swap(&mut curr, &mut next);
                curr.reverse();
            }
        }
        f.write_str("\n")
    }
}

#[rustfmt::skip]
impl TreeNode {
    #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
    #[inline] pub fn wrap(self) -> T { Some(Rc::new(RefCell::new(self))) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: T = None;

    #[rustfmt::skip] fn n(v: i32, l: T, r: T) -> T { TreeNode { val: v, left: l, right: r }.wrap() }
    #[rustfmt::skip] fn l(v: i32            ) -> T { TreeNode::new(v).wrap() }

    /// ```text
    ///       1
    ///      / \
    ///     2   3
    ///    / \   \
    ///   4   5   6
    ///  /         \
    /// 7           8
    /// ```
    #[test]
    fn r_12345n67nnnn8() {
        let r = n(1, n(2, n(4, l(7), N), l(5)), n(3, N, n(6, N, l(8))));
        assert_eq!(Solution::deepest_leaves_sum(r), 15);
    }
    /// ```text
    ///         6
    ///       /   \
    ///     7       8
    ///    / \     / \
    ///   2   7   1   3
    ///  /   / \       \
    /// 9   1   4       5
    /// ```
    #[rustfmt::skip] #[test]
    fn r_67827139n14nnn5() {
        let r = n(6, n(7, n(2, l(9), N), n(7, l(1), l(4))), n(8, l(1), n(3, N, l(5))));
        assert_eq!(Solution::deepest_leaves_sum(r), 19);
    }

    #[rustfmt::skip] #[test] fn r_n() { assert_eq!(Solution::deepest_leaves_sum(N), 0); }
    #[rustfmt::skip] #[test] fn r_1() { assert_eq!(Solution::deepest_leaves_sum(l(1)), 1); }

    /// ```text
    ///       38
    ///     /    \
    ///    43     70
    ///   /      /  \
    /// 16      78  91
    ///  \     /    /
    ///  71   27   71
    ///      /
    ///     71
    /// ```
    #[rustfmt::skip] #[test]
    fn test7() {
        let r = n(38, n(43, n(16, N, l(71)), N), n(70, n(78, n(27, l(71), N), N), n(91, l(71), N)));
        assert_eq!(Solution::deepest_leaves_sum(r), 71);
    }
}
