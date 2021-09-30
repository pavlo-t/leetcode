#![allow(dead_code)]
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}
impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

/// Reverse Nodes in k-Group
/// ========================
///
/// Given a linked list, reverse the nodes of a linked list k at a time and return its modified list.
///
/// _k_ is a positive integer and is less than or equal to the length of the linked list.
/// If the number of nodes is not a multiple of k then left-out nodes, in the end, should remain as it is.
///
/// You may not alter the values in the list's nodes, only nodes themselves may be changed.
///
/// __Constraints:__
///
/// - The number of nodes in the list is in the range `sz`.
/// - `1 <= sz <= 5000`
/// - `0 <= Node.val <= 1000`
/// - `1 <= k <= sz`
///
/// __Follow-up:__ Can you solve the problem in O(1) extra memory space?
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/610/week-3-july-15th-july-21st/3818/
struct Solution;
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 1 {
            head
        } else {
            let mut v = vec![];
            let mut c = head;
            while let Some(n) = c {
                v.push(n.val);
                c = n.next;
            }

            let mut i = 0;
            let mut j = i + k as usize - 1;
            while j < v.len() {
                v.swap(i, j);
                i += 1;
                j -= 1;
                if i >= j {
                    i += (k as usize) / 2;
                    if k % 2 == 1 {
                        i += 1;
                    }
                    j = i + k as usize - 1;
                }
            }

            let mut h = Some(Box::new(ListNode::new(v[v.len() - 1])));
            for &val in v.iter().rev().skip(1) {
                h = Some(Box::new(ListNode { val, next: h }));
            }

            h
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Node = Option<Box<ListNode>>;

    fn wrap(n: ListNode) -> Node { Some(Box::new(n)) }
    fn l(v: i32, n: Node) -> Node { wrap(ListNode { val: v, next: n }) }

    #[test]
    fn h_1_2_3_4_5_k_2_produces_2_1_4_3_5() {
        let head = l(1, l(2, l(3, l(4, l(5, None)))));
        let e = l(2, l(1, l(4, l(3, l(5, None)))));
        assert_eq!(Solution::reverse_k_group(head, 2), e);
    }
    #[test]
    fn h_1_2_3_4_5_k_3_produces_3_2_1_4_5() {
        let head = l(1, l(2, l(3, l(4, l(5, None)))));
        let e = l(3, l(2, l(1, l(4, l(5, None)))));
        assert_eq!(Solution::reverse_k_group(head, 3), e);
    }
    #[test]
    fn h_1_2_3_4_5_k_1_produces_1_2_3_4_5() {
        let head = l(1, l(2, l(3, l(4, l(5, None)))));
        let e = l(1, l(2, l(3, l(4, l(5, None)))));
        assert_eq!(Solution::reverse_k_group(head, 1), e);
    }
    #[test]
    fn h_1_k_1_produces_1() {
        assert_eq!(Solution::reverse_k_group(l(1, None), 1), l(1, None));
    }
    #[test]
    fn h_1_2_3_4_5_6_k_2_produces_2_1_4_3_6_5() {
        let head = l(1, l(2, l(3, l(4, l(5, l(6, None))))));
        let e = l(2, l(1, l(4, l(3, l(6, l(5, None))))));
        assert_eq!(Solution::reverse_k_group(head, 2), e);
    }
    #[test]
    fn h_1_2_3_4_5_6_k_3_produces_3_2_1_6_5_4() {
        let head = l(1, l(2, l(3, l(4, l(5, l(6, None))))));
        let e = l(3, l(2, l(1, l(6, l(5, l(4, None))))));
        assert_eq!(Solution::reverse_k_group(head, 3), e);
    }
}
