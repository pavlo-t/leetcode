#![allow(dead_code)]
//! \#92. Reverse Linked List II
//! ============================
//!
//! <https://leetcode.com/problems/reverse-linked-list-ii>
//!
//! Given the `head` of a singly linked list and two integers `left` and `right` where `left <= right`,
//! reverse the nodes of the list from position `left` to position `right`, and return _the reversed list_.
//!
//! __Constraints:__
//!
//! - The number of nodes in the list is `n`.
//! - `1 <= n <= 500`
//! - `-500 <= Node.val <= 500`
//! - `1 <= left <= right <= n`

pub struct Solution;
impl Solution {
    pub fn reverse_between(
        mut head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut vec = vec![];
        while let Some(node) = head {
            vec.push(node.val);
            head = node.next;
        }

        let (mut l, mut r) = (left as usize - 1, right as usize - 1);
        while l < r {
            vec.swap(l, r);
            l += 1;
            r -= 1;
        }

        let mut head = None;
        while let Some(val) = vec.pop() {
            head = Some(Box::new(ListNode { val, next: head }));
        }

        head
    }
}

type L = Option<Box<ListNode>>;

#[rustfmt::skip]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode { pub val: i32, pub next: L }
#[rustfmt::skip]
impl ListNode {
    #[inline] fn new(val: i32) -> Self { ListNode { next: None, val } }
    #[inline] fn wrap(self) -> L { Some(Box::new(self)) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: L = None;
    macro_rules! l {
        () => { N };
        ($x:expr) => { ListNode::new($x).wrap() };
        ($x:expr, $($rest:expr),+) => { ListNode { val: $x, next: l![$($rest),+] }.wrap() };
    }

    #[test]
    fn h_5_l_1_r_1() {
        let h = l![5];
        assert_eq!(Solution::reverse_between(h, 1, 1), l![5]);
    }
    #[test]
    fn h_1_2_3_4_5_l_2_r_4() {
        let h = l![1, 2, 3, 4, 5];
        let e = l![1, 4, 3, 2, 5];
        assert_eq!(Solution::reverse_between(h, 2, 4), e);
    }
    #[test]
    fn h_1_2_3_4_5_l_3_r_4() {
        let h = l![1, 2, 3, 4, 5];
        let e = l![1, 2, 4, 3, 5];
        assert_eq!(Solution::reverse_between(h, 3, 4), e);
    }
}
