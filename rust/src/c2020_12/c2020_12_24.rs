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
        ListNode {
            next: None,
            val,
        }
    }
}

/// ### Swap Nodes in Pairs
///
/// https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3579/
struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            head
        } else {
            let head = head.unwrap();
            let mut a = ListNode::new(head.val);
            let next = head.next.unwrap();
            let mut b = ListNode::new(next.val);

            a.next = Self::swap_pairs(next.next);
            b.next = Some(Box::new(a));

            Some(Box::new(b))
        }
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
    fn example1_h1234_is_2143() {
        let head = l(1, l(2, l(3, l(4, None))));
        let expected = l(2, l(1, l(4, l(3, None))));
        assert_eq!(Solution::swap_pairs(head), expected);
    }

    #[test]
    fn example2_hn_is_n() {
        let head = None;
        let expected = None;
        assert_eq!(Solution::swap_pairs(head), expected);
    }

    #[test]
    fn example3_h1_is_1() {
        let head = l(1, None);
        let expected = l(1, None);
        assert_eq!(Solution::swap_pairs(head), expected);
    }

    #[test]
    fn h123_is_213() {
        let head = l(1, l(2, l(3, None)));
        let expected = l(2, l(1, l(3, None)));
        assert_eq!(Solution::swap_pairs(head), expected);
    }
}
