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
    pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
}

/// # Construct Binary Tree from String
///
/// You need to construct a binary tree from a string consisting of parenthesis and integers.
///
/// The whole input represents a binary tree.
/// It contains an integer followed by zero, one or two pairs of parenthesis.
/// The integer represents the root's value and a pair of parenthesis contains
/// a child binary tree with the same structure.
///
/// You always start to construct the __left__ child node of the parent first if it exists.
///
/// __Constraints:__
///
/// - `0 <= s.length <= 30_000`
/// - `s` consists of digits, `'('`, `')'`, and `'-'` only.
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/590/week-3-march-15th-march-21st/3672/
struct Solution;
//noinspection DuplicatedCode
impl Solution {
    pub fn str2tree(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        Self::bytes2tree(s.as_bytes())
    }
    fn bytes2tree(bs: &[u8]) -> Option<Rc<RefCell<TreeNode>>> {
        if bs.len() == 0 {
            None
        } else if let Some((l, _))  = bs.iter().enumerate().find(|&(_, &b)| b == b'(') {
            let mut r = l + 1;
            let mut parens = 1;
            while parens > 0 {
                match bs[r] {
                    b'(' => parens += 1,
                    b')' => parens -= 1,
                    _ => (),
                }
                r += 1;
            }
            let left = Self::bytes2tree(&bs[l + 1..r - 1]);
            let right = if r == bs.len() {
                None
            } else {
                Self::bytes2tree(&bs[r + 1..bs.len() - 1])
            };
            let val = std::str::from_utf8(&bs[..l]).unwrap().parse::<i32>().unwrap();
            Self::orr(TreeNode { val, left, right })
        } else {
            let val = std::str::from_utf8(bs).unwrap().parse::<i32>().unwrap();
            Self::orr(TreeNode::new(val))
        }
    }
    fn orr(n: TreeNode) -> Option<Rc<RefCell<TreeNode>>> { Some(Rc::new(RefCell::new(n))) }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Node = Option<Rc<RefCell<TreeNode>>>;
    fn wrap(n: TreeNode) -> Node { Some(Rc::new(RefCell::new(n))) }
    fn n(v: i32) -> Node { wrap(TreeNode::new(v)) }
    fn nl(v: i32, l: Node) -> Node { wrap(TreeNode { val: v, left: l, right: None }) }
    fn nlr(v: i32, l: Node, r: Node) -> Node { wrap(TreeNode { val: v, left: l, right: r }) }

    #[test]
    fn example1() {
        let s = "4(2(3)(1))(6(5))".to_string();
        let e = nlr(4, nlr(2, n(3), n(1)), nl(6, n(5)));
        assert_eq!(Solution::str2tree(s), e);
    }
    #[test]
    fn example2() {
        let s = "4(2(3)(1))(6(5)(7))".to_string();
        let e = nlr(4, nlr(2, n(3), n(1)), nlr(6, n(5), n(7)));
        assert_eq!(Solution::str2tree(s), e);
    }
    #[test]
    fn example3() {
        let s = "-4(2(3)(1))(6(5)(7))".to_string();
        let e = nlr(-4, nlr(2, n(3), n(1)), nlr(6, n(5), n(7)));
        assert_eq!(Solution::str2tree(s), e);
    }

    #[test]fn test_s_empty() { assert_eq!(Solution::str2tree("".to_string()), None); }
    #[test]fn test_s4() { assert_eq!(Solution::str2tree("4".to_string()), n(4)); }
    #[test]fn test_sm4() { assert_eq!(Solution::str2tree("-4".to_string()), n(-4)); }
    #[test]fn test_s1p2() { assert_eq!(Solution::str2tree("1(2)".to_string()), nl(1, n(2))); }
    #[test]fn test_s1p2p3() { assert_eq!(Solution::str2tree("1(2)(3)".to_string()), nlr(1, n(2), n(3))); }
}
