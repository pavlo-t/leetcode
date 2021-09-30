#![allow(dead_code)]
/// Maximum Length of a Concatenated String with Unique Characters
/// ==============================================================
///
/// Given an array of strings `arr`.
/// String `s` is a concatenation of a sub-sequence of `arr` which have __unique characters__.
///
/// Return _the maximum possible length of `s`_.
///
/// __Constraints:__
///
/// - `1 <= arr.length <= 16`
/// - `1 <= arr[i].length <= 26`
/// - `arr[i]` contains only lower case English letters.
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/639/week-4-september-22nd-september-28th/3984/
struct Solution;
impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        println!("max_length({:?})", arr);
        fn encode(s: &String) -> i32 {
            s.bytes()
                .map(|b| 1 << (b - b'a'))
                .fold(0, |p, b| if p & b == 0 { p | b } else { -1 })
        }
        fn rec(i: usize, seen: i32, es: &[i32]) -> i32 {
            if i >= es.len() {
                seen.count_ones() as i32
            } else if seen & es[i] != 0 {
                0
            } else {
                let mut r = 0;
                let new_seen = seen | es[i];
                for j in i + 1..=es.len() {
                    r = r.max(rec(j, new_seen, es));
                }
                r
            }
        }
        let vi = std::iter::once(0)
            .chain(arr.iter().map(encode).filter(|&i| i > 0))
            .collect::<Vec<_>>();
        rec(0, 0, &vi)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($x:tt),*) => {vec![$($x.to_string()),*]}}

    #[test]
    fn un_iq_ue_produces_4() {
        let arr = vs!["un", "iq", "ue"];
        assert_eq!(Solution::max_length(arr), 4);
        // Explanation: All possible concatenations are "","un","iq","ue","uniq" and "ique".
        // Maximum length is 4.
    }
    #[test]
    fn cha_r_act_ers_produces_6() {
        let arr = vs!["cha", "r", "act", "ers"];
        assert_eq!(Solution::max_length(arr), 6);
        // Explanation: Possible solutions are "chaers" and "acters".
    }
    #[test]
    fn abcdefghijklmnopqrstuvwxyz_produces_26() {
        let arr = vs!["abcdefghijklmnopqrstuvwxyz"];
        assert_eq!(Solution::max_length(arr), 26);
    }
    #[test]
    fn xyz_abcdefgha_produces_3() {
        let arr = vs!["xyz", "abcdefgha"];
        assert_eq!(Solution::max_length(arr), 3);
    }
    #[test]
    fn xx_aa_produces_0() {
        let arr = vs!["xx", "aa"];
        assert_eq!(Solution::max_length(arr), 0);
    }
    #[test]
    fn xx_ab_produces_2() {
        let arr = vs!["xx", "ab"];
        assert_eq!(Solution::max_length(arr), 2);
    }
    #[test]
    fn a_ab_produces_2() {
        let arr = vs!["a", "ab"];
        assert_eq!(Solution::max_length(arr), 2);
    }
    #[test]
    fn a_abc_d_de_def_produces_6() {
        let arr = vs!["a", "abc", "d", "de", "def"];
        assert_eq!(Solution::max_length(arr), 6);
    }
}
