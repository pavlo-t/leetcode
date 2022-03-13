#![allow(dead_code)]
/// 21. Merge Two Sorted Lists
/// ==========================
///
/// You are given the heads of two sorted linked lists `list1` and `list2`.
///
/// Merge the two lists in a one __sorted__ list.
/// The list should be made by splicing together the nodes of the first two lists.
///
/// Return _the head of the merged linked list_.
///
/// __Constraints:__
///
/// - The number of nodes in both lists is in the range `[0, 50]`.
/// - `-100 <= Node.val <= 100`
/// - Both `list1` and `list2` are sorted in __non-decreasing__ order.
///
/// https://leetcode.com/problems/merge-two-sorted-lists/
struct Solution;
type L = Option<Box<ListNode>>;
impl Solution {
    pub fn merge_two_lists(mut list1: L, mut list2: L) -> L {
        let mut result = ListNode::new(0);
        let mut curr = &mut result;
        while list1.is_some() || list2.is_some() {
            match (list1.is_some(), list2.is_some()) {
                (true, true) => {
                    let v1 = list1.as_ref().map(|l| l.val).unwrap();
                    let v2 = list2.as_ref().map(|l| l.val).unwrap();
                    if v1 <= v2 {
                        let mut n = list1.unwrap();
                        list1 = n.next.take();
                        curr.next = Some(n);
                    } else {
                        let mut n = list2.unwrap();
                        list2 = n.next.take();
                        curr.next = Some(n);
                    }
                }
                (true, false) => {
                    let mut n = list1.unwrap();
                    list1 = n.next.take();
                    curr.next = Some(n);
                }
                (false, true) => {
                    std::mem::swap(&mut list1, &mut list2);
                }
                _ => (),
            }
            if curr.next.is_some() {
                curr = curr.next.as_mut().unwrap();
            }
        }
        result.next
    }
}

#[rustfmt::skip] #[derive(PartialEq, Eq, Clone)]
pub struct ListNode { pub val: i32, pub next: L }
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
        ($x:expr,$($rest:expr),+) => { ListNode { val: $x, next: l![$($rest),+] }.wrap() };
    }

    #[test]
    fn a_empty_b_empty() {
        assert_eq!(Solution::merge_two_lists(l![], l![]), l![]);
    }
    #[test]
    fn a_empty_b_0() {
        assert_eq!(Solution::merge_two_lists(l![], l![0]), l![0]);
    }
    #[test]
    fn a_1_2_4_b_1_3_4() {
        let a = l![1, 2, 4];
        let b = l![1, 3, 4];
        let e = l![1, 1, 2, 3, 4, 4];
        assert_eq!(Solution::merge_two_lists(a, b), e);
    }
}
