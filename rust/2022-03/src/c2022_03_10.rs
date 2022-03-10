#![allow(dead_code)]
/// 2. Add Two Numbers
/// ==================
///
/// You are given two __non-empty__ linked lists representing two non-negative integers.
/// The digits are stored in __reverse order__, and each of their nodes contains a single digit.
/// Add the two numbers and return the sum as a linked list.
///
/// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
///
/// __Constraints:__
///
/// - The number of nodes in each linked list is in the range `[1, 100]`.
/// - `0 <= Node.val <= 9`
/// - It is guaranteed that the list represents a number that does not have leading zeros.
///
/// https://leetcode.com/problems/add-two-numbers/
struct Solution;
type L = Option<Box<ListNode>>;
impl Solution {
    pub fn add_two_numbers(mut l1: L, mut l2: L) -> L {
        let mut result = vec![];
        let mut carry = 0;
        while l1.is_some() || l2.is_some() {
            let a = l1.as_ref().map(|l| l.val).unwrap_or(0);
            let b = l2.as_ref().map(|l| l.val).unwrap_or(0);

            let curr = a + b + carry;
            carry = (curr > 9) as i32;
            result.push(curr % 10);

            l1 = l1.and_then(|l| l.next);
            l2 = l2.and_then(|l| l.next);
        }
        if carry != 0 {
            result.push(1);
        }

        let mut list: L = None;
        while let Some(val) = result.pop() {
            list = Some(Box::new(ListNode { val, next: list }));
        }
        list
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq, Clone)]
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
        ($x:expr) => { ListNode::new($x).wrap() };
        ($x:expr,$($n:expr),+) => { ListNode { val: $x, next: l![$($n),+] }.wrap() };
    }

    #[rustfmt::skip] #[test] fn a_0_b_0() { assert_eq!(Solution::add_two_numbers(l![0], l![0]), l![0]); }
    #[rustfmt::skip] #[test] fn a_1_b_0() { assert_eq!(Solution::add_two_numbers(l![1], l![0]), l![1]); }
    #[rustfmt::skip] #[test] fn a_0_b_1() { assert_eq!(Solution::add_two_numbers(l![0], l![1]), l![1]); }
    #[rustfmt::skip] #[test] fn a_1_b_1() { assert_eq!(Solution::add_two_numbers(l![1], l![1]), l![2]); }
    #[rustfmt::skip] #[test] fn a_1_b_9() { assert_eq!(Solution::add_two_numbers(l![1], l![9]), l![0, 1]); }
    #[rustfmt::skip] #[test] fn a_9_b_1() { assert_eq!(Solution::add_two_numbers(l![9], l![1]), l![0, 1]); }

    #[test]
    fn a_2_4_3_b_5_6_4() {
        let (a, b) = (l![2, 4, 3], l![5, 6, 4]);
        let e = l![7, 0, 8];
        assert_eq!(Solution::add_two_numbers(a, b), e);
        // Explanation: 342 + 465 = 807.
    }
    #[test]
    fn a_9_9_9_9_9_9_9_l2_9_9_9_9() {
        let (a, b) = (l![9, 9, 9, 9, 9, 9, 9], l![9, 9, 9, 9]);
        let e = l![8, 9, 9, 9, 0, 0, 0, 1];
        assert_eq!(Solution::add_two_numbers(a, b), e);
    }
}
