#![allow(dead_code)]
//! \#1647. Minimum Deletions to Make Character Frequencies Unique
//! ==============================================================
//!
//! A string `s` is called __good__ if there are no two different characters in `s` that have the same __frequency__.
//!
//! Given a string `s`, return _the __minimum__ number of characters you need to delete to make `s` __good___.
//!
//! The __frequency__ of a character in a string is the number of times it appears in the string.
//! For example, in the string `"aab"`, the __frequency__ of `'a'` is `2`, while the __frequency__ of `'b'` is `1`.
//!
//! __Constraints:__
//!
//! - `1 <= s.length <= 100_000`
//! - `s` contains only lowercase English letters.
//!
//! <https://leetcode.com/problems/minimum-deletions-to-make-character-frequencies-unique>

pub struct Solution;
impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        use std::collections::HashSet;

        let mut frequencies = vec![0i32; 26];
        s.bytes()
            .map(|b| (b - b'a') as usize)
            .for_each(|i| frequencies[i] += 1);
        frequencies.sort_unstable();

        let mut seen: HashSet<i32> = HashSet::new();
        let mut result = 0;
        for i in (0..frequencies.len()).rev() {
            let mut curr = frequencies[i];
            while seen.contains(&curr) {
                curr -= 1;
                result += 1;
            }
            if curr > 0 {
                seen.insert(curr);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aab() {
        assert_eq!(Solution::min_deletions("aab".into()), 0);
        // Explanation: s is already good.
    }
    #[test]
    fn aaabbbcc() {
        assert_eq!(Solution::min_deletions("aaabbbcc".into()), 2);
        // 2,3,3
        // Explanation: You can delete two 'b's resulting in the good string "aaabcc".
        // Another way it to delete one 'b' and one 'c' resulting in the good string "aaabbc".
    }
    #[test]
    fn ceabaacb() {
        assert_eq!(Solution::min_deletions("ceabaacb".into()), 2);
        // 1,2,2,3
        // Explanation: You can delete both 'c's resulting in the good string "eabaab".
        // Note that we only care about characters that are still in the string at the end
        // (i.e. frequency of 0 is ignored).
    }
    #[test]
    fn aabb() {
        assert_eq!(Solution::min_deletions("aabb".into()), 1);
    }
    #[test]
    fn aabbbbcccc() {
        assert_eq!(Solution::min_deletions("aabbbbcccc".into()), 1);
    }
}
