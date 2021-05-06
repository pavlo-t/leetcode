#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

/// Definition for singly-linked list.
#[rustfmt::skip]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }
#[rustfmt::skip]
impl ListNode {
    #[inline] fn new(val: i32) -> Self { ListNode { next: None, val } }
}

/// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
#[rustfmt::skip]
impl TreeNode {
    #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
}

/// Convert Sorted List to Binary Search Tree
/// =========================================
///
/// Given the `head` of a singly linked list where elements are __sorted in ascending order__,
/// convert it to a height balanced BST.
///
/// For this problem, a height-balanced binary tree is defined as a binary tree in which the depth
/// of the two subtrees of _every_ node never differ by more than 1.
///
/// __Constraints:__
///
/// - The number of nodes in `head` is in the range `[0, 20_000]`.
/// - `-100_000 <= Node.val <= 100_000`
///
/// https://leetcode.com/explore/featured/card/may-leetcoding-challenge-2021/598/week-1-may-1st-may-7th/3733/
struct Solution;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
            let mut result = Vec::new();
            let mut curr = head;
            while let Some(n) = curr {
                result.push(n.val);
                curr = n.next;
            }
            result
        }
        fn wrap_tree_node(tn: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
            Some(Rc::new(RefCell::new(tn)))
        }
        fn slice_to_bt(xs: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            match xs.len() {
                0 => None,
                1 => wrap_tree_node(TreeNode::new(xs[0])),
                len => {
                    let m = len / 2;
                    wrap_tree_node(TreeNode {
                        val: xs[m],
                        left: slice_to_bt(&xs[..m]),
                        right: slice_to_bt(&xs[m + 1..]),
                    })
                }
            }
        }

        slice_to_bt(&list_to_vec(head))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type List = Option<Box<ListNode>>;
    type Tree = Option<Rc<RefCell<TreeNode>>>;

    #[rustfmt::skip] fn l(v: i32, n: List) -> List { Some(Box::new(ListNode { val: v, next: n })) }

    #[rustfmt::skip] fn wrap_tree(t: TreeNode) -> Tree { Some(Rc::new(RefCell::new(t))) }
    #[rustfmt::skip] fn tlr(v: i32, l: Tree, r: Tree) -> Tree { wrap_tree(TreeNode { val: v, left: l, right: r }) }
    #[rustfmt::skip] fn tl(v: i32, l: Tree) -> Tree { wrap_tree(TreeNode { val: v, left: l, right: None }) }
    #[rustfmt::skip] fn tr(v: i32, r: Tree) -> Tree { wrap_tree(TreeNode { val: v, left: None, right: r }) }
    #[rustfmt::skip] fn t(v: i32) -> Tree { wrap_tree(TreeNode { val: v, left: None, right: None }) }

    #[test]
    fn example1_h_m10_m3_0_5_9() {
        let head = l(-10, l(-3, l(0, l(5, l(9, None)))));
        let e = tlr(0, tl(-3, t(-10)), tl(9, t(5)));
        assert_eq!(Solution::sorted_list_to_bst(head), e);
        // Explanation:
        // One possible answer is [0,-3,9,-10,null,5], which represents the shown height balanced BST.
        //      0
        //     / \
        //   -3   9
        //   /   /
        // -10  5
    }
    #[test]
    fn example2_h_empty_produces_empty() {
        assert_eq!(Solution::sorted_list_to_bst(None), None);
    }
    #[test]
    fn example3_h0_produces_0() {
        assert_eq!(Solution::sorted_list_to_bst(l(0, None)), t(0));
    }
    #[test]
    fn example4_h_1_3_produces_1_3() {
        let head = l(1, l(3, None));
        let e = tl(3, t(1));
        assert_eq!(Solution::sorted_list_to_bst(head), e);
    }

    mod performance {
        use super::*;

        /// If getting stack overflow:
        ///
        /// ```sh
        /// thread 'c2021_05::c2021_05_06::tests::performance::h1to20000_produces_bst' has overflowed its stack
        /// fatal runtime error: stack overflow
        /// ```
        ///
        /// Add `RUST_MIN_STACK=33554432` to env
        // #[rustfmt::skip]
        fn list(from: i32, to: i32) -> List {
            if from > to {
                None
            } else {
                l(from, list(from + 1, to))
            }
        }

        #[test]
        fn h1to20000_produces_bst() {
            let head = list(1, 20000);
            assert_ne!(Solution::sorted_list_to_bst(head), None);
        }
    }
}
