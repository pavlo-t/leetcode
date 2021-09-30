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
impl From<Vec<i32>> for ListNode {
    fn from(v: Vec<i32>) -> Self {
        if v.is_empty() {
            panic!("Expecting a non-empty vector, got: {:?}", v);
        }
        let mut next: Option<Box<ListNode>> = None;
        for val in v.into_iter().rev() {
            next = Some(Box::new(ListNode { val, next }));
        }
        *next.unwrap()
    }
}

/// # Swapping Nodes in a Linked List
///
/// You are given the `head` of a linked list, and an integer `k`.
///
/// Return _the head of the linked list after __swapping__ the values of the_ `k`_th node from the
/// beginning and the_ `k`_th node from the end (the list is __1-indexed__)_.
///
/// __Constraints:__
///
/// - The number of nodes in the list is `n`.
/// - `1 <= k <= n <= 100_000`
/// - `0 <= Node.val <= 100`
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/589/week-2-march-8th-march-14th/3671/
struct Solution;
impl Solution {
    pub fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut v = Vec::new();
        while let Some(curr) = head {
            v.push(curr.val);
            head = curr.next;
        }
        let a = (k - 1) as usize;
        let b = v.len() - k as usize;
        v.swap(a, b);
        Some(Box::new(ListNode::from(v)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Node = Option<Box<ListNode>>;
    fn wrap(l: ListNode) -> Node {
        Some(Box::new(l))
    }

    #[test]
    fn example1_h12345k2_should_produce_14325() {
        let h = wrap(ListNode::from(vec![1, 2, 3, 4, 5]));
        let e = wrap(ListNode::from(vec![1, 4, 3, 2, 5]));
        assert_eq!(Solution::swap_nodes(h, 2), e);
    }
    #[test]
    fn example2_h7966783095k5_should_produce_7966873095() {
        let h = wrap(ListNode::from(vec![7, 9, 6, 6, 7, 8, 3, 0, 9, 5]));
        let e = wrap(ListNode::from(vec![7, 9, 6, 6, 8, 7, 3, 0, 9, 5]));
        assert_eq!(Solution::swap_nodes(h, 5), e);
    }
    #[test]
    fn example3_h1k1_should_produce_1() {
        let h = wrap(ListNode::from(vec![1]));
        let e = wrap(ListNode::from(vec![1]));
        assert_eq!(Solution::swap_nodes(h, 1), e);
    }
    #[test]
    fn example4_h12k1_should_produce_21() {
        let h = wrap(ListNode::from(vec![1, 2]));
        let e = wrap(ListNode::from(vec![2, 1]));
        assert_eq!(Solution::swap_nodes(h, 1), e);
    }
    #[test]
    fn example5_h123k2_should_produce_123() {
        let h = wrap(ListNode::from(vec![1, 2, 3]));
        let e = wrap(ListNode::from(vec![1, 2, 3]));
        assert_eq!(Solution::swap_nodes(h, 2), e);
    }

    /// If getting stack overflow:
    ///
    /// ```sh
    /// thread 'c2021::c2021_03::c2021_03_14::tests::performance1_h1to100_000k2_should_work' has overflowed its stack
    /// fatal runtime error: stack overflow
    /// ```
    ///
    /// Add `RUST_MIN_STACK=33554432` to env
    #[test]
    fn performance1_h1to100_000k2_should_work() {
        let h = wrap(ListNode::from((1..=100_000).collect::<Vec<_>>()));
        let mut e = (1..=100_000).collect::<Vec<_>>();
        e.swap(1, 99_998);
        let e = wrap(ListNode::from(e));
        assert_eq!(Solution::swap_nodes(h, 2), e);
    }
}
