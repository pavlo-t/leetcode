#![allow(dead_code)]

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
use std::cell::RefCell;
use std::rc::Rc;

/// Binary Tree Longest Consecutive Sequence II
/// ===========================================
///
/// Given the `root` of a binary tree, return _the length of the longest consecutive path in the tree_.
///
/// A consecutive path is a path where the values of the consecutive nodes in the path differ by one.
/// This path can be either increasing or decreasing.
///
/// - For example, `[1,2,3,4]` and `[4,3,2,1]` are both considered valid, but the path `[1,2,4,3]` is not valid.
///
/// On the other hand, the path can be in the child-Parent-child order, where not necessarily be parent-child order.
///
/// Constraints:
///
/// - The number of nodes in the tree is in the range `[1, 30_000]`.
/// - `-30_000 <= Node.val <= 30_000`
///
/// https://leetcode.com/explore/challenge/card/july-leetcoding-challenge-2021/612/week-5-july-29th-july-31st/3830/
struct Solution;
impl Solution {
    /// Approach #2 Single traversal
    ///
    /// https://leetcode.com/problems/binary-tree-longest-consecutive-sequence-ii/solution/
    ///
    /// https://rustgym.com/leetcode/549
    pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        type Node = Option<Rc<RefCell<TreeNode>>>;

        fn longest_path(n: &Node, max: &mut i32) -> Option<(i32, i32, i32)> {
            if let Some(n) = n.as_ref() {
                let nb = n.borrow();
                let (mut inc, mut dec, val) = (1, 1, nb.val);
                let mut inc_dec = |r: Option<(i32, i32, i32)>| if let Some((v, i, d)) = r {
                    if v == val + 1 {
                        inc = inc.max(i + 1);
                    } else if v == val - 1 {
                        dec = dec.max(d + 1);
                    }
                };
                inc_dec(longest_path(&nb.left, max));
                inc_dec(longest_path(&nb.right, max));
                *max = (*max).max(inc + dec - 1);

                Some((nb.val, inc, dec))
            } else {
                None
            }
        }
        let mut result = 0;
        longest_path(&root, &mut result);
        result
    }

    pub fn longest_consecutive_my(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        type Node = Option<Rc<RefCell<TreeNode>>>;

        fn lc_tree(n: &Node) -> (i32, i32) {
            if let Some(n) = n.as_ref() {
                let nb = n.borrow();
                let lu = lc_branch(&nb.left, nb.val, true);
                let ld = lc_branch(&nb.left, nb.val, false);
                let ru = lc_branch(&nb.right, nb.val, true);
                let rd = lc_branch(&nb.right, nb.val, false);
                let lt = lc_tree(&nb.left).1;
                let rt = lc_tree(&nb.right).1;
                let max_branch = lu.max(ld).max(ru).max(rd);
                let max_through = (lu + rd + 1).max(ld + ru + 1).max(lt).max(rt);
                (max_branch, max_through)
            } else {
                (0, 0)
            }
        }
        fn lc_branch(n: &Node, pv: i32, up: bool) -> i32 {
            if let Some(n) = n.as_ref() {
                let nb = n.borrow();
                if (up && nb.val == pv + 1) || (!up && nb.val == pv - 1) {
                    lc_branch(&nb.left, nb.val, up).max(lc_branch(&nb.right, nb.val, up)) + 1
                } else {
                    0
                }
            } else {
                0
            }
        }

        lc_tree(&root).1
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
        wrap(TreeNode {
            val: v,
            left: None,
            right: None,
        })
    }

    #[test]
    fn r_1_2_3_produces_2() {
        let root = nlr(1, n(2), n(3));
        assert_eq!(Solution::longest_consecutive(root), 2);
        //   1
        //  / \
        // 2   3
    }
    #[test]
    fn r_2_1_3_produces_3() {
        let root = nlr(2, n(1), n(3));
        assert_eq!(Solution::longest_consecutive(root), 3);
        //   2
        //  / \
        // 1   3
    }
    #[test]
    fn r_1_3_1_2_4_produces_3() {
        let root = nlr(1, nlr(3, n(2), n(4)), n(1));
        assert_eq!(Solution::longest_consecutive(root), 3);
        //     1
        //    / \
        //   3   1
        //  / \
        // 2   4
    }

    mod performance {
        use super::*;

        /// If getting stack overflow:
        ///
        /// ```sh
        /// thread 'c2020::c2020_12::d2020_12_01::tests::max_size' has overflowed its stack
        /// fatal runtime error: stack overflow
        /// ```
        ///
        /// Add `RUST_MIN_STACK=67108864` to env
        #[test]
        fn r_1to30000in_left_produces_30000() {
            let mut root = n(1);
            for i in 2..=30000 {
                root = nl(i, root);
            }
            assert_eq!(Solution::longest_consecutive(root), 30_000);
        }
    }
}
