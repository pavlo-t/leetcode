#![allow(dead_code)]
//! \#387. First Unique Character in a String
//! =========================================
//!
//! <https://leetcode.com/problems/first-unique-character-in-a-string>
//!
//! Given a string `s`, _find the first non-repeating character in it and return its index_.
//! If it does not exist, return `-1`.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_08::c2022_08_16::*;
//! assert_eq!(Solution::first_uniq_char("leetcode".into()), 0);
//! ```
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_16::*;
//! assert_eq!(Solution::first_uniq_char("loveleetcode".into()), 2);
//! ```
//!
//! ###### Example 3
//!
//! ```
//! # use c2022_08::c2022_08_16::*;
//! assert_eq!(Solution::first_uniq_char("aabb".into()), -1);
//! ```
//!
//! ##### Constraints
//!
//! - `1 <= s.length <= 100_000`
//! - `s` consists of only lowercase English letters.

pub struct Solution;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        #[derive(Copy, Clone)]
        enum Count {
            NotSeen,
            SeenOnce(usize),
            SeenMoreThanOnce,
        }
        use Count::*;

        s.bytes()
            .map(|b| (b - b'a') as usize)
            .enumerate()
            .fold([NotSeen; 26], |mut counts, (i, b)| {
                counts[b] = match counts[b] {
                    NotSeen => SeenOnce(i),
                    _       => SeenMoreThanOnce,
                };
                counts
            })
            .into_iter()
            .filter_map(|count| match count {
                SeenOnce(i) => Some(i as i32),
                _           => None,
            })
            .min()
            .unwrap_or(-1)
    }
}
