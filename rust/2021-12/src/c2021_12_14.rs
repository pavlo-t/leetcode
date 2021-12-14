#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 938. Range Sum of BST
/// =====================
///
/// Given the `root` node of a binary search tree and two integers `low` and `high`,
/// return _the sum of values of all nodes with a value in the __inclusive__ range `[low, high]`_.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[1, 20_000]`.
/// - 1 <= Node.val <= 100_000`
/// - 1 <= low <= high <= 100_000`
/// - All `Node.val` are __unique__.
///
/// https://leetcode.com/problems/range-sum-of-bst/
struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn range_sum_bst_rec(root: T, low: i32, high: i32) -> i32 {
        println!("range_sum_bst({:?}, {}, {})", root, low, high);
        root.map(|n| {
            let b = n.borrow();
            if b.val > high {
                Self::range_sum_bst(b.left.clone(), low, high)
            } else if b.val < low {
                Self::range_sum_bst(b.right.clone(), low, high)
            } else {
                let l = Self::range_sum_bst(b.left.clone(), low, high);
                let r = Self::range_sum_bst(b.right.clone(), low, high);
                b.val + l + r
            }
        })
        .unwrap_or(0)
    }
    pub fn range_sum_bst(root: T, low: i32, high: i32) -> i32 {
        println!("range_sum_bst({:?}, {}, {})", root, low, high);
        let mut stack = vec![root];
        let mut result = 0;
        while let Some(n) = stack.pop() {
            if let Some(n) = n {
                let b = n.borrow();
                if b.val < high {
                    stack.push(b.right.clone());
                }
                if b.val > low {
                    stack.push(b.left.clone());
                }
                if b.val >= low && b.val <= high {
                    result += b.val;
                }
            }
        }
        result
    }
}

#[rustfmt::skip] #[derive(PartialEq, Eq)]
pub struct TreeNode { pub val: i32, pub left: T, pub right: T }
impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::collections::VecDeque;
        let mut q = VecDeque::new();
        if self.left.is_some() || self.right.is_some() {
            q.push_back(self.left.clone());
            q.push_back(self.right.clone());
        }
        let mut vals = vec![self.val.to_string()];
        while let Some(n) = q.pop_front() {
            if let Some(n) = n {
                let b = n.borrow();
                vals.push(b.val.to_string());
                if b.left.is_some() || b.right.is_some() {
                    q.push_back(b.left.clone());
                    q.push_back(b.right.clone());
                }
            } else {
                vals.push("n".to_string());
            }
        }
        f.debug_list().entries(vals.iter()).finish()
    }
}
#[rustfmt::skip] impl TreeNode {
    #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
    #[inline] fn wrap(self) -> T { Some(Rc::new(RefCell::new(self))) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: T = None;

    #[rustfmt::skip] fn n(v: i32, l: T, r: T) -> T { TreeNode { val: v, left: l, right: r }.wrap() }
    #[rustfmt::skip] fn l(v: i32            ) -> T { TreeNode::new(v).wrap() }

    #[test]
    fn r_n_l6_h10() {
        assert_eq!(Solution::range_sum_bst(N, 6, 10), 0);
    }
    #[test]
    fn r_10_5_15_3_7_n_18_l7_h15() {
        let r = n(10, n(5, l(3), l(7)), n(15, N, l(18)));
        assert_eq!(Solution::range_sum_bst(r, 7, 15), 32);
        //     10
        //    /  \
        //   5   15
        //  / \    \
        // 3   7   18
        // Explanation: Nodes 7, 10, and 15 are in the range [7, 15]. 7 + 10 + 15 = 32.
    }
    #[test]
    fn r_10_5_18_3_7_13_18_n_6_l6_h10() {
        let r = n(10, n(5, n(3, l(1), N), n(7, l(6), N)), n(15, l(13), l(18)));
        assert_eq!(Solution::range_sum_bst(r, 6, 10), 23);
        //        10
        //      /    \
        //     5     15
        //    / \    / \
        //   3   7  13 18
        //  /   /
        // 1   6
        // Explanation: Nodes 6, 7, and 10 are in the range [6, 10]. 6 + 7 + 10 = 23.
    }
    #[test]
    fn r_10_5_18_3_7_13_18_n_6_l0_h4() {
        let r = n(10, n(5, n(3, l(1), N), n(7, l(6), N)), n(15, l(13), l(18)));
        assert_eq!(Solution::range_sum_bst(r, 0, 4), 4);
    }
    #[test]
    fn r_10_5_18_3_7_13_18_n_6_l14_h20() {
        let r = n(10, n(5, n(3, l(1), N), n(7, l(6), N)), n(15, l(13), l(18)));
        assert_eq!(Solution::range_sum_bst(r, 14, 20), 33);
    }
}
