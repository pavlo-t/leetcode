#![allow(dead_code)]

use std::fmt;

// Definition for singly-linked list.
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

impl fmt::Debug for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.next.is_some() {
            write!(f, "{},{:?}", self.val, self.next.as_ref().unwrap())
        } else {
            write!(f, "{}", self.val)
        }
    }
}

/// # Intersection of Two Linked Lists
///
/// Write a program to find the node at which the intersection of two singly linked lists begins.
///
/// For example, the following two linked lists:
///
/// ```text
/// A:       a1 -> a2 \
///                     c1 -> c2 -> c3
/// B: b1 -> b2 -> b3 /
/// ```
///
/// begin to intersect at node c1.
///
/// __Notes:__
///
/// - If the two linked lists have no intersection at all, return `null`.
/// - The linked lists must retain their original structure after the function returns.
/// - You may assume there are no cycles anywhere in the entire linked structure.
/// - Each value on each linked list is in the range `[1, 10^9]`.
/// - Your code should preferably run in `O(n)` time and use only `O(1)` memory.
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/588/week-1-march-1st-march-7th/3660/
struct Solution;

impl Solution {
    // pub fn get_intersection_node(
    //     head_a: Option<Box<ListNode>>,
    //     head_b: Option<Box<ListNode>>,
    // ) -> Option<Box<ListNode>> {
    // }
}

#[cfg(test)]
mod tests {
    // use super::*;
    //
    // // @formatter:off
    // type List = Option<Box<&'static ListNode>>;
    // fn wrap(l: &'static ListNode) -> List { Some(Box::new(l)) }
    // fn l(v: i32, n: List) -> List {
    //     let n: &'static ListNode = &ListNode { val: v, next: n };
    //     wrap(n)
    // }
    // const N: List = None;
    // // @formatter:on
    //
    // #[test]
    // fn example1() {
    //     // let intersect = l(8, l(4, l(5, N)));
    //     // let intersect = Box::new(ListNode { val: 8, next: l(4, l(5, N)) });
    //     let intersect: &'static ListNode = &ListNode { val: 8, next: l(4, l(5, N)) };
    //     let la = l(4, l(1, Some(Box::new(intersect))));
    //     let lb = l(4, l(1, Some(Box::new(intersect))));
    //     let e = Some(Box::new(intersect));
    //
    //     assert_eq!(Solution::get_intersection_node(la, lb), e);
    // }
    // Input: intersectVal = 8, listA = [4,1,8,4,5], listB = [5,6,1,8,4,5], skipA = 2, skipB = 3
    // Output: Reference of the node with value = 8
    // Input Explanation: The intersected node's value is 8 (note that this must not be 0 if the two lists intersect). From the head of A, it reads as [4,1,8,4,5]. From the head of B, it reads as [5,6,1,8,4,5]. There are 2 nodes before the intersected node in A; There are 3 nodes before the intersected node in B.
    //
    //
    // Example 2:
    //
    //
    // Input: intersectVal = 2, listA = [1,9,1,2,4], listB = [3,2,4], skipA = 3, skipB = 1
    // Output: Reference of the node with value = 2
    // Input Explanation: The intersected node's value is 2 (note that this must not be 0 if the two lists intersect). From the head of A, it reads as [1,9,1,2,4]. From the head of B, it reads as [3,2,4]. There are 3 nodes before the intersected node in A; There are 1 node before the intersected node in B.
    //
    //
    // Example 3:
    //
    //
    // Input: intersectVal = 0, listA = [2,6,4], listB = [1,5], skipA = 3, skipB = 2
    // Output: null
    // Input Explanation: From the head of A, it reads as [2,6,4]. From the head of B, it reads as [1,5]. Since the two lists do not intersect, intersectVal must be 0, while skipA and skipB can be arbitrary values.
    // Explanation: The two lists do not intersect, so return null.
    //

    #[test]
    fn compare_pointers() {
        let a = 1;
        let p1: *const _ = &a;
        let p2: *const _ = &a;
        let b = 1;
        let p3: *const _ = &b;
        let c = 2;
        let p4: *const _ = &c;
        println!("p1 == p2: {}", p1 == p2);
        println!("p1 == p3: {}", p1 == p3);
        println!("p1 == p4: {}", p1 == p4);

        let p1: *const i32;
        {
            let x = Box::new(0);
            p1 = &*x;
        }
        {
            let y = Box::new(1);
            let p2: *const _ = &*y;
            println!("dangling p1 == p2: {}", p1 == p2);
        }
        {
            let x = Box::new(0);
            let p1: *const _ = &*x;
            let p2: *const _ = &*x;
            println!("same box p1 == p2: {}", p1 == p2);
        }
        {
            let x = Box::new(0);
            let p1: *const _ = &*x;
            let y = Box::new(0);
            let p2: *const _ = &*y;
            println!("different boxes x == y: {}", x == y);
            println!("different boxes p1 == p2: {}", p1 == p2);
        }
    }
}
