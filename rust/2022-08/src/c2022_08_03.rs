#![allow(dead_code)]
//! \#729. My Calendar I
//! ====================
//!
//! <https://leetcode.com/problems/my-calendar-i>
//!
//! You are implementing a program to use as your calendar.
//! We can add a new event if adding the event will not cause a __double booking__.
//!
//! A __double booking__ happens when two events have some non-empty intersection
//! (i.e., some moment is common to both events.).
//!
//! The event can be represented as a pair of integers `start` and `end`
//! that represents a booking on the half-open interval `[start, end)`,
//! the range of real numbers `x` such that `start <= x < end`.
//!
//! Implement the `MyCalendar` class:
//!
//! - `MyCalendar()` Initializes the calendar object.
//! - `boolean book(int start, int end)`
//!   Returns `true` if the event can be added to the calendar successfully without causing a __double booking__.
//!   Otherwise, return `false` and do not add the event to the calendar.
//!
//! ##### Constraints
//!
//! - `0 <= start < end <= 1_000_000_000`
//! - At most `1000` calls will be made to book.
//!
//! ##### Examples
//!
//! ```
//! # use c2022_08::c2022_08_03::v2::MyCalendar;
//! let my_calendar = MyCalendar::new();
//! assert_eq!(my_calendar.book(10, 20), true);
//! // It can not be booked because time 15 is already booked by another event.
//! assert_eq!(my_calendar.book(15, 25), false);
//! // The event can be booked, as the first event takes every time less than 20, but not including 20.
//! assert_eq!(my_calendar.book(20, 30), true);
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_03::v2::MyCalendar;
//! let my_calendar = MyCalendar::new();
//! assert_eq!(my_calendar.book(10, 20), true);
//! assert_eq!(my_calendar.book(5, 11), false);
//! assert_eq!(my_calendar.book(19, 25), false);
//! assert_eq!(my_calendar.book(11, 15), false);
//! assert_eq!(my_calendar.book(0, 5), true);
//! assert_eq!(my_calendar.book(4, 6), false);
//! assert_eq!(my_calendar.book(9, 11), false);
//! assert_eq!(my_calendar.book(9, 11), false);
//! assert_eq!(my_calendar.book(5, 10), true);
//! assert_eq!(my_calendar.book(25, 30), true);
//! assert_eq!(my_calendar.book(20, 25), true);
//! ```

/// Using `BTreeMap`
///
/// - time: `O(n log n)`
/// - memory: `O(n)`
pub mod v3 {
    use std::cell::RefCell;
    use std::collections::BTreeMap;

    #[derive(Default)]
    pub struct MyCalendar {
        booked: RefCell<BTreeMap<i32, i32>>,
    }
    impl MyCalendar {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn book(&self, start: i32, end: i32) -> bool {
            !self.overlaps(start, end) && {
                self.booked.borrow_mut().insert(start, end);
                true
            }
        }

        fn overlaps(&self, start: i32, end: i32) -> bool {
            self.booked
                .borrow()
                .range(..end)
                .rev()
                .next()
                .filter(|(_, &prev_end)| prev_end > start)
                .is_some()
        }
    }
}

/// Using `Vec`'s `binary_search_by`
///
/// - time: `O(n²)` (`O(n log n)` if cannot book)
/// - memory: `O(n)`
pub mod v2 {
    use std::cell::RefCell;
    use std::cmp::Ordering;

    #[derive(Default)]
    pub struct MyCalendar {
        booked: RefCell<Vec<(i32, i32)>>,
    }
    impl MyCalendar {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn book(&self, start: i32, end: i32) -> bool {
            let overlaps = |&(start_a, end_a): &(i32, i32)| {
                if start >= end_a {
                    Ordering::Less
                } else if end <= start_a {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            };
            let mut booked = self.booked.borrow_mut();

            match booked.binary_search_by(overlaps) {
                Ok(_) => false,
                Err(idx) => {
                    booked.insert(idx, (start, end));
                    true
                }
            }
        }
    }
}

/// Brute force
///
/// - time: `O(n²)`
/// - memory: `O(n)`
pub mod v1 {
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct MyCalendar {
        booked: RefCell<Vec<(i32, i32)>>,
    }
    impl MyCalendar {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn book(&self, start: i32, end: i32) -> bool {
            let overlaps = |&(start_a, end_a): &(i32, i32)| start < end_a && end > start_a;
            let mut booked = self.booked.borrow_mut();

            !booked.iter().any(overlaps) && {
                booked.push((start, end));
                true
            }
        }
    }
}
