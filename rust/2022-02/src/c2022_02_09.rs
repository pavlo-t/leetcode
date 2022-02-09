#![allow(dead_code)]
/// 532. K-diff Pairs in an Array
/// =============================
///
/// Given an array of integers `nums` and an integer `k`,
/// return _the number of __unique__ k-diff pairs in the array_.
///
/// A __k-diff__ pair is an integer pair `(nums[i], nums[j])`, where the following are true:
///
/// - `0 <= i < j < nums.length`
/// - `|nums[i] - nums[j]| == k`
///
/// __Notice__ that `|val|` denotes the absolute value of `val`.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 10_000`
/// - `-100_000_000 <= nums[i] <= 100_000_000`
/// - `0 <= k <= 100_000_000`
///
/// https://leetcode.com/problems/k-diff-pairs-in-an-array/
struct Solution;
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() < 2 {
            0
        } else if k == 0 {
            use std::collections::HashMap;
            nums.into_iter()
                .fold((0, HashMap::new()), |(rsf, mut counts), n| {
                    let count = counts.entry(n).or_insert(0);
                    *count += 1;
                    (rsf + (count == &2) as i32, counts)
                })
                .0
        } else {
            use std::collections::HashSet;
            let nums = {
                let mut vec = nums
                    .into_iter()
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>();
                vec.sort_unstable();
                vec
            };

            let (mut l, mut r) = (0, 1);
            let mut result = 0;
            while r < nums.len() {
                match (nums[r] - nums[l] - k).signum() {
                    0 => {
                        result += 1;
                        l += 1;
                        r += 1;
                    },
                    1 => l += 1,
                    _ => r += 1,
                };
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_2_1_k_1() {
        let n = vec![2, 1];
        assert_eq!(Solution::find_pairs(n, 1), 1);
    }
    #[test]
    fn n_3_1_4_1_5_k_2() {
        let n = vec![3, 1, 4, 1, 5];
        assert_eq!(Solution::find_pairs(n, 2), 2);
        // Explanation: There are two 2-diff pairs in the array, (1, 3) and (3, 5).
        // Although we have two 1s in the input, we should only return the number of unique pairs.
    }
    #[test]
    fn n_1_2_3_4_5_k_1() {
        let n = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::find_pairs(n, 1), 4);
        // Explanation: There are four 1-diff pairs in the array, (1, 2), (2, 3), (3, 4) and (4, 5).
    }
    #[test]
    fn n_1_3_1_5_4_k_0() {
        let n = vec![1, 3, 1, 5, 4];
        assert_eq!(Solution::find_pairs(n, 0), 1);
        // Explanation: There is one 0-diff pair in the array, (1, 1).
    }
}
