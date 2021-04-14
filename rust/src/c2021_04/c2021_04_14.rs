#![allow(dead_code)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// Partition List
/// ==============
///
/// Given the `head` of a linked list and a value `x`, partition it such that
/// all nodes __less than__ `x` come before nodes __greater than or equal__ to `x`.
///
/// You should __preserve__ the original relative order of the nodes in each of the two partitions.
///
/// __Constraints:__
///
/// - The number of nodes in the list is in the range `[0, 200]`.
/// - `-100 <= Node.val <= 100`
/// - `-200 <= x <= 200`
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/594/week-2-april-8th-april-14th/3707/
struct Solution;
impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(Default::default()));

        let mut after_head = Box::new(ListNode::new(Default::default()));
        let mut before = &mut dummy;
        let mut after = &mut after_head;

        while let Some(mut node) = head {
            if node.val < x {
                head = node.next.take();
                before.next = Some(node);
                before = before.next.as_mut().unwrap();
            } else {
                head = node.next.take();
                after.next = Some(node);
                after = after.next.as_mut().unwrap();
            }
        }
        after.next = None;

        before.next = after_head.next;
        dummy.next
    }

    pub fn partition_my_2vec(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut ls = Vec::new();
        let mut rs = Vec::new();
        while let Some(c) = head {
            if c.val < x {
                ls.push(c.val);
            } else {
                rs.push(c.val);
            }
            head = c.next;
        }

        let mut next: Option<Box<ListNode>> = None;
        for i in rs.into_iter().rev().chain(ls.into_iter().rev()) {
            next = Some(Box::new(ListNode { val: i, next }));
        }

        next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type List = Option<Box<ListNode>>;
    fn wrap(l: ListNode) -> List {
        Some(Box::new(l))
    }
    fn l(i: i32, n: List) -> List {
        wrap(ListNode { val: i, next: n })
    }

    #[test]
    fn example1() {
        let h = l(1, l(4, l(3, l(2, l(5, l(2, None))))));
        let e = l(1, l(2, l(2, l(4, l(3, l(5, None))))));
        assert_eq!(Solution::partition(h, 3), e);
    }
    #[test]
    fn example2() {
        let h = l(2, l(1, None));
        let e = l(1, l(2, None));
        assert_eq!(Solution::partition(h, 2), e);
    }

    #[test]
    fn h_none_produces_none() {
        assert_eq!(Solution::partition(None, 3), None);
    }
}
