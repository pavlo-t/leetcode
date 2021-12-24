#![allow(dead_code)]
/// 1153. String Transforms Into Another String
/// ===========================================
///
/// Given two strings `str1` and `str2` of the same length,
/// determine whether you can transform `str1` into `str2` by doing __zero or more__ _conversions_.
///
/// In one conversion you can convert __all__ occurrences of one character in `str1`
/// to __any__ other lowercase English character.
///
/// Return `true` if and only if you can transform `str1` into `str2`.
///
/// __Constraints:__
///
/// - `1 <= str1.length == str2.length <= 10_000`
/// - `str1` and `str2` contain only lowercase English letters.
///
/// https://leetcode.com/problems/string-transforms-into-another-string/
struct Solution;
impl Solution {
    pub fn can_convert(str1: String, str2: String) -> bool {
        println!("can_convert({}, {})", str1, str2);
        str1 == str2 || {
            const SEEN_ALL: i32 = (1 << 26) - 1;

            let mut byte_map_1 = vec![vec![]; 26];
            for (i, b) in str1.bytes().enumerate() {
                byte_map_1[(b - b'a') as usize].push(i);
            }

            let mut seen_bytes_2 = 0;
            let bytes_2 = str2.as_bytes();
            for mut idxs in byte_map_1 {
                if let Some(i) = idxs.pop() {
                    seen_bytes_2 |= 1 << (bytes_2[i] - b'a');
                    while let Some(j) = idxs.pop() {
                        seen_bytes_2 |= 1 << (bytes_2[j] - b'a');
                        if bytes_2[j] != bytes_2[i] {
                            return false;
                        }
                    }
                }
            }

            seen_bytes_2 != SEEN_ALL
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn a_a() { assert!(Solution::can_convert("a".into(), "a".into())); }
    #[rustfmt::skip] #[test] fn a_b() { assert!(Solution::can_convert("a".into(), "b".into())); }

    #[rustfmt::skip] #[test] fn aa_aa() { assert!(Solution::can_convert("aa".into(), "aa".into())); }
    #[rustfmt::skip] #[test] fn bb_aa() { assert!(Solution::can_convert("bb".into(), "aa".into())); }
    #[rustfmt::skip] #[test] fn ab_aa() { assert!(Solution::can_convert("ab".into(), "aa".into())); }
    #[rustfmt::skip] #[test] fn ba_aa() { assert!(Solution::can_convert("ba".into(), "aa".into())); }
    #[rustfmt::skip] #[test] fn ab_ba() { assert!(Solution::can_convert("ab".into(), "ba".into())); }

    #[rustfmt::skip] #[test] fn aa_ab() { assert!(!Solution::can_convert("aa".into(), "ab".into())); }
    #[rustfmt::skip] #[test] fn aa_ba() { assert!(!Solution::can_convert("aa".into(), "ba".into())); }

    #[test]
    fn aabcc_ccdee() {
        let a = "aabcc".to_string();
        let b = "ccdee".to_string();
        assert!(Solution::can_convert(a, b));
        // Explanation: Convert 'c' to 'e' then 'b' to 'd' then 'a' to 'c'. Note that the order of conversions matter.
    }
    #[test]
    fn leetcode_code_leet() {
        let a = "leetcode".to_string();
        let b = "codeleet".to_string();
        assert!(!Solution::can_convert(a, b));
        // Explanation: There is no way to transform str1 to str2.
    }

    #[test]
    fn abcdefghijklmnopqrstuvwxyz_abcdefghijklmnopqrstuvwxyz() {
        let a = "abcdefghijklmnopqrstuvwxyz".to_string();
        let b = "abcdefghijklmnopqrstuvwxyz".to_string();
        assert!(Solution::can_convert(a, b));
    }
    #[test]
    fn test_37_abcdefghijklmnopqrstuvwxyz_bcdefghijklmnopqrstuvwxyza() {
        let a = "abcdefghijklmnopqrstuvwxyz".to_string();
        let b = "bcdefghijklmnopqrstuvwxyza".to_string();
        assert!(!Solution::can_convert(a, b));
    }
}
