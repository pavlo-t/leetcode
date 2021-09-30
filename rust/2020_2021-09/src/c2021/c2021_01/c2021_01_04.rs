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
    fn new(val: i32) -> Self { ListNode { next: None, val } }
}

impl fmt::Debug for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.next.is_none() {
            write!(f, "{}", self.val)
        } else {
            write!(f, "{},{:?}", self.val, self.next.as_ref().unwrap())
        }
    }
}

/// ### Merge Two Sorted Lists
///
/// https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3592/
struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sentinel = Box::new(ListNode::new(0));
        let mut curr = Some(sentinel.as_mut());

        while l1.is_some() || l2.is_some() {
            let a = l1.as_deref().map(|node| node.val).unwrap_or(std::i32::MAX);
            let b = l2.as_deref().map(|node| node.val).unwrap_or(std::i32::MAX);
            let target = if a <= b { &mut l1 } else { &mut l2 };
            *target = target.as_deref_mut().and_then(|node| node.next.take());

            let node = ListNode::new(std::cmp::min(a, b));

            let ref_curr = curr.unwrap();
            ref_curr.next = Some(Box::new(node));

            curr = ref_curr.next.as_deref_mut();
        }

        sentinel.next
    }

    pub fn merge_two_lists_rec(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (l, None) => l,
            (Some(mut l), Some(r)) if l.val <= r.val => {
                l.next = Solution::merge_two_lists(l.next, Some(r));
                Some(l)
            },
            (l, r) => Solution::merge_two_lists(r, l),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // @formatter:off
    type List = Option<Box<ListNode>>;
    fn wrap(l: ListNode) -> List { Some(Box::new(l)) }
    fn l(v: i32, n: List) -> List { wrap(ListNode { val: v, next: n }) }
    const N: List = None;
    // @formatter:on

    #[test]
    fn example1_l124_l134_produces_l112344() {
        let l1 = l(1, l(2, l(4, N)));
        let l2 = l(1, l(3, l(4, N)));
        let e = l(1, l(1, l(2, l(3, l(4, l(4, N))))));
        assert_eq!(Solution::merge_two_lists(l1, l2), e);
    }

    #[test]
    fn example2_ln_ln_produces_ln() {
        assert_eq!(Solution::merge_two_lists(N, N), N);
    }

    #[test]
    fn example3_ln_l0_produces_l0() {
        assert_eq!(Solution::merge_two_lists(N, l(0, N)), l(0, N));
    }
}
