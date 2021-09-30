#![allow(dead_code)]
/// Subsets II
/// ==========
///
/// Given an integer array `nums` that may contain duplicates, return _all possible subsets (the power set)_.
///
/// The solution set __must not__ contain duplicate subsets.
/// Return the solution in __any order__.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 10`
/// - `-10 <= nums[i] <= 10`
///
/// https://leetcode.com/explore/featured/card/august-leetcoding-challenge-2021/613/week-1-august-1st-august-7th/3837/
struct Solution;
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        fn pick_n(nums: &[i32], n: usize) -> Vec<Vec<i32>> {
            if n == 0 {
                vec![vec![]]
            } else if n == 1 {
                nums.iter().map(|&i| vec![i]).collect()
            } else {
                let mut i = 0;
                let mut result = vec![];
                while i <= nums.len() - n {
                    for mut v in pick_n(&nums[i + 1..], n - 1) {
                        v.push(nums[i]);
                        result.push(v);
                    }
                    i += 1;
                }
                result
            }
        }

        let mut result = HashSet::new();
        for n in 0..=nums.len() {
            for mut v in pick_n(&nums, n) {
                v.sort_unstable();
                result.insert(v);
            }
        }

        result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn n_1_2_2_produces_6_subsets() {
        let nums = vec![1, 2, 2];
        let e = vv![[], [1], [1, 2], [1, 2, 2], [2], [2, 2]];
        let mut r = Solution::subsets_with_dup(nums);
        r.sort_unstable();
        assert_eq!(r, e);
    }
    #[test]
    fn n_0_produces_2_subsets() {
        let nums = vec![0];
        let e = vv![[], [0]];
        let mut r = Solution::subsets_with_dup(nums);
        r.sort_unstable();
        assert_eq!(r, e);
    }

    #[test]
    fn n_1_2_produces_4_subsets() {
        let nums = vec![1, 2];
        let e = vv![[], [1], [1, 2], [2]];
        let mut r = Solution::subsets_with_dup(nums);
        r.sort_unstable();
        assert_eq!(r, e);
    }
    #[test]
    fn n_1_2_3_produces_8_subsets() {
        let nums = vec![1, 2, 3];
        let e = vv![[], [1], [1, 2], [1, 2, 3], [1, 3], [2], [2, 3], [3]];
        let mut r = Solution::subsets_with_dup(nums);
        r.sort_unstable();
        assert_eq!(r, e);
    }
    #[test]
    fn n_1to10_produces_1024_subsets() {
        let nums = (1..=10).collect();
        let r = Solution::subsets_with_dup(nums);
        assert_eq!(r.len(), 1024);
    }

    #[test]
    fn n_4_4_4_1_4_produces_10_subsets() {
        let nums = vec![4, 4, 4, 1, 4];
        let e = vv![
            [],
            [1],
            [1, 4],
            [1, 4, 4],
            [1, 4, 4, 4],
            [1, 4, 4, 4, 4],
            [4],
            [4, 4],
            [4, 4, 4],
            [4, 4, 4, 4]
        ];
        let mut r = Solution::subsets_with_dup(nums);
        r.sort_unstable();
        assert_eq!(r, e);
    }
}
