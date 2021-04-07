#![allow(dead_code, non_snake_case)]

/// Determine if String Halves Are Alike
/// ====================================
///
/// You are given a string `s` of even length.
/// Split this string into two halves of equal lengths,
/// and let `a` be the first half and `b` be the second half.
///
/// Two strings are __alike__ if they have the same number of vowels
/// (`'a'`, `'e'`, `'i'`, `'o'`, `'u'`, `'A'`, `'E'`, `'I'`, `'O'`, `'U'`).
/// Notice that `s` contains uppercase and lowercase letters.
///
/// Return `true` _if_ `a` _and_ `b` _are __alike___.
/// Otherwise, return `false`.
///
/// __Constraints:__
///
/// - `2 <= s.length <= 1000`
/// - `s.length` is even.
/// - `s` consists of __uppercase and lowercase__ letters.
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/593/week-1-april-1st-april-7th/3699/
struct Solution;
impl Solution {
    pub fn halves_are_alike_chars(s: String) -> bool {
        fn count_vowels(s: &str) -> usize {
            s.chars().fold(0, |acc, c| match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => acc + 1,
                _ => acc,
            })
        }
        let half = s.chars().count() / 2;

        count_vowels(&s[0..half]) == count_vowels(&s[half..])
    }

    pub fn halves_are_alike_bytes(s: String) -> bool {
        fn count_vowels(bs: &[u8]) -> usize {
            bs.iter().fold(0, |acc, b| match b {
                b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U' => acc + 1,
                _ => acc,
            })
        }
        let bytes = s.as_bytes();
        let half = bytes.len() / 2;

        count_vowels(&bytes[0..half]) == count_vowels(&bytes[half..])
    }

    pub fn halves_are_alike(s: String) -> bool {
        let count_vowels = |bs: &[u8]| {
            bs.iter().fold(0, |acc, b| match b {
                b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U' => acc + 1,
                _ => acc,
            })
        };
        let bytes = s.as_bytes();
        let half = bytes.len() / 2;

        count_vowels(&bytes[0..half]) == count_vowels(&bytes[half..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_s_book_produces_true() {
        assert!(Solution::halves_are_alike("book".to_string()));
        // Explanation: a = "bo" and b = "ok".
        // a has 1 vowel and b has 1 vowel. Therefore, they are alike.
    }
    #[test]
    fn example2_s_textbook_produces_false() {
        assert!(!Solution::halves_are_alike("textbook".to_string()));
        // Explanation: a = "text" and b = "book".
        // a has 1 vowel whereas b has 2. Therefore, they are not alike.
        // Notice that the vowel o is counted twice.
    }
    #[test]
    fn example3_s_MerryChristmas_produces_false() {
        assert!(!Solution::halves_are_alike("MerryChristmas".to_string()));
    }
    #[test]
    fn example4_s_AbCdEfGh_produces_true() {
        assert!(Solution::halves_are_alike("AbCdEfGh".to_string()));
    }

    mod performance {
        use super::*;
        #[test]
        fn s_1000a_produces_true() {
            assert!(Solution::halves_are_alike("a".repeat(1000)));
        }
        #[test]
        fn s_5_000_000a_produces_true() {
            assert!(Solution::halves_are_alike("a".repeat(5_000_000)));
        }
    }
}
