#![allow(dead_code)]
/// 75. Sort Colors
/// ===============
///
/// Given an array `nums` with `n` objects colored red, white, or blue,
/// sort them [in-place](https://en.wikipedia.org/wiki/In-place_algorithm)
/// so that objects of the same color are adjacent, with the colors in the order red, white, and blue.
///
/// We will use the integers `0`, `1`, and `2` to represent the color red, white, and blue, respectively.
///
/// You must solve this problem without using the library's sort function.
///
/// __Constraints:__
///
/// - `n == nums.length`
/// - `1 <= n <= 300`
/// - `nums[i]` is `0`, `1`, or `2`.
///
/// __Follow up:__ Could you come up with a one-pass algorithm using only constant extra space?
///
/// https://leetcode.com/problems/sort-colors/
struct Solution;
impl Solution {
    /// Approach 1: One Pass
    /// https://leetcode.com/problems/sort-colors/solution/
    pub fn sort_colors(nums: &mut Vec<i32>) {
        println!("sort_colors({:?})", nums);
        let (mut l, mut i, mut r) = (0, 0, nums.len() - 1);
        while i <= r {
            match nums[i] {
                0 => {
                    nums.swap(l, i);
                    l += 1;
                    i += 1;
                },
                1 => i += 1,
                2 => {
                    nums.swap(i, r);
                    r -= 1;
                },
                _ => unreachable!()
            }
        }
    }
    pub fn sort_colors_my_2_pass_from_hints(nums: &mut Vec<i32>) {
        println!("sort_colors({:?})", nums);
        let mut cnts = nums.iter().fold(vec![0; 3], |mut rsf, &c| {
            rsf[c as usize] += 1;
            rsf
        });
        for i in 1..cnts.len() {
            cnts[i] += cnts[i - 1];
        }
        for i in 0..nums.len() {
            nums[i] = if i < cnts[0] {
                0
            } else if i < cnts[1] {
                1
            } else {
                2
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_2_0_2_1_1_0() {
        let mut n = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut n);
        assert_eq!(n, [0, 0, 1, 1, 2, 2]);
    }
    #[test]
    fn n_2_0_1() {
        let mut n = vec![2, 0, 1];
        Solution::sort_colors(&mut n);
        assert_eq!(n, [0, 1, 2]);
    }
    #[test]
    fn n_0() {
        let mut n = vec![0];
        Solution::sort_colors(&mut n);
        assert_eq!(n, [0]);
    }
    #[test]
    fn n_1() {
        let mut n = vec![1];
        Solution::sort_colors(&mut n);
        assert_eq!(n, [1]);
    }
}
