// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

/// Palindrome Linked List
/// ======================
///
/// Given the `head` of a singly linked list, return `true` if it is a palindrome.
///
/// __Constraints:__
///
/// - The number of nodes in the list is in the range `[1, 10^5]`.
/// - `0 <= Node.val <= 9`
///
/// __Follow up:__ Could you do it in `O(n)` time and `O(1)` space?
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/593/week-1-april-1st-april-7th/3693/
struct Solution;
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut vec = Vec::new();
        let mut c = &head;
        while let Some(n) = c {
            vec.push(n.val);
            c = &n.next;
        }
        let mut l = 0;
        let mut r = vec.len() - 1;
        while l < r {
            if vec[l] != vec[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}

mod tests {
    use super::*;

    type Node = Option<Box<ListNode>>;
    fn wrap(l: ListNode) -> Node { Some(Box::new(l)) }
    fn l(v: i32, n: Node) -> Node { wrap(ListNode { val: v, next: n }) }

    #[test]
    fn example1() {
        let head = l(1, l(2, l(2, l(1, None))));
        assert!(Solution::is_palindrome(head));
    }
    #[test]
    fn example2() {
        let head = l(1, l(2, None));
        assert!(!Solution::is_palindrome(head));
    }
}
