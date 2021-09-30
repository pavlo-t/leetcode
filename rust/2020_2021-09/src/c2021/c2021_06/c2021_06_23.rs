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

/// Reverse Linked List II
/// ======================
///
/// Given the `head` of a singly linked list and two integers `left` and `right` where `left <= right`,
/// reverse the nodes of the list from position `left` to position `right`,
/// and return _the reversed list_.
///
/// __Constraints:__
///
/// - The number of nodes in the list is `n`.
/// - `1 <= n <= 500`
/// - `-500 <= Node.val <= 500`
/// - `1 <= left <= right <= n`
///
///
/// __Follow up__: Could you do it in one pass?
///
/// https://leetcode.com/explore/featured/card/june-leetcoding-challenge-2021/606/week-4-june-22nd-june-28th/3789/
struct Solution;
impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        if head.is_some() && left > 0 && right > 0 {
            //
        }
        todo!("Check https://leetcode.com/problems/reverse-linked-list-ii/solution/")
    }
    pub fn reverse_between_my_with_vec(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut v = vec![];
        let mut c = &head;
        while let Some(n) = c {
            v.push(n.val);
            c = &n.next;
        }
        let mut l = (left - 1) as usize;
        let mut r = (right - 1) as usize;
        while l < r {
            v.swap(l, r);
            l += 1;
            r -= 1;
        }
        let mut c = None;
        for i in (0..v.len()).rev() {
            c = Some(Box::new(ListNode { val: v[i], next: c }));
        }
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn l(v: i32, n: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode { val: v, next: n }))
    }

    #[test]
    fn example1() {
        let h = l(1, l(2, l(3, l(4, l(5, None)))));
        let e = l(1, l(4, l(3, l(2, l(5, None)))));
        assert_eq!(Solution::reverse_between(h, 2, 4), e);
    }
    #[test]
    fn example2() {
        let h = l(5, None);
        let e = l(5, None);
        assert_eq!(Solution::reverse_between(h, 1, 1), e);
    }
}
