#![allow(dead_code)]
/// Delete Operation for Two Strings
/// ================================
///
/// Given two strings `word1` and `word2`,
/// return _the minimum number of __steps__ required to make `word1` and `word2` the same_.
///
/// In one __step__, you can delete exactly one character in either string.
///
/// __Constraints:__
///
/// - `1 <= word1.length, word2.length <= 500`
/// - `word1` and `word2` consist of only lowercase English letters.
///
/// https://leetcode.com/explore/featured/card/may-leetcoding-challenge-2021/598/week-1-may-1st-may-7th/3734/
struct Solution;
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let v1 = word1.as_bytes();
        let v2 = word2.as_bytes();
        let mut dp = vec![0; v2.len() + 1];
        for i in 0..=v1.len() {
            let mut temp = vec![0; v2.len() + 1];
            for j in 0..=v2.len() {
                if i == 0 || j == 0 {
                    temp[j] = i + j;
                } else if v1[i - 1] == v2[j - 1] {
                    temp[j] = dp[j - 1];
                } else {
                    temp[j] = 1 + dp[j].min(temp[j - 1])
                }
            }
            dp = temp;
        }
        dp[v2.len()] as i32
    }
    pub fn min_distance_dp_1(word1: String, word2: String) -> i32 {
        let v1 = word1.as_bytes();
        let v2 = word2.as_bytes();
        let mut dp = (0..=v1.len()).collect::<Vec<_>>();
        for j in 1..=v2.len() {
            let mut last = dp[0];
            dp[0] = j;
            for i in 1..=v1.len() {
                let curr = dp[i];
                dp[i] = if v1[i - 1] == v2[j - 1] {
                    last
                } else {
                    1 + dp[i].min(dp[i - 1])
                };
                last = curr;
            }
        }
        dp[v1.len()] as i32
    }

    pub fn min_distance_rec_memo_my(word1: String, word2: String) -> i32 {
        fn rec(v1: &[u8], v2: &[u8], cache: &mut [Vec<i32>]) -> i32 {
            if v1.is_empty() {
                v2.len() as i32
            } else if v2.is_empty() {
                v1.len() as i32
            } else if cache[v1.len()][v2.len()] >= 0 {
                cache[v1.len()][v2.len()]
            } else if v1[0] == v2[0] {
                rec(&v1[1..], &v2[1..], cache)
            } else {
                let r = 1 + rec(&v1[1..], v2, cache).min(rec(v1, &v2[1..], cache));
                cache[v1.len()][v2.len()] = r;
                r
            }
        }

        let mut cache = vec![vec![-1; word2.len() + 1]; word1.len() + 1];
        rec(word1.as_bytes(), word2.as_bytes(), &mut cache)
    }

    pub fn min_distance_rec_brute_force(word1: String, word2: String) -> i32 {
        fn rec(v1: &[u8], v2: &[u8]) -> i32 {
            if v1.is_empty() {
                v2.len() as i32
            } else if v2.is_empty() {
                v1.len() as i32
            } else if v1[0] == v2[0] {
                rec(&v1[1..], &v2[1..])
            } else {
                1 + rec(&v1[1..], v2).min(rec(v1, &v2[1..]))
            }
        }

        rec(word1.as_bytes(), word2.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_sea_eat_produces_2() {
        let word1 = "sea".to_string();
        let word2 = "eat".to_string();
        assert_eq!(Solution::min_distance(word1, word2), 2);
        // Explanation: You need one step to make "sea" to "ea" and another step to make "eat" to "ea".
    }
    #[test]
    fn example2_leetcode_etco_produces_4() {
        let word1 = "leetcode".to_string();
        let word2 = "etco".to_string();
        assert_eq!(Solution::min_distance(word1, word2), 4);
    }

    #[test]
    fn a_a_produces_0() {
        let word1 = "a".to_string();
        let word2 = "a".to_string();
        assert_eq!(Solution::min_distance(word1, word2), 0);
    }
    #[test]
    fn aaaaaee_aee_produces_4() {
        let word1 = "aaaaaee".to_string();
        let word2 = "aee".to_string();
        assert_eq!(Solution::min_distance(word1, word2), 4);
    }
    #[test]
    fn aee_aaaaaee_produces_4() {
        let word1 = "aee".to_string();
        let word2 = "aaaaaee".to_string();
        assert_eq!(Solution::min_distance(word1, word2), 4);
    }

    #[test]
    fn a500_b500_produces_1000() {
        let word1 = "a".repeat(500);
        let word2 = "b".repeat(500);
        assert_eq!(Solution::min_distance(word1, word2), 1000);
    }
    #[test]
    fn a500_a500_produces_0() {
        let word1 = "a".repeat(500);
        let word2 = "a".repeat(500);
        assert_eq!(Solution::min_distance(word1, word2), 0);
    }
}
