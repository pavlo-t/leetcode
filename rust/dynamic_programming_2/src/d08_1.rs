#![allow(dead_code)]
/// 712. Minimum ASCII Delete Sum for Two Strings
/// =============================================
///
/// Given two strings `s1` and `s2`,
/// return _the lowest __ASCII__ sum of deleted characters to make two strings equal_.
///
/// __Constraints:__
///
/// - `1 <= s1.length, s2.length <= 1000`
/// - `s1` and `s2` consist of lowercase English letters.
///
/// https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/
struct Solution;
impl Solution {
    pub fn minimum_delete_sum_rec(s1: String, s2: String) -> i32 {
        println!("minimum_delete_sum({}, {})", s1, s2);
        fn rec(i1: usize, i2: usize, s1: &[u8], s2: &[u8]) -> i32 {
            if i1 == s1.len() {
                (i2..s2.len()).map(|i| s2[i] as i32).sum()
            } else if i2 == s2.len() {
                (i1..s1.len()).map(|i| s1[i] as i32).sum()
            } else {
                if s1[i1] == s2[i2] {
                    rec(i1 + 1, i2 + 1, s1, s2)
                } else {
                    let skip1 = s1[i1] as i32 + rec(i1 + 1, i2, s1, s2);
                    let skip2 = s2[i2] as i32 + rec(i1, i2 + 1, s1, s2);
                    skip1.min(skip2)
                }
            }
        }
        let (bs1, bs2) = (s1.as_bytes(), s2.as_bytes());
        rec(0, 0, bs1, bs2)
    }
    pub fn minimum_delete_sum_rec_with_memo(s1: String, s2: String) -> i32 {
        println!("minimum_delete_sum({}, {})", s1, s2);
        fn rec(i1: usize, i2: usize, s1: &[u8], s2: &[u8], memo: &mut Vec<Vec<i32>>) -> i32 {
            if memo[i1][i2] != -1 {
                memo[i1][i2]
            } else {
                memo[i1][i2] = if i1 == s1.len() {
                    (i2..s2.len()).map(|i| s2[i] as i32).sum()
                } else if i2 == s2.len() {
                    (i1..s1.len()).map(|i| s1[i] as i32).sum()
                } else if s1[i1] == s2[i2] {
                    rec(i1 + 1, i2 + 1, s1, s2, memo)
                } else {
                    let skip1 = s1[i1] as i32 + rec(i1 + 1, i2, s1, s2, memo);
                    let skip2 = s2[i2] as i32 + rec(i1, i2 + 1, s1, s2, memo);
                    skip1.min(skip2)
                };
                memo[i1][i2]
            }
        }
        let (bs1, bs2) = (s1.as_bytes(), s2.as_bytes());
        let mut memo = vec![vec![-1; bs2.len() + 1]; bs1.len() + 1];
        rec(0, 0, bs1, bs2, &mut memo)
    }
    pub fn minimum_delete_sum_dp_vec_vec(s1: String, s2: String) -> i32 {
        println!("minimum_delete_sum({}, {})", s1, s2);
        let (bs1, bs2) = (s1.as_bytes(), s2.as_bytes());
        let (l1, l2) = (bs1.len(), bs2.len());
        let mut dp = vec![vec![0; l2 + 1]; l1 + 1];
        for i1 in (0..l1).rev() {
            dp[i1][l2] = bs1[i1] as i32 + dp[i1 + 1][l2];
        }
        for i2 in (0..l2).rev() {
            dp[l1][i2] = bs2[i2] as i32 + dp[l1][i2 + 1];
        }
        for i1 in (0..l1).rev() {
            for i2 in (0..l2).rev() {
                dp[i1][i2] = if bs1[i1] == bs2[i2] {
                    dp[i1 + 1][i2 + 1]
                } else {
                    let skip1 = bs1[i1] as i32 + dp[i1 + 1][i2];
                    let skip2 = bs2[i2] as i32 + dp[i1][i2 + 1];
                    skip1.min(skip2)
                };
            }
        }
        dp[0][0]
    }
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        println!("minimum_delete_sum({}, {})", s1, s2);
        let (bs1, bs2) = {
            let (bs1, bs2) = (s1.as_bytes(), s2.as_bytes());
            if bs1.len() < bs2.len() {
                (bs2, bs1)
            } else {
                (bs1, bs2)
            }
        };
        let (l1, l2) = (bs1.len(), bs2.len());
        let mut dp = vec![0; l2 + 1];

        for i2 in (0..l2).rev() {
            dp[i2] = bs2[i2] as i32 + dp[i2 + 1];
        }
        for i1 in (0..l1).rev() {
            let mut prev = dp[l2];
            dp[l2] += bs1[i1] as i32;
            for i2 in (0..l2).rev() {
                let curr = dp[i2];
                dp[i2] = if bs1[i1] == bs2[i2] {
                    prev
                } else {
                    let skip1 = bs1[i1] as i32 + dp[i2];
                    let skip2 = bs2[i2] as i32 + dp[i2 + 1];
                    skip1.min(skip2)
                };
                prev = curr;
            }
        }

        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn a_a()  { assert_eq!(Solution::minimum_delete_sum("a".into(), "a".into() ),   0); }
    #[rustfmt::skip] #[test] fn a_b()  { assert_eq!(Solution::minimum_delete_sum("a".into(), "b".into() ), 195); }
    #[rustfmt::skip] #[test] fn a_ab() { assert_eq!(Solution::minimum_delete_sum("a".into(), "ab".into()),  98); }
    #[rustfmt::skip] #[test] fn b_ba() { assert_eq!(Solution::minimum_delete_sum("b".into(), "ba".into()),  97); }
    #[rustfmt::skip] #[test] fn b_bb() { assert_eq!(Solution::minimum_delete_sum("b".into(), "bb".into()),  98); }
    #[test]
    fn sea_eat() {
        let s1 = "sea".into();
        let s2 = "eat".into();
        assert_eq!(Solution::minimum_delete_sum(s1, s2), 231);
        // Explanation: Deleting "s" from "sea" adds the ASCII value of "s" (115) to the sum.
        // Deleting "t" from "eat" adds 116 to the sum.
        // At the end, both strings are equal, and 115 + 116 = 231 is the minimum sum possible to achieve this.
    }
    #[test]
    fn delete_leet() {
        let s1 = "delete".into();
        let s2 = "leet".into();
        assert_eq!(Solution::minimum_delete_sum(s1, s2), 403);
        // Explanation: Deleting "dee" from "delete" to turn the string into "let",
        // adds 100[d] + 101[e] + 101[e] to the sum.
        // Deleting "e" from "leet" adds 101[e] to the sum.
        // At the end, both strings are equal to "let", and the answer is 100+101+101+101 = 403.
        // If instead we turned both strings into "lee" or "eet", we would get answers of 433 or 417, which are higher.
    }

    #[test]
    fn a_repeat_1000_a_repeat_1000() {
        let s1 = "a".repeat(1000);
        let s2 = "a".repeat(1000);
        assert_eq!(Solution::minimum_delete_sum(s1, s2), 0);
    }
    #[test]
    fn a_repeat_1000_b_repeat_1000() {
        let s1 = "a".repeat(1000);
        let s2 = "b".repeat(1000);
        assert_eq!(Solution::minimum_delete_sum(s1, s2), 195_000);
    }
}
