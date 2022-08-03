#![allow(dead_code)]
//! \#366. Find Leaves of Binary Tree
//! =================================
//!
//! <https://leetcode.com/problems/find-leaves-of-binary-tree>
//!
//! Given the `root` of a binary tree, collect a tree's nodes as if you were doing this:
//!
//! - Collect all the leaf nodes.
//! - Remove all the leaf nodes.
//! - Repeat until the tree is empty.
//!
//! ##### Constraints:
//!
//! - The number of nodes in the tree is in the range `[1, 100]`.
//! - `-100 <= Node.val <= 100`
//!
//! ##### Examples
//!
//! ```
//! # use c2022_08::c2022_08_w1::*;
//! # macro_rules! vv { ($($t:tt),*) => { vec![$(vec!$t),*] }; }
//! assert_eq!(Solution::find_leaves(l(1)), vv![[1]]);
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_w1::*;
//! # macro_rules! vv { ($($t:tt),*) => { vec![$(vec!$t),*] }; }
//! //     1
//! //    / \       1
//! //   2   3 =>  /  => 1
//! //  / \       2
//! // 4   5
//! let root = t(1, t(2, l(4), l(5)), l(3));
//! let expected = vv![[4,5,3],[2],[1]];
//! assert_eq!(Solution::find_leaves(root), expected);
//! ```
//!
//! __Explanation:__
//! `[[3,5,4],[2],[1]]` and `[[3,4,5],[2],[1]]` are also considered correct answers
//! since per each level it does not matter the order on which elements are returned.
//!
//! ```
//! # use c2022_08::c2022_08_w1::*;
//! # macro_rules! vv { ($($t:tt),*) => { vec![$(vec!$t),*] }; }
//! //      1
//! //    /   \       1
//! //   2     2 =>  / \  => 1
//! //  / \   / \   2   2
//! // 3   4 3   4
//! let root = t(1, t(2, l(3), l(4)), t(2, l(3), l(4)));
//! let expected = vv![[3,4,3,4], [2,2], [1]];
//! assert_eq!(Solution::find_leaves(root), expected);
//! ```

use std::cell::RefCell;
use std::iter::once;
use std::rc::Rc;

type T = Option<Rc<RefCell<TreeNode>>>;

pub struct Solution;
impl Solution {
    /// Cut leaves layer-by-layer (mutate input)
    ///
    /// - Time: `O(n²)`
    /// - Memory: `O(n)`
    pub fn find_leaves_v1(mut root: T) -> Vec<Vec<i32>> {
        fn is_leaf(root: &T) -> Option<bool> {
            root.as_ref()
                .map(|node| node.borrow())
                .map(|borrow| borrow.left.is_none() && borrow.right.is_none())
        }

        fn push_stack_or_result(
            node: &mut T,
            stack: &mut Vec<Rc<RefCell<TreeNode>>>,
            result: &mut Vec<i32>,
        ) {
            match is_leaf(&node) {
                Some(true) => result.push(node.take().unwrap().borrow().val),
                Some(false) => stack.push(node.clone().unwrap()),
                _ => (),
            }
        }

        fn cut_leaves(root: &mut T) -> Vec<i32> {
            let mut result = vec![];
            let mut stack = vec![];
            push_stack_or_result(root, &mut stack, &mut result);
            while let Some(node) = stack.pop() {
                let mut node = node.borrow_mut();
                push_stack_or_result(&mut node.left, &mut stack, &mut result);
                push_stack_or_result(&mut node.right, &mut stack, &mut result);
            }
            result
        }

        let mut results = vec![];
        while root.is_some() {
            results.push(cut_leaves(&mut root));
        }

        results
    }

    /// Copy tree to a vector and cut its leaves
    ///
    /// - Time: `O(n²)`
    /// - Memory: `O(n)`
    pub fn find_leaves_v2(root: T) -> Vec<Vec<i32>> {
        fn tree_to_vec(root: T) -> Vec<Option<i32>> {
            let mut result = vec![];
            let mut curr = vec![];

            fn add(node: T, result: &mut Vec<Option<i32>>, next: &mut Vec<T>) -> bool {
                if let Some(node) = node {
                    let node = node.borrow();
                    result.push(Some(node.val));
                    next.push(node.left.clone());
                    next.push(node.right.clone());

                    node.left.is_some() || node.right.is_some()
                } else {
                    result.push(None);
                    next.push(None);
                    next.push(None);

                    false
                }
            }

            if add(root, &mut result, &mut curr) {
                curr.reverse();
            } else {
                return result;
            }

            while !curr.is_empty() {
                let mut next = vec![];
                let mut seen_some = false;
                while let Some(node) = curr.pop() {
                    seen_some |= add(node, &mut result, &mut next);
                }
                if seen_some {
                    next.reverse();
                    curr = next;
                }
            }

            result
        }

        fn children(i: usize) -> impl Iterator<Item = usize> {
            once(i * 2 + 1).chain(once(i * 2 + 2))
        }

        fn is_leaf(i: usize, tree: &[Option<i32>]) -> Option<bool> {
            tree[i].map(|_| children(i).all(|i| i >= tree.len() || tree[i].is_none()))
        }

        fn cut_leaves(tree: &mut Vec<Option<i32>>) -> Vec<i32> {
            let mut result = vec![];
            let mut stack = vec![];
            if let Some(true) = is_leaf(0, tree) {
                result.push(tree[0].take().unwrap());
            } else {
                stack.push(0);
            }
            while let Some(i) = stack.pop() {
                children(i).for_each(|i| match is_leaf(i, tree) {
                    Some(true) => {
                        result.push(tree[i].unwrap());
                        tree[i] = None;
                    }
                    Some(false) => stack.push(i),
                    _ => (),
                })
            }

            result
        }

        let mut tree = tree_to_vec(root.clone());
        let mut result = vec![];
        while tree[0].is_some() {
            result.push(cut_leaves(&mut tree));
        }
        result
    }

    /// Post-order recursion
    ///
    /// - Time: `O(n)`
    /// - Memory: `O(n)` call stack
    pub fn find_leaves(root: T) -> Vec<Vec<i32>> {
        fn get_hight(root: T, result: &mut Vec<Vec<i32>>) -> i32 {
            if let Some(node) = root {
                let node = node.borrow();
                let left_hight = get_hight(node.left.clone(), result);
                let right_hight = get_hight(node.right.clone(), result);
                let height = left_hight.max(right_hight) + 1;
                while result.len() <= height as usize {
                    result.push(vec![]);
                }
                result[height as usize].push(node.val);
                height
            } else {
                -1
            }
        }

        let mut result = vec![];
        get_hight(root, &mut result);
        result
    }
}

#[rustfmt::skip] fn wrap(t: TreeNode) -> T { Some(Rc::new(RefCell::new(t))) }

pub const N: T = None;

#[rustfmt::skip] pub fn l(val: i32                   ) -> T { wrap(TreeNode::new(val)) }
#[rustfmt::skip] pub fn t(val: i32, left: T, right: T) -> T { wrap(TreeNode { val, left, right }) }

#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode { pub val: i32, pub left: T, pub right: T }

#[rustfmt::skip]
impl TreeNode {
    #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
}
