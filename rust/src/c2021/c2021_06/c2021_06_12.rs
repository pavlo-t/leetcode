#![allow(dead_code)]
/// Minimum Number of Refueling Stops
/// =================================
///
/// A car travels from a starting position to a destination which is `target` miles east of the starting position.
///
/// Along the way, there are gas stations.
/// Each `station[i]` represents a gas station that is `station[i][0]` miles east of the starting position,
/// and has `station[i][1]` liters of gas.
///
/// The car starts with an infinite tank of gas, which initially has `startFuel` liters of fuel in it.
/// It uses 1 liter of gas per 1 mile that it drives.
///
/// When the car reaches a gas station, it may stop and refuel,
/// transferring all the gas from the station into the car.
///
/// What is the least number of refueling stops the car must make in order to reach its destination?
/// If it cannot reach the destination, return `-1`.
///
/// Note that if the car reaches a gas station with 0 fuel left, the car can still refuel there.
/// If the car reaches the destination with 0 fuel left, it is still considered to have arrived.
///
/// __Note:__
///
/// - `1 <= target, startFuel, stations[i][1] <= 10^9`
/// - `0 <= stations.length <= 500`
/// - `0 < stations[0][0] < stations[1][0] < ... < stations[stations.length-1][0] < target`
///
/// https://leetcode.com/explore/featured/card/june-leetcoding-challenge-2021/604/week-2-june-8th-june-14th/3776/
struct Solution;
impl Solution {
    pub fn min_refuel_stops(target: i32, mut start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::new();
        let mut i = 0;
        let mut result = 0;
        while start_fuel < target {
            while i < stations.len() && start_fuel >= stations[i][0] {
                heap.push(stations[i][1]);
                i += 1;
            }
            if let Some(f) = heap.pop() {
                start_fuel += f;
                result += 1;
            } else {
                return -1;
            }
        }
        result
    }

    /// Greedy using PQ/max-heap; time: O(n log n); space: O(n)
    /// https://spacedleet.vercel.app/solutions/minimum-number-of-refueling-stops/cpp
    pub fn min_refuel_stops_pq(target: i32, mut start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::new();
        let mut i = 0;
        let mut result = 0;
        while start_fuel < target {
            while i < stations.len() && start_fuel >= stations[i][0] {
                heap.push(stations[i][1]);
                i += 1;
            }
            if let Some(f) = heap.pop() {
                start_fuel += f;
                result += 1;
            } else {
                break;
            }
        }
        if start_fuel >= target {
            result
        } else {
            -1
        }
    }

    pub fn min_refuel_stops_bfs(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        use std::mem::swap;

        let mut q1: Vec<(i32, i32, &[Vec<i32>])> = vec![(0, start_fuel, &stations)];
        let mut q2 = vec![];

        let mut stops = 0;

        while !q1.is_empty() {
            while let Some((m, f, ss)) = q1.pop() {
                let max_reach = m + f;
                if max_reach >= target {
                    return stops;
                }
                ss.iter()
                    .enumerate()
                    .take_while(|&(_, s)| s[0] <= max_reach)
                    .for_each(|(i, s)| {
                        let nm = s[0];
                        let nf = max_reach - s[0] + s[1];
                        q2.push((nm, nf, &ss[i + 1..]));
                    })
            }
            stops += 1;
            swap(&mut q1, &mut q2);
        }

        -1
    }

    pub fn min_refuel_stops_dfs(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        fn dfs(k: i32, f: i32, ss: &[Vec<i32>], t: i32) -> Option<i32> {
            let max_reach = k + f;
            if max_reach >= t {
                Some(0)
            } else {
                ss.iter()
                    .enumerate()
                    .find(|&(_, s)| s[0] > k && s[0] <= max_reach)
                    .and_then(|(i, _)| {
                        (i..ss.len())
                            .take_while(|&i| ss[i][0] <= max_reach)
                            .map(|i| {
                                let nk = ss[i][0];
                                let nf = max_reach - ss[i][0] + ss[i][1];
                                dfs(nk, nf, &ss[i + 1..], t).map(|i| i + 1)
                            })
                            .filter_map(|o| o)
                            .min()
                    })
            }
        }

        dfs(0, start_fuel, &stations, target).unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn t_1_sf_1_ss_empty_produces_0() {
        let target = 1;
        let start_fuel = 1;
        let stations = vec![];
        assert_eq!(Solution::min_refuel_stops(target, start_fuel, stations), 0);
        // Explanation: We can reach the target without refueling.
    }
    #[test]
    fn t_100_sf_100_ss_10f90_50f90_70f90_produces_0() {
        let target = 100;
        let start_fuel = 100;
        let stations = vv![[10, 90], [50, 90], [70, 90]];
        assert_eq!(Solution::min_refuel_stops(target, start_fuel, stations), 0);
    }
    #[test]
    fn t_100_sf_1_ss_10f100_produces_m1() {
        let target = 100;
        let start_fuel = 1;
        let stations = vv![[10, 100]];
        assert_eq!(Solution::min_refuel_stops(target, start_fuel, stations), -1);
        // Explanation: We can't reach the target (or even the first gas station).
    }
    #[test]
    fn t_100_sf_10_ss_10f60_20f30_30f30_60f40_produces_2() {
        let target = 100;
        let start_fuel = 10;
        let stations = vv![[10, 60], [20, 30], [30, 30], [60, 40]];
        assert_eq!(Solution::min_refuel_stops(target, start_fuel, stations), 2);
        // Explanation:
        // We start with 10 liters of fuel.
        // We drive to position 10, expending 10 liters of fuel.  We refuel from 0 liters to 60 liters of gas.
        // Then, we drive from position 10 to position 60 (expending 50 liters of fuel),
        // and refuel from 10 liters to 50 liters of gas.  We then drive to and reach the target.
        // We made 2 refueling stops along the way, so we return 2.
    }

    #[test]
    fn t_500_sf_1_ss_1to500f1_produces_499() {
        let t = 500;
        let sf = 1;
        let ss = (1..=500).map(|i| vec![i, 1]).collect();
        assert_eq!(Solution::min_refuel_stops(t, sf, ss), 499);
    }

    #[test]
    fn t_500_sf_100_ss_1to500f100_produces_4() {
        let t = 500;
        let sf = 100;
        let ss = (1..=500).map(|i| vec![i, 100]).collect();
        assert_eq!(Solution::min_refuel_stops(t, sf, ss), 4);
    }
}
