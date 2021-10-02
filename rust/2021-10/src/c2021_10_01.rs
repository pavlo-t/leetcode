#![allow(dead_code)]
/// 1143. Longest Common Subsequence
/// ================================
///
/// Given two strings `text1` and `text2`,
/// return _the length of their longest __common subsequence___.
/// If there is no __common subsequence__, return `0`.
///
/// A __subsequence__ of a string is a new string generated from the original string with some characters (can be none)
/// deleted without changing the relative order of the remaining characters.
///
/// For example, `"ace"` is a subsequence of `"abcde"`.
///
/// A __common subsequence__ of two strings is a subsequence that is common to both strings.
///
/// __Constraints:__
///
/// - `1 <= text1.length, text2.length <= 1000`
/// - `text1` and `text2` consist of only lowercase English characters.
///
/// https://leetcode.com/problems/longest-common-subsequence/
struct Solution;
impl Solution {
    /// Approach 4: Dynamic Programming with Space Optimization
    /// https://leetcode.com/problems/longest-common-subsequence/solution/
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        #[rustfmt::skip]
        let (ti, tj) = {
            let (t1, t2) = (text1.as_bytes(), text2.as_bytes());
            if t1.len() > t2.len() { (t1, t2) } else { (t2, t1) }
        };
        let (ni, nj) = (ti.len(), tj.len());
        let mut prev = vec![0; nj + 1];
        let mut curr = vec![0; nj + 1];
        for i in 1..=ni {
            for j in 1..=nj {
                if ti[i - 1] == tj[j - 1] {
                    curr[j] = prev[j - 1] + 1;
                } else {
                    curr[j] = prev[j].max(curr[j - 1]);
                }
            }
            std::mem::swap(&mut prev, &mut curr);
        }
        prev[nj]
    }

    pub fn longest_common_subsequence_dp(text1: String, text2: String) -> i32 {
        let (ti, tj) = (text1.as_bytes(), text2.as_bytes());
        let (ni, nj) = (ti.len(), tj.len());
        let mut dp = vec![vec![0; nj + 1]; ni + 1];
        for i in 1..=ni {
            for j in 1..=nj {
                if ti[i - 1] == tj[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }
        dp[ni][nj]
    }
    pub fn longest_common_subsequence_dfs_with_memo(text1: String, text2: String) -> i32 {
        fn dfs(i: usize, j: usize, memo: &mut Vec<Vec<i32>>, ti: &[u8], tj: &[u8]) -> i32 {
            if i == ti.len() || j == tj.len() {
                0
            } else if memo[i][j] >= 0 {
                memo[i][j]
            } else if ti[i] == tj[j] {
                memo[i][j] = 1 + dfs(i + 1, j + 1, memo, ti, tj);
                memo[i][j]
            } else {
                let skip_i = dfs(i + 1, j, memo, ti, tj);
                let skip_j = dfs(i, j + 1, memo, ti, tj);
                memo[i][j] = skip_i.max(skip_j);
                memo[i][j]
            }
        }
        let (ti, tj) = (text1.as_bytes(), text2.as_bytes());
        dfs(0, 0, &mut vec![vec![-1; tj.len()]; ti.len()], ti, tj)
    }
    pub fn longest_common_subsequence_dfs(text1: String, text2: String) -> i32 {
        fn dfs(i: usize, j: usize, ti: &[u8], tj: &[u8]) -> i32 {
            if i == ti.len() || j == tj.len() {
                0
            } else if ti[i] == tj[j] {
                1 + dfs(i + 1, j + 1, ti, tj)
            } else {
                dfs(i + 1, j, ti, tj).max(dfs(i, j + 1, ti, tj))
            }
        }
        dfs(0, 0, text1.as_bytes(), text2.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abcde_ace() {
        let text1 = "abcde".to_string();
        let text2 = "ace".to_string();
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);
        // Explanation: The longest common subsequence is "ace" and its length is 3.
    }
    #[test]
    fn abc_abc() {
        let text1 = "abc".to_string();
        let text2 = "abc".to_string();
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);
        // Explanation: The longest common subsequence is "abc" and its length is 3.
    }
    #[test]
    fn abc_def() {
        let text1 = "abc".to_string();
        let text2 = "def".to_string();
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 0);
        // Explanation: There is no such common subsequence, so the result is 0.
    }
    #[test]
    fn abcde_bcda() {
        let text1 = "abcde".to_string();
        let text2 = "bcda".to_string();
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);
    }
    #[test]
    fn bcda_abcde() {
        let text1 = "bcda".to_string();
        let text2 = "abcde".to_string();
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);
    }
    #[test]
    fn a_a() {
        let text1 = "a".to_string();
        let text2 = "a".to_string();
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 1);
    }
    #[test]
    fn a_b() {
        let text1 = "a".to_string();
        let text2 = "b".to_string();
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 0);
    }
    #[test]
    fn t1_1000_a_t2_1000_a() {
        let text1 = "a".repeat(1000);
        let text2 = "a".repeat(1000);
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 1000);
    }
    #[test]
    fn t1_1000_a_t2_1000_b() {
        let text1 = "a".repeat(1000);
        let text2 = "b".repeat(1000);
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 0);
    }
}
