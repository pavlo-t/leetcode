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
    pub fn delete_and_earn_brute_force_recursion(nums: Vec<i32>) -> i32 {
        println!("delete_and_earn({nums:?})");
        use std::collections::{HashMap, HashSet};

        fn rec(i: usize, nums_sorted: &[i32], counts: &HashMap<i32, i32>) -> i32 {
            if i >= nums_sorted.len() {
                0
            } else {
                let n = nums_sorted[i];
                if i == nums_sorted.len() - 1 || nums_sorted[i + 1] > n + 1 {
                    n * counts.get(&n).unwrap() + rec(i + 1, nums_sorted, counts)
                } else {
                    let take = n * counts.get(&n).unwrap() + rec(i + 2, nums_sorted, counts);
                    let skip = rec(i + 1, nums_sorted, counts);
                    take.max(skip)
                }
            }
        }

        let (nums_sorted, counts) = {
            let mut set = HashSet::new();
            let mut counts = HashMap::new();
            for n in nums {
                set.insert(n);
                *counts.entry(n).or_insert(0) += 1;
            }
            let mut vec = set.into_iter().collect::<Vec<_>>();
            vec.sort_unstable();
            (vec, counts)
        };

        rec(0, &nums_sorted, &counts)
    }

    pub fn delete_and_earn_dp_vec(nums: Vec<i32>) -> i32 {
        println!("delete_and_earn({nums:?})");
        use std::collections::{HashMap, HashSet};

        let (nums_sorted, counts) = {
            let mut set = HashSet::new();
            let mut counts = HashMap::new();
            for n in nums {
                set.insert(n);
                *counts.entry(n).or_insert(0) += 1;
            }
            let mut vec = set.into_iter().collect::<Vec<_>>();
            vec.sort_unstable();
            (vec, counts)
        };

        let n = nums_sorted.len();
        let mut dp = vec![0; n + 2];
        for i in (0..n).rev() {
            let num = nums_sorted[i];
            dp[i] = if i == n - 1 || nums_sorted[i + 1] > num + 1 {
                num * counts.get(&num).unwrap() + dp[i + 1]
            } else {
                let take = num * counts.get(&num).unwrap() + dp[i + 2];
                let skip = dp[i + 1];
                take.max(skip)
            };
        }
        dp[0]
    }

    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        use std::collections::{HashMap, HashSet};

        let (nums_sorted, counts) = {
            let mut set = HashSet::new();
            let mut counts = HashMap::new();
            for n in nums {
                set.insert(n);
                *counts.entry(n).or_insert(0) += 1;
            }
            let mut vec = set.into_iter().collect::<Vec<_>>();
            vec.sort_unstable();
            (vec, counts)
        };

        let n = nums_sorted.len();
        let (mut curr, mut next) = (0, 0);
        for i in (0..n).rev() {
            let num = nums_sorted[i];
            next = if i == n - 1 || nums_sorted[i + 1] > num + 1 {
                num * counts.get(&num).unwrap() + curr
            } else {
                let take = num * counts.get(&num).unwrap() + next;
                let skip = curr;
                take.max(skip)
            };
            std::mem::swap(&mut curr, &mut next);
        }
        curr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1() { assert_eq!(Solution::delete_and_earn(vec![1]), 1); }
    #[rustfmt::skip] #[test] fn n_1_1() { assert_eq!(Solution::delete_and_earn(vec![1, 1]), 2); }
    #[rustfmt::skip] #[test] fn n_1_2() { assert_eq!(Solution::delete_and_earn(vec![1, 2]), 2); }
    #[rustfmt::skip] #[test] fn n_1_3() { assert_eq!(Solution::delete_and_earn(vec![1, 3]), 4); }
    #[rustfmt::skip] #[test] fn n_1_2_3() { assert_eq!(Solution::delete_and_earn(vec![1, 2, 3]), 4); }

    #[test]
    fn n_3_4_2() {
        let n = vec![3, 4, 2];
        assert_eq!(Solution::delete_and_earn(n), 6);
        // Explanation: You can perform the following operations:
        // - Delete 4 to earn 4 points. Consequently, 3 is also deleted. nums = [2].
        // - Delete 2 to earn 2 points. nums = [].
        // You earn a total of 6 points.
    }
    #[test]
    fn n_2_2_3_3_3_4() {
        let n = vec![2, 2, 3, 3, 3, 4];
        assert_eq!(Solution::delete_and_earn(n), 9);
        // Explanation: You can perform the following operations:
        // - Delete a 3 to earn 3 points. All 2's and 4's are also deleted. nums = [3,3].
        // - Delete a 3 again to earn 3 points. nums = [3].
        // - Delete a 3 once more to earn 3 points. nums = [].
        // You earn a total of 9 points.
    }

    /// Got stack overflow with default stack
    #[test]
    fn n_1to20000() {
        let child = std::thread::Builder::new()
            .stack_size(2usize.pow(31))
            .spawn(|| {
                let n = (1..=20000).collect::<Vec<_>>();
                assert_eq!(Solution::delete_and_earn(n), 100_010_000);
            })
            .unwrap();
        child.join().unwrap();
    }
    #[test]
    fn n_1_repeat_20000() {
        let n = vec![1; 20000];
        assert_eq!(Solution::delete_and_earn(n), 20000);
    }
}
