#![allow(dead_code)]
use std::collections::{HashSet, VecDeque};
/// 1429. First Unique Number
/// =========================
///
/// You have a queue of integers, you need to retrieve the first unique integer in the queue.
///
/// Implement the `FirstUnique` class:
///
/// - `FirstUnique(int[] nums)` Initializes the object with the numbers in the queue.
/// - `int showFirstUnique()` returns the value of __the first unique__ integer of the queue,
///   and returns __-1__ if there is no such integer.
/// - `void add(int value)` insert value to the queue.
///
/// Constraints:
///
/// - `1 <= nums.length <= 100_000`
/// - `1 <= nums[i] <= 100_000_000`
/// - `1 <= value <= 100_000_000`
/// - At most `50000` calls will be made to `showFirstUnique` and `add`.
///
/// https://leetcode.com/problems/first-unique-number/
struct FirstUnique {
    queue: VecDeque<i32>,
    unique: HashSet<i32>,
    non_unique: HashSet<i32>,
}
impl FirstUnique {
    fn new(nums: Vec<i32>) -> Self {
        //println!("new({:?})", nums);
        let mut result = Self {
            queue: VecDeque::new(),
            unique: HashSet::new(),
            non_unique: HashSet::new(),
        };
        nums.into_iter().for_each(|value| result.add(value));
        result
    }
    fn show_first_unique(&self) -> i32 {
        //println!("show_first_unique()");
        if let Some(&curr) = self.queue.front() {
            curr
        } else {
            -1
        }
    }
    fn add(&mut self, value: i32) {
        //println!("add({})", value);
        if self.unique.contains(&value) {
            self.unique.remove(&value);
            self.non_unique.insert(value);
            if self.queue[0] == value {
                while let Some(curr) = self.queue.pop_front() {
                    if self.unique.contains(&curr) {
                        self.queue.push_front(curr);
                        break;
                    }
                }
            }
        } else if !self.non_unique.contains(&value) {
            self.unique.insert(value);
            self.queue.push_back(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut fu = FirstUnique::new(vec![2, 3, 5]);
        assert_eq!(fu.show_first_unique(), 2);
        fu.add(5); // the queue is now [2,3,5,5]
        assert_eq!(fu.show_first_unique(), 2);
        fu.add(2); // the queue is now [2,3,5,5,2]
        assert_eq!(fu.show_first_unique(), 3);
        fu.add(3); // the queue is now [2,3,5,5,2,3]
        assert_eq!(fu.show_first_unique(), -1);
    }
    #[test]
    fn example2() {
        let mut fu = FirstUnique::new(vec![7, 7, 7, 7, 7, 7]);
        assert_eq!(fu.show_first_unique(), -1);
        fu.add(7); // the queue is now [7,7,7,7,7,7,7]
        fu.add(3); // the queue is now [7,7,7,7,7,7,7,3]
        fu.add(3); // the queue is now [7,7,7,7,7,7,7,3,3]
        fu.add(7); // the queue is now [7,7,7,7,7,7,7,3,3,7]
        fu.add(17); // the queue is now [7,7,7,7,7,7,7,3,3,7,17]
        assert_eq!(fu.show_first_unique(), 17);
    }
    #[test]
    fn example3() {
        let mut fu = FirstUnique::new(vec![809]);
        assert_eq!(fu.show_first_unique(), 809);
        fu.add(809); // the queue is now [809,809]
        assert_eq!(fu.show_first_unique(), -1);
    }

    #[test]
    fn test_50000_nums_50000_add_50000_show_first_unique() {
        let mut fu = FirstUnique::new((0..50000).collect());
        for i in 0..49999 {
            fu.add(i);
            assert_eq!(fu.show_first_unique(), i + 1);
        }
    }
}
