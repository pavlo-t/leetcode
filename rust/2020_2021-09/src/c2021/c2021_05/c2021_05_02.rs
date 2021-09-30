#![allow(dead_code)]
/// Course Schedule III
/// ===================
///
/// There are `n` different online courses numbered from `1` to `n`.
/// You are given an array `courses` where `courses[i] = [duration_i, last_day_i]` indicate that
/// the `i`th course should be taken __continuously__ for `duration_i` days
/// and must be finished before or on `last_day_i`.
///
/// You will start on the `1`st day and you cannot take two or more courses simultaneously.
///
/// Return _the maximum number of courses that you can take_.
///
/// __Constraints:__
///
/// - `1 <= courses.length <= 10_000`
/// - `1 <= duration_i, last_day_i <= 10_000`
///
/// https://leetcode.com/explore/featured/card/may-leetcoding-challenge-2021/598/week-1-may-1st-may-7th/3729/
struct Solution;
impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;

        courses.sort_unstable_by_key(|c| c[1]);
        let mut durations = BinaryHeap::new();
        let mut total_duration = 0;
        for course in courses {
            let (duration, last_day) = (course[0], course[1]);
            if total_duration + duration <= last_day {
                durations.push(duration);
                total_duration += duration;
            } else if let Some(&max_duration) = durations.peek() {
                if max_duration > duration {
                    durations.pop();
                    durations.push(duration);
                    total_duration += duration - max_duration;
                }
            }
        }
        durations.len() as i32
    }

    pub fn schedule_course_iterative_optimized(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_unstable_by_key(|c| c[1]);
        let mut durations = courses.iter().map(|c| c[0]).collect::<Vec<_>>();
        let mut total_duration = 0;
        let mut taken_count = 0usize;
        for course in courses {
            let (duration, last_day) = (course[0], course[1]);
            if total_duration + duration <= last_day {
                total_duration += duration;
                taken_count += 1;
                durations[taken_count.saturating_sub(1)] = duration;
            } else {
                if let Some((j, &max_duration)) = durations
                    .iter()
                    .take(taken_count.saturating_sub(1))
                    .filter(|&&d| d > duration)
                    .enumerate()
                    .max_by_key(|(_, &d)| d)
                {
                    total_duration += duration - max_duration;
                    durations[j] = duration;
                }
            }
        }
        taken_count as i32
    }

    pub fn schedule_course_iterative_quadratic(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_unstable_by_key(|c| c[1]);
        let mut durations = courses.iter().map(|c| c[0]).collect::<Vec<_>>();
        let mut total_duration = 0;
        let mut taken_count = 0;
        for (i, course) in courses.iter().enumerate() {
            let (duration, last_day) = (course[0], course[1]);
            if total_duration + duration <= last_day {
                total_duration += duration;
                taken_count += 1;
            } else {
                if let Some((j, &max_duration)) = durations
                    .iter()
                    .take(i)
                    .filter(|&&d| d > duration)
                    .enumerate()
                    .max_by_key(|(_, &d)| d)
                {
                    total_duration += duration - max_duration;
                    durations[j] = -1;
                } else {
                    durations[i] = -1;
                }
            }
        }
        taken_count
    }

    pub fn schedule_course_rec_memo(mut courses: Vec<Vec<i32>>) -> i32 {
        fn schedule(cs: &[Vec<i32>], i: usize, time: i32, memo: &mut [Vec<Option<i32>>]) -> i32 {
            if i == cs.len() {
                0
            } else if let Some(result) = memo[i][time as usize] {
                result
            } else {
                let taken = if time + cs[i][0] <= cs[i][1] {
                    1 + schedule(cs, i + 1, time + cs[i][0], memo)
                } else {
                    0
                };
                let not_taken = schedule(cs, i + 1, time, memo);
                let result = taken.max(not_taken);
                memo[i][time as usize] = Some(result);
                result
            }
        }

        courses.sort_unstable_by_key(|c| c[1]);
        let max_last_day = courses.last().unwrap()[1] as usize;
        let mut memo = vec![vec![None; max_last_day + 1]; courses.len()];
        schedule(&courses, 0, 0, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {
        ($x:tt; $n:tt) => { vec![vec!$x; $n] };
        ($($x:tt),*) => { vec![$(vec!$x),*] };
    }

    #[test]
    fn test_vv() {
        let v = vv![[1, 2]; 10];
        let e = vec![vec![1, 2]; 10];
        assert_eq!(v, e);
    }

    #[test]
    fn example1() {
        let courses = vv![[100, 200], [200, 1300], [1000, 1250], [2000, 3200]];
        assert_eq!(Solution::schedule_course(courses), 3);
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
    fn example2() {
        let courses = vv![[1, 2]];
        assert_eq!(Solution::schedule_course(courses), 1);
    }
    #[test]
    fn example3() {
        let courses = vv![[3, 2], [4, 3]];
        assert_eq!(Solution::schedule_course(courses), 0);
    }

    #[test]
    fn test_55() {
        let courses = vv![
            [5, 15],
            [3, 19],
            [6, 7],
            [2, 10],
            [5, 16],
            [8, 14],
            [10, 11],
            [2, 19]
        ];
        assert_eq!(Solution::schedule_course(courses), 5);
    }
    #[test]
    fn test_84() {
        let courses = vv![
            [7, 17],
            [3, 12],
            [10, 20],
            [9, 10],
            [5, 20],
            [10, 19],
            [4, 18]
        ];
        assert_eq!(Solution::schedule_course(courses), 4);
    }

    #[test]
    fn c11_11_11_produces_1() {
        let courses = vv![[1, 1], [1, 1], [1, 1]];
        assert_eq!(Solution::schedule_course(courses), 1);
    }

    mod performance {
        use super::*;

        /// If getting stack overflow:
        ///
        /// ```sh
        /// thread 'c2021::c2021_05::c2021_05_02::tests::performance::c10k1_1_produces_1' has overflowed its stack
        /// fatal runtime error: stack overflow
        /// ```
        ///
        /// Add `RUST_MIN_STACK=33554432` to env
        #[test]
        fn c10k1_1_produces_1() {
            let courses = vv![[1, 1]; 10_000];
            assert_eq!(Solution::schedule_course(courses), 1);
        }
        #[test]
        fn c1_10kto1_1_produces_10k() {
            let courses = (1..=10_000).rev().map(|i| vec![1, i]).collect();
            assert_eq!(Solution::schedule_course(courses), 10_000);
        }
    }
}
