#![allow(dead_code)]
/// 1010. Pairs of Songs With Total Durations Divisible by 60
/// =========================================================
///
/// You are given a list of songs where the `i`th song has a duration of `time[i]` seconds.
///
/// Return _the number of pairs of songs for which their total duration in seconds is divisible by `60`_.
/// Formally, we want the number of indices `i`, `j` such that `i < j` with `(time[i] + time[j]) % 60 == 0`.
///
/// __Constraints:__
///
/// - `1 <= time.length <= 60_000`
/// - `1 <= time[i] <= 500`
///
/// https://leetcode.com/problems/pairs-of-songs-with-total-durations-divisible-by-60/
struct Solution;
impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        fn combinations(mut n: i32) -> i32 {
            if n < 2 {
                0
            } else {
                let mut result = 0;
                while n > 1 {
                    result += n - 1;
                    n -= 1;
                }
                result
            }
        }
        let mut remainders = [0; 60];
        for t in time {
            remainders[(t % 60) as usize] += 1;
        }
        let mut result = 0;
        for i in 1..30 {
            result += remainders[i] * remainders[60 - i];
        }
        result += combinations(remainders[30]);
        result += combinations(remainders[0]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn t_1() { assert_eq!(Solution::num_pairs_divisible_by60(vec![1]), 0); }
    #[rustfmt::skip] #[test] fn t_30() { assert_eq!(Solution::num_pairs_divisible_by60(vec![30]), 0); }
    #[rustfmt::skip] #[test] fn t_60() { assert_eq!(Solution::num_pairs_divisible_by60(vec![60]), 0); }

    #[rustfmt::skip] #[test] fn t_30_30() { assert_eq!(Solution::num_pairs_divisible_by60(vec![30, 30]), 1); }
    #[rustfmt::skip] #[test] fn t_30_30_30() { assert_eq!(Solution::num_pairs_divisible_by60(vec![30,30,30]), 3); }
    #[rustfmt::skip] #[test] fn t_30_30_30_30() { assert_eq!(Solution::num_pairs_divisible_by60(vec![30,30,30,30]), 6); }

    /// 60 60 60 60 60
    /// 0   1  3  6 10
    ///              4+6
    ///           3+3
    ///        2+1
    /// f(n) = n-1 + f(n-1)
    #[rustfmt::skip] #[test] fn t_60_repeat_2() { assert_eq!(Solution::num_pairs_divisible_by60(vec![60; 2]), 1); }
    /// Explanation: All three pairs have a total duration of 120, which is divisible by 60.
    #[rustfmt::skip] #[test] fn t_60_repeat_3() { assert_eq!(Solution::num_pairs_divisible_by60(vec![60; 3]), 3); }
    #[rustfmt::skip] #[test] fn t_60_repeat_4() { assert_eq!(Solution::num_pairs_divisible_by60(vec![60; 4]), 6); }
    #[rustfmt::skip] #[test] fn t_60_repeat_5() { assert_eq!(Solution::num_pairs_divisible_by60(vec![60; 5]), 10); }

    #[rustfmt::skip] #[test] fn t_1_59()  { assert_eq!(Solution::num_pairs_divisible_by60(vec![1, 59]), 1); }
    #[rustfmt::skip] #[test] fn t_1_1_59_59()  { assert_eq!(Solution::num_pairs_divisible_by60(vec![1,1,59,59]), 4); }
    #[rustfmt::skip] #[test] fn t_1_1_1_59_59()  { assert_eq!(Solution::num_pairs_divisible_by60(vec![1,1,1,59,59]), 6); }
    #[rustfmt::skip] #[test] fn t_1_1_1_1_59_59()  { assert_eq!(Solution::num_pairs_divisible_by60(vec![1,1,1,1,59,59]), 8); }

    #[test]
    fn t_30_20_150_100_40() {
        let t = vec![30, 20, 150, 100, 40];
        assert_eq!(Solution::num_pairs_divisible_by60(t), 3);
        // Output: 3
        // Explanation: Three pairs have a total duration divisible by 60:
        // (time[0] = 30, time[2] = 150): total duration 180
        // (time[1] = 20, time[3] = 100): total duration 120
        // (time[1] = 20, time[4] = 40): total duration 60
    }
}
