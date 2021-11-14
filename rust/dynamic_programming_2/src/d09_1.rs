#![allow(dead_code)]
/// 673. Number of Longest Increasing Subsequence
/// =============================================
///
/// Given an integer array `nums`, return _the number of longest increasing subsequences_.
///
/// __Notice__ that the sequence has to be __strictly__ increasing.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 2000`
/// - `-1_000_000 <= nums[i] <= 1_000_000`
///
/// https://leetcode.com/problems/number-of-longest-increasing-subsequence/
struct Solution;
impl Solution {
    /// From my solution in `scala` from a year ago:
    pub fn find_number_of_lis_old_dp(nums: Vec<i32>) -> i32 {
        println!("find_number_of_lis({:?})", nums);
        let mut dp = vec![[0, 1]; nums.len()];
        let mut longest = 0;
        for r in 0..nums.len() {
            for l in 0..r {
                if nums[l] < nums[r] {
                    if dp[l][0] >= dp[r][0] {
                        dp[r][0] = dp[l][0] + 1;
                        dp[r][1] = dp[l][1];
                    } else if dp[l][0] + 1 == dp[r][0] {
                        dp[r][1] = dp[r][1] + dp[l][1];
                    }
                }
            }
            longest = longest.max(dp[r][0]);
        }
        dp.into_iter()
            .filter(|[l, _]| l == &longest)
            .map(|[_, count]| count)
            .fold(0, |rsf, c| rsf.wrapping_add(c))
    }
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        println!("find_number_of_lis({:?})", nums);
        fn rec(i: usize, p: usize, ns: &[i32]) -> (i32, i32) {
            if i == ns.len() {
                (0, 1)
            } else if p == 0 || ns[p - 1] < ns[i] {
                let (skip_len, skip_count) = rec(i + 1, p, ns);
                let (pick_len, pick_count) = rec(i + 1, i + 1, ns);
                if skip_len >= pick_len {
                    (skip_len + 1, skip_count)
                } else if skip_len + 1 == pick_len {
                    (pick_len, pick_count)
                } else {
                    (0, 1)
                }
            } else {
                //let (skip_len, skip_count) = rec(i + 1, p, ns);
                //(skip_len, skip_count + 1)
                (0, 1)
            }
        }

        let results = (0..nums.len()).map(|i| rec(i, 0, &nums)).collect::<Vec<_>>();
        let longest = results.iter().map(|&(l, _)| l).max().unwrap();
        results.into_iter().filter(|(l, _)| l == &longest).map(|(_, c)| c).sum()
    }
    pub fn find_number_of_lis_12(nums: Vec<i32>) -> i32 {
        println!("find_number_of_lis({:?})", nums);
        fn rec(i: usize, p: usize, ns: &[i32], memo: &mut Vec<Vec<(i32, i32)>>) -> (i32, i32) {
            if memo[i][p].0 != -1 {
                memo[i][p]
            } else {
                memo[i][p] = if i == ns.len() {
                    (0, 1)
                } else if p == 0 || ns[p - 1] < ns[i] {
                    let (skip_len, skip_count) = rec(i + 1, p, ns, memo);
                    let (pick_len, pick_count) = rec(i + 1, i + 1, ns, memo);
                    let pick_len = 1 + pick_len;
                    if skip_len >= pick_len {
                        (skip_len, skip_count + 1)
                    } else {
                        (pick_len, pick_count)
                    }
                } else {
                    let (skip_len, skip_count) = rec(i + 1, p, ns, memo);
                    (skip_len, skip_count + 1)
                };
                memo[i][p]
            }
        }

        let n = nums.len();
        let mut memo = vec![vec![(-1, -1); n + 1]; n + 1];
        rec(0, 0, &nums, &mut memo).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1()  { assert_eq!(Solution::find_number_of_lis(vec![1  ]), 1); }
    #[rustfmt::skip] #[test] fn n_11() { assert_eq!(Solution::find_number_of_lis(vec![1,1]), 2); }
    #[rustfmt::skip] #[test] fn n_21() { assert_eq!(Solution::find_number_of_lis(vec![2,1]), 2); }
    #[rustfmt::skip] #[test] fn n_12() { assert_eq!(Solution::find_number_of_lis(vec![1,2]), 1); }
    #[test]
    fn n_13547() {
        let n = vec![1, 3, 5, 4, 7];
        assert_eq!(Solution::find_number_of_lis(n), 2);
        // Explanation: The two longest increasing subsequences are [1, 3, 4, 7] and [1, 3, 5, 7].
    }
    #[test]
    fn n_22222() {
        let n = vec![2, 2, 2, 2, 2];
        assert_eq!(Solution::find_number_of_lis(n), 5);
        // Explanation: The length of longest continuous increasing subsequence is 1,
        // and there are 5 subsequences' length is 1, so output 5.
    }
    #[test]
    fn n_123123123() {
        let n = vec![1, 2, 3, 1, 2, 3, 1, 2, 3];
        assert_eq!(Solution::find_number_of_lis(n), 10);
    }

    #[ignore]
    #[test]
    fn n_1_repeat_2000() {
        let n = vec![1; 2000];
        assert_eq!(Solution::find_number_of_lis(n), 2000);
    }
    #[ignore]
    #[test]
    fn n_1_to_2000() {
        let n = (1..=2000).collect();
        assert_eq!(Solution::find_number_of_lis(n), 1);
    }
    #[ignore]
    #[test]
    fn n_1_to_5_repeat_400_produces_i32_overflow() {
        let n = (0..2000).map(|i| i % 5).collect();
        assert_eq!(Solution::find_number_of_lis(n), 1);
    }
}
