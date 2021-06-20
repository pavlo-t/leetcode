#![allow(dead_code)]
/// K Inverse Pairs Array
/// =====================
///
/// For an integer array `nums`, an __inverse pair__ is a pair of integers `[i, j]`
/// where `0 <= i < j < nums.length` and `nums[i] > nums[j]`.
///
/// Given two integers `n` and `k`,
/// return _the number of different arrays consisting of numbers
/// from `1` to `n` such that there are exactly `k` __inverse pairs__.
/// Since the answer can be huge, return it __modulo__ `10^9 + 7`_.
///
/// __Constraints:__
///
/// - `1 <= n <= 1000`
/// - `0 <= k <= 1000`
///
/// https://leetcode.com/explore/challenge/card/june-leetcoding-challenge-2021/605/week-3-june-15th-june-21st/3784/
struct Solution;
impl Solution {
    /// https://dreamume.medium.com/leetcode-629-k-inverse-pairs-array-8f6b1c05e3ea
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        const M: i64 = 1_000_000_007;
        if k > n * (n - 1) / 2 || k < 0 {
            0
        } else {
            let n = n as usize;
            let k = k as usize;
            let mut dp = vec![vec![0i64; k + 1]; n + 1];
            for i in 1..=n {
                dp[i][0] = 1;
                if i + 1 <= n {
                    dp[i + 1][0] = 1;
                }
                for j in 1..=k.min(i * (i - 1) / 2) {
                    dp[i][j] = dp[i][j - 1] + dp[i - 1][j];
                    if j >= i {
                        dp[i][j] -= dp[i - 1][j - i];
                    }
                    dp[i][j] = (dp[i][j] + M) % M;
                }
            }
            dp[n][k] as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n3k0_produces_1() {
        assert_eq!(Solution::k_inverse_pairs(3, 0), 1);
        // Explanation:
        // Only the array [1,2,3] which consists of numbers from 1 to 3 has exactly 0 inverse pairs.
    }
    #[test]
    fn n3k1_produces_2() {
        assert_eq!(Solution::k_inverse_pairs(3, 1), 2);
        // Explanation:
        // The array [1,3,2] and [2,1,3] have exactly 1 inverse pair.
    }

    mod performance {
        use super::*;

        #[test]
        fn n1000k1000_produces_663677020() {
            assert_eq!(Solution::k_inverse_pairs(1000, 1000), 663_677_020);
        }
    }
}
