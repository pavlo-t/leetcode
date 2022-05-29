#![allow(dead_code)]
/// \#1136. Parallel Courses
/// ========================
///
/// You are given an integer `n`, which indicates that there are `n` courses labeled from `1` to `n`.
/// You are also given an array `relations` where `relations[i] = [prevCoursei, nextCoursei]`,
/// representing a prerequisite relationship between course `prevCoursei` and course `nextCoursei`:
/// course `prevCoursei` has to be taken before course `nextCoursei`.
///
/// In one semester, you can take __any number__ of courses as long as you have taken
/// all the prerequisites in the __previous__ semester for the courses you are taking.
///
/// Return _the __minimum__ number of semesters needed to take all courses_.
/// If there is no way to take all the courses, return `-1`.
///
/// __Constraints:__
///
/// - `1 <= n <= 5000`
/// - `1 <= relations.length <= 5000`
/// - `relations[i].length == 2`
/// - `1 <= prevCoursei, nextCoursei <= n`
/// - `prevCoursei != nextCoursei`
/// - All the pairs `[prevCoursei, nextCoursei]` are __unique__.
///
/// https://leetcode.com/problems/parallel-courses
struct Solution;
impl Solution {
    pub fn minimum_semesters(n: i32, relations: Vec<Vec<i32>>) -> i32 {
        use std::collections::{HashMap, HashSet};

        // Map<prev, Vec<next>>
        let mut nexts: HashMap<i32, HashSet<i32>> = HashMap::new();
        // Map<next, Set<prev>>
        let mut prevs: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut can_start = (1..=n).collect::<HashSet<_>>();
        for r in relations {
            let (prev, next) = (r[0], r[1]);
            can_start.remove(&next);
            nexts.entry(prev).or_default().insert(next);
            prevs.entry(next).or_default().insert(prev);
        }

        let mut remaining_courses = (1..=n).collect::<HashSet<_>>();
        let mut semesters = 0;
        let mut can_start_next = HashSet::new();

        while !can_start.is_empty() {
            semesters += 1;
            for curr in &can_start {
                remaining_courses.remove(curr);
                if let Some(nexts) = nexts.get(&curr) {
                    for &next in nexts {
                        let next_reqs = prevs.entry(next).or_default();
                        next_reqs.remove(&curr);
                        if next_reqs.is_empty() {
                            can_start_next.insert(next);
                        }
                    }
                }
            }
            can_start.clear();
            std::mem::swap(&mut can_start, &mut can_start_next);
        }

        if remaining_courses.is_empty() {
            semesters
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn n_3_r_13_23() {
        assert_eq!(Solution::minimum_semesters(3, vv![[1, 3], [2, 3]]), 2);
        // Explanation: The figure above represents the given graph.
        // In the first semester, you can take courses 1 and 2.
        // In the second semester, you can take course 3.
    }
    #[test]
    fn n_3_r_12_23_31() {
        let r = vv![[1, 2], [2, 3], [3, 1]];
        assert_eq!(Solution::minimum_semesters(3, r), -1);
        // Explanation: No course can be studied because they are prerequisites of each other.
    }
    #[test]
    fn n_4_r_12_23_31_41() {
        let r = vv![[1, 2], [2, 3], [3, 1], [4, 1]];
        assert_eq!(Solution::minimum_semesters(4, r), -1);
    }

    #[test]
    fn n_5000_r_1_2_to_r_4999_5000() {
        let r = (1..5000).map(|i| vec![i, i + 1]).collect();
        assert_eq!(Solution::minimum_semesters(5000, r), 5000);
    }
    #[test]
    fn n_5000_empty() {
        let r = vec![];
        assert_eq!(Solution::minimum_semesters(5000, r), 1);
    }
}
