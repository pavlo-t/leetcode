#![allow(dead_code)]
/// Range Addition
/// ==============
///
/// You are given an integer `length` and an array `updates` where `updates[i] = [start_idx_i, end_idx_i, inc_i]`.
///
/// You have an array `arr` of length `length` with all zeros, and you have some operation to apply on `arr`.
/// In the `i`th operation, you should increment all the elements
/// `arr[start_idx_i], arr[start_idx_i + 1], ..., arr[end_idx_i]` by `inc_i`.
///
/// Return `arr` _after applying all the_ `updates`.
///
/// __Constraints:__
///
/// - `1 <= length <= 100_000`
/// - `0 <= updates.length <= 10_000`
/// - `0 <= start_idx_i <= end_idx_i < length`
/// - `-1000 <= inci <= 1000`
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/610/week-3-july-15th-july-21st/3814/
struct Solution;
impl Solution {
    pub fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
        let length = length as usize;
        let mut result = vec![0; length];
        for u in updates {
            let (l, r, d) = (u[0] as usize, u[1] as usize, u[2]);
            result[l] += d;
            if r + 1 < length {
                result[r + 1] -= d;
            }
        }
        let mut sum = 0;
        for i in 0..length {
            sum += result[i];
            result[i] = sum;
        }
        result
    }

    pub fn get_modified_array_brute_force(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![0; length as usize];
        for u in updates {
            let (l, r, d) = (u[0] as usize, u[1] as usize, u[2]);
            for i in l..=r {
                result[i] += d;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => {vec![$(vec!$x),*]}; }

    #[test]
    fn l_5_u_1p3p2_2p4p3_0p2m2_produces_m2_0_3_5_3() {
        let updates = vv![[1, 3, 2], [2, 4, 3], [0, 2, -2]];
        let e = vec![-2, 0, 3, 5, 3];
        assert_eq!(Solution::get_modified_array(5, updates), e);
    }
    #[test]
    fn l_10_u_2p4p6_5p6p8_1p9m4_produces_0_m4_2_2_2_4_4_m4_m4_m4() {
        let updates = vv![[2, 4, 6], [5, 6, 8], [1, 9, -4]];
        let e = vec![0, -4, 2, 2, 2, 4, 4, -4, -4, -4];
        assert_eq!(Solution::get_modified_array(10, updates), e);
    }

    mod performance {
        use super::*;

        #[test]
        fn l_100_000_u_0p99_999p1_x_10_000_produces_10_000x100_000() {
            let updates = vec![vec![0, 99999, 1]; 10_000];
            let e = vec![10_000; 100_000];
            assert_eq!(Solution::get_modified_array(100_000, updates), e);
        }
    }
}
