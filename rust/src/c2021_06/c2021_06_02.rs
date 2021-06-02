#![allow(dead_code)]
/// Interleaving String
/// ===================
///
/// Given strings `s1`, `s2`, and `s3`, find whether `s3` is formed by an __interleaving__ of `s1` and `s2`.
///
/// An __interleaving__ of two strings `s` and `t` is a configuration where they are divided into
/// __non-empty__ substrings such that:
///
/// - `s = s1 + s2 + ... + sn`
/// - `t = t1 + t2 + ... + tm`
/// - `|n - m| <= 1`
/// - The __interleaving__ is `s1 + t1 + s2 + t2 + s3 + t3 + ...` or `t1 + s1 + t2 + s2 + t3 + s3 + ...`
///
/// __Note:__ `a + b` is the concatenation of strings `a` and `b`.
///
/// __Constraints:__
///
/// - `0 <= s1.length, s2.length <= 100`
/// - `0 <= s3.length <= 200`
/// - `s1`, `s2`, and `s3` consist of lowercase English letters.
///  
/// __Follow up:__ Could you solve it using only `O(s2.length)` additional memory space?
///
/// https://leetcode.com/explore/featured/card/june-leetcoding-challenge-2021/603/week-1-june-1st-june-7th/3765/
struct Solution;
impl Solution {
    /// Approach 4: Using 1D Dynamic Programming
    /// https://leetcode.com/problems/interleaving-string/solution/
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (n1, n2, n3) = (s1.len(), s2.len(), s3.len());
        if n1 + n2 != n3 {
            false
        } else {
            let mut dp = vec![false; n2 + 1];
            let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
            for i in 0..=n1 {
                for j in 0..=n2 {
                    if i == 0 && j == 0 {
                        dp[j] = true;
                    } else if i == 0 {
                        dp[j] = dp[j - 1] && s2[j - 1] == s3[i + j - 1];
                    } else if j == 0 {
                        dp[j] = dp[j] && s1[i - 1] == s3[i + j - 1];
                    } else {
                        dp[j] = (dp[j] && s1[i - 1] == s3[i + j - 1])
                            || (dp[j - 1] && s2[j - 1] == s3[i + j - 1]);
                    }
                }
            }
            dp[n2]
        }
    }

    pub fn is_interleave_2d_cache(s1: String, s2: String, s3: String) -> bool {
        fn dfs(ss: (&[u8], &[u8], &[u8]), is: (usize, usize), seen: &mut Vec<Vec<bool>>) -> bool {
            let (s1, s2, s3) = ss;
            let (i1, i2) = is;
            let (n1, n2, n3) = (s1.len(), s2.len(), s3.len());
            let i3 = i1 + i2;

            if i1 == n1 && i2 == n2 && i3 == n3 {
                true
            } else if seen[i1][i2] {
                false
            } else {
                seen[i1][i2] = true;
                match (i1 == n1, i2 == n2) {
                    (false, false) => match (s1[i1] == s3[i3], s2[i2] == s3[i3]) {
                        (false, false) => false,
                        (true, true) => dfs(ss, (i1 + 1, i2), seen) || dfs(ss, (i1, i2 + 1), seen),
                        (true, false) => dfs(ss, (i1 + 1, i2), seen),
                        (false, true) => dfs(ss, (i1, i2 + 1), seen),
                    },
                    (true, false) => s2[i2] == s3[i3] && dfs(ss, (i1, i2 + 1), seen),
                    (false, true) => s1[i1] == s3[i3] && dfs(ss, (i1 + 1, i2), seen),
                    (true, true) => false,
                }
            }
        }
        let (n1, n2, n3) = (s1.len(), s2.len(), s3.len());
        if n1 + n2 == n3 {
            let mut seen = vec![vec![false; n2 + 1]; n1 + 1];
            dfs(
                (s1.as_bytes(), s2.as_bytes(), s3.as_bytes()),
                (0, 0),
                &mut seen,
            )
        } else {
            false
        }
    }

    pub fn is_interleave_brute_force_2(s1: String, s2: String, s3: String) -> bool {
        fn dfs(ss: (&[u8], &[u8], &[u8]), is: (usize, usize, usize)) -> bool {
            let (s1, s2, s3) = ss;
            let (i1, i2, i3) = is;
            let (n1, n2, n3) = (s1.len(), s2.len(), s3.len());

            if i1 == n1 && i2 == n2 && i3 == n3 {
                true
            } else if n1 + n2 != n3 {
                false
            } else {
                match (i1 == n1, i2 == n2) {
                    (false, false) => match (s1[i1] == s3[i3], s2[i2] == s3[i3]) {
                        (false, false) => false,
                        (true, true) => {
                            dfs(ss, (i1 + 1, i2, i3 + 1)) || dfs(ss, (i1, i2 + 1, i3 + 1))
                        }
                        (true, false) => dfs(ss, (i1 + 1, i2, i3 + 1)),
                        (false, true) => dfs(ss, (i1, i2 + 1, i3 + 1)),
                    },
                    (true, false) => s2[i2] == s3[i3] && dfs(ss, (i1, i2 + 1, i3 + 1)),
                    (false, true) => s1[i1] == s3[i3] && dfs(ss, (i1 + 1, i2, i3 + 1)),
                    (true, true) => false,
                }
            }
        }
        dfs((s1.as_bytes(), s2.as_bytes(), s3.as_bytes()), (0, 0, 0))
    }

    pub fn is_interleave_brute_force(s1: String, s2: String, s3: String) -> bool {
        fn dfs(s1: &[u8], s2: &[u8], s3: &[u8]) -> bool {
            match (s1.len(), s2.len(), s3.len()) {
                (0, 0, 0) => true,
                (0, 0, _) | (_, _, 0) => false,
                (l1, l2, l3) if l3 != l1 + l2 => false,
                (0, _, _) => dfs(s2, s1, s3),
                (_, 0, _) => s1[0] == s3[0] && dfs(&s1[1..], s2, &s3[1..]),
                _ => {
                    (s1[0] == s3[0] && dfs(&s1[1..], s2, &s3[1..]))
                        || (s2[0] == s3[0] && dfs(s1, &s2[1..], &s3[1..]))
                }
            }
        }
        dfs(s1.as_bytes(), s2.as_bytes(), s3.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s1_aabcc_s2_dbbca_s3_aadbbcbcac_is_interleave() {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbcbcac".to_string();
        assert!(Solution::is_interleave(s1, s2, s3));
    }
    #[test]
    fn s1_aabcc_s2_dbbca_s3_aadbbbaccc_is_not_interleave() {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbbaccc".to_string();
        assert!(!Solution::is_interleave(s1, s2, s3));
    }
    #[test]
    fn s1_empty_s2_empty_s3_empty_is_interleave() {
        let s1 = "".to_string();
        let s2 = "".to_string();
        let s3 = "".to_string();
        assert!(Solution::is_interleave(s1, s2, s3));
    }

    #[test]
    fn test103_s1_27a_s2_82a_s3_53a_is_interleave() {
        let s1 = "a".repeat(27);
        let s2 = "a".repeat(82);
        let s3 = "a".repeat(53);
        assert!(!Solution::is_interleave(s1, s2, s3));
    }

    #[test]
    fn s1_100a_s2_100b_s3_100ab_is_interleave() {
        let s1 = "a".repeat(100);
        let s2 = "b".repeat(100);
        let s3 = "ab".repeat(100);
        assert!(Solution::is_interleave(s1, s2, s3));
    }
    #[test]
    fn s1_100a_s2_100a_s3_100aa_is_interleave() {
        let s1 = "a".repeat(100);
        let s2 = "a".repeat(100);
        let s3 = "aa".repeat(100);
        assert!(Solution::is_interleave(s1, s2, s3));
    }
}
