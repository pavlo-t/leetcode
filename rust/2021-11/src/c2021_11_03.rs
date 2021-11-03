#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 129. Sum Root to Leaf Numbers
/// =============================
///
/// You are given the `root` of a binary tree containing digits from `0` to `9` only.
///
/// Each root-to-leaf path in the tree represents a number.
///
/// For example, the root-to-leaf path `1 -> 2 -> 3` represents the number `123`.
///
/// Return _the total sum of all root-to-leaf numbers_.
/// Test cases are generated so that the answer will fit in a __32-bit__ integer.
///
/// A __leaf__ node is a node with no children.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[1, 1000]`.
/// - `0 <= Node.val <= 9`
/// - The depth of the tree will not exceed `10`.
///
/// https://leetcode.com/problems/sum-root-to-leaf-numbers/
struct Solution;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        println!("sum_numbers({:?})", root);
        let mut stack = vec![(root, 0)];
        let mut result = 0;
        while let Some((n, mut rsf)) = stack.pop() {
            if let Some(n) = n {
                let mut n = n.borrow_mut();
                rsf = rsf * 10 + n.val;
                match (n.left.take(), n.right.take()) {
                    (None, None) => result += rsf,
                    (l, r) => {
                        stack.push((l, rsf));
                        stack.push((r, rsf));
                    }
                }
            }
        }
        result
    }
    pub fn sum_numbers_iter_1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        println!("sum_numbers({:?})", root);
        let mut stack = vec![(root.clone(), 0)];
        let mut result = 0;
        while let Some((n, mut rsf)) = stack.pop() {
            if let Some(n) = n {
                let n = n.borrow();
                rsf = rsf * 10 + n.val;
                if n.left.is_none() && n.right.is_none() {
                    result += rsf;
                } else {
                    stack.push((n.left.clone(), rsf));
                    stack.push((n.right.clone(), rsf));
                }
            }
        }
        result
    }
    pub fn sum_numbers_inspired_by_other_answers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        println!("sum_numbers({:?})", root);
        fn sum_rec(n: Option<&Rc<RefCell<TreeNode>>>, mut rsf: i32) -> i32 {
            n.map(|n| {
                let n = n.borrow();
                rsf = rsf * 10 + n.val;
                let (l, r) = (n.left.as_ref(), n.right.as_ref());
                match (sum_rec(l, rsf), sum_rec(r, rsf)) {
                    (0, 0) => rsf,
                    (l, r) => l + r,
                }
            })
            .unwrap_or(0)
        }
        sum_rec(root.as_ref(), 0)
    }
    pub fn sum_numbers_my_top_down(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        println!("sum_numbers({:?})", root);
        fn pushed<T>(mut vec: Vec<T>, v: T) -> Vec<T> {
            vec.push(v);
            vec
        }
        fn numbers(n: Option<&Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
            n.map(|n| {
                let n = n.borrow();
                let v = n.val;
                let mut lns = numbers(n.left.as_ref());
                let mut rns = numbers(n.right.as_ref());
                lns.append(&mut rns);
                if lns.is_empty() {
                    pushed(lns, vec![v])
                } else {
                    lns.into_iter().map(|c| pushed(c, v)).collect()
                }
            })
            .unwrap_or(vec![])
        }
        let ns = numbers(root.as_ref());
        ns.into_iter()
            .map(|vec| vec.into_iter().rev().fold(0, |rsf, n| rsf * 10 + n))
            .sum()
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    type T = Option<Rc<RefCell<TreeNode>>>;
    const N: T = None;
    #[rustfmt::skip] fn wrap(n: TreeNode) -> T { Some(Rc::new(RefCell::new(n))) }
    #[rustfmt::skip] fn n(v: i32, l: T, r: T) -> T { wrap(TreeNode { val: v, left: l, right: r }) }
    #[rustfmt::skip] fn l(v: i32            ) -> T { wrap(TreeNode::new(v)) }

    #[test]
    fn r_123() {
        let r = n(1, l(2), l(3));
        assert_eq!(Solution::sum_numbers(r), 25);
        // Explanation:
        // The root-to-leaf path 1->2 represents the number 12.
        // The root-to-leaf path 1->3 represents the number 13.
        // Therefore, sum = 12 + 13 = 25.
    }
    #[test]
    fn r_49051() {
        let r = n(4, n(9, l(5), l(1)), l(0));
        assert_eq!(Solution::sum_numbers(r), 1026);
        // Explanation:
        // The root-to-leaf path 4->9->5 represents the number 495.
        // The root-to-leaf path 4->9->1 represents the number 491.
        // The root-to-leaf path 4->0 represents the number 40.
        // Therefore, sum = 495 + 491 + 40 = 1026.
    }
    #[test]
    fn r_7() {
        assert_eq!(Solution::sum_numbers(l(7)), 7);
    }

    fn balanced(v: i32, d: i32) -> T {
        if d == 0 {
            N
        } else {
            n(v, balanced(v, d - 1), balanced(v, d - 1))
        }
    }
    #[test]
    fn r_10lvls_of_0() {
        let r = balanced(0, 10);
        assert_eq!(Solution::sum_numbers(r), 0);
    }
    #[test]
    fn r_10lvls_of_1() {
        let r = balanced(1, 8);
        assert_eq!(Solution::sum_numbers(r), 1_422_222_208);
    }
}
