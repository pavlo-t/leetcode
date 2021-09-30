#![allow(dead_code)]

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
#[derive(PartialEq, Eq, Clone)]
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
impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        struct ListIter<'a> {
            curr: Option<i32>,
            next: &'a Option<Box<ListNode>>,
        }
        impl Iterator for ListIter<'_> {
            type Item = i32;

            fn next(&mut self) -> Option<Self::Item> {
                if let Some(i) = self.curr {
                    if let Some(n) = self.next {
                        self.curr = Some(n.val);
                        self.next = &n.next;
                    } else {
                        self.curr = None;
                    }
                    Some(i)
                } else {
                    None
                }
            }
        }
        let iter = ListIter {
            curr: Some(self.val),
            next: &self.next,
        };
        f.debug_list().entries(iter).finish()
        //let mut c = &self.next;
        //let mut data = vec![&self.val];
        //while let Some(n) = c {
        //    data.push(&n.val);
        //    c = &n.next;
        //}
        //f.debug_list().entries(data.iter()).finish()
    }
}

/// Split Linked List in Parts
/// ==========================
///
/// Given the `head` of a singly linked list and an integer `k`,
/// split the linked list into `k` consecutive linked list parts.
///
/// The length of each part should be as equal as possible: no two parts should have a size differing by more than one.
/// This may lead to some parts being null.
///
/// The parts should be in the order of occurrence in the input list,
/// and parts occurring earlier should always have a size greater than or equal to parts occurring later.
///
/// Return _an array of the `k` parts_.
///
/// __Constraints:__
///
/// - The number of nodes in the list is in the range `[0, 1000]`.
/// - `0 <= Node.val <= 1000`
/// - `1 <= k <= 50`
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/640/week-5-september-29th-september-30th/3992/
struct Solution;
impl Solution {
    pub fn split_list_to_parts(
        mut head: Option<Box<ListNode>>,
        k: i32,
    ) -> Vec<Option<Box<ListNode>>> {
        println!("split_list_to_parts({:?}, {})", head, k);

        type L = Option<Box<ListNode>>;

        fn size(mut l: &L) -> usize {
            let mut size = 0;
            while let Some(n) = l {
                size += 1;
                l = &n.next;
            }
            size
        }
        fn take(l: &mut L, mut k: usize) -> L {
            if k == 0 {
                None
            } else {
                let mut head = l.take();
                let mut curr = &mut head;
                while let Some(node) = curr {
                    if k <= 1 {
                        if let Some(new_head) = node.next.take() {
                            l.replace(new_head);
                        }
                        break;
                    } else {
                        curr = &mut node.next;
                        k -= 1;
                    }
                }
                head
            }
        }

        let k = k as usize;
        if head.is_none() {
            vec![None; k]
        } else {
            let mut results = vec![None; k];
            let size = size(&head);
            let bucket_size = size / k;
            let larger_buckets = size % k;
            for i in 0..k {
                let size = bucket_size + if i < larger_buckets { 1 } else { 0 };
                results[i] = take(&mut head, size);
            }
            results
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type L = Option<Box<ListNode>>;
    #[rustfmt::skip]
    fn l(v: i32, n: L) -> L { Some(Box::new(ListNode { val: v, next: n })) }

    macro_rules! l {
        () => {None};
        ($x:expr) => {l($x, None)};
        ($x:expr,$($r:expr),+) => {l($x, l!($($r),+))};
    }
    macro_rules! vl {
        ($($x:tt),*) => {vec![$(l!$x),*]};
    }

    #[test]
    fn h_1_2_3_k_5() {
        let h = l![1, 2, 3];
        let k = 5;
        let e = vl![[1], [2], [3], [], []];
        assert_eq!(Solution::split_list_to_parts(h, k), e);
        // Explanation:
        // The first element output[0] has output[0].val = 1, output[0].next = null.
        // The last element output[4] is null, but its string representation as a ListNode is [].
    }
    #[test]
    fn h_1_2_3_4_5_6_7_8_9_10_k_3() {
        let h = l![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let k = 3;
        let e = vl![[1, 2, 3, 4], [5, 6, 7], [8, 9, 10]];
        assert_eq!(Solution::split_list_to_parts(h, k), e);
        // Explanation:
        // The input has been split into consecutive parts with size difference at most 1,
        // and earlier parts are a larger size than the later parts.
    }
    #[test]
    fn h_n_k_2() {
        let h = None;
        let k = 2;
        let e = vl![[], []];
        assert_eq!(Solution::split_list_to_parts(h, k), e);
    }
}
