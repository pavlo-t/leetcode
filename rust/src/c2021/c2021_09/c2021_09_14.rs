#![allow(dead_code)]
/// Reverse Only Letters
/// ====================
///
/// Given a string `s`, reverse the string according to the following rules:
///
/// - All the characters that are not English letters remain in the same position.
/// - All the English letters (lowercase or uppercase) should be reversed.
///
/// Return `s` _after reversing it_.
///
/// Constraints:
///
/// - `1 <= s.length <= 100`
/// - `s` consists of characters with ASCII values in the range `[33, 122]`.
/// - `s` does not contain `'\"'` or `'\\'`.
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/637/week-2-september-8th-september-14th/3974/
struct Solution;
impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut bs = s.into_bytes();
        let (mut l, mut r) = (0, bs.len() - 1);
        while l < r {
            if !bs[l].is_ascii_alphabetic() {
                l += 1;
            } else if !bs[r].is_ascii_alphabetic() {
                r -= 1;
            } else {
                bs.swap(l, r);
                l += 1;
                r -= 1;
            }
        }
        unsafe { String::from_utf8_unchecked(bs) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s = "ab-cd".to_string();
        let e = "dc-ba".to_string();
        assert_eq!(Solution::reverse_only_letters(s), e);
    }
    #[test]
    fn example2() {
        let s = "a-bC-dEf-ghIj".to_string();
        let e = "j-Ih-gfE-dCba".to_string();
        assert_eq!(Solution::reverse_only_letters(s), e);
    }
    #[test]
    fn example3() {
        let s = "Test1ng-Leet=code-Q!".to_string();
        let e = "Qedo1ct-eeLg=ntse-T!".to_string();
        assert_eq!(Solution::reverse_only_letters(s), e);
    }
}
