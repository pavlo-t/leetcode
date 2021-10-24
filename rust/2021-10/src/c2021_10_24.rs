#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 222. Count Complete Tree Nodes
/// ==============================
///
/// Given the `root` of a __complete__ binary tree, return the number of the nodes in the tree.
///
/// According to [Wikipedia](http://en.wikipedia.org/wiki/Binary_tree#Types_of_binary_trees),
/// every level, except possibly the last, is completely filled in a complete binary tree,
/// and all nodes in the last level are as far left as possible.
/// It can have between `1` and `2h` nodes inclusive at the last level `h`.
///
/// Design an algorithm that runs in less than `O(n)` time complexity.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[0, 50_000]`.
/// - `0 <= Node.val <= 50_000`
/// - The tree is guaranteed to be __complete__.
///
/// https://leetcode.com/problems/count-complete-tree-nodes/
struct Solution;
impl Solution {
    /// Approach 2: Binary search
    /// https://leetcode.com/problems/count-complete-tree-nodes/solution/
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        println!("count_nodes({:?})", root);
        type N = Option<Rc<RefCell<TreeNode>>>;
        fn height_l(n: &N) -> usize {
            let mut curr = n.clone();
            let mut h = 0;
            while let Some(n) = curr {
                h += 1;
                curr = n.borrow().left.clone();
            }
            h
        }
        /// Last level nodes are enumerated from 0 to 2**(h - 1) - 1.
        /// Return true if last level node `i` exists.
        /// Binary search with `O(h)` complexity.
        fn exists(n: &N, h: usize, i: usize) -> bool {
            let (mut l, mut r) = (0, 2usize.pow(h as u32 - 1) - 1);
            let mut curr = n.clone();
            for _ in 1..h {
                let m = l + (r - l) / 2;
                if m >= i {
                    curr = curr.unwrap().borrow().left.clone();
                    r = m;
                } else {
                    curr = curr.unwrap().borrow().right.clone();
                    l = m + 1;
                }
            }
            curr.is_some()
        }
        /// Last level nodes are enumerated from 0 to 2**(h - 1) - 1.
        /// Perform binary search to check how many nodes exist.
        fn count_last_level(n: &N, h: usize) -> usize {
            let (mut l, mut r) = (1, 2usize.pow(h as u32 - 1) - 1);
            while l <= r {
                let m = l + (r - l) / 2;
                if exists(n, h, m) {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }
            l
        }

        let hl = height_l(&root);
        if hl < 2 {
            hl as i32
        } else {
            // The tree contains 2**(h - 1) - 1 nodes on the first (h - 1) levels + last level.
            2i32.pow(hl as u32 - 1) - 1 + count_last_level(&root, hl) as i32
        }
    }
    /// 05:49-06:18
    pub fn count_nodes_my(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        println!("count_nodes({:?})", root);
        fn height_l(n: Option<&Rc<RefCell<TreeNode>>>) -> usize {
            n.map(|n| 1 + height_l(n.borrow().left.as_ref()))
                .unwrap_or(0)
        }
        fn height_r(n: Option<&Rc<RefCell<TreeNode>>>) -> usize {
            n.map(|n| 1 + height_r(n.borrow().right.as_ref()))
                .unwrap_or(0)
        }
        fn count(n: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
            n.map(|n| {
                let n = n.borrow();
                let hl = height_l(n.left.as_ref());
                let hr = height_r(n.right.as_ref());
                if hl == hr {
                    (1 << (hl + 1)) - 1
                } else {
                    1 + count(n.left.as_ref()) + count(n.right.as_ref())
                }
            })
            .unwrap_or(0)
        }
        count(root.as_ref())
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
    #[rustfmt::skip] #[inline]    pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: N = None;
    type N = Option<Rc<RefCell<TreeNode>>>;
    #[rustfmt::skip] fn wrap(n: TreeNode) -> N {Some(Rc::new(RefCell::new(n)))}
    #[rustfmt::skip] fn n(v: i32, l: N, r: N) -> N {wrap(TreeNode{val:v,left:l,right:r})}
    #[rustfmt::skip] fn l(v: i32            ) -> N {wrap(TreeNode::new(v))}

    #[test]
    fn r_n() {
        assert_eq!(Solution::count_nodes(N), 0);
    }
    #[test]
    fn r_1() {
        assert_eq!(Solution::count_nodes(l(1)), 1);
    }
    #[test]
    fn r_1_2() {
        let r = n(1, l(2), N);
        assert_eq!(Solution::count_nodes(r), 2);
    }
    #[test]
    fn r_1_2_3() {
        let r = n(1, l(2), l(3));
        assert_eq!(Solution::count_nodes(r), 3);
    }
    #[test]
    fn r_1_2_3_4() {
        let r = n(1, n(2, l(4), N), l(3));
        assert_eq!(Solution::count_nodes(r), 4);
    }
    #[test]
    fn r_1_2_3_4_5() {
        let r = n(1, n(2, l(4), l(5)), l(3));
        assert_eq!(Solution::count_nodes(r), 5);
    }
    #[test]
    fn r_1_2_3_4_5_6() {
        let r = n(1, n(2, l(4), l(5)), n(3, l(6), N));
        assert_eq!(Solution::count_nodes(r), 6);
    }
    #[test]
    fn r_1_2_3_4_5_6_7() {
        let r = n(1, n(2, l(4), l(5)), n(3, l(6), l(7)));
        assert_eq!(Solution::count_nodes(r), 7);
    }
}
