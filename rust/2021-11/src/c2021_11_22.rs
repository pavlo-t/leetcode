#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 450. Delete Node in a BST
/// =========================
///
/// Given a root node reference of a BST and a key, delete the node with the given key in the BST.
/// Return the root node reference (possibly updated) of the BST.
///
/// Basically, the deletion can be divided into two stages:
///
/// 1. Search for a node to remove.
/// 2. If the node is found, delete the node.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[0, 10_000]`.
/// - `-100_000 <= Node.val <= 100_000`
/// - Each node has a __unique__ value.
/// - `root` is a valid binary search tree.
/// - `-100_000 <= key <= 100_000`
///
/// __Follow up:__ Could you solve it with time complexity `O(height of tree)`?
///
/// https://leetcode.com/problems/delete-node-in-a-bst/
struct Solution;
impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        println!("delete_node({:?}, {})", root, key);
        let mut curr = root.clone();
        let dummy = Some(Rc::new(RefCell::new(TreeNode {
            val: i32::MIN,
            left: None,
            right: root.clone(),
        })));
        let mut prev = dummy.clone();
        while let Some(n) = curr.clone() {
            let b = n.borrow();
            match (key - b.val).signum() {
                0 => break,
                s => {
                    prev = curr.clone();
                    curr = if s == 1 {
                        b.right.clone()
                    } else {
                        b.left.clone()
                    };
                }
            }
        }
        println!("  prev: {:?}", prev.clone().map(|n| n.borrow().val));
        println!("  curr: {:?}", curr.clone().map(|n| n.borrow().val));

        type T = Option<Rc<RefCell<TreeNode>>>;
        fn set_curr(prev: T, curr: T, new_curr: T) {
            let prev = prev.clone().unwrap();
            let mut pb = prev.borrow_mut();
            let curr = curr.clone().unwrap();
            let cb = curr.borrow();
            if pb.left.clone().map(|n| n.borrow().val).unwrap_or(i32::MIN) == cb.val {
                pb.left = new_curr;
            } else {
                pb.right = new_curr;
            }
        }
        if let Some(curr_n) = curr.clone() {
            let curr_b = curr_n.borrow();
            match (curr_b.left.clone(), curr_b.right.clone()) {
                (None, None) => set_curr(prev, curr, None),
                (None, r) => set_curr(prev, curr, r),
                (l, None) => set_curr(prev, curr, l),
                (Some(l), Some(r)) => {
                    drop(curr_b);
                    let mut left_most_child = r.borrow().left.clone();
                    if left_most_child.is_some() {
                        let mut parent = Some(r);
                        while let Some(l) = left_most_child.clone().unwrap().borrow().left.clone() {
                            parent = left_most_child.clone();
                            left_most_child = Some(l);
                        }
                        let lmc = left_most_child.unwrap();
                        parent.unwrap().borrow_mut().left = lmc.borrow_mut().right.take();
                        curr_n.borrow_mut().val = lmc.borrow().val;
                    } else {
                        r.borrow_mut().left = Some(l);
                        set_curr(prev, curr, curr_n.borrow().right.clone());
                    }
                }
            }
        }
        dummy.unwrap().borrow_mut().right.take()
    }
}

type T = Option<Rc<RefCell<TreeNode>>>;

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode { pub val: i32, pub left: T, pub right: T }
#[rustfmt::skip]
impl TreeNode { #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } } }

#[cfg(test)]
mod tests {
    use super::*;

    const N: T = None;
    #[rustfmt::skip] fn wrap(t: TreeNode) -> T { Some(Rc::new(RefCell::new(t))) }
    #[rustfmt::skip] fn n(v: i32, l: T, r: T) -> T { wrap(TreeNode { val: v, left: l, right: r }) }
    #[rustfmt::skip] fn l(v: i32            ) -> T { wrap(TreeNode::new(v)) }

    #[rustfmt::skip] #[test] fn rn_k0() { assert_eq!(Solution::delete_node(None, 0), None); }
    #[rustfmt::skip] #[test] fn r1_k0() { assert_eq!(Solution::delete_node(l(1), 0), l(1)); }
    #[rustfmt::skip] #[test] fn r1_k1() { assert_eq!(Solution::delete_node(l(1), 1), None); }
    #[test]
    fn r53624n7_k3() {
        let r = n(5, n(3, l(2), l(4)), n(6, N, l(7)));
        let k = 3;
        let e = n(5, n(4, l(2), N), n(6, N, l(7)));
        assert_eq!(Solution::delete_node(r, k), e);
        //     5          5         5
        //    / \        / \       / \
        //   3   6  =>  4   6  OR 2   6
        //  / \   \    /     \     \   \
        // 2   4   7  2       7     4   7
        // Explanation: Given key to delete is 3. So we find the node with value 3 and delete it.
        // One valid answer is [5,4,6,2,null,null,7], shown in the above BST.
        // Please notice that another valid answer is [5,2,6,null,4,null,7] and it's also accepted.
    }
    #[test]
    fn r53624n7_k0() {
        let r = n(5, n(3, l(2), l(4)), n(6, N, l(7)));
        let k = 0;
        let e = n(5, n(3, l(2), l(4)), n(6, N, l(7)));
        assert_eq!(Solution::delete_node(r, k), e);
        // Explanation: The tree does not contain a node with value = 0.
    }
    #[test]
    fn r4261357_k4() {
        let r = n(4, n(2, l(1), l(3)), n(6, l(5), l(7)));
        let k = 4;
        let e = n(5, n(2, l(1), l(3)), n(6, N, l(7)));
        assert_eq!(Solution::delete_node(r, k), e);
        //      4           5
        //    /   \        / \
        //   2     6  =>  2   6
        //  / \   / \    / \   \
        // 1   3 5   7  1   3   7
    }
    #[test]
    fn r4271358nnnnn6nn_k4() {
        // [5,2,7,1,3,6,8]
        //      4            5
        //    /   \        /   \
        //   2     7  =>  2     7
        //  / \   / \    / \   / \
        // 1   3 5   8  1   3 6   8
        //        \
        //         6
        let r = n(4, n(2, l(1), l(3)), n(7, n(5, N, l(6)), l(8)));
        let k = 4;
        let e = n(5, n(2, l(1), l(3)), n(7, l(6), l(8)));
        assert_eq!(Solution::delete_node(r, k), e);
    }
}
