#![allow(dead_code)]
/// 1182. Shortest Distance to Target Color
/// =======================================
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
/// https://leetcode.com/problems/shortest-distance-to-target-color/
struct Solution;
impl Solution {
    pub fn shortest_distance_color(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        println!("shortest_distance_color({:?}, {:?})", colors, queries);
        let n = colors.len();
        let mut dists: Vec<Vec<i32>> = Vec::with_capacity(n);
        let mut prev = &vec![i32::MAX; 3];
        for c in colors {
            let mut curr: Vec<i32> = prev.iter().map(|&i| i.saturating_add(1)).collect();
            curr[(c - 1) as usize] = 0;
            dists.push(curr);
            prev = dists.last().as_ref().unwrap();
        }
        for i in (0..n - 1).rev() {
            for j in 0..dists[i].len() {
                dists[i][j] = dists[i][j].min(dists[i + 1][j].saturating_add(1));
            }
        }
        queries
            .into_iter()
            .map(|q| match dists[q[0] as usize][(q[1] - 1) as usize] {
                i32::MAX => -1,
                dist => dist,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn c_112132233_q_13_22_61() {
        let c = vec![1, 1, 2, 1, 3, 2, 2, 3, 3];
        let q = vv![[1, 3], [2, 2], [6, 1]];
        assert_eq!(Solution::shortest_distance_color(c, q), [3, 0, 3]);
        // Explanation:
        // The nearest 3 from index 1 is at index 4 (3 steps away).
        // The nearest 2 from index 2 is at index 2 itself (0 steps away).
        // The nearest 1 from index 6 is at index 3 (3 steps away).
    }
    #[test]
    fn c_12_q_03() {
        let c = vec![1, 2];
        let q = vv![[0, 3]];
        assert_eq!(Solution::shortest_distance_color(c, q), [-1]);
        // Explanation: There is no 3 in the array.
    }

    #[test]
    fn test_19() {
        let c = vec![2, 1, 2, 2, 1];
        let q = vv![[1, 1], [4, 3], [1, 3], [4, 2], [2, 1]];
        assert_eq!(Solution::shortest_distance_color(c, q), [0, -1, -1, 1, 1]);
    }

    #[test]
    fn c_123_repeat_16666_q_i1i2i3_repeat_16666() {
        let c = (0..16666).flat_map(|_| 1..=3).collect();
        let q = (0..49998).map(|i| vec![i, i % 3 + 1]).collect();
        assert_eq!(Solution::shortest_distance_color(c, q), [0; 49998]);
    }
}
