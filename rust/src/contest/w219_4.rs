#![allow(dead_code)]

/// ### 1691. Maximum Height by Stacking Cuboids
///
/// Given `n` cuboids where the dimensions of the `i`th cuboid is `cuboids[i] = [width_i, length_i, height_i]`
/// (__0-indexed__). Choose a __subset__ of `cuboids` and place them on each other.
///
/// You can place cuboid `i` on cuboid `j` if `width_i <= width_j` and `length_i <= length_j` and
/// `height_i <= height_j`. You can rearrange any cuboid's dimensions by rotating it to put it on another cuboid.
///
/// Return _the __maximum height__ of the stacked_ `cuboids`.
///
/// __Constraints:__
///
/// - `1 <= cuboids.length <= 100`
/// - `1 <= width_i, length_i, height_i <= 100`
///
/// https://leetcode.com/contest/weekly-contest-219/problems/maximum-height-by-stacking-cuboids/
struct Solution;

impl Solution {
    pub fn max_height(cuboids: Vec<Vec<i32>>) -> i32 {
        let mut cuboids = cuboids;
        cuboids.iter_mut().for_each(|c| c.sort_unstable());
        cuboids.sort_unstable();

        let mut dp: Vec<_> = cuboids.iter().map(|c| c[2]).collect();

        for bot in 1..cuboids.len() {
            for top in 0..bot {
                let ct = &cuboids[top];
                let cb = &cuboids[bot];
                if ct[1] <= cb[1] && ct[2] <= cb[2] {
                    dp[bot] = dp[bot].max(dp[top] + cb[2]);
                }
            }
        }

        dp.into_iter().max().unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_c50_45_20_c95_37_53_c45_23_12_should_be_190() {
        let cuboids = vec![vec![50, 45, 20], vec![95, 37, 53], vec![45, 23, 12]];
        assert_eq!(Solution::max_height(cuboids), 190);
        // Example 1:
        // Input: cuboids = [[50,45,20],[95,37,53],[45,23,12]]
        // Output: 190
        // Explanation:
        // Cuboid 1 is placed on the bottom with the 53x37 side facing down with height 95.
        // Cuboid 0 is placed next with the 45x20 side facing down with height 50.
        // Cuboid 2 is placed next with the 23x12 side facing down with height 45.
        // The total height is 95 + 50 + 45 = 190.
    }

    #[test]
    fn example2_c38_25_45_c76_35_3_should_be_76() {
        let cuboids = vec![vec![38, 25, 45], vec![76, 35, 3]];
        assert_eq!(Solution::max_height(cuboids), 76);
        // Example 2:
        // Input: cuboids = [[38,25,45],[76,35,3]]
        // Output: 76
        // Explanation:
        // You can't place any of the cuboids on the other.
        // We choose cuboid 1 and rotate it so that the 35x3 side is facing down and its height is 76.
    }

    #[test]
    fn example3_c7_11_17_repeat_6_should_be_102() {
        let cuboids =
            vec![vec![7, 11, 17], vec![7, 17, 11], vec![11, 7, 17], vec![11, 17, 7], vec![17, 7, 11], vec![17, 11, 7]];
        assert_eq!(Solution::max_height(cuboids), 102);
        // Example 3:
        // Input: cuboids = [[7,11,17],[7,17,11],[11,7,17],[11,17,7],[17,7,11],[17,11,7]]
        // Output: 102
        // Explanation:
        // After rearranging the cuboids, you can see that all cuboids have the same dimension.
        // You can place the 11x7 side down on all cuboids so their heights are 17.
        // The maximum height of stacked cuboids is 6 * 17 = 102.
    }

    #[test]
    fn test_c132_should_be_3() {
        assert_eq!(Solution::max_height(vec![vec![1, 3, 2]]), 3);
    }

    #[test]
    fn test_c321_repeat_100_should_be_300() {
        assert_eq!(Solution::max_height(vec![vec![3, 2, 1]; 100]), 300);
    }
}