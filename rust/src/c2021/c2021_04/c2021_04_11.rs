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

/// Deepest Leaves Sum
/// ==================
///
/// Given the `root` of a binary tree, return _the sum of values of its deepest leaves_.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[1, 10^4]`.
/// - `1 <= Node.val <= 100`
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/594/week-2-april-8th-april-14th/3704/
struct Solution;
impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() { 0 } else {
            use std::collections::VecDeque;

            let mut q = VecDeque::new();
            q.push_back(Some(root.unwrap().clone()));
            q.push_back(None);

            let mut rsf = 0;
            while let Some(maybe_n) = q.pop_front() {
                if let Some(n) = maybe_n {
                    let n = n.borrow();
                    if let Some(l) = &n.left { q.push_back(Some(l.clone())); }
                    if let Some(r) = &n.right { q.push_back(Some(r.clone())); }
                    rsf += n.val;
                } else if q.len() > 0 {
                    rsf = 0;
                    q.push_back(None)
                } else {
                    return rsf;
                }
            }
            unreachable!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Node = Option<Rc<RefCell<TreeNode>>>;
    fn wrap(n: TreeNode) -> Node { Some(Rc::new(RefCell::new(n))) }
    fn nlr(v: i32, l: Node, r: Node) -> Node { wrap(TreeNode { val: v, left: l, right: r })}
    fn nl(v: i32, l: Node) -> Node { wrap(TreeNode { val: v, left: l, right: None })}
    fn nr(v: i32, r: Node) -> Node { wrap(TreeNode { val: v, left: None, right: r })}
    fn n(v: i32) -> Node { wrap(TreeNode::new(v))}

    #[test]
    fn example1() {
        let root = nlr(1, nlr(2, nl(4, n(7)), n(5)), nr(3, nr(6, n(8))));
        assert_eq!(Solution::deepest_leaves_sum(root), 15);
        //     1
        //   2   3
        //  4 5   6
        // 7       8
    }
    #[test]
    fn example2() {
        let root = nlr(
            6,
            nlr(7, nl(2, n(9)), nlr(7, n(1), n(4))),
            nlr(8, n(1), nr(3, n(5))));
        assert_eq!(Solution::deepest_leaves_sum(root), 19)
        //       6
        //    7     8
        //  2   7  1 3
        // 9   1 4    5
    }

    #[test]
    fn test6() {
        let root = nr(50, nlr(54, n(98), nr(6, n(34))));
        assert_eq!(Solution::deepest_leaves_sum(root), 34);
    }

    #[test]
    fn r1_produces_1() {
        assert_eq!(Solution::deepest_leaves_sum(n(1)), 1);
    }
    #[test]
    fn r_empty_produces_0() {
        assert_eq!(Solution::deepest_leaves_sum(None), 0);
    }

    mod performance {
        use super::*;

        fn gen_tree(l: i32, r: i32) -> Node {
            if l > r { None } else {
                let mid = l + (r - l) / 2;
                nlr(mid, gen_tree(l, mid - 1), gen_tree(mid + 1, r))
            }
        }

        #[test]
        fn r1to40000_produces_144_719_364() {
            let root = gen_tree(1, 40000);
            assert_eq!(Solution::deepest_leaves_sum(root), 144_719_364);
        }
    }
}
