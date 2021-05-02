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
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

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
}
