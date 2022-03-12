#![allow(dead_code)]
/// 61. Rotate List
/// ===============
///
/// Given the `head` of a linked list, rotate the list to the right by `k` places.
///
/// __Constraints:__
///
/// - The number of nodes in the list is in the range `[0, 500]`.
/// - `-100 <= Node.val <= 100`
/// - `0 <= k <= 2_000_000_000`
///
/// https://leetcode.com/problems/rotate-list/
struct Solution;
type L = Option<Box<ListNode>>;
impl Solution {
    pub fn rotate_right(mut head: L, k: i32) -> L {
        let len = {
            let mut curr = &head;
            let mut len = 0;
            while let Some(l) = curr {
                len += 1;
                curr = &l.next;
            }
            len
        };

        if len > 1 && k % len != 0 {
            let mut curr = head.as_mut();
            let mut drop = len - k % len - 1;
            while drop > 0 {
                curr = curr.unwrap().next.as_mut();
                drop -= 1;
            }
            let mut new_head = curr.unwrap().next.take();

            let mut curr = new_head.as_mut();
            while curr.as_ref().filter(|l| l.next.is_some()).is_some() {
                curr = curr.unwrap().next.as_mut();
            }
            let curr = curr.unwrap();
            curr.next = head;

            new_head
        } else {
            head
        }
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq, Clone)]
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
        ($x:expr) => { ListNode::new($x).wrap() };
        ($x:expr,$($rest:expr),+) => { ListNode { val: $x, next: l![$($rest),+] }.wrap() };
    }

    #[rustfmt::skip] #[test] fn empty_k4() { assert_eq!(Solution::rotate_right(N, 4), N); }
    #[rustfmt::skip] #[test] fn p1_k4() { assert_eq!(Solution::rotate_right(l![1], 4), l![1]); }
    #[rustfmt::skip] #[test] fn p1p2_k0() { assert_eq!(Solution::rotate_right(l![1,2], 0), l![1,2]); }
    #[rustfmt::skip] #[test] fn p1p2_k1() { assert_eq!(Solution::rotate_right(l![1,2], 1), l![2,1]); }
    #[rustfmt::skip] #[test] fn p1p2_k2() { assert_eq!(Solution::rotate_right(l![1,2], 2), l![1,2]); }
    #[rustfmt::skip] #[test] fn p1p2_k3() { assert_eq!(Solution::rotate_right(l![1,2], 3), l![2,1]); }
    #[rustfmt::skip] #[test] fn p1p2_k4() { assert_eq!(Solution::rotate_right(l![1,2], 4), l![1,2]); }
    #[rustfmt::skip] #[test] fn p1p2_k5() { assert_eq!(Solution::rotate_right(l![1,2], 5), l![2,1]); }

    #[rustfmt::skip] #[test] fn p1p2p3_k0() { assert_eq!(Solution::rotate_right(l![1,2,3], 0), l![1,2,3]); }
    #[rustfmt::skip] #[test] fn p1p2p3_k1() { assert_eq!(Solution::rotate_right(l![1,2,3], 1), l![3,1,2]); }
    #[rustfmt::skip] #[test] fn p1p2p3_k2() { assert_eq!(Solution::rotate_right(l![1,2,3], 2), l![2,3,1]); }
    #[rustfmt::skip] #[test] fn p1p2p3_k3() { assert_eq!(Solution::rotate_right(l![1,2,3], 3), l![1,2,3]); }
    #[rustfmt::skip] #[test] fn p1p2p3_k4() { assert_eq!(Solution::rotate_right(l![1,2,3], 4), l![3,1,2]); }
    #[rustfmt::skip] #[test] fn p1p2p3_k5() { assert_eq!(Solution::rotate_right(l![1,2,3], 5), l![2,3,1]); }
    #[rustfmt::skip] #[test] fn p1p2p3_k6() { assert_eq!(Solution::rotate_right(l![1,2,3], 6), l![1,2,3]); }
    #[rustfmt::skip] #[test] fn p1p2p3_k7() { assert_eq!(Solution::rotate_right(l![1,2,3], 7), l![3,1,2]); }

    #[test]
    fn p1p2p3p4p5_k2() {
        let h = l![1, 2, 3, 4, 5];
        let e = l![4, 5, 1, 2, 3];
        assert_eq!(Solution::rotate_right(h, 2), e);
    }
    #[test]
    fn n0p1p2_k4() {
        let h = l![0, 1, 2];
        let e = l![2, 0, 1];
        assert_eq!(Solution::rotate_right(h, 4), e);
    }
}
