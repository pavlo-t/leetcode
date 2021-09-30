#![allow(dead_code, unused_variables)]
/// # Parallel Courses
///
/// You are given an integer `n` which indicates that we have `n` courses, labeled from `1` to `n`.
/// You are also given an array `relations` where `relations[i] = [a, b]`,
/// representing a prerequisite relationship between course `a` and course `b`:
/// course `a` has to be studied before course `b`.
///
/// In one semester, you can study any number of courses as long as you have studied all the
/// prerequisites for the course you are studying.
///
/// Return _the minimum number of semesters needed to study all courses_.
/// If there is no way to study all the courses, return `-1`.
///
/// __Constraints:__
///
/// - `1 <= n <= 5000`
/// - `1 <= relations.length <= 5000`
/// - `1 <= a, b <= n`
/// - `a != b`
/// - All the pairs `[a, b]` are __unique__.
///
/// https://leetcode.com/explore/challenge/card/march-leetcoding-challenge-2021/592/week-5-march-29th-march-31st/3688/
struct Solution;
//noinspection DuplicatedCode
impl Solution {
    pub fn minimum_semesters(n: i32, relations: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        use std::collections::HashSet;

        let mut rs = vec![vec![]; (n + 1) as usize];
        for r in relations {
            rs[r[1] as usize].push(r[0]);
        }

        fn get_semesters(
            c: i32,
            rs: &[Vec<i32>],
            seen: &mut HashSet<i32>,
            cache: &mut HashMap<i32, i32>,
        ) -> i32 {
            if seen.contains(&c) {
                10_001
            } else if let Some(&s) = cache.get(&c) {
                s
            } else {
                seen.insert(c);
                let s = 1 + rs[c as usize]
                    .iter()
                    .map(|&c0| get_semesters(c0, rs, seen, cache))
                    .max()
                    .unwrap_or(0);
                cache.insert(c, s);
                s
            }
        }

        let mut cache = HashMap::new();
        match (1..=n)
            .map(|c| get_semesters(c, &rs, &mut HashSet::new(), &mut cache))
            .max()
            .unwrap_or(0)
        {
            s if s > 10_000 => -1,
            s => s,
        }
    }

    pub fn minimum_semesters_brute_force_recursion(n: i32, relations: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;

        fn get_semesters(c: i32, relations: &[Vec<i32>], seen: &mut HashSet<i32>) -> i32 {
            if seen.contains(&c) {
                10_001
            } else {
                seen.insert(c);
                relations
                    .iter()
                    .filter_map(|r| {
                        if r[1] == c {
                            Some(1 + get_semesters(r[0], relations, seen))
                        } else {
                            None
                        }
                    })
                    .max()
                    .unwrap_or(1)
            }
        }

        match (1..=n)
            .map(|c| get_semesters(c, &relations, &mut HashSet::new()))
            .max()
            .unwrap_or(0)
        {
            s if s > 10_000 => -1,
            s => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let relations = vec![vec![1, 3], vec![2, 3]];
        assert_eq!(Solution::minimum_semesters(3, relations), 2);
        // Output: 2
        // Explanation:
        // In the first semester, courses 1 and 2 are studied.
        // In the second semester, course 3 is studied.
    }
    #[test]
    fn example2() {
        let relations = vec![vec![1, 2], vec![2, 3], vec![3, 1]];
        assert_eq!(Solution::minimum_semesters(3, relations), -1);
        // Explanation: No course can be studied because they depend on each other.
    }

    #[test]
    fn test_3_courses_3_semesters() {
        let relations = vec![vec![1, 2], vec![2, 3]];
        assert_eq!(Solution::minimum_semesters(3, relations), 3);
    }
    #[test]
    fn test_5_courses_3_semesters() {
        let relations = vec![vec![1, 2], vec![2, 3], vec![4, 5], vec![5, 3]];
        assert_eq!(Solution::minimum_semesters(5, relations), 3);
    }

    mod performance {
        use super::*;

        #[test]
        fn test_5000_courses_5000_semesters() {
            let relations = (1..5000).map(|i| vec![i, i + 1]).collect();
            assert_eq!(Solution::minimum_semesters(5000, relations), 5000);
        }
    }
}
