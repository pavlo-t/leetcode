#![allow(dead_code)]
/// 377. Combination Sum IV
/// =======================
///
/// Given an array of __distinct__ integers `nums` and a target integer `target`,
/// return _the number of possible combinations that add up to `target`_.
///
/// The answer is __guaranteed__ to fit in a __32-bit__ integer.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 200`
/// - `1 <= nums[i] <= 1000`
/// - All the elements of `nums` are __unique__.
/// - `1 <= target <= 1000`
///
/// __Follow up: __ What if negative numbers are allowed in the given array?
/// How does it change the problem?
/// What limitation we need to add to the question to allow negative numbers?
///
/// https://leetcode.com/problems/combination-sum-iv/
struct Solution;
impl Solution {
    /// 21:50-21:56
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        println!("combination_sum4({:?}, {})", nums, target);
        let target = target as usize;
        let mut dp = vec![0; target + 1];
        dp[0] = 1;
        for t in 1..=target {
            for &n in &nums {
                if n <= t as i32 {
                    dp[t] += dp[t - n as usize];
                }
            }
        }
        dp[target]
    }
    /// 21:44-21:50
    pub fn combination_sum4_rec_with_memo(nums: Vec<i32>, target: i32) -> i32 {
        println!("combination_sum4({:?}, {})", nums, target);
        fn rec(t: i32, ns: &[i32], memo: &mut Vec<i32>) -> i32 {
            let ti = t as usize;
            if t == 0 {
                1
            } else if memo[ti] >= 0 {
                memo[ti]
            } else {
                let mut result = 0;
                for &n in ns {
                    if n <= t {
                        result += rec(t - n, ns, memo);
                    }
                }
                memo[ti] = result;
                memo[ti]
            }
        }
        let mut memo = vec![-1; target as usize + 1];
        rec(target, &nums, &mut memo)
    }
    /// 21:25-21:44
    pub fn combination_sum4_rec(nums: Vec<i32>, target: i32) -> i32 {
        println!("combination_sum4({:?}, {})", nums, target);
        fn rec(t: i32, ns: &[i32]) -> i32 {
            if t == 0 {
                1
            } else {
                let mut result = 0;
                for &n in ns {
                    if n <= t {
                        result += rec(t - n, ns);
                    }
                }
                result
            }
        }
        rec(target, &nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_9_t_3() { assert_eq!(Solution::combination_sum4(vec![9], 3), 0); }

    #[rustfmt::skip] #[test] fn n_2_t_1() { assert_eq!(Solution::combination_sum4(vec![2], 1), 0); }
    #[rustfmt::skip] #[test] fn n_1_t_1() { assert_eq!(Solution::combination_sum4(vec![1], 1), 1); }
    #[rustfmt::skip] #[test] fn n_1_t_2() { assert_eq!(Solution::combination_sum4(vec![1], 2), 1); }
    #[rustfmt::skip] #[test] fn n_1_t_3() { assert_eq!(Solution::combination_sum4(vec![1], 3), 1); }
    #[rustfmt::skip] #[test] fn n_1_t_4() { assert_eq!(Solution::combination_sum4(vec![1], 4), 1); }

    #[rustfmt::skip] #[test] fn n_1_2_t_1() { assert_eq!(Solution::combination_sum4(vec![1,2], 1), 1); }
    #[rustfmt::skip] #[test] fn n_1_2_t_2() { assert_eq!(Solution::combination_sum4(vec![1,2], 2), 2); }
    #[rustfmt::skip] #[test] fn n_1_2_t_3() { assert_eq!(Solution::combination_sum4(vec![1,2], 3), 3); } //111 12 21
    #[rustfmt::skip] #[test] fn n_1_2_t_4() { assert_eq!(Solution::combination_sum4(vec![1,2], 4), 5); } //1111 112 121 211 22

    #[rustfmt::skip] #[test] fn n_1_2_3_t_1() { assert_eq!(Solution::combination_sum4(vec![1,2,3], 1), 1); }
    #[rustfmt::skip] #[test] fn n_1_2_3_t_2() { assert_eq!(Solution::combination_sum4(vec![1,2,3], 2), 2); }
    #[rustfmt::skip] #[test] fn n_1_2_3_t_3() { assert_eq!(Solution::combination_sum4(vec![1,2,3], 3), 4); } //111 12 21 3
    #[rustfmt::skip] #[test] fn n_1_2_3_t_4() { assert_eq!(Solution::combination_sum4(vec![1,2,3], 4), 7);
        // 1111 112 121 13 211 22 31
        // Note that different sequences are counted as different combinations.
    }

    //#[ignore]
    #[test]
    fn n_1_2_t_45() {
        assert_eq!(Solution::combination_sum4((1..=2).collect(), 45), 1_836_311_903);
    }
    //#[ignore]
    #[test]
    fn n_801to1000_t_1000() {
        assert_eq!(Solution::combination_sum4((801..=1000).collect(), 1000), 1);
    }
}
