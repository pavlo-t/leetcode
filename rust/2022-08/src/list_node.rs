#[allow(dead_code)]
use std::fmt;

/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl fmt::Debug for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        let mut curr = &self.next;

        result.push('[');
        result.push_str(&self.val.to_string());
        while let Some(node) = curr {
            result.push_str(", ");
            result.push_str(&node.val.to_string());
            curr = &node.next;
        }
        result.push(']');

        f.write_str(&result)
    }
}

pub const NIL: Option<Box<ListNode>> = None;

#[macro_export]
macro_rules! l {
    ()                        => { NIL };
    ($x:expr)                 => { Some(Box::new(ListNode::new($x))) };
    ($x:expr,$($rest:expr),+) => { Some(Box::new(ListNode { val: $x, next: l![$($rest),+] })) };
}
