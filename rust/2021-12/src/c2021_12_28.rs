#![allow(dead_code)]
/// 876. Middle of the Linked List
/// ==============================
///
/// Given the `head` of a singly linked list, return _the middle node of the linked list_.
///
/// If there are two middle nodes, return __the second middle__ node.
///
/// __Constraints:__
///
/// - The number of nodes in the list is in the range `[1, 100]`.
/// - `1 <= Node.val <= 100`
///
/// https://leetcode.com/problems/middle-of-the-linked-list/
struct Solution;
type L = Option<Box<ListNode>>;
impl Solution {
    pub fn middle_node_my_count_and_drop_rec(head: L) -> L {
        fn len(head: &L) -> usize {
            if let Some(n) = head {
                1 + len(&n.next)
            } else {
                0
            }
        }
        fn drop(head: L, n: usize) -> L {
            if n == 0 {
                head
            } else {
                drop(head.unwrap().next, n - 1)
            }
        }

        let mid = len(&head) / 2;
        drop(head, mid)
    }

    pub fn middle_node_my_count_and_drop_iterative(head: L) -> L {
        fn len(mut head: &L) -> usize {
            let mut result = 0;
            while let Some(n) = head {
                result += 1;
                head = &n.next;
            }
            result
        }
        fn drop(mut head: L, mut n: usize) -> L {
            while n > 0 {
                head = head.unwrap().next;
                n -= 1;
            }
            head
        }

        let mid = len(&head) / 2;
        drop(head, mid)
    }

    /// from other submissions https://leetcode.com/submissions/detail/608510026/
    pub fn middle_node(head: L) -> L {
        let mut fast_p = &head;
        let mut slow_p = &head;

        while fast_p.is_some() && fast_p.as_ref().unwrap().next.is_some() {
            slow_p = &slow_p.as_ref().unwrap().next;
            fast_p = &fast_p.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        slow_p.clone()
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq, Clone)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }
impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut data = vec![self.val];
        let mut curr = &self.next;
        while let Some(n) = curr {
            data.push(n.val);
            curr = &n.next;
        }
        f.debug_list().entries(data.iter()).finish()
    }
}
impl ListNode {
    #[rustfmt::skip] #[inline]    fn new(val: i32) -> Self { ListNode { next: None, val } }
    #[rustfmt::skip] #[inline]    fn wrap(self) -> L { Some(Box::new(self)) }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! l {
        () => { None };
        ($x:expr) => { ListNode::new($x).wrap() };
        ($x:expr, $($rest:expr),+) => { ListNode { val: $x, next: l![$($rest),+] }.wrap() };
    }

    #[rustfmt::skip] #[test] fn h_1() { assert_eq!(Solution::middle_node(l![1]), l![1]); }
    #[rustfmt::skip] #[test] fn h_12() { assert_eq!(Solution::middle_node(l![1, 2]), l![2]); }
    #[rustfmt::skip] #[test] fn h_123() { assert_eq!(Solution::middle_node(l![1, 2, 3]), l![2, 3]); }
    #[rustfmt::skip] #[test] fn h_1234() { assert_eq!(Solution::middle_node(l![1, 2, 3, 4]), l![3, 4]); }
    #[test]
    fn h_12345() {
        let h = l![1, 2, 3, 4, 5];
        let e = l![3, 4, 5];
        assert_eq!(Solution::middle_node(h), e);
        // Explanation: The middle node of the list is node 3.
    }
    #[test]
    fn h_123456() {
        let h = l![1, 2, 3, 4, 5, 6];
        let e = l![4, 5, 6];
        assert_eq!(Solution::middle_node(h), e);
        // Explanation: Since the list has two middle nodes with values 3 and 4, we return the second one.
    }
}
