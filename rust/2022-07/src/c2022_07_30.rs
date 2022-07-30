#![allow(dead_code)]
//! \#916. Word Subsets
//! ===================
//!
//! <https://leetcode.com/problems/word-subsets>
//!
//! You are given two string arrays `words1` and `words2`.
//!
//! A string `b` is a __subset__ of string `a` if every letter in `b` occurs in `a` including multiplicity.
//!
//! - For example, `"wrr"` is a subset of `"warrior"` but is not a subset of `"world"`.
//!
//! A string `a` from `words1` is __universal__ if for every string `b` in `words2`, `b` is a subset of `a`.
//!
//! Return an array of all the __universal__ strings in `words1`.
//! You may return the answer in __any order__.
//!
//! __Constraints:__
//!
//! - `1 <= words1.length, words2.length <= 10_000`
//! - `1 <= words1[i].length, words2[i].length <= 10`
//! - `words1[i]` and `words2[i]` consist only of lowercase English letters.
//! - All the strings of `words1` are __unique__.
//!
//! # Examples
//!
//! ```
//! # use c2022_07::c2022_07_30::Solution;
//! # macro_rules! vs { ($($x:expr),*) => { vec![$($x.to_string()),*] }; }
//! let words1 = vs!["amazon","apple","facebook","google","leetcode"];
//! let words2 = vs!["e","o"];
//! let expected = vs!["facebook","google","leetcode"];
//! assert_eq!(Solution::word_subsets(words1, words2), expected);
//! ```
//!
//! ```
//! # use c2022_07::c2022_07_30::Solution;
//! # macro_rules! vs { ($($x:expr),*) => { vec![$($x.to_string()),*] }; }
//! let words1 = vs!["amazon","apple","facebook","google","leetcode"];
//! let words2 = vs!["l","e"];
//! let expected = vs!["apple","google","leetcode"];
//! assert_eq!(Solution::word_subsets(words1, words2), expected);
//! ```
//!
//! ```
//! # use c2022_07::c2022_07_30::Solution;
//! let words1 = vec!["a".repeat(10); 10_000];
//! let words2 = vec!["a".repeat(10); 10_000];
//! let expected = vec!["a".repeat(10); 10_000];
//! assert_eq!(Solution::word_subsets(words1, words2), expected);
//! ```

pub struct Solution;
impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        fn counts(w: &str) -> [usize; 26] {
            w.bytes()
                .map(|b| (b - b'a') as usize)
                .fold([0; 26], |mut counts, i| {
                    counts[i] += 1;
                    counts
                })
        }

        let counts2 = words2
            .iter()
            .map(|w| counts(w))
            .fold([0; 26], |mut acc, curr| {
                (0..26).for_each(|i| acc[i] = acc[i].max(curr[i]));
                acc
            });

        words1
            .into_iter()
            .filter(|w1| {
                let counts1 = counts(w1);
                (0..26).all(|i| counts2[i] <= counts1[i])
            })
            .collect()
    }
}
