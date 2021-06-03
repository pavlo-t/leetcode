#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Clone)]
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

/// Inorder Successor in BST
/// ========================
///
/// Given the `root` of a binary search tree and a node `p` in it,
/// return _the in-order successor of that node in the BST_.
/// If the given node has no in-order successor in the tree, return `null`.
///
/// The successor of a node `p` is the node with the smallest key greater than `p.val`.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[1, 10_000]`.
/// - `-100_000 <= Node.val <= 100_000`
/// - All Nodes will have unique values.
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/594/week-2-april-8th-april-14th/3700/
struct Solution;
impl Solution {
    pub fn inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if p.is_none() { None } else {
            fn search(
                n: &Option<Rc<RefCell<TreeNode>>>,
                p: i32,
                successor: &mut Option<Rc<RefCell<TreeNode>>>
            ) {
                if let Some(rc) = n {
                    let n = rc.borrow();
                    if n.val > p {
                        *successor = Some(rc.clone());
                        search(&n.left, p, successor);
                    } else {
                        search(&n.right, p, successor);
                    }
                }
            }

            let mut result = None;
            search(&root, p.unwrap().borrow().val, &mut result);
            result
        }
    }

    pub fn inorder_successor_rust_gym(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if p.is_none() { None } else {
            fn inorder(
                n: &Option<Rc<RefCell<TreeNode>>>,
                p: i32,
                successor: &mut Option<Rc<RefCell<TreeNode>>>
            ) {
                if let Some(rc) = n {
                    let n = rc.borrow();
                    inorder(&n.left, p, successor);
                    if successor.is_none() && n.val > p {
                        *successor = Some(rc.clone());
                    }
                    inorder(&n.right, p, successor);
                }
            }
            let p = p.unwrap().borrow().val;
            let mut result = None;
            inorder(&root, p, &mut result);
            result
        }
    }

    pub fn inorder_successor_brute_force_my(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn inorder(
            n: &Option<Rc<RefCell<TreeNode>>>,
            p: i32,
            should_return: bool
        ) -> (Option<Rc<RefCell<TreeNode>>>, bool) {
            match n {
                None => (None, should_return),
                Some(rc) => {
                    let n = rc.borrow();
                    if should_return {
                        match inorder(&n.left, p, should_return) {
                            (Some(r), f) => (Some(r), f),
                            _ => (Some(rc.clone()), false)
                        }
                    } else {
                        match inorder(&n.left, p, should_return) {
                            (Some(r), f) => (Some(r), f),
                            (_, true) => (Some(rc.clone()), true),
                            _ if n.val == p => inorder(&n.right, p, true),
                            _ => inorder(&n.right, p, should_return)
                        }
                    }
                }
            }
        }

        if let Some(p) = p {
            inorder(&root, p.borrow().val, false).0
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn inorder_print(n: &Option<Rc<RefCell<TreeNode>>>) {
        n.as_ref().map(|rc| {
            let n = rc.borrow();
            inorder_print(&n.left);
            println!("{}", n.val);
            inorder_print(&n.right);
        });
    }

    type Node = Option<Rc<RefCell<TreeNode>>>;
    fn wrap(n: TreeNode) -> Node { Some(Rc::new(RefCell::new(n))) }
    fn nlr(v: i32, l: Node, r: Node) -> Node { wrap(TreeNode { val: v, left: l, right: r } ) }
    fn nl(v: i32, l: Node) -> Node { wrap(TreeNode { val: v, left: l, right: None } ) }
    fn nr(v: i32, r: Node) -> Node { wrap(TreeNode { val: v, left: None, right: r } ) }
    fn n(v: i32) -> Node { wrap(TreeNode::new(v)) }

    #[test]
    fn example1() {
        let root = nlr(2, n(1), n(3));
        let p = n(1);
        let e = nlr(2, n(1), n(3));
        assert_eq!(Solution::inorder_successor(root, p), e);
        // Explanation: 1's in-order successor node is 2.
        // Note that both p and the return value is of TreeNode type.
    }
    #[test]
    fn example2() {
        let root = nlr(5, nlr(3, nl(2, n(1)), n(4)), n(6));
        let p = n(6);
        let e = None;
        assert_eq!(Solution::inorder_successor(root, p), e);
        // Explanation: There is no in-order successor of the current node, so the answer is null.
    }

    #[test]
    fn test_p_is_none_produce_none() {
        let root = nlr(5, nlr(3, nl(2, n(1)), n(4)), n(6));
        let p = None;
        let e = None;
        assert_eq!(Solution::inorder_successor(root, p), e);
    }
    #[test]
    fn test_return_direct_parent_1_2() {
        let root = nlr(5, nlr(3, nl(2, n(1)), n(4)), n(6));
        let p = n(1);
        let e = nl(2, n(1));
        assert_eq!(Solution::inorder_successor(root, p), e);
    }
    #[test]
    fn test_return_grandparent_4_5() {
        let root = nlr(5, nlr(3, nl(2, n(1)), n(4)), n(6));
        let p = n(4);
        let e = nlr(5, nlr(3, nl(2, n(1)), n(4)), n(6));
        assert_eq!(Solution::inorder_successor(root, p), e);
    }
    #[test]
    fn test_return_right_child_3_4() {
        let root = nlr(5, nlr(3, nl(2, n(1)), n(4)), n(6));
        let p = n(3);
        let e = n(4);
        assert_eq!(Solution::inorder_successor(root, p), e);
    }
    #[test]
    fn test_return_grandchild_2_3() {
        let root = nlr(2, n(1), nlr(4, n(3), n(5)));
        let p = n(2);
        let e = n(3);
        assert_eq!(Solution::inorder_successor(root, p), e);
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
        fn test_10000_returns_none() {
            let root = gen_tree(1, 10000);
            let p = n(10000);
            let e = None;
            assert_eq!(Solution::inorder_successor(root, p), e);
        }
        #[test]
        fn test_10000_returns_10000() {
            let root = gen_tree(1, 10000);
            let p = n(9999);
            let e = n(10000);
            assert_eq!(Solution::inorder_successor(root, p), e);
        }
    }
}
