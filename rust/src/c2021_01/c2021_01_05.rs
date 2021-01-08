#![allow(dead_code)]

use std::fmt;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl fmt::Debug for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.next.is_some() {
            write!(f, "{},{:?}", self.val, self.next.as_ref().unwrap())
        } else {
            write!(f, "{}", self.val)
        }
    }
}

/// ### Remove Duplicates from Sorted List II
///
/// https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3593/
struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut next = &mut head;

        while next.is_some() {
            let mut p = next.as_mut().unwrap().next.take();
            if p.is_some() && next.as_ref().unwrap().val == p.as_ref().unwrap().val {
                while p.is_some() && next.as_ref().unwrap().val == p.as_ref().unwrap().val {
                    p = p.unwrap().next;
                }
                *next = p.take();
            } else {
                next.as_mut().unwrap().next = p.take();
                next = &mut next.as_mut().unwrap().next;
            }
        }

        head
    }

    pub fn delete_duplicates_my(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sentinel = Box::new(ListNode::new(i32::max_value()));
        let mut prev_val = sentinel.val;
        let mut curr = Some(sentinel.as_mut());

        let mut head = &head;

        while head.is_some() {
            let h = head.as_ref().unwrap();
            if h.val != prev_val && (h.next.is_none() || h.val != h.next.as_ref().unwrap().val) {
                let n = curr.unwrap();
                n.next = Some(Box::new(ListNode::new(h.val)));
                curr = n.next.as_deref_mut();
            }
            prev_val = h.val;
            head = &head.as_ref().unwrap().next;
        }

        sentinel.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // @formatter:off
    type List = Option<Box<ListNode>>;
    fn wrap(l: ListNode) -> List { Some(Box::new(l)) }
    fn l(v: i32, n: List) -> List { wrap(ListNode { val: v, next: n }) }
    const N: List = None;
    // @formatter:on

    #[test]
    fn example1_h1233445_produces_l125() {
        let head = l(1, l(2, l(3, l(3, l(4, l(4, l(5, N)))))));
        let e = l(1, l(2, l(5, N)));
        assert_eq!(Solution::delete_duplicates(head), e);
    }

    #[test]
    fn example2_h11123_produces_l23() {
        let head = l(1, l(1, l(1, l(2, l(3, N)))));
        let e = l(2, l(3, N));
        assert_eq!(Solution::delete_duplicates(head), e);
    }
}