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
/// Maximum Average Subtree
/// =======================
///
/// Given the `root` of a binary tree, return _the maximum __average__ value of a __subtree__ of that tree_.
/// Answers within `0.00001` of the actual answer will be accepted.
///
/// A __subtree__ of a tree is any node of that tree plus all its descendants.
///
/// The __average__ value of a tree is the sum of its values, divided by the number of nodes.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[1, 10_000]`.
/// - `0 <= Node.val <= 100_000`
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/636/week-1-september-1st-september-7th/3959/
struct Solution;
impl Solution {
    pub fn maximum_average_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> f64 {
        fn avgs(n: &Option<Rc<RefCell<TreeNode>>>) -> (f64, i32, f64) {
            match n {
                None => (0.0, 0, 0.0),
                Some(n) => {
                    let bn = n.borrow();
                    let (ls, lc, la) = avgs(&bn.left);
                    let (rs, rc, ra) = avgs(&bn.right);
                    let ns = ls + rs + bn.val as f64;
                    let nc = lc + rc + 1;
                    let na = ns / nc as f64;

                    (ns, nc, na.max(la).max(ra))
                }
            }
        }

        avgs(&root).2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type N = Option<Rc<RefCell<TreeNode>>>;

    fn wrap(n: TreeNode) -> N {
        Some(Rc::new(RefCell::new(n)))
    }
    fn nlr(v: i32, l: N, r: N) -> N {
        wrap(TreeNode {
            val: v,
            left: l,
            right: r,
        })
    }
    fn nl(v: i32, l: N) -> N {
        wrap(TreeNode {
            val: v,
            left: l,
            right: None,
        })
    }
    fn nr(v: i32, r: N) -> N {
        wrap(TreeNode {
            val: v,
            left: None,
            right: r,
        })
    }
    fn n(v: i32) -> N {
        wrap(TreeNode::new(v))
    }

    #[test]
    fn root_5_6_1_produces_6() {
        let root = nlr(5, n(6), n(1));
        assert_eq!(Solution::maximum_average_subtree(root), 6.0);
        // Explanation:
        // For the node with value = 5 we have an average of (5 + 6 + 1) / 3 = 4.
        // For the node with value = 6 we have an average of 6 / 1 = 6.
        // For the node with value = 1 we have an average of 1 / 1 = 1.
        // So the answer is 6 which is the maximum.
    }
    #[test]
    fn root_0_n_1_produces_1() {
        let root = nr(0, n(1));
        assert_eq!(Solution::maximum_average_subtree(root), 1.0);
    }

    #[test]
    fn root_5_6_1_3_7_produces_7() {
        let root = nlr(5, nlr(6, n(3), n(7)), n(1));
        assert_eq!(Solution::maximum_average_subtree(root), 7.0);
    }
    #[test]
    fn root_5_6_1_3_4_produces_4_33333() {
        let root = nlr(5, nlr(6, n(3), n(4)), n(1));
        let result = Solution::maximum_average_subtree(root);
        println!("result: {}", result);
        assert!((result - 4.33333).abs() < 0.00001);
    }
}
