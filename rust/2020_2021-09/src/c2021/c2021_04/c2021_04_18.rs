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
/// Remove Nth Node From End of List
/// ================================
///
/// Given the `head` of a linked list, remove the `n`th node from the end of the list and return its head.
///
/// __Follow up:__ Could you do this in one pass?
///
/// __Constraints:__
///
/// - `The number of nodes in the list is sz.
/// - `1 <= sz <= 30`
/// - `0 <= Node.val <= 100`
/// - `1 <= n <= sz`
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/595/week-3-april-15th-april-21st/3712/
struct Solution;
impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let s = Self::size(&head);
        if s < 2 {
            None
        } else {
            let n_from_start = s - n + 1;
            Self::remove_nts_from_start(&mut head, n_from_start);
            head
        }
    }

    fn size(head: &Option<Box<ListNode>>) -> i32 {
        match head {
            None => 0,
            Some(h) => 1 + Self::size(&h.next),
        }
    }

    fn remove_nts_from_start(head: &mut Option<Box<ListNode>>, n: i32) {
        if n > 1 {
            let mut h = head.take().unwrap();
            Self::remove_nts_from_start(&mut h.next, n - 1);
            head.replace(h);
        } else {
            let node = head.take();
            if let Some(n) = node.unwrap().next {
                head.replace(n);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Node = Option<Box<ListNode>>;
    fn wrap(n: ListNode) -> Node {
        Some(Box::new(n))
    }
    fn l(v: i32, n: Node) -> Node {
        wrap(ListNode { val: v, next: n })
    }

    #[test]
    fn example1_h12345n2_produces_1235() {
        let head = l(1, l(2, l(3, l(4, l(5, None)))));
        let e = l(1, l(2, l(3, l(5, None))));
        assert_eq!(Solution::remove_nth_from_end(head, 2), e);
    }
    #[test]
    fn example2_h1n1_produces_none() {
        assert_eq!(Solution::remove_nth_from_end(l(1, None), 1), None);
    }
    #[test]
    fn example3_h12n1_produces_1() {
        let head = l(1, l(2, None));
        let e = l(1, None);
        assert_eq!(Solution::remove_nth_from_end(head, 1), e);
    }
}
