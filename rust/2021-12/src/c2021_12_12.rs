#![allow(dead_code)]
/// 416. Partition Equal Subset Sum
/// ===============================
///
/// Given a __non-empty__ array `nums` containing __only positive integers__,
/// find if the array can be partitioned into two subsets such that the sum of elements in both subsets is equal.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 200`
/// - `1 <= nums[i] <= 100`
///
/// https://leetcode.com/problems/partition-equal-subset-sum/
struct Solution;
impl Solution {
    pub fn can_partition_rec(nums: Vec<i32>) -> bool {
        println!("can_partition({:?})", nums);

        fn rec(i: usize, s: i32, nums: &[i32]) -> bool {
            s == 0
                || (s > 0
                    && i < nums.len()
                    && ((nums[i] <= s && rec(i + 1, s - nums[i], nums)) || rec(i + 1, s, nums)))
        }

        let sum: i32 = nums.iter().sum();
        sum % 2 == 0 && rec(0, sum / 2, &nums)
    }
    pub fn can_partition_rec_with_memo(nums: Vec<i32>) -> bool {
        println!("can_partition({:?})", nums);

        fn rec(i: usize, s: usize, nums: &[i32], seen: &mut Vec<Vec<bool>>) -> bool {
            s == 0
                || (i < nums.len() && !seen[i][s] && {
                    seen[i][s] = true;
                    (nums[i] as usize <= s && rec(i + 1, s - nums[i] as usize, nums, seen))
                        || rec(i + 1, s, nums, seen)
                })
        }

        let sum: i32 = nums.iter().sum();
        sum % 2 == 0 && {
            let half = (sum / 2) as usize;
            let mut seen = vec![vec![false; half + 1]; nums.len()];
            rec(0, half, &nums, &mut seen)
        }
    }
    pub fn can_partition_dp_vec_vec(nums: Vec<i32>) -> bool {
        println!("can_partition({:?})", nums);

        let sum: i32 = nums.iter().sum();
        sum % 2 == 0 && {
            let half = (sum / 2) as usize;
            let mut can_partition = vec![vec![false; half + 1]; nums.len() + 1];
            can_partition[nums.len()][0] = true;
            for i in (0..nums.len()).rev() {
                can_partition[i][0] = true;
                let n = nums[i] as usize;
                for s in 1..=half {
                    if s >= n {
                        can_partition[i][s] =
                            can_partition[i + 1][s] || can_partition[i + 1][s - n];
                    } else {
                        can_partition[i][s] = can_partition[i + 1][s];
                    }
                }
            }
            can_partition[0][half]
        }
    }
    pub fn can_partition(nums: Vec<i32>) -> bool {
        println!("can_partition({:?})", nums);

        let sum: i32 = nums.iter().sum();
        sum % 2 == 0 && {
            let half = (sum / 2) as usize;
            let mut can_partition = vec![false; half + 1];
            can_partition[0] = true;
            for n in nums.into_iter().map(|n| n as usize) {
                for s in (n..=half).rev() {
                    can_partition[s] |= can_partition[s - n];
                }
            }
            can_partition[half]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1() { assert!(!Solution::can_partition(vec![1])); }
    #[rustfmt::skip] #[test] fn n_2() { assert!(!Solution::can_partition(vec![2])); }

    #[rustfmt::skip] #[test] fn n_1_1() { assert!(Solution::can_partition(vec![1,1])); }
    #[rustfmt::skip] #[test] fn n_2_2() { assert!(Solution::can_partition(vec![2,2])); }
    #[rustfmt::skip] #[test] fn n_1_2() { assert!(!Solution::can_partition(vec![1,2])); }
    #[rustfmt::skip] #[test] fn n_2_1() { assert!(!Solution::can_partition(vec![2,1])); }

    #[rustfmt::skip] #[test] fn n_1_1_2() { assert!(Solution::can_partition(vec![1,1,2])); }
    #[rustfmt::skip] #[test] fn n_1_2_1() { assert!(Solution::can_partition(vec![1,2,1])); }
    #[rustfmt::skip] #[test] fn n_2_1_1() { assert!(Solution::can_partition(vec![2,1,1])); }
    #[rustfmt::skip] #[test] fn n_1_1_3() { assert!(!Solution::can_partition(vec![1,1,3])); }

    #[test]
    fn n_1_5_11_5() {
        assert!(Solution::can_partition(vec![1, 5, 11, 5]));
        // Output: true
        // Explanation: The array can be partitioned as [1, 5, 5] and [11].
    }
    #[test]
    fn n_1_2_3_5() {
        assert!(!Solution::can_partition(vec![1, 2, 3, 5]));
        // Output: false
        // Explanation: The array cannot be partitioned into equal sum subsets.
    }

    #[test]
    fn n_1_2_5() {
        assert!(!Solution::can_partition(vec![1, 2, 5]));
    }

    #[test]
    fn n_100_repeat_199_appended_98() {
        let mut nums = vec![100; 199];
        nums.push(98);
        assert!(!Solution::can_partition(nums));
    }
}
