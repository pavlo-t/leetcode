#![allow(dead_code)]
/// 189. Rotate Array
/// =================
///
/// Given an array, rotate the array to the right by `k` steps, where `k` is non-negative.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 100_000`
/// - `-2**31 <= nums[i] <= 2**31 - 1`
/// - `0 <= k <= 100_000`
///
/// __Follow up:__
///
/// - Try to come up with as many solutions as you can.
///   There are at least __three__ different ways to solve this problem.
/// - Could you do it in-place with `O(1)` extra space?
///
/// https://leetcode.com/problems/rotate-array/
struct Solution;
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize;
        let mut result = vec![0; n];
        for (i, &num) in nums.iter().enumerate() {
            let j = (i + k) % n;
            result[j] = num;
        }
        std::mem::swap(nums, &mut result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_1_k_3() {
        let mut n = vec![1];
        Solution::rotate(&mut n, 3);
        assert_eq!(n, [1]);
    }
    #[test]
    fn n_1_2_3_k_3() {
        let mut n = vec![1, 2, 3];
        Solution::rotate(&mut n, 3);
        assert_eq!(n, [1, 2, 3]);
    }

    #[test]
    fn n_1_2_3_4_5_6_7_k_3() {
        let mut n = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut n, 3);
        assert_eq!(n, [5, 6, 7, 1, 2, 3, 4]);
        // Explanation:
        // rotate 1 steps to the right: [7,1,2,3,4,5,6]
        // rotate 2 steps to the right: [6,7,1,2,3,4,5]
        // rotate 3 steps to the right: [5,6,7,1,2,3,4]
    }
    #[test]
    fn n_m1m100p3p99_k_2() {
        let mut n = vec![-1, -100, 3, 99];
        Solution::rotate(&mut n, 2);
        assert_eq!(n, [3, 99, -1, -100]);
        // Explanation:
        // rotate 1 steps to the right: [99,-1,-100,3]
        // rotate 2 steps to the right: [3,99,-1,-100]
    }

    #[test]
    fn n_1_to_100_000_k_12345() {
        let mut n = (1..=100000).collect();
        Solution::rotate(&mut n, 12345);
        let e = (87656..=100000).chain(1..=87655).collect::<Vec<_>>();
        assert_eq!(n, e);
    }
}
