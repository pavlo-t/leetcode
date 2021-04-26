#![allow(dead_code)]
/// Furthest Building You Can Reach
/// ===============================
///
/// You are given an integer array `heights` representing the heights of buildings, some `bricks`,
/// and some `ladders`.
///
/// You start your journey from building `0` and move to the next building by possibly using bricks
/// or ladders.
///
/// While moving from building `i` to building `i+1` (__0-indexed__),
///
/// - If the current building's height is __greater than or equal__ to the next building's height,
///   you do __not__ need a ladder or bricks.
/// - If the current building's height is __less than__ the next building's height,
///   you can either use __one ladder__ or `(h[i+1] - h[i])` __bricks__.
///
/// _Return the furthest building index (0-indexed) you can reach if you use the given ladders and
/// bricks optimally_.
///
/// __Constraints:__
///
/// - `1 <= heights.length <= 100_000`
/// - `1 <= heights[i] <= 1_000_000`
/// - `0 <= bricks <= 1_000_000_000`
/// - `0 <= ladders <= heights.length`
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/596/week-4-april-22nd-april-28th/3721/
struct Solution;
impl Solution {
    pub fn furthest_building_max_heap(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
        use std::collections::BinaryHeap;

        let mut climbs = BinaryHeap::new();
        for (i, w) in heights.windows(2).enumerate() {
            let (l, r) = (w[0], w[1]);
            if l < r {
                let climb = r - l;
                climbs.push(climb);
                bricks -= climb;
                if bricks < 0 {
                    if ladders > 0 {
                        let climb = climbs.pop().unwrap();
                        bricks += climb;
                        ladders -= 1;
                    } else {
                        return i as i32;
                    }
                }
            }
        }
        heights.len() as i32 - 1
    }

    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, ladders: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut climbs = BinaryHeap::new();
        for (i, w) in heights.windows(2).enumerate() {
            let (l, r) = (w[0], w[1]);
            if l < r {
                climbs.push(Reverse(r - l));
                if climbs.len() > ladders as usize {
                    let climb = climbs.pop().unwrap().0;
                    if bricks - climb < 0 {
                        return i as i32;
                    } else {
                        bricks -= climb;
                    }
                }
            }
        }
        heights.len() as i32 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let heights = vec![4, 2, 7, 6, 9, 14, 12];
        assert_eq!(Solution::furthest_building(heights, 5, 1), 4);
        // Explanation: Starting at building 0, you can follow these steps:
        // - Go to building 1 without using ladders nor bricks since 4 >= 2.
        // - Go to building 2 using 5 bricks. You must use either bricks or ladders because 2 < 7.
        // - Go to building 3 without using ladders nor bricks since 7 >= 6.
        // - Go to building 4 using your only ladder. You must use either bricks or ladders because 6 < 9.
        // It is impossible to go beyond building 4 because you do not have any more bricks or ladders.
    }
    #[test]
    fn example2() {
        let heights = vec![4, 12, 2, 7, 3, 18, 20, 3, 19];
        assert_eq!(Solution::furthest_building(heights, 10, 2), 7);
    }
    #[test]
    fn example3() {
        let heights = vec![14, 3, 19, 3];
        assert_eq!(Solution::furthest_building(heights, 17, 0), 3);
    }

    #[test]
    fn h4_2_7_6_9_14_12_b5l1_produces_5() {
        let heights = vec![4, 2, 7, 6, 9, 11, 12];
        assert_eq!(Solution::furthest_building(heights, 5, 1), 5);
    }

    mod performance {
        use super::*;

        #[test]
        fn h1to100000_b100000_l0_produces_99999() {
            let heights = (1..=100_000).collect();
            assert_eq!(Solution::furthest_building(heights, 100_000, 0), 99_999);
        }
        #[test]
        fn h1to100000_b0_l100000_produces_99999() {
            let heights = (1..=100_000).collect();
            assert_eq!(Solution::furthest_building(heights, 0, 100_000), 99_999);
        }
        #[test]
        fn h1to100000_b50000_l50000_produces_99999() {
            let heights = (1..=100_000).collect();
            assert_eq!(Solution::furthest_building(heights, 50_000, 50_000), 99_999);
        }
    }
}
