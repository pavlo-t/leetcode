#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 106. Construct Binary Tree from Inorder and Postorder Traversal
/// ===============================================================
///
/// Given two integer arrays `inorder` and `postorder`
/// where `inorder is the inorder traversal of a binary tree
/// and `postorder` is the postorder traversal of the same tree,
/// construct and return _the binary tree_.
///
/// __Constraints:__
///
/// - `1 <= inorder.length <= 3000`
/// - `postorder.length == inorder.length`
/// - `-3000 <= inorder[i], postorder[i] <= 3000`
/// - `inorder` and `postorder` consist of __unique__ values.
/// - Each value of `postorder` also appears in `inorder`.
/// - `inorder` is __guaranteed__ to be the inorder traversal of the tree.
/// - `postorder` is __guaranteed__ to be the postorder traversal of the tree.
///
/// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
struct Solution;
impl Solution {
    pub fn build_tree_rec(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        println!("build_tree({:?}, {:?})", inorder, postorder);

        fn rec(i: &[i32], p: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            p.last().map(|&val| {
                let idx = i.iter().position(|v| v == &val).unwrap();
                Rc::new(RefCell::new(TreeNode {
                    val,
                    left: rec(&i[0..idx], &p[0..idx]),
                    right: rec(&i[idx + 1..], &p[idx..p.len() - 1]),
                }))
            })
        }

        rec(&inorder, &postorder)
    }

    /// Approach 1: Recursion (with HashMap<val, idx>) for constant time search
    /// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/solution/
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        println!("build_tree({:?}, {:?})", inorder, postorder);
        use std::collections::HashMap;

        type T = Option<Rc<RefCell<TreeNode>>>;

        fn rec(is: usize, i_ord: &[i32], p_ord: &[i32], idx_map: &HashMap<i32, usize>) -> T {
            p_ord.last().map(|&val| {
                let &idx = idx_map.get(&val).unwrap();
                let i = idx - is;
                Rc::new(RefCell::new(TreeNode {
                    val,
                    left: rec(is, &i_ord[0..i], &p_ord[0..i], idx_map),
                    right: rec(is + i + 1, &i_ord[i + 1..], &p_ord[i..p_ord.len() - 1], idx_map),
                }))
            })
        }

        let mut idx_map = HashMap::with_capacity(inorder.len());
        for (i, &v) in inorder.iter().enumerate() {
            idx_map.insert(v, i);
        }
        rec(0, &inorder, &postorder, &idx_map)
    }
}

type T = Option<Rc<RefCell<TreeNode>>>;

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode { pub val: i32, pub left: T, pub right: T }
#[rustfmt::skip] impl TreeNode { #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } } }

#[cfg(test)]
mod tests {
    use super::*;

    const N: T = None;
    #[rustfmt::skip] fn wrap(t: TreeNode) -> T { Some(Rc::new(RefCell::new(t))) }
    #[rustfmt::skip] fn t(v: i32, l: T, r: T) -> T { wrap(TreeNode { val: v, left: l, right: r }) }
    #[rustfmt::skip] fn l(v: i32            ) -> T { wrap(TreeNode::new(v)) }

    #[test]
    fn i_m1_p_m1() {
        assert_eq!(Solution::build_tree(vec![-1], vec![-1]), l(-1));
    }
    #[test]
    fn i_12_p_12() {
        assert_eq!(Solution::build_tree(vec![1, 2], vec![1, 2]), t(2, l(1), N));
        //   2
        //  /
        // 1
    }
    #[test]
    fn i_12_p_21() {
        assert_eq!(Solution::build_tree(vec![1, 2], vec![2, 1]), t(1, N, l(2)));
        // 1
        //  \
        //   2
    }
    #[test]
    fn i_123_p_132() {
        let i = vec![1, 2, 3];
        let p = vec![1, 3, 2];
        let e = t(2, l(1), l(3));
        assert_eq!(Solution::build_tree(i, p), e);
        //   2
        //  / \
        // 1   3
    }
    #[test]
    fn i_1234567_p_1325764() {
        let i = vec![1, 2, 3, 4, 5, 6, 7];
        let p = vec![1, 3, 2, 5, 7, 6, 4];
        let e = t(4, t(2, l(1), l(3)), t(6, l(5), l(7)));
        assert_eq!(Solution::build_tree(i, p), e);
        //      4
        //    /   \
        //   2     6
        //  / \   / \
        // 1   3 5   7
    }
    #[test]
    fn i_9_3_15_20_7_p_9_15_7_20_3() {
        let i = vec![9, 3, 15, 20, 7];
        let p = vec![9, 15, 7, 20, 3];
        let e = t(3, l(9), t(20, l(15), l(7)));
        assert_eq!(Solution::build_tree(i, p), e);
        //   3
        //  / \
        // 9  20
        //   /  \
        //  15   7
    }
}
