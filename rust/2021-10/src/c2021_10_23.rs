#![allow(dead_code)]
/// 154. Find Minimum in Rotated Sorted Array II
/// ============================================
///
/// Suppose an array of length `n` sorted in ascending order is __rotated__ between `1` and `n` times.
/// For example, the array nums = [0,1,4,4,5,6,7] might become:
///
/// - `[4,5,6,7,0,1,4]` if it was rotated `4` times.
/// - `[0,1,4,4,5,6,7]` if it was rotated `7` times.
///
/// Notice that __rotating__ an array `[a[0], a[1], a[2], ..., a[n-1]]` 1 time
/// results in the array `[a[n-1], a[0], a[1], a[2], ..., a[n-2]]`.
///
/// Given the sorted rotated array `nums` that may contain __duplicates__,
/// return _the minimum element of this array_.
///
/// You must decrease the overall operation steps as much as possible.
///
/// __Constraints:__
///
/// - `n == nums.length`
/// - `1 <= n <= 5000`
/// - `-5000 <= nums[i] <= 5000`
/// - `nums` is sorted and rotated between `1` and `n` times.
///
/// __Follow up:__ This problem is similar to `Find Minimum in Rotated Sorted Array`,
/// but `nums` may contain __duplicates__.
/// Would this affect the runtime complexity? How and why?
///
/// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/
struct Solution;
impl Solution {
    /// Approach 1: Variant of Binary Search
    /// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/solution/
    pub fn find_min(nums: Vec<i32>) -> i32 {
        println!("find_min({:?})", nums);
        use std::cmp::Ordering;
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let m = l + (r - l) / 2;
            match nums[m].cmp(&nums[r]) {
                Ordering::Less => r = m,
                Ordering::Greater => l = m + 1,
                Ordering::Equal => r -= 1,
            };
        }
        nums[l]
    }
    pub fn find_min_my_binary_search(nums: Vec<i32>) -> i32 {
        println!("find_min({:?})", nums);
        use std::cmp::Ordering;
        #[rustfmt::skip]
        fn bs(l: usize, r: usize, ns: &[i32]) -> i32 {
            if l >= r || ns[l] < ns[r] {
                ns[l]
            } else {
                let m = l + (r - l) / 2;
                match ns[m].cmp(&ns[r]) {
                    Ordering::Greater                 => bs(m + 1, r, ns),
                    Ordering::Equal if ns[m] == ns[l] => bs(l, m, ns).min(bs(m + 1, r, ns)),
                    _                                 => bs(l, m, ns),
                }
            }
        }
        bs(0, nums.len() - 1, &nums)
    }
    pub fn find_min_linear(nums: Vec<i32>) -> i32 {
        println!("find_min({:?})", nums);
        nums.into_iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ns_1_3_5() {
        let n = vec![1, 3, 5];
        assert_eq!(Solution::find_min(n), 1);
    }
    #[test]
    fn ns_2_2_2_0_1() {
        let n = vec![2, 2, 2, 0, 1];
        assert_eq!(Solution::find_min(n), 0);
    }
    #[test]
    fn ns_2_2_2_1_2_2_2() {
        let n = vec![2, 2, 2, 1, 2, 2, 2];
        assert_eq!(Solution::find_min(n), 1);
    }
    #[test]
    fn ns_2_1_2_2_2_2_2() {
        let n = vec![2, 1, 2, 2, 2, 2, 2];
        assert_eq!(Solution::find_min(n), 1);
    }
    #[test]
    fn ns_2_2_2_2_2_1_2() {
        let n = vec![2, 2, 2, 2, 2, 1, 2];
        assert_eq!(Solution::find_min(n), 1);
    }

    #[test]
    fn ns_1to5000() {
        let n = (1..=5000).collect();
        assert_eq!(Solution::find_min(n), 1);
    }
    #[test]
    fn ns_5000x3() {
        let n = vec![3; 5000];
        assert_eq!(Solution::find_min(n), 3);
    }
}
