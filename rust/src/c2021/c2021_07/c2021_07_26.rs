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

/// Convert Sorted Array to Binary Search Tree
/// ==========================================
///
/// Given an integer array `nums` where the elements are sorted in __ascending order__,
/// convert _it to a __height-balanced__ binary search tree_.
///
/// A __height-balanced__ binary tree is a binary tree in which the depth of the two subtrees of every node
/// never differs by more than one.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 10_000`
/// - `-10_000 <= nums[i] <= 10_000`
/// - `nums` is sorted in a __strictly increasing__ order.
///
/// https://leetcode.com/explore/challenge/card/july-leetcoding-challenge-2021/611/week-4-july-22nd-july-28th/3827/
struct Solution;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn rec(ns: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            match ns.len() {
                0 => None,
                1 => Some(Rc::new(RefCell::new(TreeNode::new(ns[0])))),
                n => {
                    let m = n / 2;
                    let left = rec(&ns[..m]);
                    let right = rec(&ns[m + 1..]);
                    let val = ns[m];
                    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
                }
            }
        }
        rec(&nums)
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
    fn ns_m10m3p0p5p9() {
        let nums = vec![-10, -3, 0, 5, 9];
        let e = nlr(0, nl(-3, n(-10)), nl(9, n(5)));
        assert_eq!(Solution::sorted_array_to_bst(nums), e);
        // Explanation: [0,-10,5,null,-3,null,9] is also accepted:
        // let e = nlr(0, nr(-10, n(-3)), nr(5, n(9)));
    }
    #[test]
    fn ns_p1p3() {
        let nums = vec![1, 3];
        let e = nl(3, n(1));
        assert_eq!(Solution::sorted_array_to_bst(nums), e);
        // Explanation: [1,3] and [3,1] are both a height-balanced BSTs.
        // let e = nr(1, n(3));
    }
    #[test]
    fn ns_1to10000() {
        let nums = (1..=10000).collect();
        assert!(Solution::sorted_array_to_bst(nums).is_some());
    }
}
