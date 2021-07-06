#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;

/// Design Underground System
/// =========================
///
/// Implement the `UndergroundSystem` class:
///
/// - `void checkIn(int id, string stationName, int t)`
///   - A customer with a card id equal to `id`, gets in the station `stationName` at time `t`.
///   - A customer can only be checked into one place at a time.
/// - `void checkOut(int id, string stationName, int t)`
///   - A customer with a card id equal to `id`, gets out from the station `stationName` at time `t`.
/// - `double getAverageTime(string startStation, string endStation)`
///   - Returns the average time to travel between the `startStation` and the `endStation`.
///   - The average time is computed from all the previous traveling from `startStation` to
///     `endStation` that happened __directly__.
///   - Call to `getAverageTime` is always valid.
///
/// You can assume all calls to `checkIn` and `checkOut` methods are consistent.
/// If a customer gets in at time `t1` at some station, they get out at time `t2` with `t2 > t1`.
/// All events happen in chronological order.
///
/// __Constraints:__
///
/// - There will be at most `20000` operations.
/// - `1 <= id, t <= 1_000_000`
/// - All strings consist of uppercase and lowercase English letters, and digits.
/// - `1 <= stationName.length <= 10`
/// - Answers within `0.00001` of the actual value will be accepted as correct.
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/590/week-3-march-15th-march-21st/3678/
struct UndergroundSystem {
    ongoing_trips: RefCell<HashMap<i32, (String, i32)>>,
    averages: RefCell<HashMap<(String, String), (f64, f64)>>,
}
impl UndergroundSystem {
    fn new() -> Self {
        Self {
            ongoing_trips: RefCell::new(HashMap::new()),
            averages: RefCell::new(HashMap::new()),
        }
    }

    fn check_in(&self, id: i32, station_name: String, t: i32) {
        self.ongoing_trips
            .borrow_mut()
            .insert(id, (station_name, t));
    }

    fn check_out(&self, id: i32, station_name: String, t: i32) {
        let (start_station, st) = self
            .ongoing_trips
            .borrow_mut()
            .remove(&id)
            .unwrap_or_else(|| panic!("customer did not check in"));
        let mut averages = self.averages.borrow_mut();
        let (total_time, count) = averages.entry((start_station, station_name)).or_default();
        *total_time += (t - st) as f64;
        *count += 1f64;
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let &(total, count) = self
            .averages
            .borrow()
            .get(&(start_station, end_station))
            .unwrap_or_else(|| panic!("not found"));
        total / count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_feq(a: f64, b: f64) {
        const ERROR_MARGIN: f64 = 0.00001;
        assert!((a - b).abs() < ERROR_MARGIN, "{} != {}", a, b);
    }

    #[test]
    fn example1() {
        let u = UndergroundSystem::new();
        u.check_in(45, "Leyton".to_string(), 3);
        u.check_in(32, "Paradise".to_string(), 8);
        u.check_in(27, "Leyton".to_string(), 10);
        u.check_out(45, "Waterloo".to_string(), 15);
        u.check_out(27, "Waterloo".to_string(), 20);
        u.check_out(32, "Cambridge".to_string(), 22);
        check_feq(
            u.get_average_time("Paradise".to_string(), "Cambridge".to_string()),
            14.0,
        );
        // There was only one travel from "Paradise" (at time 8) to "Cambridge" (at time 22)

        check_feq(
            u.get_average_time("Leyton".to_string(), "Waterloo".to_string()),
            11.0,
        );
        // There were two travels from "Leyton" to "Waterloo",
        // a customer with id=45 from time=3 to time=15 and
        // a customer with id=27 from time=10 to time=20.
        // So the average time is ( (15-3) + (20-10) ) / 2 = 11.00000

        u.check_in(10, "Leyton".to_string(), 24);
        check_feq(
            u.get_average_time("Leyton".to_string(), "Waterloo".to_string()),
            11.0,
        );
        u.check_out(10, "Waterloo".to_string(), 38);
        check_feq(
            u.get_average_time("Leyton".to_string(), "Waterloo".to_string()),
            12.0,
        );
    }
    #[test]
    fn example2() {
        let u = UndergroundSystem::new();
        u.check_in(10, "Leyton".to_string(), 3);
        u.check_out(10, "Paradise".to_string(), 8);
        check_feq(
            u.get_average_time("Leyton".to_string(), "Paradise".to_string()),
            5.0,
        );
        u.check_in(5, "Leyton".to_string(), 10);
        u.check_out(5, "Paradise".to_string(), 16);
        check_feq(
            u.get_average_time("Leyton".to_string(), "Paradise".to_string()),
            5.5,
        );
        u.check_in(2, "Leyton".to_string(), 21);
        u.check_out(2, "Paradise".to_string(), 30);
        check_feq(
            u.get_average_time("Leyton".to_string(), "Paradise".to_string()),
            6.66667,
        );
    }
}
