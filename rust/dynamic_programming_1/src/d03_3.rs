#![allow(dead_code)]
/// 740. Delete and Earn
/// ====================
///
/// You are given an integer array `nums`.
/// You want to maximize the number of points you get by performing the following operation any number of times:
///
/// Pick any `nums[i]` and delete it to earn `nums[i]` points.
/// Afterwards, you must delete __every__ element equal to `nums[i] - 1` and __every__ element equal to `nums[i] + 1`.
///
/// Return _the __maximum number of points__ you can earn by applying the above operation some number of times_.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 20_000`
/// - `1 <= nums[i] <= 10_000`
///
/// https://leetcode.com/problems/delete-and-earn/
struct Solution;
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let ns = Self::preprocess_nums(nums);

        let (mut a, mut b) = (0, 0);
        for n in ns {
            std::mem::swap(&mut a, &mut b);
            a = b.max(a + n);
        }
        a
    }
    fn preprocess_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut max_n = 0;
        let mut ns = vec![0; 10001];
        for &n in nums.iter() {
            ns[n as usize] += n;
            max_n = max_n.max(n);
        }
        ns.truncate(max_n as usize + 1);
        ns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_3_4_2() {
        let nums = vec![3, 4, 2];
        assert_eq!(Solution::delete_and_earn(nums), 6);
        // Explanation: You can perform the following operations:
        // - Delete 4 to earn 4 points. Consequently, 3 is also deleted. nums = [2].
        // - Delete 2 to earn 2 points. nums = [].
        // You earn a total of 6 points.
    }
    #[test]
    fn n_2_2_3_3_3_4() {
        let nums = vec![2, 2, 3, 3, 3, 4];
        assert_eq!(Solution::delete_and_earn(nums), 9);
        // Explanation: You can perform the following operations:
        // - Delete a 3 to earn 3 points. All 2's and 4's are also deleted. nums = [3,3].
        // - Delete a 3 again to earn 3 points. nums = [3].
        // - Delete a 3 once more to earn 3 points. nums = [].
        // You earn a total of 9 points.
        // (9, 3) (4, 4) (4, 2)
    }
    #[test]
    fn n_3() {
        let nums = vec![3];
        assert_eq!(Solution::delete_and_earn(nums), 3);
    }
    #[test]
    fn n_1_2() {
        let nums = vec![1, 2];
        assert_eq!(Solution::delete_and_earn(nums), 2);
    }
    #[test]
    fn n_2_1() {
        let nums = vec![2, 1];
        assert_eq!(Solution::delete_and_earn(nums), 2);
    }

    #[test]
    fn n_1to10000x2() {
        let nums = (1..=10000).chain(1..=10000).collect();
        assert_eq!(Solution::delete_and_earn(nums), 50_010_000);
    }
}
