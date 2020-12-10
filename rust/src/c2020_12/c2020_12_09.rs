#![allow(dead_code)]

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

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::LinkedList;

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut bst_iterator = Self { stack: Vec::new() };
        bst_iterator.push_left_on_stack(root);
        bst_iterator
    }

    fn push_left_on_stack(&mut self, root: Option<Rc<RefCell<TreeNode>>>) {
        let mut head = root;

        while let Some(node) = head {
            self.stack.push(node.clone());
            head = node.borrow().left.clone();
        }
    }

    fn next(&mut self) -> i32 {
        let node = self.stack.pop().expect("Called next() on an empty BSTIterator!");
        let val = node.borrow().val;

        self.push_left_on_stack(node.borrow().right.clone());

        val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

// @formatter:off
struct BSTIteratorEnqueueAll {
    stack: LinkedList<i32>
}
impl BSTIteratorEnqueueAll {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        fn rec(root: Option<&Rc<RefCell<TreeNode>>>, rsf: LinkedList<i32>) -> LinkedList<i32> {
            match root {
                None => rsf,
                Some(tn) => {
                    let tn = tn.borrow();
                    let mut rsf = rec(tn.left.as_ref(), rsf);
                    rsf.push_back(tn.val);
                    rec(tn.right.as_ref(), rsf)
                }
            }
        }
        let stack = rec(root.as_ref(), LinkedList::new());

        Self { stack }
    }

    fn next(&mut self) -> i32 {
        let n = self.stack.pop_front().expect("Called next() on an empty BSTIterator!");
        n
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}
// @formatter:on

#[cfg(test)]
mod tests {
    // @formatter:off
    use super::*;

    type Node = Option<Rc<RefCell<TreeNode>>>;

    fn wrap(n: TreeNode) -> Node { Some(Rc::new(RefCell::new(n))) }
    fn n(v: i32) -> Node { wrap(TreeNode::new(v)) }
    fn nl(v: i32, l: Node) -> Node { wrap(TreeNode { val: v, left: l, right: None }) }
    fn nr(v: i32, r: Node) -> Node { wrap(TreeNode { val: v, left: None, right: r }) }
    fn nlr(v: i32, l: Node, r: Node) -> Node { wrap(TreeNode { val: v, left: l, right: r }) }
    // @formatter:on

    #[test]
    fn example1() {
        let root = nlr(7, n(3), nlr(15, n(9), n(20)));
        let mut bst_iterator = BSTIterator::new(root);
        assert_eq!(bst_iterator.next(), 3);
        assert_eq!(bst_iterator.next(), 7);
        assert!(bst_iterator.has_next());
        assert_eq!(bst_iterator.next(), 9);
        assert!(bst_iterator.has_next());
        assert_eq!(bst_iterator.next(), 15);
        assert!(bst_iterator.has_next());
        assert_eq!(bst_iterator.next(), 20);
        assert!(!bst_iterator.has_next());
    }

    #[test]
    fn test_empty() {
        let bst_iterator = BSTIterator::new(None);
        assert!(!bst_iterator.has_next());
    }

    //private def buildTree(depth: Int): TreeNode = {
    //  var value = 1
    //  def loop(depth: Int): TreeNode =
    //    if (depth == 0) null
    //    else {
    //      val l = loop(depth - 1)
    //      val root = n(value, l)
    //      value += 1
    //      root.right = loop(depth - 1)
    //
    //      root
    //    }
    //  loop(depth)
    //}
    fn build_tree(depth: usize) -> Node {
        fn in_order(depth: usize, val: i32) -> (Node, i32) {
            if depth == 0 {
                (None, val)
            } else {
                let (l, mut val) = in_order(depth - 1, val);
                let root_v = val;
                val += 1;
                let (r, val) = in_order(depth - 1, val);

                (nlr(root_v, l, r), val)
            }
        }

        let (root, max_val) = in_order(depth, 1);

        println!("build_tree({}): max_val={}", depth, max_val);

        root
    }

    #[test]
    fn test_65535_nodes() {
        let root = build_tree(16);
        let mut bst = BSTIterator::new(root);

        let mut expected = 1;
        while bst.has_next() {
            assert_eq!(bst.next(), expected);
            expected += 1;
        }
        assert_eq!(expected, 65536);
    }

    #[test]
    fn test_131071_nodes() {
        let root = build_tree(17);
        let mut bst = BSTIterator::new(root);

        let mut expected = 1;
        while bst.has_next() {
            assert_eq!(bst.next(), expected);
            expected += 1;
        }
        assert_eq!(expected, 131072);
    }
}