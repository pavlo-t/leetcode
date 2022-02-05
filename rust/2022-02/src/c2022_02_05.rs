#![allow(dead_code)]
/// 23. Merge k Sorted Lists
/// ========================
///
/// You are given an array of `k` linked-lists `lists`, each linked-list is sorted in ascending order.
///
/// _Merge all the linked-lists into one sorted linked-list and return it_.
///
/// __Constraints:__
///
/// - `k == lists.length`
/// - `0 <= k <= 10_000`
/// - `0 <= lists[i].length <= 500`
/// - `-10_000 <= lists[i][j] <= 10_000`
/// - `lists[i]` is sorted in __ascending order__.
/// - The sum of `lists[i].length` won't exceed `10_000`.
///
/// https://leetcode.com/problems/merge-k-sorted-lists/
struct Solution;
type L = Option<Box<ListNode>>;
impl Solution {
    pub fn merge_k_lists(lists: Vec<L>) -> L {
        use std::cmp;
        use std::collections::BinaryHeap;

        #[rustfmt::skip] impl PartialOrd for ListNode {
            fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> { Some(other.val.cmp(&self.val)) }
        }
        #[rustfmt::skip] impl Ord for ListNode {
            fn cmp(&self, other: &Self) -> cmp::Ordering { other.val.cmp(&self.val) }
        }

        if lists.is_empty() {
            None
        } else if lists.len() == 1 {
            lists[0].to_owned()
        } else {
            let mut heap = BinaryHeap::new();
            lists
                .into_iter()
                .filter_map(|l| l)
                .for_each(|l| heap.push(l));
            let mut dummy = ListNode::new(0);
            let mut curr = &mut dummy;
            while let Some(mut l) = heap.pop() {
                if let Some(next) = l.next.take() {
                    heap.push(next);
                }
                curr.next = Some(l);
                curr = curr.next.as_mut().unwrap();
            }
            dummy.next
        }
    }
}

#[rustfmt::skip]
#[derive(PartialEq, Eq, Clone)]
pub struct ListNode { pub val: i32, pub next: L }
#[rustfmt::skip] impl ListNode {
    #[inline] fn new(val: i32) -> Self { ListNode { next: None, val } }
    #[inline] fn wrap(self) -> L { Some(Box::new(self)) }
    #[inline] fn from_slice(slice: &[i32]) -> L {
        let mut curr = None;
        for &val in slice.iter().rev() {
            curr = ListNode { val, next: curr }.wrap();
        }
        curr
    }
}
impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut entries = vec![self.val];
        let mut curr = self.next.as_ref();
        while let Some(l) = curr {
            entries.push(l.val);
            curr = l.next.as_ref();
        }
        f.debug_list().entries(&entries).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: L = None;
    macro_rules! l {
        () => { N };
        ($x:expr) => { ListNode::new($x).wrap() };
        ($x:expr, $($rest:expr),+) => { ListNode { val: $x, next: l![$($rest),+] }.wrap() };
    }

    #[rustfmt::skip] #[test] fn empty_vec() { assert_eq!(Solution::merge_k_lists(vec![]), l![]); }
    #[rustfmt::skip] #[test] fn l_n() { assert_eq!(Solution::merge_k_lists(vec![l![]]), l![]); }
    #[rustfmt::skip] #[test] fn l_n_l_n() { assert_eq!(Solution::merge_k_lists(vec![l![], l![]]), l![]); }
    #[rustfmt::skip] #[test] fn l_1() { assert_eq!(Solution::merge_k_lists(vec![l![1]]), l![1]); }

    #[test]
    fn l_2_l_1() {
        let ls = vec![l![2], l![1]];
        let e = l![1, 2];
        assert_eq!(Solution::merge_k_lists(ls), e);
    }
    #[test]
    fn l_1_l_3_l_2() {
        let ls = vec![l![1], l![3], l![2]];
        let e = l![1, 2, 3];
        assert_eq!(Solution::merge_k_lists(ls), e);
    }
    #[test]
    fn l_1_4_5_l_1_3_4_l_2_6() {
        let ls = vec![l![1, 4, 5], l![1, 3, 4], l![2, 6]];
        let e = l![1, 1, 2, 3, 4, 4, 5, 6];
        assert_eq!(Solution::merge_k_lists(ls), e);
        // Explanation: The linked-lists are:
        // [
        //   1->4->5,
        //   1->3->4,
        //   2->6
        // ]
        // merging them into one sorted list:
        // 1->1->2->3->4->4->5->6
    }

    /// If getting stack overflow: (2 ** 27)
    /// RUST_MIN_STACK=134217728 cargo test --lib c2022_02_05
    #[test]
    fn l_1_to_500_repeat_10000() {
        let child = std::thread::Builder::new()
            .stack_size(2usize.pow(31))
            .spawn(move || {
                let mut ls = vec![];
                let vec = (1..=500).collect::<Vec<_>>();
                for _ in 0..10000 {
                    ls.push(ListNode::from_slice(&vec));
                }
                let e = {
                    let vec = (1..=500)
                        .flat_map(|val| std::iter::repeat(val).take(10000))
                        .collect::<Vec<i32>>();
                    ListNode::from_slice(&vec)
                };
                assert_eq!(Solution::merge_k_lists(ls), e);
            })
            .unwrap();
        child.join().unwrap();
    }
}
