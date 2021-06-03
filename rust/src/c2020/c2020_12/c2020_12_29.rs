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

/// ### Pseudo-Palindromic Paths in a Binary Tree
///
/// https://leetcode.com/explore/featured/card/december-leetcoding-challenge/573/week-5-december-29th-december-31st/3585/
struct Solution;

impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn next_path(i: i32, path: i16) -> i16 { path ^ (1 << (i - 1)) }
        fn is_pseudo_palindrome(path: i16) -> bool { path.count_ones() <= 1 }
        // fn is_pseudo_palindrome(path: i16) -> bool { (path & (path - 1)) == 0 }

        let mut q = Vec::new();
        q.push((root, 0));

        let mut result = 0;

        while let Some((n, p)) = q.pop() {
            if let Some(n) = n {
                let n = n.borrow();
                let p = next_path(n.val, p);
                if n.left.is_none() && n.right.is_none() && is_pseudo_palindrome(p) {
                    result += 1;
                } else {
                    q.push((n.left.clone(), p));
                    q.push((n.right.clone(), p));
                }
            }
        }

        result
    }
}

struct SolutionRecursive;

impl SolutionRecursive {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn paths(root: Option<Rc<RefCell<TreeNode>>>, map: i32) -> i32 {
            let node = root.unwrap();
            let node = node.borrow();
            let map = map ^ (1 << node.val);
            if node.left.is_none() && node.right.is_none() {
                return if map.count_ones() <= 1 { 1 } else { 0 }
            }
            let mut ans = 0;
            if node.left.is_some() {
                ans += paths(node.left.clone(), map);
            }
            if node.right.is_some() {
                ans += paths(node.right.clone(), map);
            }
            ans
        }

        paths(root, 0)
    }
}

//noinspection DuplicatedCode
#[cfg(test)]
mod tests {
    use super::*;

    // @formatter:off
    type Node = Option<Rc<RefCell<TreeNode>>>;
    fn wrap(n: TreeNode) -> Node { Some(Rc::new(RefCell::new(n))) }
    fn n(v: i32) -> Node { wrap(TreeNode { val: v, left: None, right: None }) }
    fn nl(v: i32, l: Node) -> Node { wrap(TreeNode { val: v, left: l, right: None }) }
    fn nr(v: i32, r: Node) -> Node { wrap(TreeNode { val: v, left: None, right: r }) }
    fn nlr(v: i32, l: Node, r: Node) -> Node { wrap(TreeNode { val: v, left: l, right: r }) }
    // @formatter:on
    #[test]
    fn example1_r23131n1_is_2() {
        let root = nlr(2, nlr(3, n(3), n(1)), nr(1, n(1)));
        assert_eq!(Solution::pseudo_palindromic_paths(root), 2);
        // Explanation:
        //    2
        //  3   1
        // 3 1   1
        // There are three paths going from the root node to leaf nodes: [2,3,3], [2,1,1], [2,3,1].
        // Path [2,3,3] can be rearranged in [3,2,3] (palindrome)
        // Path [2,1,1] can be rearranged in [1,2,1] (palindrome).
    }

    #[test]
    fn example2_r21113nnnnn1_is_1() {
        let root = nlr(2, nlr(1, n(1), nr(3, n(1))), n(1));
        assert_eq!(Solution::pseudo_palindromic_paths(root), 1);
        // Explanation:
        //    2
        //  1   1
        // 1 3
        //    1
        // There are three paths going from the root node to leaf nodes: [2,1,1], [2,1,3,1], [2,1].
        // Only the path [2,1,1] can be rearranged in [1,2,1] (palindrome).
    }

    #[test]
    fn example3_r9_is_1() {
        assert_eq!(Solution::pseudo_palindromic_paths(n(9)), 1);
    }

    #[test]
    fn example3_r123_is_0() {
        assert_eq!(Solution::pseudo_palindromic_paths(nlr(1, n(2), n(3))), 0);
    }

    #[test]
    fn r131071_balanced_11111111123456789_is_0() {
        fn build_tree(depth: i32) -> Node {
            if depth == 0 { None } else {
                let v = if depth > 9 { 1 } else { depth };
                nlr(v, build_tree(depth - 1), build_tree(depth - 1))
            }
        }
        let root = build_tree(17);
        assert_eq!(Solution::pseudo_palindromic_paths(root), 0);
    }

    #[test]
    fn r131071_balanced_1s_is_a_lot() {
        fn build_tree(depth: i32) -> Node {
            if depth == 0 { None } else { nlr(1, build_tree(depth - 1), build_tree(depth - 1)) }
        }
        let root = build_tree(17);
        assert_eq!(Solution::pseudo_palindromic_paths(root), 65536);
    }

    #[test]
    fn r1to100_000_in_left_branch_is_0() {
        let mut root = None;
        for i in 1..=100_00 {
            root = nl(if i > 9 { 1 } else { i }, root);
        }
        assert_eq!(Solution::pseudo_palindromic_paths(root), 0);
    }

    #[test]
    fn r100_000_1s_in_left_branch_is_1() {
        let mut root = None;
        for _ in 1..=100_00 {
            root = nl(1, root);
        }
        assert_eq!(Solution::pseudo_palindromic_paths(root), 1);
    }

    // #[test]
    // fn write_input() {
    //     use std::fs::File;
    //     use std::io::prelude::*;
    //
    //     fn write(s: &[u8]) -> std::io::Result<()> {
    //         let mut file = File::create("/tmp/downloads/foo.txt")?;
    //         file.write_all(s)?;
    //         Ok(())
    //     }
    //
    //     let mut s = String::from("[1,1,null");
    //     for _ in 1..99999 {
    //         s.push_str(",1,null");
    //     }
    //     s.push(']');
    //     let count = s.as_bytes().iter().filter(|&&c| c == b'1').count();
    //     println!("Got {} 1s", count);
    //     write(s.as_bytes()).expect("too bad");
    // }
}
