#![allow(dead_code)]
/// 973. K Closest Points to Origin
/// ===============================
///
/// Given an array of `points` where `points[i] = [xi, yi]` represents a point on the __X-Y__ plane and an integer `k`,
/// return the `k` closest points to the origin `(0, 0)`.
///
/// The distance between two points on the __X-Y__ plane is the Euclidean distance
/// (i.e., `√(x1 - x2)**2 + (y1 - y2)**2`).
///
/// You may return the answer in __any order__.
/// The answer is __guaranteed__ to be __unique__ (except for the order that it is in).
///
/// __Constraints:__
///
/// - `1 <= k <= points.length <= 10_000`
/// - `-10_000 < xi, yi < 10_000`
///
/// https://leetcode.com/problems/k-closest-points-to-origin/
struct Solution;
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, mut k: i32) -> Vec<Vec<i32>> {
        println!("k_closest({:?}, {})", points, k);
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut min_heap = BinaryHeap::new();
        for p in points {
            // distance square
            let ds = p[0].pow(2) + p[1].pow(2);
            min_heap.push(Reverse((ds, p)));
        }

        let mut result = vec![];
        while k > 0 {
            let Reverse((_, p)) = min_heap.pop().unwrap();
            result.push(p);
            k -= 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn ps_p1p3_m2p2_k_1() {
        let ps = vv![[1, 3], [-2, 2]];
        let k = 1;
        let e = vv![[-2, 2]];
        assert_eq!(Solution::k_closest(ps, k), e);
        // Explanation:
        // The distance between (1, 3) and the origin is sqrt(10).
        // The distance between (-2, 2) and the origin is sqrt(8).
        // Since sqrt(8) < sqrt(10), (-2, 2) is closer to the origin.
        // We only want the closest k = 1 points from the origin, so the answer is just [[-2,2]].
    }
    #[test]
    fn ps_p3p3_p5m1_m2p4_k_2() {
        let ps = vv![[3, 3], [5, -1], [-2, 4]];
        let k = 2;
        let e = vv![[3, 3], [-2, 4]];
        assert_eq!(Solution::k_closest(ps, k), e);
        // Explanation: The answer [[-2,4],[3,3]] would also be accepted.
    }
}
