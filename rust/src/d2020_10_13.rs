//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
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

/// ### Sort List
///
/// Given the head of a linked list, return the list after sorting it in ascending order.
///
/// __Follow up__: Can you sort the linked list in <code>O(n log<sub>n</sub>)</code> time
/// and <code>O(1) memory</code> (i.e. constant space)?
///
/// <b>Constraints:</b><ul>
/// <li> The number of nodes in the list is in the range <code>[0, 5 * 10<sup>4</sup>]</code>.
/// <li> <code>-10<sup>5</sup> <= Node.val <= 10<sup>5</sup></code>
/// </ul>
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn sort_list_vector(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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

    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(h) if h.next == None => Some(h),
            Some(h) => {
                let (left, right) = Solution::split(Some(h));
                Solution::merge(Solution::sort_list(left), Solution::sort_list(right))
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
                    let next = Solution::merge(x.next, Some(y));
                    Some(Box::new(ListNode { val: x.val, next }))
                } else {
                    let next = Solution::merge(Some(x), y.next);
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
    fn example_1() {
        let input = ln(4, ln(2, ln(1, ln(3, None))));
        let expected = ln(1, ln(2, ln(3, ln(4, None))));

        let result = Solution::sort_list(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let input = ln(-1, ln(5, ln(3, ln(4, ln(0, None)))));
        let expected = ln(-1, ln(0, ln(3, ln(4, ln(5, None)))));

        let result = Solution::sort_list(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn example_3() {
        let input = None;
        let expected = None;

        let result = Solution::sort_list(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_1_el() {
        let input = ListNode { val: 1, next: None };
        let expected = Some(Box::new(ListNode { val: 1, next: None }));

        let result = Solution::sort_list(Some(Box::new(input)));

        assert_eq!(result, expected);
    }

    // =============================================================================================

    #[test]
    fn test_split_odd() {
        let input = ln(1, ln(2, ln(3, ln(4, ln(5, None)))));
        let x_left = ln(5, ln(3, ln(1, None)));
        let x_right = ln(4, ln(2, None));

        let (left, right) = Solution::split(input);

        assert_eq!(left, x_left);
        assert_eq!(right, x_right);
    }

    #[test]
    fn test_split_even() {
        let input = ln(1, ln(2, ln(3, ln(4, None))));
        let x_left = ln(3, ln(1, None));
        let x_right = ln(4, ln(2, None));

        let (left, right) = Solution::split(input);

        assert_eq!(left, x_left);
        assert_eq!(right, x_right);
    }
}
