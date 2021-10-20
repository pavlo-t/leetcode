#![allow(dead_code)]
/// 151. Reverse Words in a String
/// ==============================
///
/// Given an input string `s`, reverse the order of the __words__.
///
/// A __word__ is defined as a sequence of non-space characters.
/// The __words__ in `s` will be separated by at least one space.
///
/// Return _a string of the words in reverse order concatenated by a single space_.
///
/// __Note__ that `s` may contain leading or trailing spaces or multiple spaces between two words.
/// The returned string should only have a single space separating the words.
/// Do not include any extra spaces.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 10_000`
/// - `s` contains English letters (upper-case and lower-case), digits, and spaces `' '`.
/// - There is __at least one word__ in `s`.
///
/// __Follow-up:__ If the string data type is mutable in your language,
/// can you solve it __in-place__ with `O(1)` extra space?
///
/// https://leetcode.com/problems/reverse-words-in-a-string/
struct Solution;
impl Solution {
    /// Approach 2: Reverse the Whole String and Then Reverse Each Word
    /// https://leetcode.com/problems/reverse-words-in-a-string/solution/
    pub fn reverse_words(mut s: String) -> String {
        println!("reverse_words({})", s);
        unsafe {
            let v = s.as_mut_vec();
            v.reverse();
            let n = v.len();
            let mut idx = 0;
            let mut start = 0;
            while start < n {
                if v[start] != b' ' {
                    // add a single space after previous word
                    if idx != 0 {
                        v[idx] = b' ';
                        idx += 1;
                    }

                    // write word at idx
                    let mut end = start;
                    while end < n && v[end] != b' ' {
                        v[idx] = v[end];
                        idx += 1;
                        end += 1;
                    }

                    // reverse the word
                    let (mut l, mut r) = (idx - (end - start), idx - 1);
                    while l < r {
                        v.swap(l, r);
                        l += 1;
                        r -= 1;
                    }

                    // move to the next word
                    start = end;
                }
                start += 1;
            }
            v.truncate(idx);
            v.shrink_to_fit();
        }
        s
    }

    pub fn reverse_words_builtins(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }

    pub fn reverse_words_my(s: String) -> String {
        println!("reverse_words({})", s);
        let mut result = String::new();
        let mut word = Vec::new();
        let mut chars = s.chars().rev();
        while let Some(c) = chars.next() {
            if c != ' ' {
                word.push(c);
                while let Some(c) = chars.next() {
                    if c == ' ' {
                        while let Some(c) = word.pop() {
                            result.push(c);
                        }
                        result.push(' ');
                        break;
                    } else {
                        word.push(c);
                    }
                }
            }
        }
        if word.is_empty() {
            result.pop();
        } else {
            while let Some(c) = word.pop() {
                result.push(c);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(Solution::reverse_words("a".to_string()), "a");
    }
    #[test]
    fn abc() {
        assert_eq!(Solution::reverse_words("abc".to_string()), "abc");
    }
    #[test]
    fn the_sky_is_blue() {
        let s = "the sky is blue".to_string();
        let e = "blue is sky the";
        assert_eq!(Solution::reverse_words(s), e);
    }
    #[test]
    fn __hello_world__() {
        let s = "  hello world  ".to_string();
        let e = "world hello";
        assert_eq!(Solution::reverse_words(s), e);
        // Explanation: Your reversed string should not contain leading or trailing spaces.
    }
    #[allow(non_snake_case)]
    #[test]
    fn a_good___example() {
        let s = "a good   example".to_string();
        let e = "example good a";
        assert_eq!(Solution::reverse_words(s), e);
        // Explanation: You need to reduce multiple spaces between two words to a single space in the reversed string.
    }
    #[allow(non_snake_case)]
    #[test]
    fn __Bob____Loves__Alice___() {
        let s = "  Bob    Loves  Alice   ".to_string();
        let e = "Alice Loves Bob";
        assert_eq!(Solution::reverse_words(s), e);
    }
    #[allow(non_snake_case)]
    #[test]
    fn Alice_does_not_even_like_bob() {
        let s = "Alice does not even like bob".to_string();
        let e = "bob like even not does Alice";
        assert_eq!(Solution::reverse_words(s), e);
    }

    #[test]
    fn ax10000() {
        let s = "a".repeat(10000);
        let e = "a".repeat(10000);
        assert_eq!(Solution::reverse_words(s), e);
    }
}
