#![allow(dead_code)]
/// Find First and Last Position of Element in Sorted Array
/// =======================================================
///
/// Given an array of integers `nums` sorted in ascending order,
/// find the starting and ending position of a given `target` value.
///
/// If `target` is not found in the array, return `[-1, -1]`.
///
/// __Follow up:__ Could you write an algorithm with `O(log n)` runtime complexity?
///
/// __Constraints:__
///
/// - `0 <= nums.length <= 10^5`
/// - `-10^9 <= nums[i] <= 10^9`
/// - `nums` is a non-decreasing array.
/// - `-10^9 <= target <= 10^9`
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/597/week-5-april-29th-april-30th/3725/
struct Solution;
impl Solution {
    pub fn search_range_binary_search(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            vec![-1, -1]
        } else {
            let mut result = vec![-1, -1];

            let mut l = 0;
            let mut r = nums.len() - 1;
            while l < r {
                let m = l + (r - l) / 2;
                if target <= nums[m] {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            if nums[l] == target {
                result[0] = l as i32;

                l = 0;
                r = nums.len() - 1;
                while l < r {
                    let m = l + (r - l + 1) / 2;
                    if nums[m] <= target {
                        l = m;
                    } else {
                        r = m - 1;
                    }
                }
                if nums[l] == target {
                    result[1] = l as i32;
                }
            }

            result
        }
    }

    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.into_iter()
            .enumerate()
            .filter_map(|(i, n)| if n == target { Some(i as i32) } else { None })
            .fold([-1, -1], |[l, _], i| if l >= 0 { [l, i] } else { [i, i] })
            .to_vec()
    }

    pub fn search_range_linear(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut r = vec![-1, -1];
        for (i, n) in nums.into_iter().enumerate() {
            if n == target {
                if r[0] == -1 {
                    r[0] = i as i32;
                }
                r[1] = i as i32;
            }
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        assert_eq!(Solution::search_range(nums, 8), [3, 4]);
    }
    #[test]
    fn example2() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        assert_eq!(Solution::search_range(nums, 6), [-1, -1]);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::search_range(Vec::new(), 0), [-1, -1]);
    }

    #[test]
    fn test6() {
        let nums = vec![2, 2];
        assert_eq!(Solution::search_range(nums, 1), [-1, -1]);
    }

    #[test]
    fn tst1() {
        let nums = vec![5, 7, 8, 8, 8, 8, 10];
        assert_eq!(Solution::search_range(nums, 8), [2, 5]);
    }
    #[test]
    fn tst2() {
        let nums = vec![2, 2];
        assert_eq!(Solution::search_range(nums, 3), [-1, -1]);
    }
}
