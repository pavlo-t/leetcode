#![allow(dead_code)]
/// 567. Permutation in String
/// ==========================
///
/// Given two strings `s1` and `s2`, return _`true` if `s2` contains a permutation of `s1`, or `false` otherwise_.
///
/// In other words, return `true` if one of `s1`'s permutations is the substring of `s2`.
///
/// __Constraints:__
///
/// - `1 <= s1.length, s2.length <= 10_000`
/// - `s1` and `s2` consist of lowercase English letters.
///
/// https://leetcode.com/problems/permutation-in-string/
struct Solution;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        println!("check_inclusion({s1}, {s2})");
        if s1.len() > s2.len() {
            false
        } else {
            fn byte_to_idx(b: u8) -> usize {
                (b - b'a') as usize
            }
            fn add_b(mut rsf: [usize; 26], b: u8) -> [usize; 26] {
                rsf[byte_to_idx(b)] += 1;
                rsf
            }

            let counts_1 = s1.bytes().fold([0; 26], add_b);
            let mut counts_2 = s2.bytes().take(s1.len()).fold([0; 26], add_b);
            if counts_2 == counts_1 {
                return true;
            }
            for i in s1.len()..s2.len() {
                counts_2[byte_to_idx(s2.as_bytes()[i - s1.len()])] -= 1;
                counts_2[byte_to_idx(s2.as_bytes()[i])] += 1;
                if counts_2 == counts_1 {
                    return true;
                }
            }

            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ab_eidbaooo() {
        let s1 = "ab".to_string();
        let s2 = "eidbaooo".to_string();
        assert_eq!(Solution::check_inclusion(s1, s2), true);
        // Explanation: s2 contains one permutation of s1 ("ba").
    }
    #[test]
    fn ab_eidboaoo() {
        let s1 = "ab".to_string();
        let s2 = "eidboaoo".to_string();
        assert_eq!(Solution::check_inclusion(s1, s2), false);
    }

    #[test]
    fn ab_ab() {
        let s1 = "ab".to_string();
        let s2 = "ab".to_string();
        assert_eq!(Solution::check_inclusion(s1, s2), true);
    }
    #[test]
    fn ab_ba() {
        let s1 = "ab".to_string();
        let s2 = "ba".to_string();
        assert_eq!(Solution::check_inclusion(s1, s2), true);
    }
    #[test]
    fn ab_ca() {
        let s1 = "ab".to_string();
        let s2 = "ca".to_string();
        assert_eq!(Solution::check_inclusion(s1, s2), false);
    }

    #[test]
    fn a_repeat_10000_a_repeat_10000() {
        let s1 = "a".repeat(10000);
        let s2 = "a".repeat(10000);
        assert_eq!(Solution::check_inclusion(s1, s2), true);
    }
    #[test]
    fn a_b_repeat_10000() {
        let s1 = "a".to_string();
        let s2 = "b".repeat(10000);
        assert_eq!(Solution::check_inclusion(s1, s2), false);
    }
}
