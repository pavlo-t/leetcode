#![allow(dead_code)]

use std::fmt;

//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl fmt::Debug for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.next.is_none() { write!(f, "{}", self.val) } else {
            write!(f, "{},", self.val)?;
            self.next.as_ref().unwrap().fmt(f)
        }
    }
}

/// ### Sort List
///
/// Given the `head` of a linked list, return _the list after sorting it in **ascending order**_.
///
/// __Follow up__:
/// Can you sort the linked list in `O(n log n)` time and `O(1) memory` (i.e. constant space)?
///
/// __Constraints:__
///
/// - The number of nodes in the list is in the range `[0,50_000]`.
/// - `-100_000 <= Node.val <= 100_000`
struct Solution {}

struct Solution1 {}

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut values = Vec::new();
        let mut tmp = &head;
        while let Some(n) = tmp {
            values.push(n.val);
            tmp = &n.as_ref().next;
        }
        values.sort_by(|a, b| b.cmp(&a));

        let mut tmp = head;
        let mut current = &mut tmp;
        while let Some(n) = current {
            n.val = values.pop().unwrap();
            current = &mut n.next;
        }

        tmp
    }
}

impl Solution1 {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(h) if h.next == None => Some(h),
            Some(h) => {
                let (left, right) = Self::split(Some(h));
                Self::merge(Self::sort_list(left), Self::sort_list(right))
            }
        }
    }

    fn split(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut lhs = None;
        let mut rhs = None;
        let mut i = true;
        loop {
            head = match head {
                None => break,
                Some(mut headnode) => {
                    let head = headnode.next.take();
                    match i {
                        true => {
                            headnode.next = lhs.take();
                            lhs = Some(headnode);
                        }
                        false => {
                            headnode.next = rhs.take();
                            rhs = Some(headnode);
                        }
                    }
                    head
                }
            };
            i = !i;
        }
        (lhs, rhs)
    }

    fn split_my(head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let h = head.unwrap();

        let mut left_arr = [0; 25000];
        let mut left_idx = 0usize;

        let mut slow = h.clone();
        let mut fast = h.clone();

        loop {
            if fast.next == None { break; }
            fast = fast.next.unwrap();

            left_arr[left_idx] = slow.val;
            slow = slow.next.unwrap();
            left_idx += 1;

            if fast.next == None { break; }
            fast = fast.next.unwrap();
        }
        left_idx -= 1;

        let right = slow;

        let mut left: Option<Box<ListNode>> = None;
        loop {
            left = Some(Box::new(ListNode { val: left_arr[left_idx], next: left }));
            if left_idx > 0 {
                left_idx -= 1;
            } else {
                break;
            }
        }

        (left, Some(right))
    }

    fn merge(xs: Option<Box<ListNode>>, ys: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (xs, ys) {
            (None, r) => r,
            (r, None) => r,
            (Some(x), Some(y)) => {
                if x.val <= y.val {
                    let next = Self::merge(x.next, Some(y));
                    Some(Box::new(ListNode { val: x.val, next }))
                } else {
                    let next = Self::merge(Some(x), y.next);
                    Some(Box::new(ListNode { val: y.val, next }))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn ln(x: i32, n: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode { val: x, next: n }))
    }

    #[test]
    fn example_1_list_4_2_1_3() {
        let input = ln(4, ln(2, ln(1, ln(3, None))));
        let expected = ln(1, ln(2, ln(3, ln(4, None))));

        let result = Solution::sort_list(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn example_2_list_n1_5_3_4_0() {
        let input = ln(-1, ln(5, ln(3, ln(4, ln(0, None)))));
        let expected = ln(-1, ln(0, ln(3, ln(4, ln(5, None)))));

        let result = Solution::sort_list(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn example_3_empty_list() {
        let input = None;
        let expected = None;

        let result = Solution::sort_list(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_1_element_list() {
        let input = ListNode { val: 1, next: None };
        let expected = Some(Box::new(ListNode { val: 1, next: None }));

        let result = Solution::sort_list(Some(Box::new(input)));

        assert_eq!(result, expected);
    }

    /// If getting stack overflow:
    ///
    /// ```sh
    /// thread 'c2020_10::d2020_10_13::tests::max_size' has overflowed its stack
    /// fatal runtime error: stack overflow
    /// ```
    ///
    /// Add `RUST_MIN_STACK=33554432` to env
    #[test]
    fn test_max_size_list() {
        let size = 50_000;
        // let size = 50;

        let mut input: Option<Box<ListNode>> = None;
        let mut expected: Option<Box<ListNode>> = None;
        for i in 1..=size {
            input = ln(i, input);
            expected = ln(size + 1 - i, expected);
        }

        let result = Solution::sort_list(input);

        assert_eq!(result, expected);
    }

    // =============================================================================================

    // #[test]
    // fn test_split_odd() {
    //     let input = ln(1, ln(2, ln(3, ln(4, ln(5, None)))));
    //     let x_left = ln(5, ln(3, ln(1, None)));
    //     let x_right = ln(4, ln(2, None));
    //
    //     let (left, right) = Solution::split(input);
    //
    //     assert_eq!(left, x_left);
    //     assert_eq!(right, x_right);
    // }
    //
    // #[test]
    // fn test_split_even() {
    //     let input = ln(1, ln(2, ln(3, ln(4, None))));
    //     let x_left = ln(3, ln(1, None));
    //     let x_right = ln(4, ln(2, None));
    //
    //     let (left, right) = Solution::split(input);
    //
    //     assert_eq!(left, x_left);
    //     assert_eq!(right, x_right);
    // }
}
