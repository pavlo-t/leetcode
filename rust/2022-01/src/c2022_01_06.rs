#![allow(dead_code)]
/// 1094. Car Pooling
/// =================
///
/// There is a car with `capacity` empty seats.
/// The vehicle only drives east (i.e., it cannot turn around and drive west).
///
/// You are given the integer `capacity` and an array `trips` where `trip[i] = [numPassengers_i, from_i, to_i]`
/// indicates that the `i`th trip has `numPassengers_i` passengers
/// and the locations to pick them up and drop them off are `from_i` and `to_i` respectively.
/// The locations are given as the number of kilometers due east from the car's initial location.
///
/// Return `true` _if it is possible to pick up and drop off all passengers for all the given trips,
/// or `false` otherwise_.
///
/// __Constraints:__
///
/// - `1 <= trips.length <= 1000`
/// - `trips[i].length == 3`
/// - `1 <= numPassengers_i <= 100`
/// - `0 <= from_i < to_i <= 1000`
/// - `1 <= capacity <= 100_000`
///
/// https://leetcode.com/problems/car-pooling/
struct Solution;
impl Solution {
    pub fn car_pooling(mut trips: Vec<Vec<i32>>, mut capacity: i32) -> bool {
        println!("car_pooling({:?}, {})", trips, capacity);
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut dropoffs: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        trips.sort_unstable_by_key(|trip| trip[1]);
        for trip in trips {
            let (passengers, from, to) = (trip[0], trip[1], trip[2]);
            while let Some(&Reverse((to, passengers))) = dropoffs.peek() {
                if to <= from {
                    capacity += passengers;
                    dropoffs.pop();
                } else {
                    break;
                }
            }
            if passengers > capacity {
                return false;
            } else {
                capacity -= passengers;
                dropoffs.push(Reverse((to, passengers)));
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn t_2f1t5_3f3t7_c_4() {
        let t = vv![[2, 1, 5], [3, 3, 7]];
        let c = 4;
        assert_eq!(Solution::car_pooling(t, c), false);
    }
    #[test]
    fn t_2f3t5_3f1t3_c_4() {
        let t = vv![[2, 3, 5], [3, 1, 3]];
        let c = 4;
        assert_eq!(Solution::car_pooling(t, c), true);
    }
    #[test]
    fn t_2f3t5_3f1t4_c_4() {
        let t = vv![[2, 3, 5], [3, 1, 4]];
        let c = 4;
        assert_eq!(Solution::car_pooling(t, c), false);
    }
    #[test]
    fn t_2f3t5_3f1t4_c_5() {
        let t = vv![[2, 3, 5], [3, 1, 4]];
        let c = 5;
        assert_eq!(Solution::car_pooling(t, c), true);
    }
    #[test]
    fn t_2f1t5_3f3f7_c_5() {
        let t = vv![[2, 1, 5], [3, 3, 7]];
        let c = 5;
        assert_eq!(Solution::car_pooling(t, c), true);
    }
}
