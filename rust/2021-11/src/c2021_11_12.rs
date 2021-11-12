#![allow(dead_code)]
/// 203. Remove Linked List Elements
/// ================================
///
/// Given the `head` of a linked list and an integer `val`,
/// remove all the nodes of the linked list that has `Node.val == val`,
/// and return _the new head_.
///
/// __Constraints:__
///
/// - The number of nodes in the list is in the range `[0, 10_000]`.
/// - `1 <= Node.val <= 50`
/// - `0 <= val <= 50`
///
/// https://leetcode.com/problems/remove-linked-list-elements/
struct Solution;
impl Solution {
    /// from other solutions - what I was trying to do;
    /// it doesn't work with `if let Some(value) = ptr`
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut ptr = &mut head;
        loop {
            match ptr {
                None => break,
                Some(value) if value.val == val => *ptr = value.next.take(),
                Some(value) => ptr = &mut value.next,
            }
        }
        head
    }

    pub fn remove_elements_my(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        //println!("remove_elements({:?}, {})", head, val);
        let mut dummy = ListNode::new(0);
        let mut prev = &mut dummy;
        let mut curr = head.as_ref();
        while let Some(c) = curr {
            if c.val != val {
                prev.next = Some(Box::new(ListNode::new(c.val)));
                prev = prev.next.as_mut().unwrap();
            }
            curr = c.next.as_ref();
        }
        dummy.next
    }
}

//#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(PartialEq, Eq, Clone)]
#[rustfmt::skip]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }
#[rustfmt::skip]
impl ListNode { #[inline] fn new(val: i32) -> Self { ListNode { next: None, val } } }
impl std::fmt::Debug for ListNode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut curr = &self.next;
        let mut data = vec![self.val];
        while let Some(c) = curr {
            data.push(c.val);
            curr = &c.next;
        }
        fmt.debug_list().entries(data.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type L = Option<Box<ListNode>>;
    #[rustfmt::skip] fn l(v: i32, n: L) -> L {Some(Box::new(ListNode{val: v, next: n}))}
    fn l_of_n(val: i32, len: usize) -> L {
        if len == 0 {
            None
        } else {
            let mut dummy = ListNode::new(0);
            let mut curr = &mut dummy;
            for _ in 0..len {
                curr.next = Some(Box::new(ListNode::new(val)));
                curr = curr.next.as_mut().unwrap();
            }
            dummy.next
        }
    }

    macro_rules! l {
        ()                      => { None };
        ($x:expr)               => { l($x, None) };
        ($x:expr, $($r:expr),+) => { l($x, l![$($r),+]) };
        ($x:expr; $l:expr) => { l_of_n($x, $l) };
    }

    #[test]
    fn h_1263456_v_6() {
        let h = l![1, 2, 6, 3, 4, 5, 6];
        let v = 6;
        let e = l![1, 2, 3, 4, 5];
        assert_eq!(Solution::remove_elements(h, v), e);
    }
    #[test]
    fn h_empty_v_1() {
        let h = l![];
        let v = 1;
        let e = l![];
        assert_eq!(Solution::remove_elements(h, v), e);
    }
    #[test]
    fn h_7777_v_7() {
        let h = l![7, 7, 7, 7];
        let v = 7;
        let e = l![];
        assert_eq!(Solution::remove_elements(h, v), e);
    }
    #[test]
    fn h_123_v_1() {
        let h = l![1, 2, 3];
        let v = 1;
        let e = l![2, 3];
        assert_eq!(Solution::remove_elements(h, v), e);
    }

    #[test]
    fn h_10000x1_v_1() {
        let h = l![1; 10000];
        let v = 1;
        let e = l![];
        assert_eq!(Solution::remove_elements(h, v), e);
    }
    #[test]
    fn h_10000x1_v_2() {
        let h = l![1; 6000];
        let v = 2;
        let e = h.clone();
        assert_eq!(Solution::remove_elements(h, v), e);
    }
}
