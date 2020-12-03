// @formatter:off
use std::rc::Rc;
use std::cell::RefCell;

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
      right: None
    }
  }
}

struct Solution;
// @formatter:on
impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let vals = Self::tree_node_to_vec(&root);

        let mut result = None;
        for val in vals {
            result = Some(Rc::new(RefCell::new(TreeNode { val, left: None, right: result })));
        }

        result
    }

    // reversed values
    fn tree_node_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() { vec![] } else {
            let n = root.as_ref().unwrap().borrow();
            let mut left = Self::tree_node_to_vec(&n.left);
            let mut right = Self::tree_node_to_vec(&n.right);
            right.push(n.val);
            right.append(&mut left);
            right
        }
    }
}
// @formatter:off

#[cfg(test)]
mod tests {
    use super::*;

    type Node = Option<Rc<RefCell<TreeNode>>>;

    fn wrap(n: TreeNode) -> Node { Some(Rc::new(RefCell::new(n))) }

    fn n(v: i32) -> Node { wrap(TreeNode::new(v)) }
    fn nl(v: i32, l: Node) -> Node { wrap(TreeNode { val: v, left: l, right: None }) }
    fn nr(v: i32, r: Node) -> Node { wrap(TreeNode { val: v, left: None, right: r }) }
    fn nlr(v: i32, l: Node, r: Node) -> Node { wrap(TreeNode { val: v, left: l, right: r }) }

    #[test]
    fn example1_9_nodes() {
        let root =
            nlr(5,
                nlr(3, nl(2, n(1)), n(4)),
                nr(6, nlr(8, n(7), n(9))));
        let expected =
            nr(1, nr(2, nr(3, nr(4, nr(5, nr(6, nr(7, nr(8, n(9)))))))));
        assert_eq!(Solution::increasing_bst(root), expected);

    }
    #[test]
    fn example2_3_nodes() {
        let root = nlr(5, n(1), n(7));
        let expected = nr(1, nr(5, n(7)));
        assert_eq!(Solution::increasing_bst(root), expected);
    }
    #[test]
    fn test3_1_node() {
        let root = n(217);
        let expected = n(217);
        assert_eq!(Solution::increasing_bst(root), expected);
    }

    #[test]
    fn test_100_nodes() {
        let mut root = n(1);
        for i in 2..=100 { root = nl(i, root); }
        let mut expected = n(100);
        for i in (1..100).rev() { expected = nr(i, expected); }

        assert_eq!(Solution::increasing_bst(root), expected);
    }
}
