#![allow(dead_code)]
/// 210. Course Schedule II
/// =======================
///
/// There are a total of `numCourses` courses you have to take, labeled from `0` to `numCourses - 1`.
/// You are given an array `prerequisites` where `prerequisites[i] = [ai, bi]` indicates
/// that you __must__ take course `bi` first if you want to take course `ai`.
///
/// For example, the pair `[0, 1]`, indicates that to take course `0` you have to first take course `1`.
///
/// Return _the ordering of courses you should take to finish all courses_.
/// If there are many valid answers, return __any__ of them.
/// If it is impossible to finish all courses, return __an empty array__.
///
/// __Constraints:__
///
/// - `1 <= numCourses <= 2000`
/// - `0 <= prerequisites.length <= numCourses * (numCourses - 1)`
/// - `prerequisites[i].length == 2`
/// - `0 <= ai, bi < numCourses`
/// - `ai != bi`
/// - All the pairs `[ai, bi]` are __distinct__.
///
/// https://leetcode.com/problems/course-schedule-ii/
struct Solution;
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        //println!("find_order({}, {:?})", num_courses, prerequisites);
        use std::collections::HashSet;

        let n = num_courses as usize;
        let mut rs = vec![HashSet::new(); n];
        for p in &prerequisites {
            let (a, b) = (p[0] as usize, p[1] as usize);
            rs[a].insert(b);
        }

        let mut seen = vec![false; n];
        let mut stack = rs
            .iter()
            .enumerate()
            .filter(|(_, reqs)| reqs.is_empty())
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        let mut result = Vec::with_capacity(n);

        while let Some(i) = stack.pop() {
            seen[i] = true;
            result.push(i as i32);
            for (j, reqs) in rs.iter_mut().enumerate() {
                if reqs.remove(&i) && reqs.is_empty() {
                    if seen[j] {
                        break;
                    }
                    stack.push(j);
                }
            }
        }

        if result.len() == n {
            result
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn n2_p10() {
        let n = 2;
        let p = vv![[1, 0]];
        let e = [0, 1];
        assert_eq!(Solution::find_order(n, p), e);
        // Explanation: There are a total of 2 courses to take.
        // To take course 1 you should have finished course 0.
        // So the correct course order is [0,1].
    }
    #[test]
    fn n2_p01() {
        let n = 2;
        let p = vv![[0, 1]];
        let e = [1, 0];
        assert_eq!(Solution::find_order(n, p), e);
    }
    #[test]
    fn n4_p10_20_31_32() {
        let n = 4;
        let p = vv![[1, 0], [2, 0], [3, 1], [3, 2]];
        let e = [0, 2, 1, 3];
        assert_eq!(Solution::find_order(n, p), e);
        // Explanation: There are a total of 4 courses to take.
        // To take course 3 you should have finished both courses 1 and 2.
        // Both courses 1 and 2 should be taken after you finished course 0.
        // So one correct course order is [0,1,2,3].
        // Another correct ordering is [0,2,1,3].
    }
    #[test]
    fn n1_p_empty() {
        let n = 1;
        let p: Vec<Vec<i32>> = vec![];
        let e = [0];
        assert_eq!(Solution::find_order(n, p), e);
    }
    #[test]
    fn n2_p10_01() {
        let n = 2;
        let p = vv![[1, 0], [0, 1]];
        let e: Vec<i32> = vec![];
        assert_eq!(Solution::find_order(n, p), e);
    }
    #[test]
    fn n3_p21_12() {
        let n = 3;
        let p = vv![[2, 1], [1, 2]];
        let e: Vec<i32> = vec![];
        assert_eq!(Solution::find_order(n, p), e);
    }

    #[test]
    fn n2000_p10_20_21_and_so_on() {
        let n = 2000;
        let mut p = vec![];
        for i in 1..n {
            for j in 0..i {
                p.push(vec![j, i]);
            }
        }
        let e = (0..=n).rev().collect::<Vec<i32>>();
        assert_eq!(Solution::find_order(n, p), e);
    }
}
