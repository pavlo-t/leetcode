#![allow(dead_code)]

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

//noinspection DuplicatedCode
impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.next.is_none() {
            write!(f, "{}", self.val)
        } else {
            write!(f, "{},{:?}", self.val, self.next.as_ref().unwrap())
        }
    }
}

/// ### Add Two Numbers
/// https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3601/
struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sentinel = Box::new(ListNode::new(0));
        let mut curr = Some(sentinel.as_mut());
        let mut carry = 0;

        while l1.is_some() || l2.is_some() {
            let a = l1.as_deref().map(|node| node.val).unwrap_or(0);
            let b = l2.as_deref().map(|node| node.val).unwrap_or(0);

            l1 = l1.as_deref_mut().and_then(|node| node.next.take());
            l2 = l2.as_deref_mut().and_then(|node| node.next.take());

            let v = a + b + carry;
            carry = v / 10;

            let ref_curr = curr.unwrap();
            ref_curr.next = Some(Box::new(ListNode::new(v % 10)));
            curr = ref_curr.next.as_deref_mut();
        }

        if carry > 0 {
            let ref_curr = curr.unwrap();
            ref_curr.next = Some(Box::new(ListNode::new(carry)));
        }

        sentinel.next
    }

    pub fn add_two_numbers_rec(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn rec(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
            match (l1, l2) {
                (None, None) if carry > 0 => Some(Box::new(ListNode::new(carry))),
                (l, None) if carry == 0 => l,
                (Some(l), None) => {
                    let v = l.val + carry;
                    Some(Box::new(ListNode { val: v % 10, next: rec(l.next, None, v / 10) }))
                },
                (Some(l), Some(r)) => {
                    let v = l.val + r.val + carry;
                    Some(Box::new(ListNode { val: v % 10, next: rec(l.next, r.next, v / 10) }))
                }
                (l, r) => rec(r, l, carry),
            }
        }
        rec(l1, l2, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // @formatter:off
    type List = Option<Box<ListNode>>;
    fn l(v: i32, n: List) -> List { Some(Box::new(ListNode { val: v, next: n })) }
    // @formatter:on

    #[test]
    fn example1_l243_r564_is_708() {
        let l1 = l(2, l(4, l(3, None)));
        let l2 = l(5, l(6, l(4, None)));
        let e = l(7, l(0, l(8, None)));
        assert_eq!(Solution::add_two_numbers(l1, l2), e);
        // Explanation: 342 + 465 = 807.
    }

    #[test]
    fn example2_l0_r0_is_0() {
        let l1 = l(0, None);
        let l2 = l(0, None);
        let e = l(0, None);
        assert_eq!(Solution::add_two_numbers(l1, l2), e);
    }

    #[test]
    fn example3_l9999999_r9999_is_89990001() {
        let l1 = l(9, l(9, l(9, l(9, l(9, l(9, l(9, None)))))));
        let l2 = l(9, l(9, l(9, l(9, None))));
        let e = l(8, l(9, l(9, l(9, l(0, l(0, l(0, l(1, None))))))));
        assert_eq!(Solution::add_two_numbers(l1, l2), e);
    }
}
