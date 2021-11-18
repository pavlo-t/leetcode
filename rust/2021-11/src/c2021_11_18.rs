#![allow(dead_code)]
/// 448. Find All Numbers Disappeared in an Array
/// =============================================
///
/// Given an array `nums` of `n` integers where `nums[i]` is in the range `[1, n]`,
/// return _an array of all the integers in the range `[1, n]` that do not appear in `nums`_.
///
/// __Constraints:__
///
/// - `n == nums.length`
/// - `1 <= n <= 100_000`
/// - `1 <= nums[i] <= n`
///
/// __Follow up:__ Could you do it without extra space and in `O(n)` runtime?
/// You may assume the returned list does not count as extra space.
///
/// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/
struct Solution;
impl Solution {
    pub fn find_disappeared_numbers_my(mut nums: Vec<i32>) -> Vec<i32> {
        println!("find_disappeared_numbers({:?})", nums);
        let n = nums.len();
        for i in 0..n {
            let t = (i + 1) as i32;
            while nums[i] != t && nums[(nums[i] - 1) as usize] != nums[i] {
                let j = (nums[i] - 1) as usize;
                nums.swap(i, j);
            }
        }
        nums.into_iter()
            .enumerate()
            .filter_map(|(i, n)| match (i + 1) as i32 {
                d if d == n => None,
                d => Some(d),
            })
            .collect()
    }
    /// Approach 2: O(1) Space InPlace Modification Solution
    /// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/solution/
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        println!("find_disappeared_numbers({:?})", nums);
        let n = nums.len();
        for i in 0..n {
            let j = nums[i].abs() as usize - 1;
            if nums[j] > 0 {
                nums[j] = -nums[j];
            }
        }
        nums.into_iter()
            .enumerate()
            .filter_map(|(i, n)| if n < 0 { None } else { Some(i as i32 + 1) })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_1() {
        let n = vec![1];
        let e: Vec<i32> = vec![];
        assert_eq!(Solution::find_disappeared_numbers(n), e);
    }
    #[test]
    fn n_11() {
        let n = vec![1, 1];
        let e = [2];
        assert_eq!(Solution::find_disappeared_numbers(n), e);
    }
    #[test]
    fn n_43278231() {
        let n = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let e = [5, 6];
        assert_eq!(Solution::find_disappeared_numbers(n), e);
    }

    #[test]
    fn n_1_to_100000() {
        let n = (1..=100000).collect();
        let e: Vec<i32> = vec![];
        assert_eq!(Solution::find_disappeared_numbers(n), e);
    }
    #[test]
    fn n_1_repeat_100000() {
        let n = vec![1; 100000];
        let e = (2..=100000).collect::<Vec<_>>();
        assert_eq!(Solution::find_disappeared_numbers(n), e);
    }
}
