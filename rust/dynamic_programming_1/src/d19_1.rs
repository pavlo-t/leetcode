#![allow(dead_code)]
/// 392. Is Subsequence
/// ===================
///
/// Given two strings `s` and `t`, return _`true` if `s` is a __subsequence__ of `t`, or `false` otherwise_.
///
/// A __subsequence__ of a string is a new string that is formed from the original string by deleting some (can be none)
/// of the characters without disturbing the relative positions of the remaining characters.
/// (i.e., `"ace"` is a subsequence of `"abcde"` while `"aec"` is not).
///
/// __Constraints:__
///
/// - `0 <= s.length <= 100`
/// - `0 <= t.length <= 10_000`
/// - `s` and `t` consist only of lowercase English letters.
///
/// __Follow up:__ Suppose there are lots of incoming `s`, say `s1, s2, ..., sk` where `k >= 10**9`,
/// and you want to check one by one to see if `t` has its subsequence.
/// In this scenario, how would you change your code?
///
/// https://leetcode.com/problems/is-subsequence
struct Solution;
impl Solution {
    /// __Follow-up solution__ Implemented myself, inspired by:
    /// Approach 3: Greedy Match with Character Indices Hashmap
    /// https://leetcode.com/problems/is-subsequence/solution/
    pub fn is_subsequence(s: String, t: String) -> bool {
        println!("is_subsequence({}, {})", s, t);
        fn fold_char_idxs(mut vec: Vec<Vec<usize>>, (i, b): (usize, u8)) -> Vec<Vec<usize>> {
            vec[(b - b'a') as usize].push(i);
            vec
        }
        let t_chars = t.bytes().enumerate().fold(vec![vec![]; 26], fold_char_idxs);
        let (mut is, mut it, mut tcis, bs) = (0, 0, [0; 26], s.as_bytes());
        while is < bs.len() {
            let c = (bs[is] - b'a') as usize;
            while tcis[c] < t_chars[c].len() && t_chars[c][tcis[c]] < it {
                tcis[c] += 1;
            }
            if tcis[c] == t_chars[c].len() {
                break;
            } else {
                it = t_chars[c][tcis[c]] + 1;
                tcis[c] += 1;
                is += 1;
            }
        }
        is == bs.len()
    }
    /// 21:14-21:25
    pub fn is_subsequence_my_2_pointers(s: String, t: String) -> bool {
        println!("is_subsequence({}, {})", s, t);
        let (mut is, mut it) = (0, 0);
        let (bs, bt) = (s.as_bytes(), t.as_bytes());
        while is < bs.len() && it < bt.len() {
            if bs[is] == bt[it] {
                is += 1;
            }
            it += 1;
        }
        is == bs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn s_0_t_0() { assert!( Solution::is_subsequence( "".to_string(),  "".to_string())); }
    #[rustfmt::skip] #[test] fn s_0_t_a() { assert!( Solution::is_subsequence( "".to_string(), "a".to_string())); }
    #[rustfmt::skip] #[test] fn s_a_t_0() { assert!(!Solution::is_subsequence("a".to_string(),  "".to_string())); }
    #[rustfmt::skip] #[test] fn s_a_t_a() { assert!( Solution::is_subsequence("a".to_string(), "a".to_string())); }

    #[rustfmt::skip]
    #[test] fn s_aabb_t_abab() { assert!(!Solution::is_subsequence("aabb".to_string(), "abab".to_string())); }

    #[test]
    fn s_abc_t_ahbgdc() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();
        assert!(Solution::is_subsequence(s, t));
    }
    #[test]
    fn s_axc_t_ahbgdc() {
        let s = "axc".to_string();
        let t = "ahbgdc".to_string();
        assert!(!Solution::is_subsequence(s, t));
    }

    #[test]
    fn s_100xa_t_10000xb() {
        let s = "a".repeat(100);
        let t = "b".repeat(10000);
        assert!(!Solution::is_subsequence(s, t));
    }
}
