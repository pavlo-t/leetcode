#![allow(dead_code)]
//! \#383. Ransom Note
//! ==================
//!
//! <https://leetcode.com/problems/ransom-note>
//!
//! Given two strings `ransom_note` and `magazine`,
//! return _`true` if `ransom_note` can be constructed by using the letters from `magazine` and `false` otherwise_.
//!
//! Each letter in `magazine` can only be used once in `ransom_note`.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_08::c2022_08_25::*;
//! assert_eq!(Solution::can_construct("a".into(), "b".into()), false);
//! ```
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_25::*;
//! assert_eq!(Solution::can_construct("aa".into(), "ab".into()), false);
//! ```
//!
//! ###### Example 3
//!
//! ```
//! # use c2022_08::c2022_08_25::*;
//! assert_eq!(Solution::can_construct("aa".into(), "aab".into()), true);
//! ```
//!
//! ##### Constraints
//!
//! - `1 <= ransom_note.length, magazine.length <= 100_000`
//! - `ransom_note` and `magazine` consist of lowercase English letters.

pub struct Solution;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        fn count_letters(s: &str) -> [usize; 26] {
            s.bytes()
                .map(|b| (b - b'a') as usize)
                .fold([0; 26], |mut counts, i| {
                    counts[i] += 1;
                    counts
                })
        }

        let ransom_note_counts = count_letters(&ransom_note);
        let magazine_counts = count_letters(&magazine);

        ransom_note_counts
            .iter()
            .enumerate()
            .all(|(i, &count)| count <= magazine_counts[i])
    }
}
