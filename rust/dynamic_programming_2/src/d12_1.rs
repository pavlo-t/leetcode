#![allow(dead_code)]
/// 1055. Shortest Way to Form String
/// =================================
///
/// A __subsequence__ of a string is a new string that is formed from the original string by deleting some
/// (can be none) of the characters without disturbing the relative positions of the remaining characters.
/// (i.e., `"ace"` is a subsequence of `"abcde"` while `"aec"` is not).
///
/// Given two strings `source` and `target`,
/// return _the minimum number of __subsequences__ of `source` such that their concatenation equals `target`_.
/// If the task is impossible, return `-1`.
///
/// Constraints:
///
/// - `1 <= source.length, target.length <= 1000`
/// - `source` and `target` consist of lowercase English letters.
///
/// https://leetcode.com/problems/shortest-way-to-form-string/
struct Solution;
impl Solution {
    pub fn shortest_way_rec(source: String, target: String) -> i32 {
        println!("shortest_way({}, {})", source, target);
        let sbs = source.bytes().fold(0, |rsf, b| rsf | (1 << (b - b'a')));
        if !target.bytes().all(|b| sbs & (1 << (b - b'a')) != 0) {
            -1
        } else {
            fn rec(si: usize, ti: usize, s: &[u8], t: &[u8]) -> i32 {
                if ti == t.len() {
                    1
                } else if si == s.len() {
                    1 + rec(0, ti, s, t)
                } else if s[si] == t[ti] {
                    rec(si + 1, ti + 1, s, t)
                } else {
                    rec(si + 1, ti, s, t)
                }
            }
            let (s, t) = (source.as_bytes(), target.as_bytes());
            rec(0, 0, s, t)
        }
    }
    pub fn shortest_way(source: String, target: String) -> i32 {
        println!("shortest_way({}, {})", source, target);
        let sbs = source.bytes().fold(0, |rsf, b| rsf | (1 << (b - b'a')));
        if !target.bytes().all(|b| sbs & (1 << (b - b'a')) != 0) {
            -1
        } else {
            let (s, t) = (source.as_bytes(), target.as_bytes());
            let mut result = 1;
            let (mut si, mut ti) = (0, 0);
            while ti < t.len() {
                if si == s.len() {
                    si = 0;
                    result += 1;
                } else if s[si] == t[ti] {
                    si += 1;
                    ti += 1;
                } else {
                    si += 1
                }
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn a_a()  { assert_eq!(Solution::shortest_way("a".into(), "a".into()), 1); }
    #[rustfmt::skip] #[test] fn aa_a() { assert_eq!(Solution::shortest_way("aa".into(), "a".into()), 1); }
    #[rustfmt::skip] #[test] fn a_aa() { assert_eq!(Solution::shortest_way("a".into(), "aa".into()), 2); }

    #[test]
    fn abc_abcbc() {
        assert_eq!(Solution::shortest_way("abc".into(), "abcbc".into()), 2);
        // Explanation: The target "abcbc" can be formed by "abc" and "bc", which are subsequences of source "abc".
    }
    #[test]
    fn abc_acdbc() {
        assert_eq!(Solution::shortest_way("abc".into(), "acdbc".into()), -1);
        // Explanation: The target string cannot be constructed from the subsequences of source string
        // due to the character "d" in target string.
    }
    #[test]
    fn xyz_xzyxz() {
        assert_eq!(Solution::shortest_way("xyz".into(), "xzyxz".into()), 3);
        // Explanation: The target string can be constructed as follows "xz" + "y" + "xz".
    }

    /// If getting stack overflow: add RUST_MIN_STACK=134217728 (2 ** 27) to env:
    /// RUST_MIN_STACK=134217728 cargo test --lib d12_1
    #[test]
    fn a_repeat_999_b_ab_repeat_50() {
        let mut s = "a".repeat(999);
        s.push('b');
        let t = "ab".repeat(500);
        assert_eq!(Solution::shortest_way(s, t), 500);
    }
}
