#![allow(dead_code)]
//! \#362. Design Hit Counter
//! =========================
//!
//! Design a hit counter which counts the number of hits received in the past `5` minutes
//! (i.e., the past `300` seconds).
//!
//! Your system should accept a `timestamp` parameter (__in seconds__ granularity),
//! and you may assume that calls are being made to the system in chronological order
//! (i.e., `timestamp` is monotonically increasing).
//! Several hits may arrive roughly at the same time.
//!
//! Implement the `HitCounter` class:
//!
//! - `HitCounter::new()`               Initializes the object of the hit counter system.
//! - `hit(timestamp: i32)`             Records a hit that happened at `timestamp` (__in seconds__).
//!                                     Several hits may happen at the same `timestamp`.
//! - `get_hits(timestamp: i32) -> i32` Returns the number of hits in the past `5` minutes from `timestamp` (i.e., the past `300` seconds).
//!
//! ##### Examples
//!
//! ```
//! # use c2022_09::c2022_09_w2::*;
//! let mut hit_counter = HitCounter::new();
//! hit_counter.hit(1);
//! hit_counter.hit(2);
//! hit_counter.hit(3);
//! assert_eq!(hit_counter.get_hits(4), 3);
//! hit_counter.hit(300);
//! assert_eq!(hit_counter.get_hits(300), 4);
//! assert_eq!(hit_counter.get_hits(301), 3);
//! ```
//!
//! ##### Constraints:
//!
//! - `1 <= timestamp <= 2_000_000_000`
//! - All the calls are being made to the system in chronological order (i.e., `timestamp` is monotonically increasing).
//! - At most `300` calls will be made to `hit` and `get_hits`.
//!
//! ##### Follow up
//!
//! What if the number of hits per second could be huge? Does your design scale?

use std::collections::VecDeque;

#[derive(Default)]
pub struct HitCounter {
    hits_per_timestamp: VecDeque<(i32, i32)>,
    hits_total: i32,
}
impl HitCounter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hit(&mut self, timestamp: i32) {
        self.drop_old(timestamp);

        let same_timestamp = |(old_timestamp, _): &&mut (i32, i32)| old_timestamp == &timestamp;
        if let Some((_, count)) = self.hits_per_timestamp.back_mut().filter(same_timestamp) {
            *count += 1;
        } else {
            self.hits_per_timestamp.push_back((timestamp, 1));
        }

        self.hits_total += 1;
    }

    pub fn get_hits(&mut self, timestamp: i32) -> i32 {
        self.drop_old(timestamp);
        self.hits_total
    }

    fn drop_old(&mut self, timestamp: i32) {
        let drop_until = timestamp - 300;
        while let Some(&(old_timestamp, hits)) = self.hits_per_timestamp.front() {
            if old_timestamp > drop_until {
                break;
            } else {
                self.hits_per_timestamp.pop_front();
                self.hits_total -= hits;
            }
        }
    }
}
