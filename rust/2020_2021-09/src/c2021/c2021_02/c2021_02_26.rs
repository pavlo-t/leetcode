#![allow(dead_code)]

/// # Validate Stack Sequences
///
/// Given two sequences `pushed` and `popped` __with distinct values__,
/// return `true` if and only if this could have been the result of
/// a sequence of push and pop operations on an initially empty stack.
///
/// __Constraints:__
///
/// - `0 <= pushed.length == popped.length <= 1000`
/// - `0 <= pushed[i], popped[i] < 1000`
/// - `pushed` is a permutation of `popped`.
/// - `pushed` and `popped` have distinct values.
///
/// https://leetcode.com/explore/featured/card/february-leetcoding-challenge-2021/587/week-4-february-22nd-february-28th/3653/
struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack: Vec<i32> = Vec::with_capacity(pushed.len());
        let mut j = 0;
        for i in pushed {
            stack.push(i);
            while let Some(last) = stack.pop() {
                if popped[j] == last {
                    j += 1;
                } else {
                    stack.push(last);
                    break;
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_pu12345_po45321_should_be_true() {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 5, 3, 2, 1];
        assert!(Solution::validate_stack_sequences(pushed, popped));
        // Explanation: We might do the following sequence:
        // push(1), push(2), push(3), push(4), pop() -> 4, push(5), pop() -> 5, pop() -> 3,
        // pop() -> 2, pop() -> 1
    }
    #[test]
    fn example2_pu12345_po43512_should_be_false() {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 3, 5, 1, 2];
        assert!(!Solution::validate_stack_sequences(pushed, popped));
        // Explanation: 1 cannot be popped before 2.
    }
    #[test]
    fn pu12345_po43521_should_be_true() {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 3, 5, 2, 1];
        assert!(Solution::validate_stack_sequences(pushed, popped));
    }
}
