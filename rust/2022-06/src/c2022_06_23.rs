#![allow(dead_code)]
//! \#630. Course Schedule III
//! ==========================
//!
//! There are `n` different online courses numbered from `1` to `n`.
//! You are given an array `courses` where `courses[i] = [durationi, lastDayi]` indicate that
//! the `ith` course should be taken __continuously__ for durationi days
//! and must be finished before or on `lastDayi`.
//!
//! You will start on the `1st` day and you cannot take two or more courses simultaneously.
//!
//! Return _the maximum number of courses that you can take_.
//!
//! __Constraints:__
//!
//! - `1 <= courses.length <= 10_000`
//! - `1 <= durationi, lastDayi <= 10_000`
//!
//! <https://leetcode.com/problems/course-schedule-iii>

pub struct Solution;
impl Solution {
    pub fn schedule_course_rec(mut courses: Vec<Vec<i32>>) -> i32 {
        fn rec(i: usize, d: i32, c: &[Vec<i32>]) -> i32 {
            if i == c.len() {
                0
            } else {
                let skip = rec(i + 1, d, c);
                if d + c[i][0] <= c[i][1] {
                    let take = 1 + rec(i + 1, d + c[i][0], c);
                    take.max(skip)
                } else {
                    skip
                }
            }
        }

        courses.sort_unstable_by_key(|c| (c[1], c[0]));

        rec(0, 1, &courses)
    }

    pub fn schedule_course_rec_memo(mut courses: Vec<Vec<i32>>) -> i32 {
        fn rec(i: usize, d: i32, c: &[Vec<i32>], memo: &mut HashMap<(usize, i32), i32>) -> i32 {
            if i == c.len() {
                0
            } else {
                if let Some(&result) = memo.get(&(i, d)) {
                    result
                } else {
                    let result = {
                        let skip = rec(i + 1, d, c, memo);
                        if d + c[i][0] <= c[i][1] {
                            let take = 1 + rec(i + 1, d + c[i][0], c, memo);
                            take.max(skip)
                        } else {
                            skip
                        }
                    };
                    memo.insert((i, d), result);
                    result
                }
            }
        }

        use std::collections::HashMap;

        courses.sort_unstable_by_key(|c| (c[1], c[0]));

        let mut memo = HashMap::new();

        rec(0, 1, &courses, &mut memo)
    }

    /// after reading the tips
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;

        courses.sort_unstable_by_key(|c| (c[1], c[0]));

        let mut curr_day = 0;
        let mut taken_durations = BinaryHeap::new();
        for c in courses {
            let (duration, last_day) = (c[0], c[1]);
            if curr_day + duration <= last_day {
                taken_durations.push(duration);
                curr_day += duration;
            } else if let Some(&max_duration) = taken_durations.peek() {
                if max_duration > duration && curr_day - max_duration + duration <= last_day {
                    taken_durations.pop();
                    taken_durations.push(duration);
                    curr_day = curr_day - max_duration + duration;
                }
            }
        }
        taken_durations.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn d100l200_d200l1300_d1000l1250_d2000l3200() {
        let c = vv![[100, 200], [200, 1300], [1000, 1250], [2000, 3200]];
        assert_eq!(Solution::schedule_course(c), 3);
        // Explanation:
        // There are totally 4 courses, but you can take 3 courses at most:
        // First, take the 1st course, it costs 100 days so you will finish it on the 100th day,
        // and ready to take the next course on the 101st day.
        // Second, take the 3rd course, it costs 1000 days so you will finish it on the 1100th day,
        // and ready to take the next course on the 1101st day.
        // Third, take the 2nd course, it costs 200 days so you will finish it on the 1300th day.
        // The 4th course cannot be taken now, since you will finish it on the 3300th day,
        // which exceeds the closed date.
    }
    #[test]
    fn d1l2() {
        let c = vv![[1, 2]];
        assert_eq!(Solution::schedule_course(c), 1);
    }
    #[test]
    fn d3l2_d4l3() {
        let c = vv![[3, 2], [4, 3]];
        assert_eq!(Solution::schedule_course(c), 0);
    }

    #[test]
    fn test_64() {
        let c = vv![[1, 2], [2, 3]];
        assert_eq!(Solution::schedule_course(c), 2);
    }
    #[rustfmt::skip] #[test]
    fn test_85() {
        let c = vv![[7, 17], [3, 12], [10, 20], [9, 10], [5, 20], [10, 19], [4, 18]];
        assert_eq!(Solution::schedule_course(c), 4);
    }

    /// Got stack overflow with default stack
    #[test]
    fn cs_d1l10000_x_10000() {
        let child = std::thread::Builder::new()
            .stack_size(2usize.pow(31))
            .spawn(|| {
                let c = vec![vec![1, 10000]; 10000];
                assert_eq!(Solution::schedule_course(c), 10000);
            })
            .unwrap();
        child.join().unwrap();
    }
}
