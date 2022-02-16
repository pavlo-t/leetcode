#![allow(dead_code)]
/// 24. Swap Nodes in Pairs
/// =======================
///
/// Given a linked list, swap every two adjacent nodes and return its head.
/// You must solve the problem without modifying the values in the list's nodes
/// (i.e., only nodes themselves may be changed.)
///
/// __Constraints:__
///
/// - The number of nodes in the list is in the range `[0, 100]`.
/// - `0 <= Node.val <= 100`
///
/// https://leetcode.com/problems/swap-nodes-in-pairs/
struct Solution;
type L = Option<Box<ListNode>>;
impl Solution {
    pub fn swap_pairs(head: L) -> L {
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

#[rustfmt::skip] #[derive(PartialEq, Eq, Clone)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut data = vec![self.val];
        let mut curr = &self.next;
        while let Some(l) = curr {
            data.push(l.val);
            curr = &l.next;
        }
        f.debug_list().entries(data.iter()).finish()
    }
}

impl ListNode {
    #[rustfmt::skip] #[inline] fn new(val: i32) -> Self { ListNode { next: None, val } }
    #[rustfmt::skip] #[inline] fn wrap(self) -> L { Some(Box::new(self)) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: L = None;

    macro_rules! l {
        () => { N };
        ($x:expr) => { ListNode::new($x).wrap() };
        ($x:expr,$($rest:expr),+) => { ListNode { val: $x, next: l![$($rest),+]}.wrap()  };
    }

    #[test]
    fn h_empty() {
        assert_eq!(Solution::swap_pairs(l![]), l![]);
    }
    #[test]
    fn h_1() {
        assert_eq!(Solution::swap_pairs(l![1]), l![1]);
    }
    #[test]
    fn h_1_2_3_4() {
        assert_eq!(Solution::swap_pairs(l![1, 2, 3, 4]), l![2, 1, 4, 3]);
    }
}
