#![allow(dead_code)]
/// 143. Reorder List
/// =================
///
/// You are given the head of a singly linked-list. The list can be represented as:
///
/// `L0 → L1 → … → Ln - 1 → Ln`
///
/// _Reorder the list to be on the following form_:
///
/// `L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …`
///
/// You may not modify the values in the list's nodes.
/// Only nodes themselves may be changed.
///
/// __Constraints:__
///
/// - The number of nodes in the list is in the range `[1, 50_000]`.
/// - `1 <= Node.val <= 1000`
///
/// https://leetcode.com/problems/reorder-list/
struct Solution;
type L = Option<Box<ListNode>>;
impl Solution {
    pub fn reorder_list(head: &mut L) {
        println!("reorder_list({:?})", head);
        fn len(mut head: &L) -> usize {
            let mut len = 0;
            while let Some(next) = head {
                len += 1;
                head = &next.next;
            }
            len
        }
        fn take_last_half(head: &mut L) -> L {
            let l = len(&head);
            let mut len_to_drop = if l % 2 == 0 { l / 2 } else { l / 2 + 1 };
            let mut curr = head;
            while len_to_drop > 1 {
                len_to_drop -= 1;
                curr = &mut curr.as_mut().unwrap().next;
            }
            curr.as_mut().unwrap().next.take()
        }
        fn reverse(mut head: L) -> L {
            let mut curr = None;
            while let Some(mut n) = head {
                head = n.next.take();
                n.next = curr;
                curr = Some(n);
            }
            curr
        }
        fn intersperse(mut head: &mut L, mut other: L) {
            while let Some(mut o) = other {
                other = o.next.take();
                let curr = head.as_mut().unwrap();
                o.next = curr.next.take();
                curr.next = Some(o);
                head = &mut curr.next.as_mut().unwrap().next;
            }
        }

        let last_half = take_last_half(head);
        intersperse(head, reverse(last_half));
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq, Clone)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }
impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut vals = vec![self.val];
        let mut curr = self.next.as_ref();
        while let Some(next) = curr {
            vals.push(next.val);
            curr = next.next.as_ref();
        }
        f.debug_list().entries(vals.iter()).finish()
    }
}

#[rustfmt::skip]
impl ListNode {
    #[inline] fn new(val: i32) -> Self { ListNode { next: None, val } }
    #[inline] fn wrap(self) -> L { Some(Box::new(self)) }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! l {
        ()        => { None };
        ($x:expr) => { ListNode::new($x).wrap() };
        ($x:expr,$($rest:expr),+) => { ListNode { val: $x, next: l![$($rest),+] }.wrap() };
    }

    #[test]
    fn h1() {
        let mut h = l![1];
        let e = l![1];
        Solution::reorder_list(&mut h);
        assert_eq!(h, e);
    }
    #[test]
    fn h12() {
        let mut h = l![1, 2];
        let e = l![1, 2];
        Solution::reorder_list(&mut h);
        assert_eq!(h, e);
    }
    #[test]
    fn h123() {
        let mut h = l![1, 2, 3];
        let e = l![1, 3, 2];
        Solution::reorder_list(&mut h);
        assert_eq!(h, e);
    }
    #[test]
    fn h1234() {
        let mut h = l![1, 2, 3, 4];
        let e = l![1, 4, 2, 3];
        Solution::reorder_list(&mut h);
        assert_eq!(h, e);
    }
    #[test]
    fn h12345() {
        let mut h = l![1, 2, 3, 4, 5];
        let e = l![1, 5, 2, 4, 3];
        Solution::reorder_list(&mut h);
        assert_eq!(h, e);
    }
    #[test]
    fn h123456() {
        let mut h = l![1, 2, 3, 4, 5, 6];
        let e = l![1, 6, 2, 5, 3, 4];
        Solution::reorder_list(&mut h);
        assert_eq!(h, e);
    }
    #[test]
    fn h1234567() {
        let mut h = l![1, 2, 3, 4, 5, 6, 7];
        let e = l![1, 7, 2, 6, 3, 5, 4];
        Solution::reorder_list(&mut h);
        assert_eq!(h, e);
    }
}
