#![allow(dead_code)]
/// 487. Max Consecutive Ones II
/// ============================
///
/// Given a binary array `nums`,
/// return _the maximum number of consecutive `1`'s in the array if you can flip at most one `0`_.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 100_000`
/// - `nums[i]` is either `0` or `1`.
///
/// __Follow up:__ What if the input numbers come in one by one as an infinite stream?
/// In other words, you can't store all numbers coming from the stream as it's too large to hold in memory.
/// Could you solve it efficiently?
///
/// https://leetcode.com/problems/max-consecutive-ones-ii/
struct Solution;
impl Solution {
    /// 22:54-22:57
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        println!("find_max_consecutive_ones({:?})", nums);
        let (mut no_flip, mut flip) = (0, 0);
        let mut result = 0;
        for &n in nums.iter() {
            if n == 0 {
                flip = no_flip + 1;
                no_flip = 0;
            } else {
                flip += 1;
                no_flip += 1;
            }
            result = result.max(flip);
        }
        result
    }
    /// 22:53-22:54
    pub fn find_max_consecutive_ones_dp_vec_improved(nums: Vec<i32>) -> i32 {
        println!("find_max_consecutive_ones({:?})", nums);
        let mut dp = vec![[0, 0]; nums.len() + 1];
        let mut result = 0;
        for i in (0..nums.len()).rev() {
            dp[i] = match nums[i] {
                0 => [0, dp[i + 1][0] + 1],
                _ => [dp[i + 1][0] + 1, dp[i + 1][1] + 1],
            };
            result = result.max(dp[i][1]);
        }
        result
    }
    /// 22:49-22:53
    pub fn find_max_consecutive_ones_dp_vec(nums: Vec<i32>) -> i32 {
        println!("find_max_consecutive_ones({:?})", nums);
        let mut dp = vec![[0, 0]; nums.len() + 1];
        for i in (0..nums.len()).rev() {
            dp[i] = match nums[i] {
                0 => [0, dp[i + 1][0] + 1],
                _ => [dp[i + 1][0] + 1, dp[i + 1][1] + 1],
            };
        }
        dp.into_iter().map(|[_, c]| c).max().unwrap()
    }
    /// 22:47-22:49
    pub fn find_max_consecutive_ones_rec_with_memo(nums: Vec<i32>) -> i32 {
        println!("find_max_consecutive_ones({:?})", nums);
        fn rec(i: usize, ns: &[i32], memo: &mut Vec<(i32, i32)>) -> (i32, i32) {
            if i == ns.len() {
                (0, 0)
            } else if memo[i].0 >= 0 {
                memo[i]
            } else {
                let (not_flipped, flipped) = rec(i + 1, ns, memo);
                memo[i] = if ns[i] == 0 {
                    (0, not_flipped + 1)
                } else {
                    (not_flipped + 1, flipped + 1)
                };
                memo[i]
            }
        }
        let mut memo = vec![(-1, -1); nums.len()];
        (0..nums.len())
            .map(|i| rec(i, &nums, &mut memo).1)
            .max()
            .unwrap()
    }
    /// 22:38-22:46
    pub fn find_max_consecutive_ones_rec(nums: Vec<i32>) -> i32 {
        println!("find_max_consecutive_ones({:?})", nums);
        fn rec(i: usize, ns: &[i32]) -> (i32, i32) {
            if i == ns.len() {
                (0, 0)
            } else {
                let (not_flipped, flipped) = rec(i + 1, ns);
                if ns[i] == 0 {
                    (0, not_flipped + 1)
                } else {
                    (not_flipped + 1, flipped + 1)
                }
            }
        }
        (0..nums.len()).map(|i| rec(i, &nums).1).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_0() { assert_eq!(Solution::find_max_consecutive_ones(vec![0]), 1); }
    #[rustfmt::skip] #[test] fn n_1() { assert_eq!(Solution::find_max_consecutive_ones(vec![1]), 1); }
    #[test]
    fn n_10110() {
        let n = vec![1, 0, 1, 1, 0];
        assert_eq!(Solution::find_max_consecutive_ones(n), 4);
        // Explanation: Flip the first zero will get the maximum number of consecutive 1s.
        // After flipping, the maximum number of consecutive 1s is 4.
    }
    #[test]
    fn n_101101() {
        let n = vec![1, 0, 1, 1, 0, 1];
        assert_eq!(Solution::find_max_consecutive_ones(n), 4);
    }
    #[test]
    fn n_1001101() {
        let n = vec![1, 0, 0, 1, 1, 0, 1];
        assert_eq!(Solution::find_max_consecutive_ones(n), 4);
    }

    // If getting stack overflow:
    // Add `RUST_MIN_STACK=67108864` to env:
    // RUST_MIN_STACK=67108864 cargo test --lib d05_1
    #[test]
    fn n_100000x1() {
        let n = vec![1; 100000];
        assert_eq!(Solution::find_max_consecutive_ones(n), 100000);
    }
    #[test]
    fn n_100000x0() {
        let n = vec![0; 100000];
        assert_eq!(Solution::find_max_consecutive_ones(n), 1);
    }
}
