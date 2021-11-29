#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 333. Largest BST Subtree
/// ========================
///
/// Given the root of a binary tree,
/// find the largest subtree,
/// which is also a Binary Search Tree (BST),
/// where the largest means subtree has the largest number of nodes.
///
/// __A Binary Search Tree (BST)__ is a tree in which all the nodes follow the below-mentioned properties:
///
/// - The left subtree values are less than the value of their parent (root) node's value.
/// - The right subtree values are greater than the value of their parent (root) node's value.
///
/// __Note:__ A subtree must include all of its descendants.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[0, 10_000]`.
/// - `-10_000 <= Node.val <= 10_000`
///
/// __Follow up:__ Can you figure out ways to solve it with `O(n)` time complexity?
///
/// https://leetcode.com/problems/largest-bst-subtree/
struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn largest_bst_subtree_my_v1(root: T) -> i32 {
        println!("largest_bst_subtree({:?})", root);
        /// return: (size, Option<(min, max)>)
        fn is_bst(t: T) -> (i32, Option<(i32, i32)>) {
            t.map(|t| {
                let tb = t.borrow();
                let v = tb.val;
                match (is_bst(tb.left.clone()), is_bst(tb.right.clone())) {
                    ((0, _), (0, _)) => (1, Some((v, v))),
                    ((ls, Some((min, l))), (0, _)) if l < v => (ls + 1, Some((min, v))),
                    ((0, _), (rs, Some((r, max)))) if v < r => (rs + 1, Some((v, max))),
                    ((ls, Some((min, l))), (rs, Some((r, max)))) if l < v && v < r => {
                        (ls + rs + 1, Some((min, max)))
                    }
                    ((l, _), (r, _)) => (l.max(r), None),
                }
            })
            .unwrap_or((0, None))
        }
        is_bst(root.clone()).0
    }
    /// Approach 3: Post-Order Traversal
    /// https://leetcode.com/problems/largest-bst-subtree/solution/
    pub fn largest_bst_subtree(root: T) -> i32 {
        println!("largest_bst_subtree({:?})", root);
        /// return: (size, min, max)
        fn is_bst(t: T) -> (i32, i32, i32) {
            t.map(|t| {
                let tb = t.borrow();
                let v = tb.val;
                let (l, l_min, l_max) = is_bst(tb.left.clone());
                let (r, r_min, r_max) = is_bst(tb.right.clone());
                if l_max < v && v < r_min {
                    (l + r + 1, l_min.min(v), r_max.max(v))
                } else {
                    (l.max(r), i32::MIN, i32::MAX)
                }
            })
            .unwrap_or((0, i32::MAX, i32::MIN))
        }
        is_bst(root.clone()).0
    }
}

#[rustfmt::skip] #[derive(Debug, PartialEq, Eq)] pub struct TreeNode { pub val: i32, pub left: T, pub right: T }
#[rustfmt::skip] impl TreeNode { #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } } }

#[cfg(test)]
mod tests {
    use super::*;

    const N: T = None;

    #[rustfmt::skip] fn wrap(t: TreeNode) -> T { Some(Rc::new(RefCell::new(t))) }
    #[rustfmt::skip] fn t(v: i32, l: T, r: T) -> T { wrap(TreeNode { val: v, left: l, right: r }) }
    #[rustfmt::skip] fn l(v: i32            ) -> T { wrap(TreeNode::new(v)) }

    #[rustfmt::skip] #[test] fn r_n() { assert_eq!(Solution::largest_bst_subtree(None), 0); }
    #[rustfmt::skip] #[test] fn r_1() { assert_eq!(Solution::largest_bst_subtree(l(1)), 1); }

    #[rustfmt::skip] #[test] fn r_1_2_3() { assert_eq!(Solution::largest_bst_subtree(t(1,l(2),l(3))), 1); }
    #[rustfmt::skip] #[test] fn r_1_3_2() { assert_eq!(Solution::largest_bst_subtree(t(1,l(3),l(2))), 1); }
    #[rustfmt::skip] #[test] fn r_2_3_1() { assert_eq!(Solution::largest_bst_subtree(t(2,l(3),l(1))), 1); }
    #[rustfmt::skip] #[test] fn r_3_1_3() { assert_eq!(Solution::largest_bst_subtree(t(3,l(1),l(3))), 1); }
    #[rustfmt::skip] #[test] fn r_3_3_1() { assert_eq!(Solution::largest_bst_subtree(t(3,l(3),l(1))), 1); }

    #[rustfmt::skip] #[test] fn r_2_1_3() { assert_eq!(Solution::largest_bst_subtree(t(2,l(1),l(3))), 3); }

    #[test]
    fn r_10_5_15_1_8_n_7() {
        let r = t(10, t(5, l(1), l(8)), t(15, N, l(7)));
        assert_eq!(Solution::largest_bst_subtree(r), 3);
        //     10
        //    /  \
        //   5   15
        //  / \    \
        // 1   8    7
        // Explanation: The Largest BST Subtree in this case is [5,1,8].
        // The return value is the subtree's size, which is 3.
    }
    #[test]
    fn r_4_2_7_2_3_5_n_2_n_n_n_n_n_1() {
        let r = t(4, t(2, t(2, t(2, l(1), N), N), l(3)), t(7, l(5), N));
        assert_eq!(Solution::largest_bst_subtree(r), 2);
        //          4
        //        /   \
        //       2     7
        //      / \   /
        //     2   3 5
        //    /
        //   2
        //  /
        // 1
        // Explanation: [7,5] or [2,1]
    }
}
