#![allow(dead_code)]
/// 78. Subsets
/// ===========
///
/// Given an integer array `nums` of __unique__ elements, return _all possible subsets (the power set)_.
///
/// The solution set __must not__ contain duplicate subsets.
/// Return the solution in __any order__.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 10`
/// - `-10 <= nums[i] <= 10`
/// - All the numbers of `nums` are __unique__.
///
/// https://leetcode.com/problems/subsets/
struct Solution;
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn bts(
            i: usize,
            len: usize,
            curr: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
            nums: &[i32],
        ) {
            let n = nums.len();
            if curr.len() == len {
                result.push(curr.clone());
            } else if i < n {
                curr.push(nums[i]);
                bts(i + 1, len, curr, result, nums);
                curr.pop();
                if i + len - curr.len() < n {
                    bts(i + 1, len, curr, result, nums);
                }
            }
        }

        let mut result = vec![vec![]];
        for len in 1..=nums.len() {
            bts(0, len, &mut vec![], &mut result, &nums);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn n_0() {
        let n = vec![0];
        let e = vv![[], [0]];
        assert_eq!(Solution::subsets(n), e);
    }
    #[test]
    fn n_0_1() {
        let n = vec![0, 1];
        let e = vv![[], [0], [1], [0, 1]];
        assert_eq!(Solution::subsets(n), e);
    }
    #[test]
    fn n_1_2_3() {
        let n = vec![1, 2, 3];
        let e = vv![[], [1], [2], [3], [1, 2], [1, 3], [2, 3], [1, 2, 3]];
        assert_eq!(Solution::subsets(n), e);
    }
    #[test]
    fn n_1_2_3_4() {
        let n = vec![1, 2, 3, 4];
        let e = vv![
            [],
            [1],
            [2],
            [3],
            [4],
            [1, 2],
            [1, 3],
            [1, 4],
            [2, 3],
            [2, 4],
            [3, 4],
            [1, 2, 3],
            [1, 2, 4],
            [1, 3, 4],
            [2, 3, 4],
            [1, 2, 3, 4]
        ];
        assert_eq!(Solution::subsets(n), e);
    }

    #[test]
    fn n_0_1_2_3_4_5_6_7_8_9() {
        let n = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(Solution::subsets(n).len(), 1024);
    }
}
