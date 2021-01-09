#![allow(dead_code)]

/// ### Palindrome Permutation
/// https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3588/
struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        use std::collections::HashMap;

        s.chars()
            .fold((0, HashMap::new()), |(odds, mut acc), c| {
                let e = acc.entry(c).or_insert(false);
                if *e {
                    *e = false;
                    (odds - 1, acc)
                } else {
                    *e = true;
                    (odds + 1, acc)
                }
            }).0 < 2
    }

    pub fn can_permute_palindrome_hash_map_char_is_odd(s: String) -> bool {
        use std::collections::HashMap;

        s.chars()
            .fold(HashMap::new(), |mut acc, c| {
                let e = acc.entry(c).or_insert(false);
                *e = if *e { false } else { true };
                acc
            })
            .values()
            .fold(0, |acc, &v| if v { acc + 1 } else { acc }) < 2
    }

    pub fn can_permute_palindrome_hash_map_char_count(s: String) -> bool {
        use std::collections::HashMap;

        s.chars()
            .fold(HashMap::new(), |mut counts, c| {
                *counts.entry(c).or_insert(0) += 1;
                counts
            })
            .values()
            .fold(0, |acc, &v| if v % 2 == 0 { acc } else { acc + 1 }) < 2
    }

    pub fn can_permute_palindrome_hash_set(s: String) -> bool {
        use std::collections::HashSet;

        s.chars()
            .fold(HashSet::new(), |mut acc, c| {
                if !acc.insert(c) { acc.remove(&c); }
                acc
            }).len() < 2
    }

    pub fn can_permute_palindrome_hash_set_contains_remove_insert(s: String) -> bool {
        use std::collections::HashSet;

        s.chars()
            .fold(HashSet::new(), |mut acc, c| {
                if acc.contains(&c) {
                    acc.remove(&c);
                } else {
                    acc.insert(c);
                }
                acc
            }).len() < 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_s_code_is_false() {
        let s = "code".to_string();
        assert!(!Solution::can_permute_palindrome(s));
    }

    #[test]
    fn example2_s_aab_is_true() {
        let s = "aab".to_string();
        assert!(Solution::can_permute_palindrome(s));
    }

    #[test]
    fn example3_s_carerac_is_true() {
        let s = "carerac".to_string();
        assert!(Solution::can_permute_palindrome(s));
    }

    #[test]
    fn s_a_repeat50000_is_true() {
        let s = "a".repeat(50000);
        assert!(Solution::can_permute_palindrome(s));
    }

    #[test]
    fn s_a_to_z_repeat1924_50024_is_true() {
        let s = ('a'..='z').collect::<String>().repeat(1924);
        assert!(Solution::can_permute_palindrome(s));
    }

    #[test]
    fn s_a_to_z_repeat1925_50050_is_false() {
        let s = ('a'..='z').collect::<String>().repeat(1925);
        assert!(!Solution::can_permute_palindrome(s));
    }
}
