#![allow(dead_code)]
/// Erect the Fence
/// ===============
///
/// You are given an array `trees` where `trees[i] = [xi, yi]` represents the location of a tree in the garden.
///
/// You are asked to fence the entire garden using the minimum length of rope as it is expensive.
/// The garden is well fenced only if __all the trees are enclosed__.
///
/// Return _the coordinates of trees that are exactly located on the fence perimeter_.
///
/// __Constraints:__
///
/// - `1 <= points.length <= 3000`
/// - `points[i].length == 2`
/// - `0 <= xi, yi <= 100`
/// - All the given points are __unique__.
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/636/week-1-september-1st-september-7th/3962/
struct Solution;
impl Solution {
    /// https://ttzztt.gitbooks.io/lc/content/jingchiai/erect-the-fence.html
    pub fn outer_trees(mut trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if trees.len() <= 3 {
            trees
        } else {
            use std::collections::HashSet;

            fn orientation(p1: &[i32], p2: &[i32], p3: &[i32]) -> i32 {
                (p2[1] - p1[1]) * (p3[0] - p2[0]) - (p2[0] - p1[0]) * (p3[1] - p2[1])
            }
            fn add<'a>(res: &mut Vec<&'a [i32]>, t: &'a [i32]) {
                while res.len() >= 2 && orientation(res[res.len() - 2], res[res.len() - 1], t) < 0 {
                    res.pop();
                }
                res.push(t)
            }

            trees.sort_unstable();
            let mut res = vec![];
            trees.iter().for_each(|t| add(&mut res, t));
            res.pop();
            trees.iter().rev().for_each(|t| add(&mut res, t));
            res.pop();
            res.into_iter()
                .map(|t| (t[0].to_owned(), t[1].to_owned()))
                .collect::<HashSet<_>>()
                .into_iter()
                .map(|(x, y)| vec![x, y])
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]}}

    #[test]
    fn example1_1x1_2x2_2x0_2x4_3x3_4x2_produces_1x1_2x0_3x3_2x4_4x2() {
        let points = vv![[1, 1], [2, 2], [2, 0], [2, 4], [3, 3], [4, 2]];
        let e = vv![[1, 1], [2, 0], [2, 4], [3, 3], [4, 2]];
        let mut res = Solution::outer_trees(points);
        res.sort_unstable();
        assert_eq!(res, e);
    }
    #[test]
    fn example2_1x2_2x2_4x2_produces_4x2_2x2_1x2() {
        let points = vv![[1, 2], [2, 2], [4, 2]];
        let e = vv![[1, 2], [2, 2], [4, 2]];
        let mut res = Solution::outer_trees(points);
        res.sort_unstable();
        assert_eq!(res, e);
    }
    #[test]
    fn test85_1x2_2x2_4x2_5x2_6x2_7x2_produces_1x2_2x2_4x2_5x2_6x2_7x2() {
        let points = vv![[1, 2], [2, 2], [4, 2], [5, 2], [6, 2], [7, 2]];
        let e = vv![[1, 2], [2, 2], [4, 2], [5, 2], [6, 2], [7, 2]];
        let mut res = Solution::outer_trees(points);
        res.sort_unstable();
        assert_eq!(res, e);
    }
}
