#![allow(dead_code)]
/// 438. Find All Anagrams in a String
/// ==================================
///
/// Given two strings `s` and `p`, return _an array of all the start indices of `p`'s anagrams in `s`_.
/// You may return the answer in __any order__.
///
/// An __Anagram__ is a word or phrase formed by rearranging the letters of a different word or phrase,
/// typically using all the original letters exactly once.
///
/// __Constraints:__
///
/// - `1 <= s.length, p.length <= 30_000`
/// - `s` and `p` consist of lowercase English letters.
///
/// https://leetcode.com/problems/find-all-anagrams-in-a-string/
struct Solution;
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        fn byte_to_index(b: u8) -> usize {
            (b - b'a') as usize
        }

        let p_counts = {
            let mut p_counts = [0; 26];
            p.bytes().map(byte_to_index).for_each(|i| p_counts[i] += 1);
            p_counts
        };
        let mut s_counts = {
            let mut s_counts = [0; 26];
            s.bytes()
                .take(p.len())
                .map(byte_to_index)
                .for_each(|i| s_counts[i] += 1);
            s_counts
        };
        let mut results = vec![];

        for i in 0..s.len().saturating_sub(p.len()) {
            if s_counts == p_counts {
                results.push(i as i32);
            }
            s_counts[byte_to_index(s.as_bytes()[i])] -= 1;
            s_counts[byte_to_index(s.as_bytes()[i + p.len()])] += 1;
        }
        if s_counts == p_counts {
            results.push((s.len() - p.len()) as i32);
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abdef_abc() {
        let s = "abdef".to_string();
        let p = "abc".to_string();
        assert_eq!(Solution::find_anagrams(s, p), vec![]);
    }
    #[test]
    fn ab_abc() {
        let s = "ab".to_string();
        let p = "abc".to_string();
        assert_eq!(Solution::find_anagrams(s, p), vec![]);
    }
    #[test]
    fn abcabc_a() {
        let s = "abcabc".to_string();
        let p = "a".to_string();
        assert_eq!(Solution::find_anagrams(s, p), vec![0, 3]);
    }
    #[test]
    fn abcabc_abc() {
        let s = "abcabc".to_string();
        let p = "abc".to_string();
        assert_eq!(Solution::find_anagrams(s, p), vec![0, 1, 2, 3]);
    }

    #[test]
    fn cbaebabacd_abc() {
        let s = "cbaebabacd".to_string();
        let p = "abc".to_string();
        assert_eq!(Solution::find_anagrams(s, p), vec![0, 6]);
        // Explanation:
        // The substring with start index = 0 is "cba", which is an anagram of "abc".
        // The substring with start index = 6 is "bac", which is an anagram of "abc".
    }
    #[test]
    fn abab_ab() {
        let s = "abab".to_string();
        let p = "ab".to_string();
        assert_eq!(Solution::find_anagrams(s, p), vec![0, 1, 2]);
        // Explanation:
        // The substring with start index = 0 is "ab", which is an anagram of "ab".
        // The substring with start index = 1 is "ba", which is an anagram of "ab".
        // The substring with start index = 2 is "ab", which is an anagram of "ab".
    }
}
