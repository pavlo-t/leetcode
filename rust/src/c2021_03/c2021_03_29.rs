#![allow(dead_code)]

use std::cell::RefCell;
use std::iter;
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

/// # Flip Binary Tree To Match Preorder Traversal
///
/// You are given the `root` of a binary tree with `n` nodes,
/// where each node is uniquely assigned a value from `1` to `n`.
/// You are also given a sequence of `n` values `voyage`,
/// which is the __desired [pre-order]__ traversal of the binary tree.
///
/// Any node in the binary tree can be __flipped__ by swapping its left and right subtrees.
/// For example, flipping node 1 will have the following effect:
///
/// ```txt
///   1            1
///  / \          / \
/// 2   3   =>   3   2
///    / \      / \
///   4   5    4   5
/// ```
///
/// Flip the __smallest__ number of nodes so that the __pre-order traversal__ of the tree
/// __matches__ `voyage`.
///
/// Return _a list of the values of all __flipped__ nodes.
/// You may return the answer in __any order__.
/// If it is __impossible__ to flip the nodes in the tree to make the pre-order traversal match
/// `voyage`, return the list `[-1]`_.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is `n`.
/// - `n == voyage.length`
/// - `1 <= n <= 100`
/// - `1 <= Node.val, voyage[i] <= n`
/// - All the values in the tree are __unique__.
/// - All the values in `voyage` are __unique__.
///
/// [pre-order]:https://en.wikipedia.org/wiki/Tree_traversal#Pre-order
///
/// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/592/week-5-march-29th-march-31st/3689/
struct Solution;
//noinspection DuplicatedCode
impl Solution {
    pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        fn preorder<'a>(
            n: &Option<Rc<RefCell<TreeNode>>>,
            v: &'a [i32],
        ) -> Option<(&'a [i32], Vec<i32>)> {
            n.as_ref().map_or(Some((v, Vec::new())), |n| {
                if n.borrow().val != v[0] {
                    None
                } else {
                    let l = &n.borrow().left;
                    let r = &n.borrow().right;
                    preorder(l, &v[1..])
                        .and_then(|(rem, lr)| preorder(r, rem)
                            .map(|(rem, rr)|
                                (rem, lr.into_iter().chain(rr.into_iter()).collect())))
                        .or_else(|| preorder(r, &v[1..])
                            .and_then(|(rem, rr)| preorder(l, rem)
                                .map(|(rem, lr)|
                                    (rem, iter::once(n.borrow().val)
                                        .chain(rr.into_iter())
                                        .chain(lr.into_iter())
                                        .collect()))))
                }
            })
        }

        match preorder(&root, &voyage) {
            None => vec![-1],
            Some((_, r)) => r,
        }
    }

    pub fn flip_match_voyage_my_1(
        root: Option<Rc<RefCell<TreeNode>>>,
        voyage: Vec<i32>,
    ) -> Vec<i32> {
        fn rec<'a>(
            root: &Option<Rc<RefCell<TreeNode>>>,
            voyage: &'a [i32],
        ) -> (&'a [i32], Vec<i32>) {
            if let Some(n) = root {
                if n.borrow().val != voyage[0] {
                    (&[], vec![-1])
                } else {
                    let (l_voyage, mut l_vec) = rec(&n.borrow().left, &voyage[1..]);
                    if l_vec.is_empty() || l_vec[0] != -1 {
                        let (r_voyage, mut r_vec) = rec(&n.borrow().right, l_voyage);
                        if r_vec.is_empty() || r_vec[0] != -1 {
                            let mut vec = Vec::new();
                            vec.append(&mut l_vec);
                            vec.append(&mut r_vec);
                            (r_voyage, vec)
                        } else {
                            (&[], vec![-1])
                        }
                    } else {
                        let (r_voyage, mut r_vec) = rec(&n.borrow().right, &voyage[1..]);
                        if r_vec.is_empty() || r_vec[0] != -1 {
                            let (l_voyage, mut l_vec) = rec(&n.borrow().left, r_voyage);
                            if l_vec.is_empty() || l_vec[0] != -1 {
                                let mut vec = vec![n.borrow().val];
                                vec.append(&mut r_vec);
                                vec.append(&mut l_vec);
                                (l_voyage, vec)
                            } else {
                                (&[], vec![-1])
                            }
                        } else {
                            (&[], vec![-1])
                        }
                    }
                }
            } else {
                (voyage, Vec::new())
            }
        }

        rec(&root, &voyage).1
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
    fn n(v: i32) -> Node {
        wrap(TreeNode::new(v))
    }

    #[test]
    fn example1() {
        let root = nl(1, n(2));
        let voyage = vec![2, 1];
        assert_eq!(Solution::flip_match_voyage(root, voyage), [-1]);
        // Explanation:
        // It is impossible to flip the nodes such that the pre-order traversal matches voyage.
    }
    #[test]
    fn example2() {
        let root = nlr(1, n(2), n(3));
        let voyage = vec![1, 3, 2];
        assert_eq!(Solution::flip_match_voyage(root, voyage), [1]);
        // Explanation:
        // Flipping node 1 swaps nodes 2 and 3, so the pre-order traversal matches voyage.
    }
    #[test]
    fn example3() {
        let root = nlr(1, n(2), n(3));
        let voyage = vec![1, 2, 3];
        assert_eq!(Solution::flip_match_voyage(root, voyage), []);
        // Explanation:
        // The tree's pre-order traversal already matches voyage, so no nodes need to be flipped.
    }

    #[test]
    fn test1to5() {
        //   1            1
        //  / \          / \
        // 2   3   =>   3   2
        //    / \      / \
        //   4   5    4   5
        let root = nlr(1, n(2), nlr(3, n(4), n(5)));
        let voyage = vec![1, 3, 4, 5, 2];
        assert_eq!(Solution::flip_match_voyage(root, voyage), [1]);
    }
    #[test]
    fn test1to7() {
        //   1                1
        //  / \              / \
        // 2   3     =>     3   2
        //    / \          / \
        //   4   5        5   4
        //      / \      / \
        //     6   7    7   6
        let root = nlr(1, n(2), nlr(3, n(4), nlr(5, n(6), n(7))));
        let voyage = vec![1, 3, 5, 7, 6, 4, 2];
        assert_eq!(Solution::flip_match_voyage(root, voyage), [1, 3, 5]);
    }
}
