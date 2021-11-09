#![allow(dead_code)]
/// 376. Wiggle Subsequence
/// =======================
///
/// A __wiggle sequence__ is a sequence where the differences between successive numbers
/// strictly alternate between positive and negative.
/// The first difference (if one exists) may be either positive or negative.
/// A sequence with one element and a sequence with two non-equal elements are trivially wiggle sequences.
///
/// For example, `[1, 7, 4, 9, 2, 5]` is a __wiggle sequence__
/// because the differences `(6, -3, 5, -7, 3)` alternate between positive and negative.
///
/// In contrast, `[1, 4, 7, 2, 5]` and `[1, 7, 4, 5, 5]` are not wiggle sequences.
/// The first is not because its first two differences are positive,
/// and the second is not because its last difference is zero.
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
    /// 23:17-23:52: solving the wrong task - assumed that the sequence should be consecutive
    pub fn wiggle_max_length_my_carelesness(nums: Vec<i32>) -> i32 {
        println!("wiggle_max_length({:?})", nums);
        fn rec(i: usize, ns: &[i32]) -> (i32, i32) {
            if i == ns.len() {
                (0, 0) // (len, signum)
            } else if i == ns.len() - 1 {
                (1, 0)
            } else {
                let (prev_len, prev_s) = rec(i + 1, ns);
                match (ns[i] - ns[i + 1]).signum() {
                    0 => (1, 0),
                    s if s == -prev_s => (prev_len + 1, s),
                    s => (2, s),
                }
            }
        }
        (0..nums.len()).map(|i| rec(i, &nums).0).max().unwrap()
    }

    /// 23:52-00:30
    pub fn wiggle_max_length_rec(nums: Vec<i32>) -> i32 {
        println!("wiggle_max_length({:?})", nums);
        fn rec(i: usize, pn: i32, ps: i32, ns: &[i32]) -> i32 {
            if i == ns.len() {
                1
            } else {
                let s = (ns[i] - pn).signum();
                if s != 0 && (ps == 0 || s == -ps) {
                    rec(i + 1, pn, ps, ns).max(1 + rec(i + 1, ns[i], s, ns))
                } else {
                    rec(i + 1, pn, ps, ns)
                }
            }
        }
        rec(1, nums[0], 0, &nums)
    }
    /// 00:30-00:36
    pub fn wiggle_max_length_rec_with_memo(nums: Vec<i32>) -> i32 {
        println!("wiggle_max_length({:?})", nums);
        fn rec(i: usize, pn: usize, ps: usize, ns: &[i32], memo: &mut Vec<Vec<[i32; 3]>>) -> i32 {
            if i == ns.len() {
                1
            } else if memo[i][pn][ps] >= 0 {
                memo[i][pn][ps]
            } else {
                let skip = rec(i + 1, pn, ps, ns, memo);
                let s = ((ns[i] - pn as i32).signum() + 1) as usize;
                memo[i][pn][ps] = if s != 1 && (ps == 1 || (s as i32 - 1) == -(ps as i32 - 1)) {
                    skip.max(1 + rec(i + 1, ns[i] as usize, s, ns, memo))
                } else {
                    skip
                };
                memo[i][pn][ps]
            }
        }
        let mut memo = vec![vec![[-1; 3]; 1001]; nums.len()];
        rec(1, nums[0] as usize, 1, &nums, &mut memo)
    }

    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        println!("wiggle_max_length({:?})", nums);
        let mut prev_sign = 0;
        let mut prev = nums[0];
        let mut result = 1;
        for &curr in nums.iter().skip(1) {
            let curr_sign = (curr - prev).signum();
            if curr_sign != 0 && curr_sign != prev_sign {
                prev_sign = curr_sign;
                prev = curr;
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1()   { assert_eq!(Solution::wiggle_max_length(vec![1]    ), 1); }
    #[rustfmt::skip] #[test] fn n_11()  { assert_eq!(Solution::wiggle_max_length(vec![1,1]  ), 1); }
    #[rustfmt::skip] #[test] fn n_12()  { assert_eq!(Solution::wiggle_max_length(vec![1,2]  ), 2); }
    #[rustfmt::skip] #[test] fn n_21()  { assert_eq!(Solution::wiggle_max_length(vec![2,1]  ), 2); }
    #[rustfmt::skip] #[test] fn n_111() { assert_eq!(Solution::wiggle_max_length(vec![1,1,1]), 1); }
    #[rustfmt::skip] #[test] fn n_121() { assert_eq!(Solution::wiggle_max_length(vec![1,2,1]), 3); }
    #[rustfmt::skip] #[test] fn n_212() { assert_eq!(Solution::wiggle_max_length(vec![2,1,2]), 3); }
    #[rustfmt::skip] #[test] fn n_123() { assert_eq!(Solution::wiggle_max_length(vec![1,2,3]), 2); }
    #[rustfmt::skip] #[test] fn n_321() { assert_eq!(Solution::wiggle_max_length(vec![3,2,1]), 2); }
    #[test]
    fn n_174925() {
        let n = vec![1, 7, 4, 9, 2, 5];
        assert_eq!(Solution::wiggle_max_length(n), 6);
        // Explanation: The entire sequence is a wiggle sequence with differences (6, -3, 5, -7, 3).
    }
    #[test]
    fn n_1_17_5_10_13_15_10_5_16_8() {
        let n = vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8];
        assert_eq!(Solution::wiggle_max_length(n), 7);
        // Explanation: There are several subsequences that achieve this length.
        // One is [1, 17, 10, 13, 10, 16, 8] with differences (16, -7, 3, -3, 6, -8).
    }
    #[test]
    fn n_123456789() {
        let n = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(Solution::wiggle_max_length(n), 2);
    }

    #[test]
    fn n_1to1000() {
        let n = (1..=1000).collect();
        assert_eq!(Solution::wiggle_max_length(n), 2);
    }
    #[test]
    fn n_01_repeat_500() {
        let n = (1..=1000).map(|i| i % 2).collect();
        assert_eq!(Solution::wiggle_max_length(n), 1000);
    }
}
