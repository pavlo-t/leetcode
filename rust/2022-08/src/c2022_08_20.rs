#![allow(dead_code)]
//! \#871. Minimum Number of Refueling Stops
//! ========================================
//!
//! <https://leetcode.com/problems/minimum-number-of-refueling-stops>
//!
//! A car travels from a starting position to a destination which is `target` miles east of the starting position.
//!
//! There are gas stations along the way.
//! The gas stations are represented as an array `stations` where `stations[i] = [position_i, fuel_i]`
//! indicates that the `i`th gas station is `position_i` miles east of the starting position
//! and has `fuel_i` liters of gas.
//!
//! The car starts with an infinite tank of gas, which initially has `start_fuel` liters of fuel in it.
//! It uses one liter of gas per one mile that it drives.
//! When the car reaches a gas station, it may stop and refuel,
//! transferring all the gas from the station into the car.
//!
//! Return _the minimum number of refueling stops the car must make in order to reach its destination_.
//! If it cannot reach the destination, return `-1`.
//!
//! Note that if the car reaches a gas station with `0` fuel left, the car can still refuel there.
//! If the car reaches the destination with `0` fuel left, it is still considered to have arrived.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_08::c2022_08_20::*;
//! assert_eq!(Solution::min_refuel_stops(1, 1, vec![]), 0);
//! ```
//!
//! __Explanation:__ We can reach the target without refueling.
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_20::*;
//! # use c2022_08::vv;
//! assert_eq!(Solution::min_refuel_stops(100, 1, vv![[10, 100]]), -1);
//! ```
//!
//! __Explanation:__ We can not reach the target (or even the first gas station).
//!
//! ###### Example 3
//!
//! ```
//! # use c2022_08::c2022_08_20::*;
//! # use c2022_08::vv;
//! let stations = vv![[10, 60], [20, 30], [30, 30], [60, 40]];
//! assert_eq!(Solution::min_refuel_stops(100, 10, stations), 2);
//! ```
//!
//! __Explanation:__
//!
//! - We start with `10` liters of fuel.
//! - We drive to position `10`, expending `10` liters of fuel.
//! - We refuel from `0` liters to `60` liters of gas.
//! - Then, we drive from position `10` to position `60` (expending `50` liters of fuel),
//!   and refuel from `10` liters to `50` liters of gas.
//! - We then drive to and reach the target.
//! - We made `2` refueling stops along the way, so we return `2`.
//!
//! ##### Constraints
//!
//! - `1 <= target, start_fuel <= 1_000_000_000`
//! - `0 <= stations.length <= 500`
//! - `0 <= position_i <= position_i+1 < target`
//! - `1 <= fuel_i < 1_000_000_000`

pub struct Solution;
impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        use std::iter::once;

        let mut refueled = 0;
        let mut position = 0;
        let mut curr_fuel = start_fuel;
        let mut passed_stations = BinaryHeap::new();

        for station in stations.iter().chain(once(&vec![target, 0])) {
            let (station_position, station_fuel) = (station[0], station[1]);

            curr_fuel -= station_position - position;
            position = station_position;

            while curr_fuel < 0 {
                if let Some(station_fuel) = passed_stations.pop() {
                    refueled += 1;
                    curr_fuel += station_fuel;
                } else {
                    return -1;
                }
            }

            if position + curr_fuel >= target {
                return refueled;
            }

            passed_stations.push(station_fuel);
        }

        if curr_fuel >= 0 {
            refueled
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vv;

    #[test]
    fn m_1_1_empty() {
        assert_eq!(Solution::min_refuel_stops(1, 1, vec![]), 0);
    }

    #[test]
    fn m_100_1_10_100() {
        assert_eq!(Solution::min_refuel_stops(100, 1, vv![[10, 100]]), -1);
    }

    #[test]
    fn m_100_10_10_100() {
        assert_eq!(Solution::min_refuel_stops(100, 10, vv![[10, 100]]), 1);
    }

    #[test]
    fn m_100_10_10_60_20_30_30_30_60_40() {
        let stations = vv![[10, 60], [20, 30], [30, 30], [60, 40]];
        assert_eq!(Solution::min_refuel_stops(100, 10, stations), 2);
    }

    #[test]
    fn t_max_sf_500_stations_1_max_to_500_max() {
        const MAX: i32 = 1_000_000_000;
        let stations = (1..=500).map(|p| vec![p, MAX]).collect();
        assert_eq!(Solution::min_refuel_stops(MAX, 500, stations), 1);
    }
}
