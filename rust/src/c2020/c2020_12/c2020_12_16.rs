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


struct Solution;

//noinspection DuplicatedCode
impl Solution {
    // Inorder iteration
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = Vec::new();
        let mut root = root;
        let mut prev = None;

        while !stack.is_empty() || root.is_some() {
            while let Some(node) = root {
                root = node.borrow().left.clone();
                stack.push(node);
            }

            let curr = stack.pop().unwrap();
            let val = curr.borrow().val;

            match prev {
                Some(p) if val <= p => return false,
                _ => {
                    prev = Some(val);
                    root = curr.borrow().right.clone();
                }
            }
        }

        true
    }

    pub fn is_valid_bst_inorder_iteration_1(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = Vec::new();
        let mut root = root;
        let mut prev = None;

        while !stack.is_empty() || root.is_some() {
            while let Some(node) = root {
                root = node.borrow().left.clone();
                stack.push(node);
            }

            let curr = stack.pop().unwrap();
            let val = curr.borrow().val;
            if prev.is_some() && val <= prev.unwrap() {
                return false;
            }

            prev = Some(val);
            root = curr.borrow().right.clone();
        }

        true
    }

    // Inorder recursive traversal
    pub fn is_valid_bst_recursive_inorder(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>) -> bool {
            if let Some(node) = node {
                if !inorder(&node.borrow().left, prev) {
                    return false
                }

                let val = node.borrow().val;
                match prev {
                    Some(p) if &val <= p =>
                        return false,
                    Some(p) =>
                        *p = val,
                    None =>
                        *prev = Some(val),
                }

                inorder(&node.borrow().right, prev)
            } else {
                true
            }
        }

        inorder(&root, &mut None)
    }

    pub fn is_valid_bst_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn validate(node: &Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
            match node {
                None => true,
                Some(node) => {
                    let val = node.borrow().val;
                    if min.map(|min| val <= min).unwrap_or_default() || max.map(|max| val >= max).unwrap_or_default() {
                        false
                    } else {
                        let left = &node.borrow().left;
                        let right = &node.borrow().right;
                        validate(left, min, Some(val)) && validate(right, Some(val), max)
                    }
                }
            }
        }

        validate(&root, None, None)
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
    fn example1_r213_should_be_valid() {
        let root = nlr(2, n(1), n(3));
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn example2_r514nn36_should_be_invalid() {
        let root = nlr(5, n(1), nlr(4, n(3), n(6)));
        assert!(!Solution::is_valid_bst(root));
    }

    #[test]
    fn r1_should_be_valid() {
        let root = n(1);
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn r_i32max_should_be_valid() {
        let root = n(i32::max_value());
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn r_i32min_should_be_valid() {
        let root = n(i32::min_value());
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn r_i32max1n_should_be_valid() {
        let root = nl(i32::max_value(), n(1));
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn r_i32minn1_should_be_valid() {
        let root = nr(i32::min_value(), n(1));
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn r517nn38_should_be_invalid() {
        let root = nlr(5, n(1), nlr(7, n(3), n(8)));
        assert!(!Solution::is_valid_bst(root));
    }

    #[test]
    fn r11_should_be_invalid() {
        let root = nl(1, n(1));
        assert!(!Solution::is_valid_bst(root));
    }

    #[test]
    fn r111_should_be_invalid() {
        let root = nlr(1, n(1), n(1));
        assert!(!Solution::is_valid_bst(root));
    }

    #[test]
    fn r211_should_be_invalid() {
        let root = nlr(2, n(1), n(1));
        assert!(!Solution::is_valid_bst(root));
    }

    #[test]
    fn r221_should_be_invalid() {
        let root = nlr(2, n(2), n(1));
        assert!(!Solution::is_valid_bst(root));
    }

    #[test]
    fn r122_should_be_invalid() {
        let root = nlr(2, n(1), n(2));
        assert!(!Solution::is_valid_bst(root));
    }

    #[test]
    fn r4261357_should_be_valid() {
        let root = nlr(4, nlr(2, n(1), n(3)), nlr(6, n(5), n(7)));
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn r10000_valid_nodes_should_be_valid() {
        fn build_valid_bst(left: i32, right: i32) -> Node {
            if left > right {
                None
            } else {
                let mid = left + (right - left) / 2;
                let l = build_valid_bst(left, mid - 1);
                let r = build_valid_bst(mid + 1, right);
                nlr(mid, l, r)
            }
        }

        let root = build_valid_bst(1, 10000);
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn r10000_invalid_nodes_should_be_invalid() {
        fn build_invalid_bst(left: i32, right: i32) -> Node {
            if left > right {
                n(left - 1)
            } else {
                let mid = left + (right - left) / 2;
                let l = build_invalid_bst(left, mid - 1);
                let r = build_invalid_bst(mid + 1, right);
                nlr(mid, l, r)
            }
        }

        let root = build_invalid_bst(1, 10000);
        assert!(!Solution::is_valid_bst(root));
    }

    // #[test]
    // fn test_mutable_option_as_parameter() {
    //     fn rec(val: &mut Option<i32>) -> bool {
    //         println!("rec({:?})", val);
    //         if let Some(v) = val {
    //             if *v >= 3 {
    //                 true
    //             } else {
    //                 *v += 1;
    //                 rec(val)
    //             }
    //         } else {
    //             *val = Some(1);
    //             rec(val)
    //         }
    //     }
    //
    //     let mut opt = None;
    //     assert!(rec(&mut opt));
    //     assert_eq!(opt, Some(3));
    // }
}
