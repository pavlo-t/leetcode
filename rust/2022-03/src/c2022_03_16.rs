#![allow(dead_code)]
/// 946. Validate Stack Sequences
/// =============================
///
/// Given two integer arrays `pushed` and `popped` each with distinct values,
/// return _`true` if this could have been the result of a sequence
/// of push and pop operations on an initially empty stack, or `false` otherwise_.
///
/// __Constraints:__
///
/// - `1 <= pushed.length <= 1000`
/// - `0 <= pushed[i] <= 1000`
/// - All the elements of `pushed` are __unique__.
/// - `popped.length == pushed.length`
/// - `popped` is a permutation of `pushed`.
///
/// https://leetcode.com/problems/validate-stack-sequences/
struct Solution;
impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut pop_i = 0;
        let mut stack = vec![];
        for n in pushed {
            stack.push(n);
            while stack.last().filter(|&&n| n == popped[pop_i]).is_some() {
                stack.pop();
                pop_i += 1;
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn push_1_pop_1() { assert!(Solution::validate_stack_sequences(vec![1], vec![1])); }
    #[rustfmt::skip] #[test] fn push_1_2_pop_1_2() { assert!(Solution::validate_stack_sequences(vec![1,2], vec![1,2])); }
    #[rustfmt::skip] #[test] fn push_1_2_pop_2_1() { assert!(Solution::validate_stack_sequences(vec![1,2], vec![2,1])); }

    #[test]
    fn push_1_2_3_4_5_pop_4_5_3_2_1() {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 5, 3, 2, 1];
        assert_eq!(Solution::validate_stack_sequences(pushed, popped), true);
        // Explanation: We might do the following sequence:
        // push(1), push(2), push(3), push(4),
        // pop() -> 4,
        // push(5),
        // pop() -> 5, pop() -> 3, pop() -> 2, pop() -> 1
    }
    #[test]
    fn push_1_2_3_4_5_pop_4_3_5_1_2() {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 3, 5, 1, 2];
        assert_eq!(Solution::validate_stack_sequences(pushed, popped), false);
        // Explanation: 1 cannot be popped before 2.
    }
}
