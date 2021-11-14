#![allow(dead_code)]
/// 739. Daily Temperatures
/// =======================
///
/// Given an array of integers `temperatures` represents the daily temperatures,
/// return _an array `answer` such that `answer[i]` is the number of days you have to wait
/// after the `i`th day to get a warmer temperature_.
/// If there is no future day for which this is possible, keep `answer[i] == 0` instead.
///
/// __Constraints:__
///
/// - `1 <= temperatures.length <= 100_000`
/// - `30 <= temperatures[i] <= 100`
///
/// https://leetcode.com/problems/daily-temperatures/
struct Solution;
impl Solution {
    pub fn daily_temperatures_my(temperatures: Vec<i32>) -> Vec<i32> {
        println!("daily_temperatures({:?})", temperatures);
        let mut tds = [usize::MAX; 101];
        let mut result = vec![0; temperatures.len()];
        for d in (0..temperatures.len()).rev() {
            let ti = temperatures[d] as usize;
            let closest_day = (ti + 1..tds.len())
                .map(|ti| tds[ti])
                .min()
                .unwrap_or(usize::MAX);
            if closest_day < usize::MAX {
                result[d] = (closest_day - d) as i32;
            }
            tds[ti] = d;
        }
        result
    }
    /// Approach 1: Monotonic Stack
    /// https://leetcode.com/problems/daily-temperatures/solution/
    pub fn daily_temperatures_leetcode_monotonic_stack(temperatures: Vec<i32>) -> Vec<i32> {
        println!("daily_temperatures({:?})", temperatures);
        let n = temperatures.len();
        let mut result = vec![0; n];
        let mut stack: Vec<(usize, i32)> = vec![];
        for (d, &t) in temperatures.iter().enumerate() {
            while let Some(p @ (pd, pt)) = stack.pop() {
                if pt < t {
                    result[pd] = (d - pd) as i32;
                } else {
                    stack.push(p);
                    break;
                }
            }
            stack.push((d, t));
        }
        result
    }
    /// Approach 2: Array, Optimized Space
    /// https://leetcode.com/problems/daily-temperatures/solution/
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        println!("daily_temperatures({:?})", temperatures);
        let n = temperatures.len();
        let mut hottest = 0;
        let mut result = vec![0; n];
        for (d, &t) in temperatures.iter().enumerate().rev() {
            if t >= hottest {
                hottest = t;
            } else {
                let mut ds = 1;
                while temperatures[d + ds] <= t {
                    ds += result[d + ds] as usize;
                }
                result[d] = ds as i32;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_73_74_75_71_69_72_76_73() {
        let t = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let e = [1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(Solution::daily_temperatures(t), e);
    }
    #[test]
    fn t_30_40_50_60() {
        let t = vec![30, 40, 50, 60];
        let e = [1, 1, 1, 0];
        assert_eq!(Solution::daily_temperatures(t), e);
    }
    #[test]
    fn t_30_60_90() {
        let t = vec![30, 60, 90];
        let e = [1, 1, 0];
        assert_eq!(Solution::daily_temperatures(t), e);
    }

    #[test]
    fn t_89_62_70_58_47_47_46_76_100_70() {
        let t = vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70];
        let e = [8, 1, 5, 4, 3, 2, 1, 1, 0, 0];
        assert_eq!(Solution::daily_temperatures(t), e);
    }

    #[test]
    fn t_90_repeat_100000() {
        let t = vec![90; 100000];
        let e = [0; 100000];
        assert_eq!(Solution::daily_temperatures(t), e);
    }
    #[test]
    fn t_90_repeat_99999_concat_91() {
        use std::iter;
        let t = iter::repeat(90).take(99999).chain(iter::once(91)).collect();
        let e = (0..100000).rev().collect::<Vec<_>>();
        assert_eq!(Solution::daily_temperatures(t), e);
    }
}
