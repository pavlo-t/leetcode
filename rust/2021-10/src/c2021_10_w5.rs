#![allow(dead_code)]
/// 1762. Buildings With an Ocean View
/// ==================================
///
/// There are `n` buildings in a line.
/// You are given an integer array `heights` of size `n` that represents the heights of the buildings in the line.
///
/// The ocean is to the right of the buildings.
/// A building has an ocean view if the building can see the ocean without obstructions.
/// Formally, a building has an ocean view if all the buildings to its right have a __smaller__ height.
///
/// Return a list of indices __(0-indexed)__ of buildings that have an ocean view, sorted in increasing order.
///
/// __Constraints:__
///
/// - `1 <= heights.length <= 100_000`
/// - `1 <= heights[i] <= 1_000_000_000`
///
/// https://leetcode.com/problems/buildings-with-an-ocean-view/
struct Solution;
impl Solution {
    pub fn find_buildings(heights: Vec<i32>) -> Vec<i32> {
        println!("find_buildings({:?})", heights);
        fn pushed(v: i32, mut results: Vec<i32>) -> Vec<i32> {
            results.push(v);
            results
        }
        fn reversed(mut results: Vec<i32>) -> Vec<i32> {
            results.reverse();
            results
        }

        reversed(
            heights
                .into_iter()
                .enumerate()
                .rev()
                .fold((vec![], 0), |(rsf, max_h), (i, h)| match h > max_h {
                    true => (pushed(i as i32, rsf), h),
                    _ => (rsf, max_h),
                })
                .0,
        )
    }
    pub fn find_buildings_iterative(heights: Vec<i32>) -> Vec<i32> {
        println!("find_buildings({:?})", heights);
        let mut results = vec![];
        let mut max_h = 0;
        for (i, &h) in heights.iter().enumerate().rev() {
            if h > max_h {
                max_h = h;
                results.push(i as i32);
            }
        }
        results.reverse();
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h_4_2_3_1() {
        let h = vec![4, 2, 3, 1];
        let e = vec![0, 2, 3];
        assert_eq!(Solution::find_buildings(h), e);
        // Explanation: Building 1 (0-indexed) does not have an ocean view because building 2 is taller.
    }
    #[test]
    fn h_4_3_2_1() {
        let h = vec![4, 3, 2, 1];
        let e = vec![0, 1, 2, 3];
        assert_eq!(Solution::find_buildings(h), e);
        // Explanation: All the buildings have an ocean view.
    }
    #[test]
    fn h_1_3_2_4() {
        let h = vec![1, 3, 2, 4];
        let e = vec![3];
        assert_eq!(Solution::find_buildings(h), e);
        // Explanation: Only building 3 has an ocean view.
    }
    #[test]
    fn h_2_2_2_2() {
        let h = vec![2, 2, 2, 2];
        let e = vec![3];
        assert_eq!(Solution::find_buildings(h), e);
        // Explanation: Buildings cannot see the ocean if there are buildings of the same height to its right.
    }

    #[test]
    fn h_100000_to_1() {
        let h = (1..=100000).rev().collect::<Vec<_>>();
        let e = (0..100000).collect::<Vec<_>>();
        assert_eq!(Solution::find_buildings(h), e);
    }
}
