#![allow(dead_code)]
/// 376. Wiggle Subsequence
/// =======================
///
/// A __wiggle sequence__ is a sequence where the differences between successive numbers
/// strictly alternate between positive and negative.
/// The first difference (if one exists) may be either positive or negative.
/// A sequence with one element and a sequence with two non-equal elements are trivially wiggle sequences.
///
/// - For example, `[1, 7, 4, 9, 2, 5]` is a __wiggle sequence__ because
///   the differences `(6, -3, 5, -7, 3)` alternate between positive and negative.
/// - In contrast, `[1, 4, 7, 2, 5]` and `[1, 7, 4, 5, 5]` are not wiggle sequences.
///   The first is not because its first two differences are positive,
///   and the second is not because its last difference is zero.
///
/// A __subsequence__ is obtained by deleting some elements (possibly zero) from the original sequence,
/// leaving the remaining elements in their original order.
///
/// Given an integer array `nums`, return _the length of the longest __wiggle subsequence__ of `nums`_.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 1000`
/// - `0 <= nums[i] <= 1000`
///
/// __Follow up:__ Could you solve this in `O(n)` time?
///
/// https://leetcode.com/problems/wiggle-subsequence/
struct Solution;
impl Solution {
    /// Approach #5 Greedy Approach
    /// https://leetcode.com/problems/wiggle-subsequence/solution/
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        println!("wiggle_max_length({:?})", nums);
        if nums.len() < 2 {
            1
        } else {
            let mut prevdiff = nums[1] - nums[0];
            let mut count = if prevdiff != 0 { 2 } else { 1 };
            for i in 2..nums.len() {
                let diff = nums[i] - nums[i - 1];
                if (diff > 0 && prevdiff <= 0) || (diff < 0 && prevdiff >= 0) {
                    count += 1;
                    prevdiff = diff;
                }
            }
            count
        }
    }
    /// Approach #4 Space-Optimized Dynamic Programming
    /// https://leetcode.com/problems/wiggle-subsequence/solution/
    pub fn wiggle_max_length_leetcode_linear_dp_optimized(nums: Vec<i32>) -> i32 {
        println!("wiggle_max_length({:?})", nums);
        let n = nums.len();
        let (mut up, mut down) = (1, 1);
        for i in (0..n - 1).rev() {
            if nums[i] < nums[i + 1] {
                up = down + 1;
            } else if nums[i] > nums[i + 1] {
                down = up + 1;
            }
        }
        up.max(down)
    }
    /// Approach #3 Linear Dynamic Programming
    /// https://leetcode.com/problems/wiggle-subsequence/solution/
    pub fn wiggle_max_length_leetcode_linear_dp(nums: Vec<i32>) -> i32 {
        println!("wiggle_max_length({:?})", nums);
        let n = nums.len();
        let mut dp = vec![(1, 1); n];
        for i in (0..n - 1).rev() {
            let (prev_u, prev_d) = dp[i + 1];
            if nums[i] < nums[i + 1] {
                dp[i] = (prev_d + 1, prev_d);
            } else if nums[i] > nums[i + 1] {
                dp[i] = (prev_u, prev_u + 1);
            } else {
                dp[i] = (prev_u, prev_d);
            }
        }
        dp[0].0.max(dp[0].1)
    }
    pub fn wiggle_max_length_leetcode_rec_with_memo_to_dp_vec(nums: Vec<i32>) -> i32 {
        println!("wiggle_max_length({:?})", nums);
        let n = nums.len();
        let mut dp = vec![(1, 1); n];
        for i in (0..n - 1).rev() {
            let (mut up, mut down) = (0, 0);
            for j in i + 1..n {
                if nums[j] > nums[i] {
                    up = up.max(1 + dp[j].1);
                } else if nums[j] < nums[i] {
                    down = down.max(1 + dp[j].0);
                }
            }
            dp[i] = (up, down);
        }
        dp[0].0.max(dp[0].1)
    }
    pub fn wiggle_max_length_leetcode_rec_with_memo(nums: Vec<i32>) -> i32 {
        println!("wiggle_max_length({:?})", nums);
        fn calculate(idx: usize, is_up: bool, ns: &[i32], memo: &mut Vec<Vec<i32>>) -> i32 {
            let is_up_i = is_up as usize;
            if memo[idx][is_up_i] >= 0 {
                memo[idx][is_up_i]
            } else {
                let mut maxcount = 0;
                for i in idx + 1..ns.len() {
                    if (is_up && ns[i] > ns[idx]) || (!is_up && ns[i] < ns[idx]) {
                        maxcount = maxcount.max(1 + calculate(i, !is_up, ns, memo));
                    }
                }
                memo[idx][is_up_i] = maxcount;
                maxcount
            }
        }
        if nums.len() < 2 {
            1
        } else {
            let mut memo = vec![vec![-1; 2]; nums.len()];
            1 + calculate(0, true, &nums, &mut memo).max(calculate(0, false, &nums, &mut memo))
        }
    }
    /// Approach #1 Brute Force - recursion
    /// https://leetcode.com/problems/wiggle-subsequence/solution/
    pub fn wiggle_max_length_leetcode_rec(nums: Vec<i32>) -> i32 {
        println!("wiggle_max_length({:?})", nums);
        fn calculate(idx: usize, is_up: bool, ns: &[i32]) -> i32 {
            let mut maxcount = 0;
            for i in idx + 1..ns.len() {
                if (is_up && ns[i] > ns[idx]) || (!is_up && ns[i] < ns[idx]) {
                    maxcount = maxcount.max(1 + calculate(i, !is_up, ns));
                }
            }
            maxcount
        }
        if nums.len() < 2 {
            1
        } else {
            1 + calculate(0, true, &nums).max(calculate(0, false, &nums))
        }
    }
    /// 22:30-22:32
    pub fn wiggle_max_length_dp_vec_tuple(nums: Vec<i32>) -> i32 {
        println!("wiggle_max_length({:?})", nums);
        let n = nums.len();
        let mut dp = vec![(0, 0, 0); n];
        for i in (0..n).rev() {
            for p in 0..=i {
                let (skip_0, skip_d, skip_u) = dp[p];
                let (_, pick_d, pick_u) = dp[i];
                let skip = skip_0.max(1 + pick_d.max(pick_u));
                let down = skip_d.max(if nums[p] < nums[i] { 1 + pick_u } else { 0 });
                let up = skip_u.max(if nums[p] > nums[i] { 1 + pick_d } else { 0 });
                dp[p] = (skip, down, up);
            }
        }
        dp[0].0
    }
    /// 22:22-22:30
    pub fn wiggle_max_length_dp_vec_vec_tuple(nums: Vec<i32>) -> i32 {
        println!("wiggle_max_length({:?})", nums);
        let n = nums.len();
        let mut dp = vec![vec![(0, 0, 0); n]; n + 1];
        for i in (0..n).rev() {
            for p in 0..=i {
                let (skip_0, skip_d, skip_u) = dp[i + 1][p];
                let (_, pick_d, pick_u) = dp[i + 1][i];
                let skip = skip_0.max(1 + pick_d.max(pick_u));
                let down = skip_d.max(if nums[p] < nums[i] { 1 + pick_u } else { 0 });
                let up = skip_u.max(if nums[p] > nums[i] { 1 + pick_d } else { 0 });
                dp[i][p] = (skip, down, up);
            }
        }
        dp[0][0].0
    }
    /// 22:06-22:22
    pub fn wiggle_max_length_dp_vec_vec_vec(nums: Vec<i32>) -> i32 {
        println!("wiggle_max_length({:?})", nums);
        let n = nums.len();
        let mut dp = vec![vec![vec![0; 3]; n]; n + 1];
        for i in (0..n).rev() {
            for p in 0..=i {
                dp[i][p][0] = dp[i + 1][p][0].max(1 + dp[i + 1][i][1].max(dp[i + 1][i][2]));
                dp[i][p][1] = if nums[p] < nums[i] {
                    dp[i + 1][p][1].max(1 + dp[i + 1][i][2])
                } else {
                    dp[i + 1][p][1]
                };
                dp[i][p][2] = if nums[p] > nums[i] {
                    dp[i + 1][p][2].max(1 + dp[i + 1][i][1])
                } else {
                    dp[i + 1][p][2]
                }
            }
        }
        println!(" dp({:?})", dp);
        dp[0][0][0]
    }
    /// 22:02-22:06
    pub fn wiggle_max_length_rec_with_memo(nums: Vec<i32>) -> i32 {
        println!("wiggle_max_length({:?})", nums);
        fn rec(i: usize, p: usize, d: usize, ns: &[i32], memo: &mut Vec<Vec<Vec<i32>>>) -> i32 {
            if i == ns.len() {
                0
            } else if memo[i][p][d] >= 0 {
                memo[i][p][d]
            } else {
                // d: 0 - not picked yet, 1 - need lt, 2 - need gt
                memo[i][p][d] = rec(i + 1, p, d, ns, memo).max(match d {
                    0 => 1 + rec(i + 1, i, 1, ns, memo).max(rec(i + 1, i, 2, ns, memo)),
                    1 if ns[i] < ns[p] => 1 + rec(i + 1, i, 2, ns, memo),
                    2 if ns[i] > ns[p] => 1 + rec(i + 1, i, 1, ns, memo),
                    _ => i32::MIN,
                });
                memo[i][p][d]
            }
        }
        let n = nums.len();
        let mut memo = vec![vec![vec![-1; 3]; n]; n];
        rec(0, 0, 0, &nums, &mut memo)
    }
    /// 21:44-22:02
    pub fn wiggle_max_length_rec(nums: Vec<i32>) -> i32 {
        println!("wiggle_max_length({:?})", nums);
        fn rec(i: usize, p: usize, d: usize, ns: &[i32]) -> i32 {
            if i == ns.len() {
                0
            } else {
                // d: 0 - not picked yet, 1 - need lt, 2 - need gt
                rec(i + 1, p, d, ns).max(match d {
                    0 => 1 + rec(i + 1, i, 1, ns).max(rec(i + 1, i, 2, ns)),
                    1 if ns[i] < ns[p] => 1 + rec(i + 1, i, 2, ns),
                    2 if ns[i] > ns[p] => 1 + rec(i + 1, i, 1, ns),
                    _ => i32::MIN,
                })
            }
        }
        rec(0, usize::MAX, 0, &nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ns_1_7_4_9_2_5() {
        let n = vec![1, 7, 4, 9, 2, 5];
        assert_eq!(Solution::wiggle_max_length(n), 6);
        // Explanation: The entire sequence is a wiggle sequence with differences (6, -3, 5, -7, 3).
    }
    #[test]
    fn ns_1_17_5_10_13_15_10_5_16_8() {
        let n = vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8];
        assert_eq!(Solution::wiggle_max_length(n), 7);
        // Explanation: There are several subsequences that achieve this length.
        // One is [1, 17, 10, 13, 10, 16, 8] with differences (16, -7, 3, -3, 6, -8).
    }
    #[test]
    fn ns_1_2_3_4_5_6_7_8_9() {
        let n = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(Solution::wiggle_max_length(n), 2);
    }

    //#[ignore]
    #[test]
    fn ns_1to1000() {
        let n = (1..=1000).collect();
        assert_eq!(Solution::wiggle_max_length(n), 2);
    }
    //#[ignore]
    #[test]
    fn ns_1000x1() {
        let n = vec![1; 1000];
        assert_eq!(Solution::wiggle_max_length(n), 1);
    }
    //#[ignore]
    #[test]
    fn ns_1_2_repeat_500() {
        let mut n = vec![];
        for _ in 0..500 {
            n.push(1);
            n.push(2);
        }
        assert_eq!(Solution::wiggle_max_length(n), 1000);
    }
}
