#![allow(dead_code)]

use std::collections::HashMap;

/// # Maximum Frequency Stack
///
/// Implement `FreqStack`, a class which simulates the operation of a stack-like data structure.
///
/// `FreqStack` has two functions:
///
/// - `push(int x)`, which pushes an integer `x` onto the stack.
/// - `pop()`, which __removes__ and returns the most frequent element in the stack.
///   - If there is a tie for most frequent element,
///     the element closest to the top of the stack is removed and returned.
///
/// __Note:__
///
/// - Calls to `FreqStack.push(int x)` will be such that `0 <= x <= 10^9`.
/// - It is guaranteed that `FreqStack.pop()` won't be called if the stack has zero elements.
/// - The total number of `FreqStack.push` calls will not exceed `10000` in a single test case.
/// - The total number of `FreqStack.pop` calls will not exceed `10000` in a single test case.
/// - The total number of `FreqStack.push` and `FreqStack.pop` calls will not exceed `150000`
///   across all test cases.
///
/// https://leetcode.com/explore/featured/card/february-leetcoding-challenge-2021/587/week-4-february-22nd-february-28th/3655/
struct FreqStack {
    frequencies: HashMap<i32, usize>,
    freq_stacks: HashMap<usize, Vec<i32>>,
    max_frequency: usize,
}

impl FreqStack {
    fn new() -> Self {
        FreqStack {
            frequencies: HashMap::new(),
            freq_stacks: HashMap::new(),
            max_frequency: 0,
        }
    }

    fn push(&mut self, x: i32) {
        let freq = self.frequencies.entry(x).or_insert(0);
        *freq += 1;
        self.freq_stacks.entry(*freq).or_insert(Vec::new()).push(x);
        self.max_frequency = self.max_frequency.max(*freq);
    }

    fn pop(&mut self) -> i32 {
        let stack = self.freq_stacks.get_mut(&self.max_frequency).unwrap();
        let el = stack.pop().unwrap();
        if stack.is_empty() {
            self.max_frequency -= 1;
        }
        *self.frequencies.get_mut(&el).unwrap() -= 1;

        el
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut stack = FreqStack::new();
        stack.push(5);
        stack.push(7);
        stack.push(5);
        stack.push(7);
        stack.push(4);
        stack.push(5);
        // Explanation:
        // After making six .push operations, the stack is [5,7,5,7,4,5] from bottom to top.

        assert_eq!(stack.pop(), 5);
        // pop() -> returns 5, as 5 is the most frequent.
        // The stack becomes [5,7,5,7,4].
        assert_eq!(stack.pop(), 7);
        // pop() -> returns 7, as 5 and 7 is the most frequent, but 7 is closest to the top.
        // The stack becomes [5,7,5,4].
        assert_eq!(stack.pop(), 5);
        // pop() -> returns 5.
        // The stack becomes [5,7,4].
        assert_eq!(stack.pop(), 4);
        // pop() -> returns 4.
        // The stack becomes [5,7].
    }

    #[test]
    fn test_push100000_elements() {
        let mut stack = FreqStack::new();
        for i in 1..=100_000 {
            stack.push(i);
        }
        for i in (1..=100_000).rev() {
            assert_eq!(stack.pop(), i);
        }
    }
}
