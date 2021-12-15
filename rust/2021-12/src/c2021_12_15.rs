#![allow(dead_code)]
/// 147. Insertion Sort List
/// ========================
///
/// Given the `head` of a singly linked list,
/// sort the list using __insertion sort__,
/// and return the _sorted list's head_.
///
/// The steps of the __insertion sort__ algorithm:
///
/// 1. Insertion sort iterates, consuming one input element each repetition and growing a sorted output list.
/// 2. At each iteration, insertion sort removes one element from the input data,
///    finds the location it belongs within the sorted list and inserts it there.
/// 3. It repeats until no input elements remain.
///
/// The following is a graphical example of the insertion sort algorithm.
/// The partially sorted list initially contains only the first element in the list.
/// One element is removed from the input data and inserted in-place into the sorted list with each iteration.
///
/// ```text
/// in: [6,5,3,1,8,7,2,4] out: []
/// in: [  5,3,1,8,7,2,4] out: [6]
/// in: [    3,1,8,7,2,4] out: [5,6]
/// in: [      1,8,7,2,4] out: [3,5,6]
/// in: [        8,7,2,4] out: [1,3,5,6]
/// in: [          7,2,4] out: [1,3,5,6,8]
/// in: [            2,4] out: [1,3,5,6,7,8]
/// in: [              4] out: [1,2,3,5,6,7,8]
/// in: [               ] out: [1,2,3,4,5,6,7,8]
/// ```
///
/// __Constraints:__
///
/// - The number of nodes in the list is in the range `[1, 5000]`.
/// - `-5000 <= Node.val <= 5000`
///
/// https://leetcode.com/problems/insertion-sort-list/
struct Solution;
type L = Option<Box<ListNode>>;
impl Solution {
    pub fn insertion_sort_list_my(head: L) -> L {
        #[inline]
        fn wrap(l: ListNode) -> L {
            Some(Box::new(l))
        }
        fn insert(val: i32, mut head: ListNode) -> ListNode {
            if val <= head.val {
                ListNode {
                    val,
                    next: wrap(head),
                }
            } else {
                let next = head.next.take();
                if let Some(node) = next {
                    head.next = wrap(insert(val, *node));
                } else {
                    head.next = wrap(ListNode::new(val));
                }
                head
            }
        }

        let n = head.unwrap();
        let mut result = ListNode::new(n.val);
        let mut curr = n.next;
        while let Some(n) = curr {
            result = insert(n.val, result);
            curr = n.next;
        }
        wrap(result)
    }

    /// from other submissions <https://leetcode.com/submissions/detail/602201699/>
    pub fn insertion_sort_list(head: L) -> L {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut curr = head;

        while curr.is_some() {
            let mut left = dummy_head.as_mut();
            while left.as_ref().unwrap().next.is_some()
                && left.as_ref().unwrap().next.as_ref().unwrap().val < curr.as_ref().unwrap().val
            {
                left = left.unwrap().next.as_mut();
            }
            let right = left.as_mut().unwrap().next.take();
            let next = curr.as_mut().unwrap().next.take();
            curr.as_mut().unwrap().next = right;
            left.as_mut().unwrap().next = curr;

            curr = next;
        }
        dummy_head.unwrap().next
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq, Clone)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }
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

#[rustfmt::skip]
impl ListNode {
    #[inline] fn new(val: i32) -> Self { ListNode { next: None, val } }
    #[inline] fn wrap(self) -> L { Some(Box::new(self)) }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! l {
        () => { N };
        ($x:expr) => { ListNode::new($x).wrap() };
        ($x:expr,$($rest:expr),+) => { ListNode { val: $x, next: l![$($rest),+] }.wrap() };
    }

    #[rustfmt::skip] #[test] fn p1()   { assert_eq!(Solution::insertion_sort_list(l![1  ]), l![1  ]); }
    #[rustfmt::skip] #[test] fn p1p2() { assert_eq!(Solution::insertion_sort_list(l![1,2]), l![1,2]); }
    #[rustfmt::skip] #[test] fn p2p1() { assert_eq!(Solution::insertion_sort_list(l![2,1]), l![1,2]); }

    #[test]
    fn p4p2p1p3() {
        let h = l![4, 2, 1, 3];
        let e = l![1, 2, 3, 4];
        assert_eq!(Solution::insertion_sort_list(h), e);
    }
    #[test]
    fn m1p5p3p4n0() {
        let h = l![-1, 5, 3, 4, 0];
        let e = l![-1, 0, 3, 4, 5];
        assert_eq!(Solution::insertion_sort_list(h), e);
    }

    fn build_list(from: i32, mut to: i32, step: i32) -> L {
        let mut result = None;
        let from_lt_to = from < to;
        while (from < to) == from_lt_to {
            result = ListNode {
                val: to,
                next: result,
            }
            .wrap();
            to -= step;
        }
        if from_lt_to {
            ListNode {
                val: to,
                next: result,
            }
            .wrap()
        } else {
            result
        }
    }
    #[test]
    fn l1_to_5000() {
        let h = build_list(1, 5000, 1);
        let e = build_list(1, 5000, 1);
        assert_eq!(Solution::insertion_sort_list(h), e);
    }
    #[test]
    fn l5000_to_1() {
        let h = build_list(5000, 1, -1);
        let e = build_list(1, 5000, 1);
        assert_eq!(Solution::insertion_sort_list(h), e);
    }
}
