#![allow(dead_code)]
/// 82. Remove Duplicates from Sorted List II
/// =========================================
///
/// Given the `head` of a sorted linked list,
/// _delete all nodes that have duplicate numbers,
/// leaving only distinct numbers from the original list_.
/// Return _the linked list __sorted__ as well_.
///
/// __Constraints:__
///
/// - The number of nodes in the list is in the range `[0, 300]`.
/// - `-100 <= Node.val <= 100`
/// - The list is guaranteed to be __sorted__ in ascending order.
///
/// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/
struct Solution;
type L = Option<Box<ListNode>>;
impl Solution {
    pub fn delete_duplicates(mut head: L) -> L {
        let mut to_delete: Option<i32> = None;
        let mut data = vec![];
        while let Some(n) = head {
            if data.last().filter(|&&last| last == n.val).is_some() {
                data.pop();
                to_delete = Some(n.val);
            }
            if to_delete.filter(|&del_val| del_val == n.val).is_none() {
                data.push(n.val);
            }
            head = n.next;
        }

        let mut result = None;
        while let Some(val) = data.pop() {
            result = Some(Box::new(ListNode { val, next: result }));
        }
        result
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq, Clone)]
pub struct ListNode { pub val: i32, pub next: L }
#[rustfmt::skip]
impl ListNode {
    #[inline] fn new(val: i32) -> Self { ListNode { next: None, val } }
    #[inline] fn wrap(self) -> L { Some(Box::new(self)) }
}
impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut data = vec![self.val];
        let mut next = &self.next;
        while let Some(n) = next {
            data.push(n.val);
            next = &n.next;
        }
        f.debug_list().entries(data.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: L = None;

    macro_rules! l {
        ($x:expr) => { ListNode::new($x).wrap() };
        ($x:expr,$($rest:expr),+) => { ListNode { val: $x, next: l![$($rest),+] }.wrap() };
    }

    #[rustfmt::skip] #[test] fn empty() { assert_eq!(Solution::delete_duplicates(N), N); }
    #[rustfmt::skip] #[test] fn p1() { assert_eq!(Solution::delete_duplicates(l![1]), l![1]); }
    #[rustfmt::skip] #[test] fn p1p2() { assert_eq!(Solution::delete_duplicates(l![1,2]), l![1,2]); }
    #[rustfmt::skip] #[test] fn p1p1() { assert_eq!(Solution::delete_duplicates(l![1,1]), N); }
    #[test]
    fn p1p1p1p2p3() {
        let h = l![1, 1, 1, 2, 3];
        let e = l![2, 3];
        assert_eq!(Solution::delete_duplicates(h), e);
    }
    #[test]
    fn p1p2p3p3p4p4p5() {
        let h = l![1, 2, 3, 3, 4, 4, 5];
        let e = l![1, 2, 5];
        assert_eq!(Solution::delete_duplicates(h), e);
    }
}
