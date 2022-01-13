#![allow(dead_code)]
/// 452. Minimum Number of Arrows to Burst Balloons
/// ===============================================
///
/// There are some spherical balloons taped onto a flat wall that represents the XY-plane.
/// The balloons are represented as a 2D integer array `points` where `points[i] = [x_start, x_end]`
/// denotes a balloon whose __horizontal diameter__ stretches between `x_start` and `x_end`.
/// You do not know the exact y-coordinates of the balloons.
///
/// Arrows can be shot up __directly vertically__ (in the positive y-direction) from different points along the x-axis.
/// A balloon with `x_start` and `x_end` is __burst__ by an arrow shot at `x` if `x_start <= x <= x_end`.
/// There is __no limit__ to the number of arrows that can be shot.
/// A shot arrow keeps traveling up infinitely, bursting any balloons in its path.
///
/// Given the array `points`, return _the __minimum__ number of arrows that must be shot to burst all balloons_.
///
/// __Constraints:__
///
/// - `1 <= points.length <= 100_000`
/// - `points[i].length == 2`
/// - `-2**31 <= xstart < xend <= 2**31 - 1`
///
/// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/
struct Solution;
impl Solution {
    pub fn find_min_arrow_shots_my(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable();
        let mut result = 1;
        let mut prev_r = points[0][1];
        for p in points.iter().skip(1) {
            let (l, r) = (p[0], p[1]);
            if prev_r < l {
                result += 1;
                prev_r = r;
            } else {
                prev_r = prev_r.min(r);
            }
        }

        result
    }

    /// from other submissions https://leetcode.com/submissions/detail/619174678/
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let mut result = 1;
        let mut prev_r = points[0][1];
        for p in points.iter().skip(1) {
            let (l, r) = (p[0], p[1]);
            if prev_r < l {
                result += 1;
                prev_r = r;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[rustfmt::skip] #[test] fn p_1t9() { assert_eq!(Solution::find_min_arrow_shots(vv![[1,9]]), 1); }
    #[rustfmt::skip] #[test] fn p_1t9_2t3() { assert_eq!(Solution::find_min_arrow_shots(vv![[1,9], [2,3]]), 1); }
    #[rustfmt::skip] #[test] fn p_1t9_2t3_3t4() { assert_eq!(Solution::find_min_arrow_shots(vv![[1,9],[2,3],[3,4]]), 1); }

    #[rustfmt::skip] #[test] fn p_1t9_2t3_4t5() {
        assert_eq!(Solution::find_min_arrow_shots(vv![[1,9],[2,3],[4,5]]), 2);
    }
    #[rustfmt::skip] #[test] fn p_1t9_2t3_4t5_5t6() {
        assert_eq!(Solution::find_min_arrow_shots(vv![[1,9],[2,3],[4,5],[5,6]]), 2);
    }
    #[rustfmt::skip] #[test] fn p_1t9_2t3_4t5_6t7() {
        assert_eq!(Solution::find_min_arrow_shots(vv![[1,9],[2,3],[4,5],[6,7]]), 3);
    }

    #[test]
    fn p_10t16_2t8_1t6_7t12() {
        let p = vv![[10, 16], [2, 8], [1, 6], [7, 12]];
        //      vv![[1, 6], [2, 8], [7, 12], [10, 16]];
        assert_eq!(Solution::find_min_arrow_shots(p), 2);
        // Explanation: The balloons can be burst by 2 arrows:
        // - Shoot an arrow at x = 6, bursting the balloons [2,8] and [1,6].
        // - Shoot an arrow at x = 11, bursting the balloons [10,16] and [7,12].
    }
    #[test]
    fn p_1t2_3t4_5t6_7t8() {
        let p = vv![[1, 2], [3, 4], [5, 6], [7, 8]];
        assert_eq!(Solution::find_min_arrow_shots(p), 4);
        // Explanation: One arrow needs to be shot for each balloon for a total of 4 arrows.
    }
    #[test]
    fn p_1t2_2t3_3t4_4t5() {
        let p = vv![[1, 2], [2, 3], [3, 4], [4, 5]];
        assert_eq!(Solution::find_min_arrow_shots(p), 2);
        // Explanation: The balloons can be burst by 2 arrows:
        // - Shoot an arrow at x = 2, bursting the balloons [1,2] and [2,3].
        // - Shoot an arrow at x = 4, bursting the balloons [3,4] and [4,5].
    }
}
