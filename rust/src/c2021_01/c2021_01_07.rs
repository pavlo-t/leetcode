#![allow(dead_code)]

/// ### Longest Substring Without Repeating Characters
/// https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3595/
struct Solution;


impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;

        s.chars().enumerate()
            .fold((0, HashMap::new(), 0),
                  |(rsf, mut chars, l), (r, c)| {
                      let l = match chars.insert(c, r) {
                          None => l,
                          Some(j) => l.max(j + 1),
                      };
                      (rsf.max(r - l + 1), chars, l)
                  }).0 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_s_abcabcbb_is_3() {
        let s = "abcabcbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3);
        // Explanation: The answer is "abc", with the length of 3.
    }

    #[test]
    fn example2_s_bbbbb_is_1() {
        let s = "bbbbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 1);
        // Explanation: The answer is "b", with the length of 1.
    }

    #[test]
    fn example3_s_pwwkew_is_3() {
        let s = "pwwkew".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3);
        // Explanation: The answer is "wke", with the length of 3.
        // Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
    }

    #[test]
    fn example4_s_empty_is_0() {
        let s = "".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 0);
    }

    #[test]
    fn test139_abba_is_2() {
        let s = "abba".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 2);
    }

    #[test]
    fn s_a_repeat50000_is_1() {
        let s = "a".repeat(50000);
        assert_eq!(Solution::length_of_longest_substring(s), 1);
    }

    #[test]
    fn s_a_to_z_repeat1924_50024_is_26() {
        let s =
            ('a'..='z')
                .fold(String::new(), |mut s, c| {
                    s.push(c);
                    s
                }).repeat(1924);
        assert_eq!(Solution::length_of_longest_substring(s), 26);
    }
}
