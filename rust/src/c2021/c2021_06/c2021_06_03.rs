#![allow(dead_code)]
/// Maximum Area of a Piece of Cake After Horizontal and Vertical Cuts
/// ==================================================================
///
/// Given a rectangular cake with height `h` and width `w`,
/// and two arrays of integers `horizontalCuts` and `verticalCuts`
/// where `horizontalCuts[i]` is the distance from the top of the rectangular cake to the `i`th horizontal cut
/// and similarly, `verticalCuts[j]` is the distance from the left of the rectangular cake to the `j`th vertical cut.
///
/// _Return the maximum area of a piece of cake after you cut at each horizontal and vertical position
/// provided in the arrays `horizontalCuts` and `verticalCuts`_.
/// Since the answer can be a huge number, return this modulo `10^9 + 7`.
///
/// __Constraints:__
///
/// - `2 <= h, w <= 10^9`
/// - `1 <= horizontalCuts.length < min(h, 10^5)`
/// - `1 <= verticalCuts.length < min(w, 10^5)`
/// - `1 <= horizontalCuts[i] < h`
/// - `1 <= verticalCuts[i] < w`
/// - It is guaranteed that all elements in `horizontalCuts` are distinct.
/// - It is guaranteed that all elements in `verticalCuts` are distinct.
///
/// https://leetcode.com/explore/featured/card/june-leetcoding-challenge-2021/603/week-1-june-1st-june-7th/3766/
struct Solution;
impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        fn get_max(mut cs: Vec<i32>, l: i32) -> i64 {
            cs.sort_unstable();
            cs.iter()
                .chain([l].iter())
                .fold((0, 0), |(max, p), &c| (max.max(c - p), c))
                .0 as i64
        }

        let mh = get_max(horizontal_cuts, h);
        let mv = get_max(vertical_cuts, w);

        ((mh * mv) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h5w4_hc_1_2_4_vc_1_3_produces_4() {
        let h = 5;
        let w = 4;
        let horizontal_cuts = vec![1, 2, 4];
        let vertical_cuts = vec![1, 3];
        assert_eq!(Solution::max_area(h, w, horizontal_cuts, vertical_cuts), 4);
        //  0 1 2 3 4
        // 0+-+---+-+
        //  |o|o o|o|
        // 1+-+---+-+
        //  |o|o o|o|
        // 2+-+---+-+
        //  |o|X X|o|
        // 3| |   | |
        //  |o|X X|o|
        // 4+-+---+-+
        //  |o|o o|o|
        // 5+-+---+-+
        // Explanation: The figure above represents the given rectangular cake.
        // Dashes (-) are the horizontal and pipes (|) are vertical cuts.
        // After you cut the cake, the `X` marked piece of cake has the maximum area.
    }
    #[test]
    fn h5w4_hc_3_1_vc_1_produces_6() {
        let h = 5;
        let w = 4;
        let horizontal_cuts = vec![3, 1];
        let vertical_cuts = vec![1];
        assert_eq!(Solution::max_area(h, w, horizontal_cuts, vertical_cuts), 6);
        //  0 1 2 3 4
        // 0+-+-----+
        //  |o|o o o|
        // 1+-+-----+
        //  |o|X X X|
        // 2| |     |
        //  |o|X X X|
        // 3+-+-----+
        //  |o|Y Y Y|
        // 4| |     |
        //  |o|Y Y Y|
        // 5+-+-----+
        // Explanation: The figure above represents the given rectangular cake.
        // Dashes (-) are the horizontal and pipes (|) are vertical cuts.
        // After you cut the cake, the X and Y pieces of cake have the maximum area.
    }
    #[test]
    fn h5w4_hc_3_vc_3_produces_9() {
        let h = 5;
        let w = 4;
        let horizontal_cuts = vec![3];
        let vertical_cuts = vec![3];
        assert_eq!(Solution::max_area(h, w, horizontal_cuts, vertical_cuts), 9);
    }

    #[test]
    fn h10pow9_w10pow9_hc_1_vc_1_produces_64() {
        let h = 1_000_000_000;
        let w = 1_000_000_000;
        let horizontal_cuts = vec![1];
        let vertical_cuts = vec![1];
        assert_eq!(Solution::max_area(h, w, horizontal_cuts, vertical_cuts), 64);
    }
    #[test]
    fn h10pow9_w10pow9_hc_1to10k_vc_1to10k_produces_100_140_049() {
        let h = 1_000_000_000;
        let w = 1_000_000_000;
        let hcs = (1..=10000).collect();
        let vcs = (1..=10000).collect();
        assert_eq!(Solution::max_area(h, w, hcs, vcs), 100_140_049);
    }
    #[test]
    fn h10pow9_w10pow9_hc_1to100k_vc_1to100k_produces_1_399_979() {
        let h = 1_000_000_000;
        let w = 1_000_000_000;
        let hcs = (1..=100_000).collect();
        let vcs = (1..=100_000).collect();
        assert_eq!(Solution::max_area(h, w, hcs, vcs), 1_399_979);
    }
}
