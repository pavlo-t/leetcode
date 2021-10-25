#![allow(dead_code)]
/// 155. Min Stack
/// ==============
///
/// Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
///
/// Implement the `MinStack` class:
///
/// - `MinStack()` initializes the stack object.
/// - `void push(int val)` pushes the element `val` onto the stack.
/// - `void pop()` removes the element on the top of the stack.
/// - `int top()` gets the top element of the stack.
/// - `int getMin()` retrieves the minimum element in the stack.
///
/// __Constraints:__
///
/// - `-2**31 <= val <= 2**31 - 1`
/// - Methods `pop`, `top` and `getMin` operations will always be called on __non-empty__ stacks.
/// - At most `30_000` calls will be made to `push`, `pop`, `top`, and `getMin`.
///
/// https://leetcode.com/problems/min-stack/
struct MinStack {
    data: Vec<(i32, i32)>,
}
impl MinStack {
    fn new() -> Self {
        Self { data: vec![] }
    }
    fn push(&mut self, val: i32) {
        let last_min = self.data.last().map(|&(_, m)| m).unwrap_or(i32::MAX);
        self.data.push((val, last_min.min(val)))
    }
    fn pop(&mut self) {
        self.data.pop();
    }
    fn top(&self) -> i32 {
        self.data.last().map(|&(v, _)| v).unwrap_or(i32::MAX)
    }
    fn get_min(&self) -> i32 {
        self.data.last().map(|&(_, m)| m).unwrap_or(i32::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut ms = MinStack::new();
        ms.push(-2);
        ms.push(0);
        ms.push(-3);
        assert_eq!(ms.get_min(), -3);
        ms.pop();
        assert_eq!(ms.top(), 0);
        assert_eq!(ms.get_min(), -2);
    }
}
