use std::cell::RefCell;
use std::rc::Rc;

pub type T = Option<Rc<RefCell<TreeNode>>>;

#[rustfmt::skip]
#[derive(PartialEq, Eq)]
pub struct TreeNode { pub val: i32, pub left: T, pub right: T }
impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::new();
        result.push_str(&self.val.to_string());

        let mut curr = vec![];
        if self.left.is_some() || self.right.is_some() {
            curr.push(self.right.clone());
            curr.push(self.left.clone());
            result.push('|');
        }

        while !curr.is_empty() {
            let mut seen_some = false;
            let mut next = vec![];
            while let Some(opt) = curr.pop() {
                if let Some(node) = opt {
                    let node = node.borrow();
                    result.push_str(&node.val.to_string());
                    next.push(node.left.clone());
                    next.push(node.right.clone());
                    if node.left.is_some() || node.right.is_some() {
                        seen_some = true;
                    }
                } else {
                    result.push('_');
                }
                result.push(',');
            }
            result.pop();
            if seen_some {
                next.reverse();
                curr = next;
                result.push('|');
            }
        }

        f.write_str(&result)
    }
}

#[rustfmt::skip]
impl TreeNode {
  #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
}

#[rustfmt::skip] fn wrap(t: TreeNode) -> T { Some(Rc::new(RefCell::new(t))) }

/// Empty tree
pub const N: T = None;

/// Wrapped leaf node
#[rustfmt::skip] pub fn l(val: i32)                    -> T { wrap(TreeNode::new(val)) }

/// Wrapped tree node
#[rustfmt::skip] pub fn t(val: i32, left: T, right: T) -> T { wrap(TreeNode { val, left, right }) }
