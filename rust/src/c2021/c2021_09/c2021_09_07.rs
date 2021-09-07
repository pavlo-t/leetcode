#![allow(dead_code)]

/// Definition for singly-linked list.
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

/// Reverse Linked List
/// ===================
///
/// Given the `head` of a singly linked list, reverse the list, and return _the reversed list_.
///
/// __Constraints:__
///
/// - The number of nodes in the list is the range `[0, 5000]`.
/// - `-5000 <= Node.val <= 5000`
///
/// __Follow up:__ A linked list can be reversed either iteratively or recursively. Could you implement both?
///
/// https://leetcode.com/explore/challenge/card/september-leetcoding-challenge-2021/636/week-1-september-1st-september-7th/3966/
struct Solution;
impl Solution {
    /// https://leetcode.com/problems/reverse-linked-list/solution/
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;
        while let Some(mut n) = curr {
            let next = match prev {
                None => n.next.take(),
                Some(p) => n.next.replace(p),
            };
            prev = Some(n);
            curr = next;
        }
        prev
    }
    pub fn reverse_list_rec(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn rec(h: Option<Box<ListNode>>, r: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            match (h, r) {
                (None, r) => r,
                (Some(mut n), None) => rec(n.next.take(), Some(n)),
                (Some(mut n), Some(r)) => rec(n.next.replace(r), Some(n)),
            }
        }
        rec(head, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type L = Option<Box<ListNode>>;
    fn l(v: i32, n: L) -> L {
        Some(Box::new(ListNode { val: v, next: n }))
    }

    macro_rules! l {
        ($x:expr) => {l($x, None)};
        ($x:expr,$($r:expr),+) => {l($x, l!($($r),+))};
    }

    #[test]
    fn h_1_2_3_4_5() {
        let head = l![1, 2, 3, 4, 5];
        let e = l![5, 4, 3, 2, 1];
        assert_eq!(Solution::reverse_list(head), e);
    }
    #[test]
    fn h_1_2() {
        assert_eq!(Solution::reverse_list(l![1, 2]), l![2, 1]);
    }
    #[test]
    fn h_1() {
        assert_eq!(Solution::reverse_list(l![1]), l![1]);
    }
    #[test]
    fn h_empty() {
        assert_eq!(Solution::reverse_list(None), None);
    }
}
