#![allow(dead_code)]
/// 72. Edit Distance
/// =================
///
/// Given two strings `word1` and `word2`,
/// return _the minimum number of operations required to convert `word1` to `word2`_.
///
/// You have the following three operations permitted on a word:
///
/// - Insert a character
/// - Delete a character
/// - Replace a character
///
/// __Constraints:__
///
/// - `0 <= word1.length, word2.length <= 500`
/// - `word1` and `word2` consist of lowercase English letters.
///
/// https://leetcode.com/problems/edit-distance/
struct Solution;
impl Solution {
    /// 18:52-18:56
    #[rustfmt::skip]
    pub fn min_distance(word1: String, word2: String) -> i32 {
        println!("min_distance({}, {})", word1, word2);
        let (w1, w2) = {
            let (w1, w2) = (word1.as_bytes(), word2.as_bytes());
            if w1.len() >= w2.len() { (w1, w2) } else { (w2, w1) }
        };
        let (l1, l2) = (w1.len(), w2.len());
        let mut dp = (0..=l2).map(|i2| l2 - i2).collect::<Vec<_>>();
        let mut prev;
        for i1 in (0..l1).rev() {
            prev = dp[l2];
            dp[l2] = l1 - i1;
            for i2 in (0..l2).rev() {
                let curr = dp[i2];
                dp[i2] = if w1[i1] == w2[i2] { prev } else { 1 + dp[i2].min(dp[i2 + 1]).min(prev) };
                prev = curr;
            }
        }
        dp[0] as i32
    }
    /// 18:37-18:49
    pub fn min_distance_dp_vec_vec(word1: String, word2: String) -> i32 {
        println!("min_distance({}, {})", word1, word2);
        #[rustfmt::skip]
        let (w1, w2) = {
            let (w1, w2) = (word1.as_bytes(), word2.as_bytes());
            if w1.len() <= w2.len() { (w1, w2) } else { (w2, w1) }
        };
        let mut dp = vec![vec![0; w2.len() + 1]; w1.len() + 1];
        for i1 in 0..w1.len() {
            dp[i1][w2.len()] = (w1.len() - i1) as i32;
        }
        for i2 in 0..w2.len() {
            dp[w1.len()][i2] = (w2.len() - i2) as i32;
        }
        for i1 in (0..w1.len()).rev() {
            for i2 in (0..w2.len()).rev() {
                dp[i1][i2] = if w1[i1] == w2[i2] {
                    dp[i1 + 1][i2 + 1]
                } else {
                    1 + (dp[i1 + 1][i2].min(dp[i1][i2 + 1]).min(dp[i1 + 1][i2 + 1]))
                };
            }
        }
        dp[0][0]
    }
    /// 18:27-18:37
    pub fn min_distance_rec_with_memo(word1: String, word2: String) -> i32 {
        println!("min_distance({}, {})", word1, word2);
        fn rec(i1: usize, i2: usize, w1: &[u8], w2: &[u8], memo: &mut Vec<Vec<i32>>) -> i32 {
            if i1 == w1.len() {
                (w2.len() - i2) as i32
            } else if i2 == w2.len() {
                (w1.len() - i1) as i32
            } else if memo[i1][i2] >= 0 {
                memo[i1][i2]
            } else {
                memo[i1][i2] = if w1[i1] == w2[i2] {
                    rec(i1 + 1, i2 + 1, w1, w2, memo)
                } else {
                    let del1 = 1 + rec(i1 + 1, i2, w1, w2, memo);
                    let del2 = 1 + rec(i1, i2 + 1, w1, w2, memo);
                    let edit = 1 + rec(i1 + 1, i2 + 1, w1, w2, memo);
                    del1.min(del2).min(edit)
                };
                memo[i1][i2]
            }
        }
        #[rustfmt::skip]
        let (w1, w2) = {
            let (w1, w2) = (word1.as_bytes(), word2.as_bytes());
            if w1.len() <= w2.len() { (w1, w2) } else { (w2, w1) }
        };
        let mut memo = vec![vec![-1; w2.len()]; w1.len()];
        rec(0, 0, w1, w2, &mut memo)
    }
    /// 18:13-18:27
    pub fn min_distance_rec(word1: String, word2: String) -> i32 {
        println!("min_distance({}, {})", word1, word2);
        fn rec(i1: usize, i2: usize, w1: &[u8], w2: &[u8]) -> usize {
            if i1 == w1.len() {
                w2.len() - i2
            } else if i2 == w2.len() {
                w1.len() - i1
            } else if w1[i1] == w2[i2] {
                rec(i1 + 1, i2 + 1, w1, w2)
            } else {
                let del1 = 1 + rec(i1 + 1, i2, w1, w2);
                let del2 = 1 + rec(i1, i2 + 1, w1, w2);
                let edit = 1 + rec(i1 + 1, i2 + 1, w1, w2);
                del1.min(del2).min(edit)
            }
        }
        rec(0, 0, word1.as_bytes(), word2.as_bytes()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn w1_0_w2_0() { assert_eq!(Solution::min_distance( "".to_string(),  "".to_string()), 0); }
    #[rustfmt::skip] #[test] fn w1_0_w2_a() { assert_eq!(Solution::min_distance( "".to_string(), "a".to_string()), 1); }
    #[rustfmt::skip] #[test] fn w1_a_w2_0() { assert_eq!(Solution::min_distance("a".to_string(),  "".to_string()), 1); }
    #[rustfmt::skip] #[test] fn w1_a_w2_a() { assert_eq!(Solution::min_distance("a".to_string(), "a".to_string()), 0); }
    #[rustfmt::skip] #[test] fn w1_a_w2_b() { assert_eq!(Solution::min_distance("a".to_string(), "b".to_string()), 1); }

    #[test]
    fn w1_horse_w2_ros() {
        let w1 = "horse".to_string();
        let w2 = "ros".to_string();
        assert_eq!(Solution::min_distance(w1, w2), 3);
        // Explanation:
        // horse -> rorse (replace 'h' with 'r')
        // rorse -> rose (remove 'r')
        // rose -> ros (remove 'e')
    }
    #[test]
    fn w1_intention_w2_execution() {
        let w1 = "intention".to_string();
        let w2 = "execution".to_string();
        assert_eq!(Solution::min_distance(w1, w2), 5);
        // Explanation:
        // intention -> inention (remove 't')
        // inention -> enention (replace 'i' with 'e')
        // enention -> exention (replace 'n' with 'x')
        // exention -> exection (replace 'n' with 'c')
        // exection -> execution (insert 'u')
    }

    #[test]
    fn w1_500a_w2_500a() {
        let w1 = "a".repeat(500);
        let w2 = "a".repeat(500);
        assert_eq!(Solution::min_distance(w1, w2), 0);
    }
    #[test]
    fn w1_500a_w2_500b() {
        let w1 = "a".repeat(500);
        let w2 = "b".repeat(500);
        assert_eq!(Solution::min_distance(w1, w2), 500);
    }
}
