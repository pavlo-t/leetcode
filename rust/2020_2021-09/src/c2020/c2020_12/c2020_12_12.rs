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

impl Solution {
    pub fn subtree_with_all_deepest(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        type Node = Option<Rc<RefCell<TreeNode>>>;

        struct TnD {
            node: Node,
            depth: i32,
        }

        fn dfs(node: &Node) -> TnD {
            match node {
                None => TnD { node: None, depth: 0 },
                Some(n) => {
                    let n = n.borrow();
                    let l = dfs(&n.left);
                    let r = dfs(&n.right);

                    if l.depth > r.depth {
                        TnD { node: l.node, depth: l.depth + 1 }
                    } else if l.depth < r.depth {
                        TnD { node: r.node, depth: r.depth + 1 }
                    } else {
                        TnD { node: node.to_owned(), depth: l.depth + 1 }
                    }
                }
            }
        }

        dfs(&root).node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // @formatter:off
    type Node = Option<Rc<RefCell<TreeNode>>>;

    fn wrap(n: TreeNode) -> Node { Some(Rc::new(RefCell::new(n))) }

    fn n(v: i32) -> Node { wrap(TreeNode::new(v)) }
    fn nl(v: i32, l: Node) -> Node { wrap(TreeNode { val: v, left: l, right: None }) }
    fn nr(v: i32, r: Node) -> Node { wrap(TreeNode { val: v, left: None, right: r }) }
    fn nlr(v: i32, l: Node, r: Node) -> Node { wrap(TreeNode { val: v, left: l, right: r }) }
    // @formatter:on

    #[test]
    fn example1() {
        let root =
            nlr(3,
                nlr(5, n(6), nlr(2, n(7), n(4))),
                nlr(1, n(0), n(8)));
        let expected = nlr(2, n(7), n(4));

        assert_eq!(Solution::subtree_with_all_deepest(root), expected);
        //Explanation: We return the node with value 2, nodes 7 and 4 are the deepest.
        // Notice that nodes 5, 3 and 2 contain the deepest nodes in the tree but node 2 is the smallest subtree among them,
        // so we return it.
    }

    #[test]
    fn example2_r1_is_1() {
        assert_eq!(Solution::subtree_with_all_deepest(n(1)), n(1));
        //Explanation: The root is the deepest node in the tree.
    }

    #[test]
    fn example3_r0_1_3_n_2_is_2() {
        let root = nlr(0, nr(1, n(2)), n(2));
        let expected = n(2);

        assert_eq!(Solution::subtree_with_all_deepest(root), expected);
        //  //Explanation: The deepest node in the tree is 2,
        //  // the valid subtrees are the subtrees of nodes 2, 1 and 0 but the subtree of node 2 is the smallest.
    }

    #[test]
    fn test13_r0_1_n_3_2() {
        let root = nl(0, nlr(1, n(3), n(2)));
        let expected = nlr(1, n(3), n(2));

        assert_eq!(Solution::subtree_with_all_deepest(root), expected);
    }

    #[test]
    fn rn_is_n() {
        assert_eq!(Solution::subtree_with_all_deepest(None), None);
    }

    #[test]
    fn r511balanced_nodes_is_root() {
        fn build_tree(depth: i32) -> Node {
            if depth == 0 {
                None
            } else {
                nlr(depth, build_tree(depth - 1), build_tree(depth - 1))
            }
        }

        let root = build_tree(9);
        let expected = build_tree(9);

        assert_eq!(Solution::subtree_with_all_deepest(root), expected);
    }

    #[test]
    fn r500_in_left_is_1() {
        fn build_tree(nodes: i32) -> Node {
            if nodes == 0 {
                None
            } else {
                nl(nodes, build_tree(nodes - 1))
            }
        }

        let root = build_tree(500);
        let expected = n(1);

        assert_eq!(Solution::subtree_with_all_deepest(root), expected);
    }

    #[test]
    fn r500_in_right_is_1() {
        fn build_tree(nodes: i32) -> Node {
            if nodes == 0 {
                None
            } else {
                nr(nodes, build_tree(nodes - 1))
            }
        }

        let root = build_tree(500);
        let expected = n(1);

        assert_eq!(Solution::subtree_with_all_deepest(root), expected);
    }

    #[test]
    fn r5000_in_left_is_1() {
        fn build_tree(nodes: i32) -> Node {
            if nodes == 0 {
                None
            } else {
                nl(nodes, build_tree(nodes - 1))
            }
        }

        let root = build_tree(5000);
        let expected = n(1);

        assert_eq!(Solution::subtree_with_all_deepest(root), expected);
    }
}
