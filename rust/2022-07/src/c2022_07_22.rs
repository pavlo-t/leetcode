#![allow(dead_code)]
//! \#86. Partition List
//! ====================
//!
//! <https://leetcode.com/problems/partition-list>
//!
//! Given the `head` of a linked list and a value `x`, partition it such that
//! all nodes __less than__ `x` come before nodes __greater than or equal__ to `x`.
//!
//! You should __preserve__ the original relative order of the nodes in each of the two partitions.
//!
//! __Constraints:__
//!
//! - The number of nodes in the list is in the range `[0, 200]`.
//! - `-100 <= Node.val <= 100`
//! - `-200 <= x <= 200`

pub struct Solution;
impl Solution {
    /// Using vectors
    pub fn partition_v1(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut lt = vec![];
        let mut gte = vec![];
        while let Some(l) = head {
            if l.val < x {
                lt.push(l.val);
            } else {
                gte.push(l.val);
            }
            head = l.next;
        }

        let mut next = None;
        for val in lt.into_iter().chain(gte.into_iter()).rev() {
            next = Some(Box::new(ListNode { val, next }));
        }
        next
    }

    /// In place
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(0);
        let mut curr_head = &mut dummy_head;
        let mut dummy_tail = ListNode::new(0);
        let mut curr_tail = &mut dummy_tail;

        while let Some(mut node) = head {
            head = node.next.take();
            if node.val < x {
                curr_head.next = Some(node);
                curr_head = curr_head.next.as_mut().unwrap();
            } else {
                curr_tail.next = Some(node);
                curr_tail = curr_tail.next.as_mut().unwrap();
            }
        }

        curr_head.next = dummy_tail.next;
        dummy_head.next
    }
}

type L = Option<Box<ListNode>>;

#[rustfmt::skip]
#[derive(PartialEq, Eq, Clone)]
pub struct ListNode { pub val: i32, pub next: L }
#[rustfmt::skip]
impl ListNode {
    #[inline] fn new(val: i32) -> Self { ListNode { next: None, val } }
}
impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut data = vec![self.val];
        let mut curr = self.next.as_ref();
        while let Some(l) = curr {
            data.push(l.val);
            curr = l.next.as_ref();
        }
        f.debug_list().entries(&data).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: L = None;
    #[rustfmt::skip] fn wrap(l: ListNode) -> L { Some(Box::new(l)) }
    macro_rules! l {
        ()                        => { N };
        ($x:expr)                 => { wrap(ListNode::new($x)) };
        ($x:expr,$($rest:expr),+) => { wrap(ListNode { val: $x, next: l![$($rest),+] }) };
    }

    #[rustfmt::skip] #[test] fn h_empty_x_2() { assert_eq!(Solution::partition(N, 2), N); }
    #[rustfmt::skip] #[test] fn h_1_x_2() { assert_eq!(Solution::partition(l![1], 2), l![1]); }
    #[rustfmt::skip] #[test] fn h_2_x_2() { assert_eq!(Solution::partition(l![2], 2), l![2]); }
    #[rustfmt::skip] #[test] fn h_2_1_x_2() { assert_eq!(Solution::partition(l![2, 1], 2), l![1, 2]); }

    #[test]
    fn h_1_4_3_2_5_2_x_3() {
        let h = l![1, 4, 3, 2, 5, 2];
        let e = l![1, 2, 2, 4, 3, 5];
        assert_eq!(Solution::partition(h, 3), e);
    }
}
