#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 701. Insert into a Binary Search Tree
/// =====================================
///
/// You are given the `root` node of a binary search tree (BST) and a `value` to insert into the tree.
/// Return _the root node of the BST after the insertion_.
/// It is __guaranteed__ that the new value does not exist in the original BST.
///
/// __Notice__ that there may exist multiple valid ways for the insertion,
/// as long as the tree remains a BST after insertion.
/// You can return __any of them__.
///
/// __Constraints:__
///
/// - The number of nodes in the tree will be in the range `[0, 10_000]`.
/// - `-100_000_000 <= Node.val <= 100_000_000`
/// - All the values `Node.val` are __unique__.
/// - `-100_000_000 <= val <= 100_000_000`
/// - It's __guaranteed__ that `val` does not exist in the original BST.
///
/// https://leetcode.com/problems/insert-into-a-binary-search-tree/
struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;
const N: T = None;
impl Solution {
    pub fn insert_into_bst(root: T, val: i32) -> T {
        println!("insert_into_bst({:?}, {})", root, val);
        if let Some(n) = root {
            let mut b = n.borrow_mut();
            if b.val < val {
                b.right = Self::insert_into_bst(b.right.take(), val);
            } else {
                b.left = Self::insert_into_bst(b.left.take(), val);
            }
            drop(b);
            Some(n)
        } else {
            Some(Rc::new(RefCell::new(TreeNode::new(val))))
        }
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq)]
pub struct TreeNode { pub val: i32, pub left: T, pub right: T }
impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const LEVEL: &str = "_";
        const NULL: &str = "n";
        let mut data = vec![self.val.to_string()];
        let mut curr = vec![];
        if self.left.is_some() || self.right.is_some() {
            curr.push(self.right.clone());
            curr.push(self.left.clone());
            data.push(LEVEL.into());
        }
        while !curr.is_empty() {
            let mut next = vec![];
            let mut has_next = false;
            while let Some(n) = curr.pop() {
                if let Some(n) = n {
                    let b = n.borrow();
                    data.push(b.val.to_string());
                    next.push(b.left.clone());
                    next.push(b.right.clone());
                    if b.left.is_some() || b.right.is_some() {
                        has_next = true;
                    }
                } else {
                    data.push(NULL.into());
                }
            }

            if has_next {
                next.reverse();
                curr = next;
                data.push(LEVEL.into());
            }
        }
        f.debug_list().entries(data.iter()).finish()
    }
}
#[rustfmt::skip]
impl TreeNode {
    #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
    #[inline] fn wrap(self) -> T { Some(Rc::new(RefCell::new(self))) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] fn t(v: i32, l: T, r: T) -> T { TreeNode { val: v, left: l, right: r }.wrap() }
    #[rustfmt::skip] fn l(v: i32            ) -> T { TreeNode::new(v).wrap() }

    #[rustfmt::skip] #[test] fn r_n_v_3() { assert_eq!(Solution::insert_into_bst(N, 3), l(3)); }
    #[rustfmt::skip] #[test] fn r_1_v_3() { assert_eq!(Solution::insert_into_bst(l(1), 3), t(1, N, l(3))); }
    #[rustfmt::skip] #[test] fn r_3_v_1() { assert_eq!(Solution::insert_into_bst(l(3), 1), t(3, l(1), N)); }

    #[test]
    fn r_4l2_7l1_3_n_n_v_5() {
        //    4
        //  2   7
        // 1 3
        let r = t(4, t(2, l(1), l(3)), l(7));
        let e = t(4, t(2, l(1), l(3)), t(7, l(5), N));
        assert_eq!(Solution::insert_into_bst(r, 5), e);
    }
    #[test]
    fn r_40l20_60l10_30_50_70_v_25() {
        let r = t(40, t(20, l(10), l(30)), t(60, l(50), l(70)));
        let e = t(40, t(20, l(10), t(30, l(25), N)), t(60, l(50), l(70)));
        assert_eq!(Solution::insert_into_bst(r, 25), e);
    }
    #[test]
    fn r_4l2_7l1_3_6_8_v_5() {
        //    4
        //  2   7
        // 1 3 6 8
        let r = t(4, t(2, l(1), l(3)), t(7, l(6), l(8)));
        let e = t(4, t(2, l(1), l(3)), t(7, t(6, l(5), N), l(8)));
        assert_eq!(Solution::insert_into_bst(r, 5), e);
    }
}
