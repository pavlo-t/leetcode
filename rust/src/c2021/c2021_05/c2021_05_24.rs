#![allow(dead_code)]

/// To Lower Case
/// =============
///
/// Given a string `s`,
/// return _the string after replacing every uppercase letter with the same lowercase letter_.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 100`
/// - `s` consists of printable ASCII characters.
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/601/week-4-may-22nd-may-28th/3754/
struct Solution;
//noinspection RsSelfConvention
impl Solution {
    pub fn to_lower_case(mut s: String) -> String {
        const DIFF: u8 = b'a' - b'A';
        unsafe {
            for b in s.as_bytes_mut() {
                if &b'A' <= b && &b'Z' >= b {
                    *b += DIFF;
                }
            }
        }
        s
    }
    pub fn to_lower_case_build_new_string(s: String) -> String {
        const DIFF: u8 = b'a' - b'A';
        String::from_utf8(
            s.bytes()
                .map(|b| if b >= b'A' && b <= b'Z' { b + DIFF } else { b })
                .collect(),
        )
        .unwrap()
    }
    pub fn to_lower_case_bytes_to_lowercase(s: String) -> String {
        String::from_utf8(s.bytes().map(|b| b.to_ascii_lowercase()).collect()).unwrap()
    }
    pub fn to_lower_case_char_to_lowercase(s: String) -> String {
        s.chars().map(|c| c.to_ascii_lowercase()).collect()
    }
    pub fn to_lower_case_string_to_lowercase(s: String) -> String {
        s.to_ascii_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_hello() {
        assert_eq!(Solution::to_lower_case("Hello".to_string()), "hello");
    }
    #[test]
    fn example2_here() {
        assert_eq!(Solution::to_lower_case("here".to_string()), "here");
    }
    #[test]
    fn example3_lovely() {
        assert_eq!(Solution::to_lower_case("LOVELY".to_string()), "lovely");
    }
}
