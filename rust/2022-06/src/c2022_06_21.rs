#![allow(dead_code)]
//! \#1642. Furthest Building You Can Reach
//! =======================================
//!
//! You are given an integer array `heights` representing the heights of buildings, some `bricks`, and some `ladders`.
//!
//! You start your journey from building `0` and move to the next building by possibly using bricks or ladders.
//!
//! While moving from building `i` to building `i+1` (__0-indexed__),
//!
//! - If the current building's height is __greater than or equal__ to the next building's height,
//!   you do __not__ need a ladder or bricks.
//! - If the current building's height is __less than__ the next building's height,
//!   you can either use __one ladder__ or (`h[i+1] - h[i]`) __bricks__.
//!
//! _Return the furthest building index (0-indexed) you can reach if you use the given ladders and bricks optimally_.
//!
//! __Constraints:__
//!
//! - `1 <= heights.length <= 100_000`
//! - `1 <= heights[i] <= 1_000_000`
//! - `0 <= bricks <= 1_000_000_000`
//! - `0 <= ladders <= heights.length`
//!
//! <https://leetcode.com/problems/furthest-building-you-can-reach>

pub struct Solution {}
impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        use std::ops::ControlFlow;

        struct Acc {
            jumps: BinaryHeap<Reverse<i32>>,
            bricks: i32,
            ladders: i32,
        }
        impl Acc {
            fn new(bricks: i32, ladders: i32) -> Self {
                let jumps = BinaryHeap::new();
                Self {
                    jumps,
                    bricks,
                    ladders,
                }
            }
            fn next(mut self, (i, jump): (usize, i32)) -> ControlFlow<usize, Self> {
                self.jumps.push(Reverse(jump));
                if self.ladders > 0 {
                    self.ladders -= 1;
                    ControlFlow::Continue(self)
                } else {
                    let Reverse(min_jump) = self.jumps.pop().unwrap();
                    if min_jump <= self.bricks {
                        self.bricks -= min_jump;
                        ControlFlow::Continue(self)
                    } else {
                        ControlFlow::Break(i)
                    }
                }
            }
        }

        if let ControlFlow::Break(i) = heights
            .windows(2)
            .map(|w| w[1] - w[0])
            .enumerate()
            .filter(|&(_, jump)| jump > 0)
            .try_fold(Acc::new(bricks, ladders), |acc, e| acc.next(e))
        {
            i as i32
        } else {
            heights.len() as i32 - 1
        }
    }

    pub fn furthest_building_try_fold_1(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        use std::ops::ControlFlow;

        struct Acc(BinaryHeap<Reverse<i32>>, i32, i32);
        fn jump_fold(
            Acc(mut jumps, bricks, ladders): Acc,
            (i, jump): (usize, i32),
        ) -> ControlFlow<usize, Acc> {
            jumps.push(Reverse(jump));
            if ladders > 0 {
                ControlFlow::Continue(Acc(jumps, bricks, ladders - 1))
            } else {
                let Reverse(min_jump) = jumps.pop().unwrap();
                if min_jump <= bricks {
                    ControlFlow::Continue(Acc(jumps, bricks - min_jump, ladders))
                } else {
                    ControlFlow::Break(i)
                }
            }
        }
        match heights
            .windows(2)
            .map(|w| w[1] - w[0])
            .enumerate()
            .filter(|&(_, jump)| jump > 0)
            .try_fold(Acc(BinaryHeap::new(), bricks, ladders), jump_fold)
        {
            ControlFlow::Continue(_) => heights.len() as i32 - 1,
            ControlFlow::Break(i) => i as i32,
        }
    }
    pub fn furthest_building_min_heap_for_loop(
        heights: Vec<i32>,
        mut bricks: i32,
        mut ladders: i32,
    ) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut jumps = BinaryHeap::new();
        for (i, jump) in heights.windows(2).map(|w| w[1] - w[0]).enumerate() {
            if jump > 0 {
                jumps.push(Reverse(jump));
                if ladders > 0 {
                    ladders -= 1;
                } else {
                    let Reverse(min_jump) = jumps.pop().unwrap();
                    if min_jump <= bricks {
                        bricks -= min_jump;
                    } else {
                        return i as i32;
                    }
                }
            }
        }
        heights.len() as i32 - 1
    }

    pub fn furthest_building_brute_force(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        fn rec(i: usize, bricks: i32, ladders: i32, heights: &[i32]) -> usize {
            if i == heights.len() - 1 {
                i
            } else {
                let diff = heights[i + 1] - heights[i];
                if diff <= 0 {
                    rec(i + 1, bricks, ladders, heights)
                } else {
                    let use_bricks = if bricks >= diff {
                        rec(i + 1, bricks - diff, ladders, heights)
                    } else {
                        i
                    };
                    let use_ladder = if ladders > 0 {
                        rec(i + 1, bricks, ladders - 1, heights)
                    } else {
                        i
                    };
                    use_bricks.max(use_ladder)
                }
            }
        }
        rec(0, bricks, ladders, &heights) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h_1_b_10_l_2() {
        assert_eq!(Solution::furthest_building(vec![1], 10, 2), 0);
    }
    #[test]
    fn h_1_2_b_10_l_2() {
        assert_eq!(Solution::furthest_building(vec![1, 2], 10, 2), 1);
    }
    #[test]
    fn h_1_10_40_60_80_b_10_l_2() {
        assert_eq!(
            Solution::furthest_building(vec![1, 10, 40, 60, 80], 10, 2),
            3
        );
    }
    #[test]
    fn h_1_11_40_60_80_b_10_l_2() {
        assert_eq!(
            Solution::furthest_building(vec![1, 11, 40, 60, 80], 10, 2),
            3
        );
    }
    #[test]
    fn h_1_12_40_60_80_b_10_l_2() {
        assert_eq!(
            Solution::furthest_building(vec![1, 12, 40, 60, 80], 10, 2),
            2
        );
    }
    #[test]
    fn h_1_20_40_60_80_b_10_l_2() {
        assert_eq!(
            Solution::furthest_building(vec![1, 20, 40, 60, 80], 10, 2),
            2
        );
    }
    #[test]
    fn h_4_2_7_6_9_14_12_b_5_l_1() {
        let h = vec![4, 2, 7, 6, 9, 14, 12];
        assert_eq!(Solution::furthest_building(h, 5, 1), 4);
        // Explanation: Starting at building 0, you can follow these steps:
        // - Go to building 1 without using ladders nor bricks since 4 >= 2.
        // - Go to building 2 using 5 bricks. You must use either bricks or ladders because 2 < 7.
        // - Go to building 3 without using ladders nor bricks since 7 >= 6.
        // - Go to building 4 using your only ladder. You must use either bricks or ladders because 6 < 9.
        // It is impossible to go beyond building 4 because you do not have any more bricks or ladders.
    }
    #[test]
    fn h_4_12_2_7_3_18_20_3_19_b_10_l_2() {
        let h = vec![4, 12, 2, 7, 3, 18, 20, 3, 19];
        assert_eq!(Solution::furthest_building(h, 10, 2), 7);
    }
    #[test]
    fn h_14_3_19_3_b_17_l_0() {
        let h = vec![14, 3, 19, 3];
        assert_eq!(Solution::furthest_building(h, 17, 0), 3);
    }
}
