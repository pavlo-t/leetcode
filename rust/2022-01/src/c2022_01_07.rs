#![allow(dead_code)]
/// 382. Linked List Random Node
/// ============================
///
/// Given a singly linked list, return a random node's value from the linked list.
/// Each node must have the __same probability__ of being chosen.
///
/// Implement the Solution class:
///
/// - `Solution(ListNode head)` Initializes the object with the integer array nums.
/// - `int getRandom()` Chooses a node randomly from the list and returns its value.
///   All the nodes of the list should be equally likely to be choosen.
///
/// __Constraints:__
///
/// - The number of nodes in the linked list will be in the range `[1, 10_000]`.
/// - `-10_000 <= Node.val <= 10_000`
/// - At most `10_000` calls will be made to `getRandom`.
///
/// __Follow up:__
///
/// - What if the linked list is extremely large and its length is unknown to you?
/// - Could you solve this efficiently without using extra space?
///
/// https://leetcode.com/problems/linked-list-random-node/
struct SolutionMyVec {
    data: Vec<i32>,
}
impl SolutionMyVec {
    fn new(mut head: L) -> Self {
        let mut data = vec![];
        while let Some(n) = head {
            data.push(n.val);
            head = n.next;
        }
        Self { data }
    }

    fn get_random(&self) -> i32 {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        *self.data.choose(&mut rng).unwrap()
    }
}

/// Approach 2: Reservoir Sampling
/// https://leetcode.com/problems/linked-list-random-node/solution/
struct Solution {
    head: L,
}
impl Solution {
    fn new(head: L) -> Self {
        Self { head }
    }

    fn get_random(&self) -> i32 {
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let mut scope = 1.0;
        let mut chosen_val = 0;
        let mut curr = &self.head;
        while let Some(h) = curr {
            // decide whether to include the element in reservoir
            if rng.gen_range(0.0, 1.0) < (1.0 / scope) {
                chosen_val = h.val;
            }
            // move on to the next node
            scope += 1.0;
            curr = &h.next;
        }
        chosen_val
    }
}

type L = Option<Box<ListNode>>;

#[derive(PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl std::fmt::Debug for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut data = vec![self.val];
        let mut curr = &self.next;
        while let Some(n) = curr {
            data.push(n.val);
            curr = &n.next
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
    use std::collections::HashSet;

    const N: L = None;

    macro_rules! l {
        () => {
            N
        };
        ($x:expr) => {
            ListNode::new($x).wrap()
        };
        ($x:expr, $($rest:expr),+) => {
            ListNode { val: $x, next: l![$($rest),+] }.wrap()
        };
    }

    #[test]
    fn example_1() {
        let solution = Solution::new(l![1, 2, 3]);
        let mut seen: HashSet<i32> = HashSet::new();
        for _ in 0..10 {
            let curr = solution.get_random();
            seen.insert(curr);
        }
        let mut seen = seen.into_iter().collect::<Vec<_>>();
        seen.sort_unstable();
        assert_eq!(seen, [1, 2, 3]);
        // getRandom() should return either 1, 2, or 3 randomly.
        // Each element should have equal probability of returning.
    }
}
