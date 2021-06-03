#![allow(dead_code)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self { ListNode { next: None, val } }
}


struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn plus_one(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn rec(n: &mut Option<Box<ListNode>>) -> bool {
            match n {
                None => true,
                Some(n) => {
                    match (rec(&mut n.next), n.val) {
                        (true, 9) => {
                            n.val = 0;
                            true
                        }
                        (true, _) => {
                            n.val += 1;
                            false
                        }
                        _ => false
                    }
                }
            }
        }

        let mut head = head;

        if rec(&mut head) {
            Some(Box::new(ListNode { val: 1, next: head }))
        } else {
            head
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // @formatter:off
    type List = Option<Box<ListNode>>;
    fn wrap(ln: ListNode) -> List { Some(Box::new(ln)) }
    fn l(v: i32, n: List) -> List { wrap(ListNode { val: v, next: n }) }
    // @formatter:on

    #[test]
    fn example1_h123_should_be_124() {
        let head = l(1, l(2, l(3, None)));
        let expected = l(1, l(2, l(4, None)));
        assert_eq!(Solution::plus_one(head), expected);
    }

    #[test]
    fn example2_h0_should_be_1() {
        assert_eq!(Solution::plus_one(l(0, None)), l(1, None));
    }

    #[test]
    fn h129_should_be_130() {
        let head = l(1, l(2, l(9, None)));
        let expected = l(1, l(3, l(0, None)));
        assert_eq!(Solution::plus_one(head), expected);
    }

    #[test]
    fn h999_should_be_1000() {
        let head = l(9, l(9, l(9, None)));
        let expected = l(1, l(0, l(0, l(0, None))));
        assert_eq!(Solution::plus_one(head), expected);
    }
}
