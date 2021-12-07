#![allow(dead_code)]
/// 1290. Convert Binary Number in a Linked List to Integer
/// =======================================================
///
/// Given `head` which is a reference node to a singly-linked list.
/// The value of each node in the linked list is either `0` or `1`.
/// The linked list holds the binary representation of a number.
///
/// Return the _decimal value_ of the number in the linked list.
///
/// __Constraints:__
///
/// - The Linked List is not empty.
/// - Number of nodes will not exceed `30`.
/// - Each node's value is either `0` or `1`.
///
/// https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/
struct Solution;
type L = Option<Box<ListNode>>;
impl Solution {
    pub fn get_decimal_value(mut head: L) -> i32 {
        println!("get_decimal_value({:?})", head);
        let mut result = 0;
        while let Some(n) = head {
            result = (result << 1) | n.val;
            head = n.next;
        }
        result
    }
}

#[rustfmt::skip]
//#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(PartialEq, Eq, Clone)]
pub struct ListNode { pub val: i32, pub next: L }

#[rustfmt::skip] impl ListNode {
    #[inline] fn new(val: i32) -> Self { ListNode { next: None, val } }
    fn wrap(self) -> L { Some(Box::new(self)) }
}
impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut data = vec![self.val];
        let mut curr = self.next.clone();
        while let Some(n) = curr {
            data.push(n.val);
            curr = n.next.clone();
        }
        f.debug_list().entries(data.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! l {
        () => { None };
        ($x:expr) => { ListNode::new($x).wrap() };
        ($x:expr,$($rest:expr),+) => { ListNode { val: $x, next: l![$($rest),+] }.wrap() };
    }

    #[rustfmt::skip] #[test] fn  h0() { assert_eq!(Solution::get_decimal_value(  l![0]), 0); }
    #[rustfmt::skip] #[test] fn  h1() { assert_eq!(Solution::get_decimal_value(  l![1]), 1); }
    #[rustfmt::skip] #[test] fn h00() { assert_eq!(Solution::get_decimal_value(l![0,0]), 0); }
    #[test]
    fn h101() {
        assert_eq!(Solution::get_decimal_value(l![1, 0, 1]), 5);
        // Explanation: (101) in base 2 = (5) in base 10
    }
    #[rustfmt::skip] #[test] fn h100100111000000() {
        assert_eq!(Solution::get_decimal_value(l![1,0,0,1,0,0,1,1,1,0,0,0,0,0,0]), 18880);
    }
}
