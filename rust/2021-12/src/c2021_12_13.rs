#![allow(dead_code)]
/// 1446. Consecutive Characters
/// ============================
///
/// The __power__ of the string is the maximum length of a non-empty substring that contains only one unique character.
///
/// Given a string `s`, return _the __power__ of `s`_.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 500`
/// - `s` consists of only lowercase English letters.
///
/// https://leetcode.com/problems/consecutive-characters/
struct Solution;
impl Solution {
    pub fn max_power(s: String) -> i32 {
        println!("max_power({})", s);
        s.chars()
            .fold((0, 0, '#'), |(max, curr, prev_ch), ch| {
                let next = if ch == prev_ch { curr + 1 } else { 1 };
                (max.max(next), next, ch)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode() {
        assert_eq!(Solution::max_power("leetcode".into()), 2);
        // Explanation: The substring "ee" is of length 2 with the character 'e' only.
    }
    #[test]
    fn abbcccddddeeeeedcba() {
        assert_eq!(Solution::max_power("abbcccddddeeeeedcba".into()), 5);
        // Explanation: The substring "eeeee" is of length 5 with the character 'e' only.
    }
    #[test]
    fn triplepillooooow() {
        assert_eq!(Solution::max_power("triplepillooooow".into()), 5);
    }
    #[test]
    fn hooraaaaaaaaaaay() {
        assert_eq!(Solution::max_power("hooraaaaaaaaaaay".into()), 11);
    }
    #[test]
    fn tourist() {
        assert_eq!(Solution::max_power("tourist".into()), 1);
    }
}
