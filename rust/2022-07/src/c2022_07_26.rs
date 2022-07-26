#![allow(dead_code)]
//! \#236. Lowest Common Ancestor of a Binary Tree
//! ==============================================
//!
//! <https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree>
//!
//! Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
//!
//! According to the [definition of LCA on Wikipedia](https://en.wikipedia.org/wiki/Lowest_common_ancestor):
//! “The lowest common ancestor is defined between two nodes `p` and `q` as the lowest node in `T`
//! that has both `p` and `q` as descendants (where we allow __a node to be a descendant of itself__).”
//!
//! __Constraints:__
//!
//! - The number of nodes in the tree is in the range `[2, 100_000]`.
//! - `-1_000_000_000 <= Node.val <= 1_000_000_000`
//! - All `Node.val` are __unique__.
//! - `p != q`
//! - `p` and `q` will exist in the tree.

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

type T = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn lowest_common_ancestor_v1(root: T, p: T, q: T) -> T {
        fn find_ancestors(node: &T, root: &T) -> Vec<i32> {
            fn bts(target: i32, root: &T, ansestors: &mut Vec<i32>) -> Option<Vec<i32>> {
                if let Some(t) = root {
                    let t = t.borrow();
                    ansestors.push(t.val);
                    let result = if t.val == target {
                        Some(ansestors.clone())
                    } else {
                        bts(target, &t.left, ansestors).or(bts(target, &t.right, ansestors))
                    };
                    ansestors.pop();
                    result
                } else {
                    None
                }
            }

            bts(node.as_ref().unwrap().borrow().val, root, &mut Vec::new()).unwrap()
        }

        fn find_node(target: i32, root: &T) -> T {
            if let Some(t) = root {
                let t = t.borrow();
                if t.val == target {
                    root.clone()
                } else {
                    find_node(target, &t.left).or(find_node(target, &t.right))
                }
            } else {
                None
            }
        }

        let p = find_ancestors(&p, &root);
        let q = find_ancestors(&q, &root);
        let lowest_common = p
            .iter()
            .zip(q.iter())
            .take_while(|(a, b)| a == b)
            .last()
            .map(|(&a, _)| a)
            .unwrap();

        find_node(lowest_common, &root)
    }

    pub fn lowest_common_ancestor_v2(root: T, p: T, q: T) -> T {
        fn contains(root: &T, target: i32) -> bool {
            if let Some(t) = root {
                let t = t.borrow();
                t.val == target || contains(&t.left, target) || contains(&t.right, target)
            } else {
                false
            }
        }

        fn rec(root: &T, p: i32, q: i32) -> (T, bool, bool) {
            if let Some(t) = root {
                let t = t.borrow();
                if (t.val == p && contains(&root, q)) || (t.val == q && contains(&root, p)) {
                    (root.clone(), true, true)
                } else if t.val == p {
                    (None, true, false)
                } else if t.val == q {
                    (None, false, true)
                } else {
                    match (rec(&t.left, p, q), rec(&t.right, p, q)) {
                        (l @ (_, true, true), _) => l,
                        (_, r @ (_, true, true)) => r,
                        ((_, true, _), (_, _, true)) => (root.clone(), true, true),
                        ((_, _, true), (_, true, _)) => (root.clone(), true, true),
                        ((_, lp, lq), (_, rp, rq)) => (None, lp || rp, lq || rq),
                    }
                }
            } else {
                (None, false, false)
            }
        }

        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;

        rec(&root, p, q).0
    }

    pub fn lowest_common_ancestor(root: T, p: T, q: T) -> T {
        enum Contains {
            Neither,
            P,
            Q,
            Both(T),
        }
        impl Contains {
            fn unwrap(self) -> T {
                match self {
                    Contains::Both(t) => t,
                    _ => panic!("Not found"),
                }
            }

            fn or_else<F>(self, other: F, root: &T) -> Self
            where
                F: FnOnce() -> Self,
            {
                use Contains::*;

                if let result @ Both(_) = self {
                    result
                } else {
                    match (self, other()) {
                        (_, result @ Both(_)) => result,
                        (Q, P) | (P, Q) => Both(root.clone()),
                        (Q, _) | (_, Q) => Q,
                        (P, _) | (_, P) => P,
                        _ => Neither,
                    }
                }
            }
        }

        fn contains(root: &T, target: i32) -> bool {
            if let Some(t) = root {
                let t = t.borrow();
                t.val == target || contains(&t.left, target) || contains(&t.right, target)
            } else {
                false
            }
        }

        fn rec(root: &T, p: i32, q: i32) -> Contains {
            if let Some(t) = root {
                let t = t.borrow();
                if (t.val == p && contains(root, q)) || (t.val == q && contains(root, p)) {
                    Contains::Both(root.clone())
                } else if t.val == p {
                    Contains::P
                } else if t.val == q {
                    Contains::Q
                } else {
                    rec(&t.left, p, q).or_else(|| rec(&t.right, p, q), root)
                }
            } else {
                Contains::Neither
            }
        }

        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;

        rec(&root, p, q).unwrap()
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq)]
pub struct TreeNode { pub val: i32, pub left: T, pub right: T }

#[rustfmt::skip]
impl TreeNode {
    #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
}

impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const START: char = '[';
        const SEPARATOR: char = ',';
        const LEVEL: char = '|';
        const EMPTY: char = '_';
        const END: char = ']';

        let mut result = String::new();
        result.push(START);
        result.push_str(&self.val.to_string());
        let mut curr = vec![];
        let mut next = vec![];
        if self.left.is_some() || self.right.is_some() {
            result.push(LEVEL);
            curr.push(self.left.clone());
            curr.push(self.right.clone());
        }
        while !curr.is_empty() {
            let mut has_next = false;
            for i in 0..curr.len() {
                if let Some(t) = &curr[i] {
                    let t = t.borrow();
                    result.push_str(&t.val.to_string());
                    next.push(t.left.clone());
                    next.push(t.right.clone());
                    if t.left.is_some() || t.right.is_some() {
                        has_next = true;
                    }
                } else {
                    result.push(EMPTY);
                    next.push(None);
                    next.push(None);
                }
                result.push(SEPARATOR);
            }
            curr.clear();
            result.pop();
            if has_next {
                result.push(LEVEL);
                std::mem::swap(&mut curr, &mut next);
            }
        }
        result.push(END);
        f.write_str(&result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: T = None;
    #[rustfmt::skip] fn l(val: i32)                    -> T { wrap(TreeNode::new(val)) }
    #[rustfmt::skip] fn t(val: i32, left: T, right: T) -> T { wrap(TreeNode { val, left, right }) }

    #[rustfmt::skip] fn wrap(t: TreeNode) -> T { Some(Rc::new(RefCell::new(t))) }

    #[test]
    fn r_1_2_p_1_q_2() {
        let q = l(2);
        let p = t(1, q.clone(), N);
        let root = p.clone();
        let expected = p.clone();
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), expected);
    }
    #[test]
    fn r_1_n_2_p_1_q_2() {
        let q = l(2);
        let p = t(1, N, q.clone());
        let root = p.clone();
        let expected = p.clone();
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), expected);
    }

    #[test]
    fn r_3_5_1_6_2_0_8_n_n_7_4_p_5_q_1() {
        //      3
        //   5     1
        // 6  2   0 8
        //   7 4
        let p = t(5, l(6), t(2, l(7), l(4)));
        let q = t(1, l(0), l(8));
        let root = t(3, p.clone(), q.clone());
        let expected = root.clone();
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), expected);
        // Explanation: The LCA of nodes 5 and 1 is 3.
    }
    #[test]
    fn r_3_5_1_6_2_0_8_n_n_7_4_p_5_q_4() {
        //      3
        //   5     1
        // 6  2   0 8
        //   7 4
        let q = l(4);
        let p = t(5, l(6), t(2, l(7), q.clone()));
        let root = t(3, p.clone(), t(1, l(0), l(8)));
        let expected = p.clone();
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), expected);
        // Explanation: The LCA of nodes 5 and 4 is 5,
        // since a node can be a descendant of itself according to the LCA definition.
    }
    #[test]
    fn r_3_5_1_6_2_0_8_n_n_7_4_p_7_q_0() {
        //      3
        //   5     1
        // 6  2   0 8
        //   7 4
        let q = l(0);
        let p = l(7);
        let root = t(3, t(5, l(6), t(2, p.clone(), l(4))), t(1, q.clone(), l(8)));
        let expected = root.clone();
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), expected);
    }
    #[test]
    fn r_3_5_1_6_2_0_8_n_n_7_4_p_7_q_4() {
        //      3
        //   5     1
        // 6  2   0 8
        //   7 4
        let q = l(4);
        let p = l(7);
        let expected = t(2, p.clone(), q.clone());
        let root = t(3, t(5, l(6), expected.clone()), t(1, l(0), l(8)));
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), expected);
    }
}
