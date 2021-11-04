#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 404. Sum of Left Leaves
/// =======================
///
/// Given the `root` of a binary tree, return the sum of all left leaves.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[1, 1000]`.
/// - `-1000 <= Node.val <= 1000`
///
/// https://leetcode.com/problems/sum-of-left-leaves/
struct Solution;
impl Solution {
    /// Approach 3: Morris Tree Traversal (Pre-order)
    /// https://leetcode.com/problems/sum-of-left-leaves/solution/
    ///
    /// ```text
    ///    C1          1          1         1          1         1        C1         1
    ///    / \        / \        / \       / \        / \       / \       / \       / \
    ///  P2   3 =>  C2   3 =>   2   3 => C2   3 =>  C2   3 =>  2   3 => P2   3 =>  2  C3
    ///  / \        / \        / \       / \        / \       / \       / \       / \
    /// 4  P5     P4   5     C4   5     4   5     P4   5     4  C5     4  P5     4   5
    ///                 \      \   \     \   \      \   \         \         \
    ///                  1      2   1    C2   1     C2   1         1        C1
    /// ```
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        println!("sum_of_left_leaves({:?})", root);
        let mut result = 0;
        let mut curr = root.clone();
        while let Some(n) = curr.clone() {
            if let Some(mut prev) = n.borrow().left.clone() {
                // Check if this left node is a leaf node.
                if prev.borrow().left.is_none() && prev.borrow().right.is_none() {
                    result += prev.borrow().val;
                }
                // Find the inorder predecessor for currentNode.
                while prev.borrow().right.is_some() && prev.borrow().right != curr {
                    let prev_right = prev.borrow().right.clone();
                    prev = prev_right.unwrap();
                }
                if prev.borrow().right.is_none() {
                    // We've not yet visited the inorder predecessor.
                    // This means that we still need to explore currentNode's left subtree.
                    // Before doing this, we will put a link back so that we can get back to the right subtree when we need to.
                    prev.borrow_mut().right = curr.clone();
                    curr = curr.unwrap().borrow().left.clone();
                } else {
                    // We have already visited the inorder predecessor.
                    // This means that we need to remove the link we added,
                    // and then move onto the right subtree and explore it.
                    prev.borrow_mut().right = None;
                    curr = curr.unwrap().borrow().right.clone();
                }
            } else {
                // If there is no left child, we can simply explore the right subtree
                // without needing to worry about keeping track of currentNode's other child.
                curr = n.borrow().right.clone();
            }
        }
        result
    }
    pub fn sum_of_left_leaves_my_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        println!("sum_of_left_leaves({:#?})", root);
        let mut stack = vec![(root.clone(), false)];
        let mut result = 0;
        while let Some((n, is_left)) = stack.pop() {
            if let Some(n) = n {
                let n = n.borrow();
                if n.left.is_none() && n.right.is_none() {
                    result += is_left as i32 * n.val;
                } else {
                    stack.push((n.left.clone(), true));
                    stack.push((n.right.clone(), false));
                }
            }
        }
        result
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
    fn r_3_9_20_n_n_15_7() {
        let r = n(3, l(9), n(20, l(15), l(7)));
        assert_eq!(Solution::sum_of_left_leaves(r), 24);
        // Explanation: There are two left leaves in the binary tree, with values 9 and 15 respectively.
    }
    #[test]
    fn r_1() {
        assert_eq!(Solution::sum_of_left_leaves(l(1)), 0);
    }

    #[rustfmt::skip]
    fn balanced(v: i32, d: i32) -> T {
        if d == 0 { N } else { n(v, balanced(v, d - 1), balanced(v, d - 1)) }
    }
    //#[ignore]
    #[test]
    fn r_10lvls_of_0() {
        let r = balanced(0, 10);
        assert_eq!(Solution::sum_of_left_leaves(r), 0);
    }
    //#[ignore]
    #[test]
    fn r_10lvls_of_1() {
        let r = balanced(1, 10);
        assert_eq!(Solution::sum_of_left_leaves(r), 256);
    }
}
