// @formatter:off
#![allow(dead_code)]
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

use rand::Rng;

struct Solution {
    data: Vec<i32>
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    /** @param head The linked list's head.
        Note that the head is guaranteed to be not null, so it contains at least one node. */
    fn new(head: Option<Box<ListNode>>) -> Self {
        let mut data = Vec::new();
        let mut current = &head;
        while let Some(node) = current {
            let n = node.as_ref();
            data.push(n.val);
            current = &n.next;
        }

        Self { data }
    }

    fn new_ok(head: Option<Box<ListNode>>) -> Self {
        let mut data = Vec::new();
        let mut current = head;
        while let Some(n) = current {
            data.push(n.val);
            current = n.next;
        }
        Self { data }
    }
    fn new_bad(head: Option<Box<ListNode>>) -> Self {
        let mut data = Vec::new();
        let mut current = head;
        while let Some(node) = current {
            let n = node.to_owned();
            data.push(n.val);
            current = n.next;
        }

        Self { data }
    }

    /** Returns a random node's value. */
    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..self.data.len());
        self.data[i]
    }
}

struct SolutionReservoirSampling {
    head: Option<Box<ListNode>>
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SolutionReservoirSampling {
    /** @param head The linked list's head.
        Note that the head is guaranteed to be not null, so it contains at least one node. */
    fn new(head: Option<Box<ListNode>>) -> Self {
        Self { head }
    }

    /** Returns a random node's value. */
    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();

        let mut result = 0;
        let mut size = 0;
        let mut current = &self.head;

        while let Some(node) = current {
            let n = node.as_ref();
            size += 1;
            if rng.gen::<f64>() < (1.0 / size as f64) {
                result = n.val;
            }

            current = &n.next;
        }

        result
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    type Node = Option<Box<ListNode>>;

    fn wrap(n: ListNode) -> Node { Some(Box::new(n)) }

    fn l(v: i32) -> Node { wrap(ListNode::new(v)) }
    fn ln(v: i32, n: Node) -> Node { wrap(ListNode { val: v, next: n }) }

    #[test]
    fn example_1_2_3() {
        let head = ln(1, ln(2, l(3)));
        let mut counts = HashMap::new();
        counts.insert(1, 0);
        counts.insert(2, 0);
        counts.insert(3, 0);

        let solution = Solution::new(head);

        for _ in 0..30000 {
            let r = solution.get_random();
            let x = counts.get_mut(&r).unwrap();
            *x += 1;
        }

        println!("{:?}", counts);
        counts.iter().for_each(|(_, &v)| assert!(v > 9500));
    }
    #[test]
    fn test_1_2() {
        let head = ln(1, l(2));
        let mut counts = HashMap::new();
        counts.insert(1, 0);
        counts.insert(2, 0);

        let solution = Solution::new(head);

        for _ in 0..1000 {
            let r = solution.get_random();
            let x = counts.get_mut(&r).unwrap();
            *x += 1;
        }

        println!("{:?}", counts);
        counts.iter().for_each(|(_, &v)| assert!(v > 450));
    }
    #[test]
    fn test_1to1000() {
        let mut head = None;
        let mut counts = HashMap::new();
        for i in (1..=1000).rev() {
            head = ln(i, head);
            counts.insert(i, 0);
        }

        let solution = Solution::new(head);

        for _ in 0..10000 {
            let r = solution.get_random();
            let x = counts.get_mut(&r).unwrap();
            *x += 1;
        }

        println!("{:?}", counts.iter().take(30));
        counts.iter().for_each(|(_, &v)| assert!(v >= 1));
    }
}