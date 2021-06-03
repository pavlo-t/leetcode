#![allow(dead_code)]

/// # Ones and Zeroes
///
/// You are given an array of binary strings `strs` and two integers `m` and `n`.
///
/// Return _the size of the largest subset of `strs` such that there are
/// __at most__ `m` `0`'s and `n` `1`'s in the subset_.
///
/// A set `x` is a __subset__ of a set `y` if all elements of `x` are also elements of `y`.
///
/// __Constraints:__
///
/// - `1 <= strs.length <= 600`
/// - `1 <= strs[i].length <= 100`
/// - `strs[i]` consists only of digits `'0'` and `'1'`.
/// - `1 <= m, n <= 100`
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/593/week-1-april-1st-april-7th/3694/
struct Solution;
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize + 1]; m as usize + 1];
        let mut result = 0;
        for s in strs {
            let (c0, c1) = s.bytes().fold((0, 0), |(c0, c1), b| match b {
                b'0' => (c0 + 1, c1),
                b'1' => (c0, c1 + 1),
                _ => unreachable!(),
            });
            for i in (c0..=m as usize).rev() {
                for j in (c1..=n as usize).rev() {
                    dp[i][j] = dp[i][j].max(1 + dp[i - c0][j - c1]);
                    result = result.max(dp[i][j]);
                }
            }
        }
        result
    }

    /// https://www.tutorialspoint.com/ones-and-zeroes-in-cplusplus
    pub fn find_max_form_dp(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize + 1]; m as usize + 1];
        let mut result = 0;
        for s in strs {
            let mut c0 = 0;
            let mut c1 = 0;
            for b in s.as_bytes() {
                match b {
                    b'0' => c0 += 1,
                    b'1' => c1 += 1,
                    _ => (),
                }
            }
            for i in (c0..=m as usize).rev() {
                for j in (c1..=n as usize).rev() {
                    dp[i][j] = dp[i][j].max(1 + dp[i - c0][j - c1]);
                    result = result.max(dp[i][j]);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let strs = ["10", "0001", "111001", "1", "0"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(Solution::find_max_form(strs, 5, 3), 4);
        // Explanation:
        // The largest subset with at most 5 0's and 3 1's is {"10", "0001", "1", "0"}, so the answer is 4.
        // Other valid but smaller subsets include {"0001", "1"} and {"10", "1", "0"}.
        // {"111001"} is an invalid subset because it contains 4 1's, greater than the maximum of 3.
    }
    #[test]
    fn example2() {
        let strs = ["10", "0", "1"].iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::find_max_form(strs, 1, 1), 2);
        // Explanation: The largest subset is {"0", "1"}, so the answer is 2.
    }

    #[test]
    fn test() {
        let strs = ["00", "00", "00", "011", "011", "011"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(Solution::find_max_form(strs, 6, 100), 4);
        // {"00", "011", "011", "011"}
    }
}
