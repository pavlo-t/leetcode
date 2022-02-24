#![allow(dead_code)]
/// 148. Sort List
/// ==============
///
/// Given the `head` of a linked list, return _the list after sorting it in __ascending order___.
///
/// __Constraints:__
///
/// - The number of nodes in the list is in the range `[0, 50_000]`.
/// - `-100_000 <= Node.val <= 100_000`
///
/// __Follow up:__ Can you sort the linked list in `O(n logn)` time and `O(1)` memory (i.e. constant space)?
///
/// https://leetcode.com/problems/sort-list/
struct Solution;
type L = Option<Box<ListNode>>;
impl Solution {
    pub fn sort_list(head: L) -> L {
        let mut data = vec![];
        let mut curr = head;
        while let Some(n) = curr {
            data.push(n.val);
            curr = n.next;
        }

        data.sort_unstable();

        let mut next: L = None;

        while let Some(val) = data.pop() {
            next = Some(Box::new(ListNode { val, next }));
        }

        next
    }
}

#[rustfmt::skip] #[derive(PartialEq, Eq, Clone)]
pub struct ListNode { pub val: i32, pub next: L }
impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut data = vec![self.val];
        let mut curr = self.next.as_ref();
        while let Some(n) = curr {
            data.push(n.val);
            curr = n.next.as_ref();
        }
        f.debug_list().entries(data.iter()).finish()
    }
}
#[rustfmt::skip] impl ListNode {
    #[inline] fn new(val: i32) -> Self { ListNode { next: None, val } }
    #[inline] fn wrap(self) -> L { Some(Box::new(self)) }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! l {
        () => { None };
        ($x:expr) => { ListNode::new($x).wrap() };
        ($x:expr,$($rest:tt),+) => { ListNode { val: $x, next: l![$($rest),+] }.wrap() };
    }

    #[test]
    fn empty() {
        let h = l![];
        let e = l![];
        assert_eq!(Solution::sort_list(h), e);
    }
    #[test]
    fn p1() {
        let h = l![1];
        let e = l![1];
        assert_eq!(Solution::sort_list(h), e);
    }
    #[test]
    fn p1p2() {
        let h = l![1, 2];
        let e = l![1, 2];
        assert_eq!(Solution::sort_list(h), e);
    }
    #[test]
    fn p2p1() {
        let h = l![2, 1];
        let e = l![1, 2];
        assert_eq!(Solution::sort_list(h), e);
    }
    #[test]
    fn p4p2p1p3() {
        let h = l![4, 2, 1, 3];
        let e = l![1, 2, 3, 4];
        assert_eq!(Solution::sort_list(h), e);
    }
    #[test]
    fn m1p5p3p4n0() {
        let h = l![-1, 5, 3, 4, 0];
        let e = l![-1, 0, 3, 4, 5];
        assert_eq!(Solution::sort_list(h), e);
    }
}
