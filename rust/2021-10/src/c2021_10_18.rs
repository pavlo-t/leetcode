#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 993. Cousins in Binary Tree
/// ===========================
///
/// Given the `root` of a binary tree with unique values and the values of two different nodes of the tree `x` and `y`,
/// return _`true` if the nodes corresponding to the values `x` and `y` in the tree are __cousins__,
/// or `false` otherwise_.
///
/// Two nodes of a binary tree are __cousins__ if they have the same depth with different parents.
///
/// Note that in a binary tree, the root node is at the depth `0`,
/// and children of each depth `k` node are at the depth `k + 1`.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[2, 100]`.
/// - `1 <= Node.val <= 100`
/// - Each node has a __unique__ value.
/// - `x != y`
/// - `x` and `y` exist in the tree.
///
/// https://leetcode.com/problems/cousins-in-binary-tree/
struct Solution;
impl Solution {
    /// 21:33-21:55
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        println!("is_cousins({:?}, {}, {})", root, x, y);
        fn find(
            x: i32,
            n: Option<&Rc<RefCell<TreeNode>>>,
            parent: Option<i32>,
            depth: usize,
        ) -> Option<(Option<i32>, usize)> {
            n.and_then(|n| {
                let n = n.borrow();
                if n.val == x {
                    Some((parent, depth))
                } else {
                    let (l, r) = (n.left.as_ref(), n.right.as_ref());
                    let p = Some(n.val);
                    find(x, l, p, depth + 1).or(find(x, r, p, depth + 1))
                }
            })
        }
        let (xp, xd) = find(x, root.as_ref(), None, 0).unwrap();
        let (yp, yd) = find(y, root.as_ref(), None, 0).unwrap();
        xd == yd && xp != yp
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    const N: N = None;
    type N = Option<Rc<RefCell<TreeNode>>>;
    #[rustfmt::skip] fn wrap(n: TreeNode) -> N {Some(Rc::new(RefCell::new(n)))}
    #[rustfmt::skip] fn n(v: i32, l: N, r: N) -> N {wrap(TreeNode{val: v, left: l, right: r})}
    #[rustfmt::skip] fn l(v: i32            ) -> N {wrap(TreeNode{val: v, left: N, right: N})}

    #[test]
    fn r_1_2_3_4_x_4_y_3() {
        let r = n(1, n(2, l(4), N), l(3));
        assert!(!Solution::is_cousins(r, 4, 3));
    }
    #[test]
    fn r_1_2_3_n_4_n_5_x_5_y_4() {
        let r = n(1, n(2, N, l(4)), n(3, N, l(5)));
        assert!(Solution::is_cousins(r, 5, 4));
    }
    #[test]
    fn r_1_2_3_n_4_x_2_y_3() {
        let r = n(1, n(2, N, l(4)), l(3));
        assert!(!Solution::is_cousins(r, 2, 3));
    }

    #[test]
    fn r_1_n_2_3_n_n_4_n_5_x_1_y_3() {
        let r = n(1, N, n(2, n(3, N, n(4, N, l(5))), N));
        assert!(!Solution::is_cousins(r, 1, 3));
    }
}
