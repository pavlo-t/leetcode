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
    /// 21:09-21:13
    pub fn can_partition_rec_1(nums: Vec<i32>) -> bool {
        println!("can_partition({:?})", nums);
        fn rec(i: usize, sum1: i32, sum2: i32, nums: &[i32]) -> bool {
            if i == nums.len() {
                sum1 == sum2
            } else {
                rec(i + 1, sum1 + nums[i], sum2, nums) || rec(i + 1, sum1, sum2 + nums[i], nums)
            }
        }
        rec(0, 0, 0, &nums)
    }
    /// 21:13-21:15
    pub fn can_partition_rec2(nums: Vec<i32>) -> bool {
        println!("can_partition({:?})", nums);
        fn rec(i: usize, sum1: i32, sum2: i32, nums: &[i32]) -> bool {
            if i == nums.len() {
                sum1 == sum2
            } else if sum1 < 0 || sum2 < 0 {
                false
            } else {
                rec(i + 1, sum1 - nums[i], sum2, nums) || rec(i + 1, sum1, sum2 - nums[i], nums)
            }
        }
        let sum: i32 = nums.iter().sum();
        sum % 2 == 0 && rec(0, sum / 2, sum / 2, &nums)
    }
    /// 21:15-21:54
    pub fn can_partition_rec_with_memo_too_much_memory(nums: Vec<i32>) -> bool {
        println!("can_partition({:?})", nums);
        fn rec(i: usize, a: usize, b: usize, nums: &[i32], seen: &mut Vec<Vec<Vec<bool>>>) -> bool {
            if seen[i][a][b] {
                false
            } else {
                let result = if i == nums.len() {
                    a == b
                } else if seen[i][a][b] {
                    false
                } else {
                    let n = nums[i] as usize;
                    (a >= n && !seen[i + 1][a - n][b] && rec(i + 1, a - n, b, nums, seen))
                        || (b >= n && !seen[i + 1][a][b - n] && rec(i + 1, a, b - n, nums, seen))
                };
                seen[i][a][b] = true;
                result
            }
        }
        let sum = nums.iter().sum::<i32>() as usize;
        sum % 2 == 0 && {
            let n = nums.len();
            let half_sum = sum / 2;
            let mut seen = vec![vec![vec![false; half_sum + 1]; half_sum + 1]; n + 1];
            rec(0, half_sum, half_sum, &nums, &mut seen)
        }
    }
    /// 21:54-21:57
    pub fn can_partition_rec3(nums: Vec<i32>) -> bool {
        println!("can_partition({:?})", nums);
        fn rec(i: usize, sum: i32, nums: &[i32]) -> bool {
            sum == 0
                || (i < nums.len() && sum > 0 && {
                    rec(i + 1, sum - nums[i], nums) || rec(i + 1, sum, nums)
                })
        }
        let sum: i32 = nums.iter().sum();
        sum % 2 == 0 && rec(0, sum / 2, &nums)
    }
    /// 21:57-22:11
    pub fn can_partition_rec_with_memo(nums: Vec<i32>) -> bool {
        println!("can_partition({:?})", nums);
        fn rec(i: usize, s: usize, nums: &[i32], seen: &mut Vec<Vec<bool>>) -> bool {
            s == 0
                || (i < nums.len() && s > 0 && {
                    seen[i][s] = true;
                    let n = nums[i] as usize;
                    (s >= n && !seen[i + 1][s - n] && rec(i + 1, s - n, nums, seen))
                        || (!seen[i + 1][s] && rec(i + 1, s, nums, seen))
                })
        }
        let sum = nums.iter().sum::<i32>() as usize;
        sum % 2 == 0 && {
            let n = nums.len();
            let half_sum = sum / 2;
            let mut seen = vec![vec![false; half_sum + 1]; n + 1];
            rec(0, half_sum, &nums, &mut seen)
        }
    }
    pub fn can_partition_dp(nums: Vec<i32>) -> bool {
        println!("can_partition({:?})", nums);
        let sum = nums.iter().sum::<i32>() as usize;
        sum % 2 == 0 && {
            let n = nums.len();
            let half_sum = sum / 2;
            let mut dp = vec![vec![false; half_sum + 1]; n + 1];
            for i in 1..=n {
                dp[i][0] = true;
                let num = nums[i - 1] as usize;
                for s in num..=half_sum {
                    dp[i][s] = dp[i - 1][s - num] || dp[i - 1][s];
                }
                if dp[i][half_sum] {
                    return true;
                }
            }
            false
        }
    }
    pub fn can_partition(nums: Vec<i32>) -> bool {
        println!("can_partition({:?})", nums);
        let sum = nums.iter().sum::<i32>() as usize;
        sum % 2 == 0 && {
            let n = nums.len();
            let half_sum = sum / 2;
            let mut dp = vec![false; half_sum + 1];
            dp[0] = true;
            for i in 0..n {
                let num = nums[i] as usize;
                for s in (num..=half_sum).rev() {
                    dp[s] = dp[s] || dp[s - num];
                }
                if dp[half_sum] {
                    return true;
                }
            }
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1()   { assert!(!Solution::can_partition(vec![1  ])); }
    #[rustfmt::skip] #[test] fn n_1_2() { assert!(!Solution::can_partition(vec![1,2])); }
    #[rustfmt::skip] #[test] fn n_2_1() { assert!(!Solution::can_partition(vec![2,1])); }
    #[rustfmt::skip] #[test] fn n_1_1() { assert!( Solution::can_partition(vec![1,1])); }

    #[rustfmt::skip] #[test] fn n_1_1_1() { assert!(!Solution::can_partition(vec![1,1,1])); }
    #[rustfmt::skip] #[test] fn n_2_1_1() { assert!( Solution::can_partition(vec![2,1,1])); }
    #[rustfmt::skip] #[test] fn n_1_2_1() { assert!( Solution::can_partition(vec![1,2,1])); }
    #[rustfmt::skip] #[test] fn n_1_1_2() { assert!( Solution::can_partition(vec![1,1,2])); }
    #[rustfmt::skip] #[test] fn n_3_1_1() { assert!(!Solution::can_partition(vec![3,1,1])); }
    #[rustfmt::skip] #[test] fn n_1_3_1() { assert!(!Solution::can_partition(vec![1,3,1])); }
    #[rustfmt::skip] #[test] fn n_1_1_3() { assert!(!Solution::can_partition(vec![1,1,3])); }

    #[test]
    fn n_1_5_11_5() {
        let n = vec![1, 5, 11, 5];
        assert!(Solution::can_partition(n));
        // Explanation: The array can be partitioned as [1, 5, 5] and [11].
    }
    #[test]
    fn n_1_2_3_5() {
        let n = vec![1, 2, 3, 5];
        assert!(!Solution::can_partition(n));
        // Explanation: The array cannot be partitioned into equal sum subsets.
    }

    //#[ignore]
    #[test]
    fn n_1to200() {
        let n = (1..=200).collect();
        assert!(Solution::can_partition(n));
    }
    //#[ignore]
    #[test]
    fn n_1_repeat_200() {
        let n = vec![1; 200];
        assert!(Solution::can_partition(n));
    }
    //#[ignore]
    #[test]
    fn n_200_repeat_200() {
        let n = vec![200; 200];
        assert!(Solution::can_partition(n));
    }
    //#[ignore]
    #[test]
    fn n_1_repeat_198_200() {
        let mut n = vec![1; 198];
        n.push(200);
        assert!(!Solution::can_partition(n));
    }
}
