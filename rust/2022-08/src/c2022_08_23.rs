#![allow(dead_code)]
//! \#234. Palindrome Linked List
//! =============================
//!
//! <https://leetcode.com/problems/palindrome-linked-list>
//!
//! Given the `head` of a singly linked list, return `true` if it is a palindrome.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_08::c2022_08_23::*;
//! # use c2022_08::l;
//! # use c2022_08::list_node::*;
//! let head = l![1, 2, 2, 1];
//! assert_eq!(Solution::is_palindrome(head), true);
//! ```
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_23::*;
//! # use c2022_08::l;
//! # use c2022_08::list_node::*;
//! let head = l![1, 2];
//! assert_eq!(Solution::is_palindrome(head), false);
//! ```
//!
//! ##### Constraints
//!
//! - The number of nodes in the list is in the range `[1, 100_000]`.
//! - `0 <= Node.val <= 9`
//!
//! ##### Follow up
//!
//! Could you do it in `O(n)` time and `O(1)` space?

use crate::list_node::*;

pub struct Solution;
impl Solution {
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        let mut vec = vec![];
        while let Some(node) = head {
            vec.push(node.val);
            head = node.next;
        }

        let (mut l, mut r) = (0, vec.len() - 1);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::l;

    #[rustfmt::skip] #[test] fn l_1() { assert_eq!(Solution::is_palindrome(l![1]), true); }

    #[rustfmt::skip] #[test] fn l_1_2() { assert_eq!(Solution::is_palindrome(l![1,2]), false); }
    #[rustfmt::skip] #[test] fn l_1_1() { assert_eq!(Solution::is_palindrome(l![1,1]), true); }

    #[rustfmt::skip] #[test] fn l_1_1_1() { assert_eq!(Solution::is_palindrome(l![1,1,1]), true); }
    #[rustfmt::skip] #[test] fn l_1_2_1() { assert_eq!(Solution::is_palindrome(l![1,2,1]), true); }
    #[rustfmt::skip] #[test] fn l_1_1_2() { assert_eq!(Solution::is_palindrome(l![1,1,2]), false); }
    #[rustfmt::skip] #[test] fn l_2_1_1() { assert_eq!(Solution::is_palindrome(l![2,1,1]), false); }

    #[rustfmt::skip] #[test] fn l_1_2_2_1() { assert_eq!(Solution::is_palindrome(l![1,2,2,1]), true); }
}
