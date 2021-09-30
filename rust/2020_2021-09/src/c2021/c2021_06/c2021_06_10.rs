#![allow(dead_code)]

use std::cell::RefCell;

/// My Calendar I
/// =============
///
/// You are implementing a program to use as your calendar.
/// We can add a new event if adding the event will not cause a __double booking__.
///
/// A __double booking__ happens when two events have some non-empty intersection
/// (i.e., some moment is common to both events.).
///
/// The event can be represented as a pair of integers `start` and `end`
/// that represents a booking on the half-open interval `[start, end)`,
/// the range of real numbers `x` such that `start <= x < end`.
///
/// Implement the `MyCalendar` class:
///
/// - `MyCalendar()` Initializes the calendar object.
/// - `boolean book(int start, int end)` Returns `true` if the event can be added to the calendar
///   successfully without causing a __double booking__.
///   Otherwise, return `false` and do not add the event to the calendar.
///
/// __Constraints:__
///
/// - `0 <= start < end <= 1_000_000_000`
/// - At most `1000` calls will be made to `book`.
///
/// https://leetcode.com/explore/challenge/card/june-leetcoding-challenge-2021/604/week-2-june-8th-june-14th/3774/
struct MyCalendar {
    events: RefCell<Vec<(i32, i32)>>,
}
impl MyCalendar {
    fn new() -> Self {
        let events = RefCell::new(Vec::new());
        Self { events }
    }

    fn book(&self, start: i32, end: i32) -> bool {
        let mut es = self.events.borrow_mut();
        for &(s, e) in es.iter() {
            if !(end <= s || start >= e) {
                return false;
            }
        }
        es.push((start, end));

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mc = MyCalendar::new();
        assert!(mc.book(10, 20));
        assert!(!mc.book(15, 25));
        assert!(mc.book(20, 30));
        // Explanation
        // MyCalendar myCalendar = new MyCalendar();
        // myCalendar.book(10, 20); // return True
        // myCalendar.book(15, 25); // return False,
        // // It can not be booked because time 15 is already booked by another event.
        // myCalendar.book(20, 30); // return True,
        // // The event can be booked, as the first event takes every time less than 20, but not
        // // including 20.
    }

    mod performance {
        use super::*;

        #[test]
        fn test_1k_calls_to_book() {
            let mc = MyCalendar::new();
            for i in 0..1000 {
                let s = i * 10;
                let e = s + 10;
                assert!(mc.book(s, e));
            }
        }
    }
}
