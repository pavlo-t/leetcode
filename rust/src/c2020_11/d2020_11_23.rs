#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

// @formatter:off
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

struct Solution;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root).0
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            None => (0, 0),
            Some(rcn) => {
                let n = rcn.borrow();
                let l = Self::dfs(&n.left);
                let r = Self::dfs(&n.right);

                ((n.val + l.1 + r.1).max(l.0 + r.0), l.0 + r.0)
            }
        }
    }
}

struct SolutionMy;
impl SolutionMy {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::HashMap;

        fn stringify_tn(tn: &Option<Rc<RefCell<TreeNode>>>) -> String {
            if let Some(rcn) = tn {
                let n = rcn.borrow();
                format!("{},{},{}", n.val, stringify_tn(&n.left), stringify_tn(&n.right))
            } else { "".to_string() }
        }

        struct Robbery { cache: HashMap<(bool, String), i32> }
        impl Robbery {
            fn rob(&mut self, can_rob: bool, root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
                let k = (can_rob, stringify_tn(root));
                if let Some(&v) = self.cache.get(&k) {
                    v
                } else if let Some(rcn) = root {
                    let n = rcn.borrow();

                    let rl = self.rob(true, &n.left);
                    let rr = self.rob(true, &n.right);
                    let nrl = self.rob(false, &n.left);
                    let nrr = self.rob(false, &n.right);

                    let result = (if can_rob { n.val } else { 0 } + nrl + nrr).max(rl + rr);

                    self.cache.insert(k, result);

                    result
                } else {
                    self.cache.insert(k, 0);
                    0
                }
            }
        }

        (Robbery { cache: HashMap::new() }).rob(true, &root)
    }
}
// @formatter:off

//noinspection DuplicatedCode
#[cfg(test)]
mod tests {
    use super::*;

fn wrap_tn(n: TreeNode) -> Rc<RefCell<TreeNode>> { Rc::new(RefCell::new(n)) }
    fn tn(v: i32, l: Option<TreeNode>, r: Option<TreeNode>) -> TreeNode {
        TreeNode { val: v, left: l.map(wrap_tn), right: r.map(wrap_tn) }
    }

    fn nlr(v: i32, l: TreeNode, r: TreeNode) -> TreeNode { tn(v, Some(l), Some(r)) }
    fn nl(v: i32, l: TreeNode) -> TreeNode { tn(v, Some(l), None) }
    fn nr(v: i32, r: TreeNode) -> TreeNode { tn(v, None, Some(r)) }
    fn n(v: i32) -> TreeNode { tn(v, None, None) }

    #[test]
    fn test_3_2_3_n_3_n_1_is_7() {
        let root =
            nlr(3,
                nr(2, n(3)),
                nr(3, n(1)));
        assert_eq!(Solution::rob(Some(wrap_tn(root))), 7);
    }
    #[test]
    fn test_3_4_5_1_3_n_1_is_9() {
        let root =
            nlr(3,
                nlr(4, n(1), n(3)),
                nr(5, n(1)));
        assert_eq!(Solution::rob(Some(wrap_tn(root))), 9);
    }

    #[test]
    fn test_n_is_0() {
        assert_eq!(Solution::rob(None), 0);
    }
    #[test]
    fn test_3_is_3() {
        let root = n(3);
        assert_eq!(Solution::rob(Some(wrap_tn(root))), 3);
    }
    #[test]
    fn test_1to15_is_80() {
        let root =
            nlr(8,
                nlr(4,
                    nlr(2, n(1), n(3)),
                    nlr(6, n(5),n(7))),
                nlr(12,
                    nlr(10, n(9), n(11)),
                    nlr(14, n(13),n(15))));
        assert_eq!(Solution::rob(Some(wrap_tn(root))), 80);
    }
    #[test]
    fn test_15nodes_root10_other0_is_10() {
        let root =
            nlr(10,
                nlr(0,
                    nlr(0, n(0), n(0)),
                    nlr(0, n(0),n(0))),
                nlr(0,
                    nlr(0, n(0), n(0)),
                    nlr(0, n(0),n(0))));
        assert_eq!(Solution::rob(Some(wrap_tn(root))), 10);
    }
    #[test]
    fn test_15nodes0or10_is_50() {
        let root =
            nlr(10,
                nlr(0,
                    nlr(0, n(10), n(0)),
                    nlr(10, n(0),n(0))),
                nlr(0,
                    nlr(10, n(0), n(0)),
                    nlr(0, n(0),n(10))));
        assert_eq!(Solution::rob(Some(wrap_tn(root))), 50);
    }
}
// @formatter:on
