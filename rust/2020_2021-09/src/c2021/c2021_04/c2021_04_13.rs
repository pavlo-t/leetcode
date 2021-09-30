#![allow(dead_code)]

use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

/// Flatten Nested List Iterator
/// ============================
///
/// You are given a nested list of integers `nestedList`.
/// Each element is either an integer or a list whose elements may also be integers or other lists.
/// Implement an iterator to flatten it.
///
/// Implement the `NestedIterator` class:
///
/// - `NestedIterator(List<NestedInteger> nestedList)` Initializes the iterator with the nested list `nestedList`.
/// - `int next()` Returns the next integer in the nested list.
/// - `boolean hasNext()` Returns `true` if there are still some integers in the nested list and `false` otherwise.
///
/// __Constraints:__
///
/// - `1 <= nestedList.length <= 500`
/// - `The values of the integers in the nested list is in the range `[-1_000_000, 1_000_000]`.
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/594/week-2-april-8th-april-14th/3706/
struct NestedIterator {
    data: Vec<i32>,
    i: RefCell<usize>,
}
impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let data = nested_list
            .iter()
            .flat_map(|ni| Self::nested_integer_to_vec(ni))
            .collect::<Vec<_>>();
        let i = RefCell::new(0);
        Self { data, i }
    }

    fn next(&self) -> i32 {
        let mut i = self.i.borrow_mut();
        *i += 1;
        self.data[*i - 1]
    }

    fn has_next(&self) -> bool {
        *self.i.borrow() < self.data.len()
    }

    fn nested_integer_to_vec(ni: &NestedInteger) -> Vec<i32> {
        let mut v = Vec::new();
        match ni {
            NestedInteger::Int(i) => v.push(i.to_owned()),
            NestedInteger::List(is) => is
                .iter()
                .flat_map(|ni| Self::nested_integer_to_vec(ni))
                .for_each(|i| v.push(i)),
        }
        v
    }
}

struct NestedIteratorV1 {
    nested_list: Vec<NestedInteger>,
    i: RefCell<usize>,
    curr: RefCell<Vec<i32>>,
    j: RefCell<usize>,
}
impl NestedIteratorV1 {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let curr = RefCell::new(Self::nested_integer_to_vec(&nested_list[0]));
        Self {
            nested_list,
            i: RefCell::new(1),
            curr,
            j: RefCell::new(0),
        }
    }

    fn next(&self) -> i32 {
        let mut j = self.j.borrow_mut();
        let mut curr = self.curr.borrow_mut();
        if *j < curr.len() {
            let r = curr[*j];
            *j += 1;
            return r;
        } else {
            let mut i = self.i.borrow_mut();
            *curr = Self::nested_integer_to_vec(&self.nested_list[*i]);
            *i += 1;
            *j = 1;
            curr[0]
        }
    }

    fn has_next(&self) -> bool {
        *self.j.borrow() < self.curr.borrow().len() || *self.i.borrow() < self.nested_list.len()
    }

    fn nested_integer_to_vec(ni: &NestedInteger) -> Vec<i32> {
        let mut v = Vec::new();
        match ni {
            NestedInteger::Int(i) => v.push(i.to_owned()),
            NestedInteger::List(is) => is
                .iter()
                .flat_map(|ni| Self::nested_integer_to_vec(ni))
                .for_each(|i| v.push(i)),
        }
        v
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
#[cfg(test)]
mod tests {
    use NestedInteger::{Int as I, List as L};

    use super::*;

    #[test]
    fn example1() {
        let ni = NestedIterator::new(vec![L(vec![I(1), I(1)]), I(2), L(vec![I(1), I(1)])]);
        assert!(ni.has_next());
        assert_eq!(ni.next(), 1);
        assert!(ni.has_next());
        assert_eq!(ni.next(), 1);
        assert!(ni.has_next());
        assert_eq!(ni.next(), 2);
        assert!(ni.has_next());
        assert_eq!(ni.next(), 1);
        assert!(ni.has_next());
        assert_eq!(ni.next(), 1);
        assert!(!ni.has_next());
        // Input: nestedList = [[1,1],2,[1,1]]
        // Output: [1,1,2,1,1]
        // Explanation:
        // By calling next repeatedly until hasNext returns false,
        // the order of elements returned by next should be: [1,1,2,1,1].
    }
    #[test]
    fn example2() {
        let ni = NestedIterator::new(vec![I(1), L(vec![I(4), L(vec![I(6)])])]);
        assert!(ni.has_next());
        assert_eq!(ni.next(), 1);
        assert!(ni.has_next());
        assert_eq!(ni.next(), 4);
        assert!(ni.has_next());
        assert_eq!(ni.next(), 6);
        assert!(!ni.has_next());
        // Input: nestedList = [1,[4,[6]]]
        // Output: [1,4,6]
        // Explanation:
        // By calling next repeatedly until hasNext returns false,
        // the order of elements returned by next should be: [1,4,6].
    }

    #[test]
    fn test12() {
        let ni = NestedIterator::new(vec![L(vec![I(1)]), L(vec![])]);
        assert!(ni.has_next());
        assert_eq!(ni.next(), 1);
        assert!(!ni.has_next());
    }
}
