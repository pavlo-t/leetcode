#![allow(dead_code)]
/// 35. Search Insert Position
/// ==========================
///
/// Given a sorted array of distinct integers and a target value, return the index if the target is found.
/// If not, return the index where it would be if it were inserted in order.
///
/// You must write an algorithm with `O(log n)` runtime complexity.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 10_000`
/// - `-10_000 <= nums[i] <= 10_000`
/// - `nums` contains __distinct__ values sorted in __ascending__ order.
/// - `-10_000 <= target <= 10_000`
///
/// https://leetcode.com/problems/search-insert-position/
struct Solution;
impl Solution {
    pub fn search_insert_rust_builtin(nums: Vec<i32>, target: i32) -> i32 {
        println!("search_insert({:?}, {})", nums, target);
        (match nums.binary_search(&target) {
            Ok(i) => i,
            Err(i) => i,
        }) as i32
    }
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        println!("search_insert({:?}, {})", nums, target);
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let m = l + (r - l) / 2;
            if nums[m] == target {
                return m as i32;
            } else if nums[m] < target {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        l as i32 + (nums[l] < target) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n1_t0() {
        let n = vec![1];
        let t = 0;
        assert_eq!(Solution::search_insert(n, t), 0);
    }
    #[test]
    fn n1356_t5() {
        let n = vec![1, 3, 5, 6];
        let t = 5;
        assert_eq!(Solution::search_insert(n, t), 2);
    }
    #[test]
    fn n1356_t2() {
        let n = vec![1, 3, 5, 6];
        let t = 2;
        assert_eq!(Solution::search_insert(n, t), 1);
    }
    #[test]
    fn n1356_t7() {
        let n = vec![1, 3, 5, 6];
        let t = 7;
        assert_eq!(Solution::search_insert(n, t), 4);
    }
    #[test]
    fn n1356_t0() {
        let n = vec![1, 3, 5, 6];
        let t = 0;
        assert_eq!(Solution::search_insert(n, t), 0);
    }
}
