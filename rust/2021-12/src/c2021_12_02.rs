#![allow(dead_code)]
/// 328. Odd Even Linked List
/// =========================
///
/// Given the `head` of a singly linked list,
/// group all the nodes with odd indices together followed by the nodes with even indices,
/// and return _the reordered list_.
///
/// The __first__ node is considered __odd__, and the __second__ node is __even__, and so on.
///
/// Note that the relative order inside both the even and odd groups should remain as it was in the input.
///
/// You must solve the problem in `O(1)` extra space complexity and `O(n)` time complexity.
///
/// __Constraints:__
///
/// - `n ==` number of nodes in the linked list
/// - `0 <= n <= 10_000`
/// - `-1_000_000 <= Node.val <= 1_000_000`
///
/// https://leetcode.com/problems/odd-even-linked-list/
struct Solution;
type L = Option<Box<ListNode>>;
impl Solution {
    pub fn odd_even_list_my_linear_memory(head: L) -> L {
        println!("odd_even_list({:?})", head);
        let mut curr = head;
        let (mut odd, mut even) = (vec![], vec![]);
        let mut is_odd = true;
        while let Some(n) = curr {
            if is_odd {
                odd.push(n.val);
            } else {
                even.push(n.val);
            }
            is_odd = !is_odd;
            curr = n.next;
        }
        odd.append(&mut even);
        let mut curr = None;
        while let Some(val) = odd.pop() {
            curr = Some(Box::new(ListNode { val, next: curr }));
        }
        curr
    }

    /// From other solutions
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd_dummy = ListNode::new(0);
        let mut even_dummy = ListNode::new(0);
        let mut odd_curr = &mut odd_dummy;
        let mut even_curr = &mut even_dummy;

        let mut odd = true;
        while let Some(mut node) = head {
            head = node.next.take();
            if odd {
                odd_curr.next = Some(node);
                odd_curr = odd_curr.next.as_mut().unwrap();
            } else {
                even_curr.next = Some(node);
                even_curr = even_curr.next.as_mut().unwrap();
            }
            odd = !odd;
        }
        odd_curr.next = even_dummy.next;
        odd_dummy.next
    }
}

use std::fmt;
//#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(PartialEq, Eq, Clone)]
#[rustfmt::skip]
pub struct ListNode { pub val: i32, pub next: L }
impl fmt::Debug for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut curr = self;
        let mut data = vec![self.val];
        while let Some(n) = curr.next.as_ref() {
            data.push(n.val);
            curr = n;
        }
        f.debug_list().entries(data.iter()).finish()
    }
}

#[rustfmt::skip] impl ListNode { #[inline] fn new(val: i32) -> Self { ListNode { next: None, val } } }

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! l {
        ()                     => { None };
        ($x:expr)              => { Some(Box::new(ListNode::new($x))) };
        ($x:expr,$($r:expr),+) => { Some(Box::new(ListNode { val: $x, next: l![$($r),+]})) };
    }

    #[rustfmt::skip] #[test] fn h_empty()  {assert_eq!(Solution::odd_even_list(l![]),            l![]);}
    #[rustfmt::skip] #[test] fn h_1()      {assert_eq!(Solution::odd_even_list(l![1]),           l![1]);}
    #[rustfmt::skip] #[test] fn h_12()     {assert_eq!(Solution::odd_even_list(l![1,2]),         l![1,2]);}
    #[rustfmt::skip] #[test] fn h_123()    {assert_eq!(Solution::odd_even_list(l![1,2,3]),       l![1,3,2]);}
    #[rustfmt::skip] #[test] fn h_1234()   {assert_eq!(Solution::odd_even_list(l![1,2,3,4]),     l![1,3,2,4]);}
    #[rustfmt::skip] #[test] fn h_12345()  {assert_eq!(Solution::odd_even_list(l![1,2,3,4,5]),   l![1,3,5,2,4]);}
    #[rustfmt::skip] #[test] fn h_123456() {assert_eq!(Solution::odd_even_list(l![1,2,3,4,5,6]), l![1,3,5,2,4,6]);}

    #[rustfmt::skip] #[test] fn h_2135647() {assert_eq!(Solution::odd_even_list(l![2,1,3,5,6,4,7]), l![2,3,6,7,1,5,4]);}
}
