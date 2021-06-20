#![allow(dead_code)]
/// Shortest Distance to Target Color
/// =================================
///
/// You are given an array `colors`, in which there are three colors: `1`, `2` and `3`.
///
/// You are also given some queries.
/// Each query consists of two integers `i` and `c`,
/// return the shortest distance between the given index `i` and the target color `c`.
/// If there is no solution return `-1`.
///
/// __Constraints:__
///
/// - `1 <= colors.length <= 50_000`
/// - `1 <= colors[i] <= 3`
/// - `1 <= queries.length <= 50_000`
/// - `queries[i].length == 2`
/// - `0 <= queries[i][0] < colors.length`
/// - `1 <= queries[i][1] <= 3`
///
/// https://leetcode.com/explore/challenge/card/june-leetcoding-challenge-2021/605/week-3-june-15th-june-21st/3779/
struct Solution;
impl Solution {
    pub fn shortest_distance_color(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        fn build_dp(n: usize, is: Vec<usize>) -> Vec<i32> {
            if is.is_empty() {
                vec![-1; n]
            } else {
                let mut result = Vec::with_capacity(n);
                let mut l = is[0];
                for i in 0..l {
                    result.push((l - i) as i32)
                }
                for &r in &is[1..] {
                    for i in l..r {
                        result.push((i - l).min(r - i) as i32);
                    }
                    l = r;
                }
                for i in l..n {
                    result.push((i - l) as i32);
                }
                result
            }
        }

        let n = colors.len();
        let (c1, c2, c3) = colors.iter().enumerate().fold(
            (
                Vec::with_capacity(n),
                Vec::with_capacity(n),
                Vec::with_capacity(n),
            ),
            |(mut c1, mut c2, mut c3), (i, c)| {
                match c {
                    1 => c1.push(i),
                    2 => c2.push(i),
                    3 => c3.push(i),
                    _ => unreachable!(),
                };
                (c1, c2, c3)
            },
        );

        let c1 = build_dp(n, c1);
        let c2 = build_dp(n, c2);
        let c3 = build_dp(n, c3);

        let mut results = Vec::with_capacity(queries.len());
        for q in queries {
            let (i, c) = (q[0] as usize, q[1]);
            let d = match c {
                1 => c1[i],
                2 => c2[i],
                3 => c3[i],
                _ => unreachable!(),
            } as i32;
            results.push(d);
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn example1() {
        let colors = vec![1, 1, 2, 1, 3, 2, 2, 3, 3];
        let queries = vv![[1, 3], [2, 2], [6, 1]];
        let e = [3, 0, 3];
        assert_eq!(Solution::shortest_distance_color(colors, queries), e);
        // Explanation:
        // The nearest 3 from index 1 is at index 4 (3 steps away).
        // The nearest 2 from index 2 is at index 2 itself (0 steps away).
        // The nearest 1 from index 6 is at index 3 (3 steps away).
    }
    #[test]
    fn example2() {
        let colors = vec![1, 2];
        let queries = vv![[0, 3]];
        let e = [-1];
        assert_eq!(Solution::shortest_distance_color(colors, queries), e);
        // Explanation: There is no 3 in the array.
    }

    mod performance {
        use super::*;

        #[test]
        fn colors_50k_queries_50k() {
            let colors = vec![1; 50_000];
            let queries = (0..50_000).map(|i| vec![i as i32, 1]).collect();
            let e = vec![0; 50_000];
            assert_eq!(Solution::shortest_distance_color(colors, queries), e);
        }
    }
}
