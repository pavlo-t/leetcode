#![allow(dead_code)]
//! \#242. Valid Anagram
//! ====================
//!
//! <https://leetcode.com/problems/valid-anagram>
//!
//! Given two strings `s` and `t`, return _`true` if `t` is an anagram of `s`, and `false` otherwise_.
//!
//! An __Anagram__ is a word or phrase formed by rearranging the letters of a different word or phrase,
//! typically using all the original letters exactly once.
//!
//! __Constraints:__
//!
//! - `1 <= s.length, t.length <= 50_000`
//! - `s` and `t` consist of lowercase English letters.
//!
//! __Follow up:__ What if the inputs contain Unicode characters? How would you adapt your solution to such a case?

pub struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::collections::HashMap;

        fn char_counts(s: &str) -> HashMap<char, usize> {
            s.chars().fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_default() += 1;
                map
            })
        }

        char_counts(&s) == char_counts(&t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn anagram_nagaram() { assert_eq!(Solution::is_anagram("anagram".into(), "nagaram".into()), true ); }
    #[rustfmt::skip] #[test] fn anagram_nagarm()  { assert_eq!(Solution::is_anagram("anagram".into(), "nagarm".into() ), false); }
    #[rustfmt::skip] #[test] fn rat_car()         { assert_eq!(Solution::is_anagram("rat".into()    , "car".into()    ), false); }
}
