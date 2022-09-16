#![allow(dead_code)]
//! \#159. Longest Substring with At Most Two Distinct Characters
//! =============================================================
//!
//! <https://leetcode.com/problems/longest-substring-with-at-most-two-distinct-characters>
//!
//! Given a string `s`, return _the length of the longest substring that contains at most __two distinct characters___.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_09::c2022_09_w3::*;
//! assert_eq!(Solution::length_of_longest_substring_two_distinct("eceba".into()), 3);
//! ```
//!
//! __Explanation:__ The substring is `"ece"` which its length is `3`.
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_09::c2022_09_w3::*;
//! assert_eq!(Solution::length_of_longest_substring_two_distinct("ccaabbb".into()), 5);
//! ```
//!
//! __Explanation:__ The substring is `"aabbb"` which its length is `5`.
//!
//! ##### Constraints
//!
//! - `1 <= s.length <= 100_000`
//! - `s` consists of English letters.

pub struct Solution;
impl Solution {
    pub fn length_of_longest_substring_two_distinct_v1(s: String) -> i32 {
        use std::collections::HashMap;

        let mut max_len = 0;

        let bytes = s.as_bytes();
        let (mut l, mut r) = (0, 0);
        let mut counts = HashMap::new();
        while r < bytes.len() {
            counts.entry(bytes[r]).and_modify(|c| *c += 1).or_insert(1);
            while counts.len() > 2 {
                counts.entry(bytes[l]).and_modify(|count| *count -= 1);
                if let Some(0) = counts.get(&bytes[l]) {
                    counts.remove(&bytes[l]);
                }
                l += 1;
            }
            max_len = max_len.max(r - l + 1);
            r += 1;
        }

        max_len as i32
    }

    pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
        let mut max_len = 0;

        let bytes = s.as_bytes();
        let (mut l, mut r) = (0, 0);
        let mut counts = [0; 128];
        let mut distinct_chars = 0;
        while r < bytes.len() {
            if counts[bytes[r] as usize] == 0 {
                distinct_chars += 1;
            }
            counts[bytes[r] as usize] += 1;
            while distinct_chars > 2 {
                counts[bytes[l] as usize] -= 1;
                if counts[bytes[l] as usize] == 0 {
                    distinct_chars -= 1;
                }
                l += 1;
            }
            max_len = max_len.max(r - l + 1);
            r += 1;
        }

        max_len as i32
    }
}
