#![allow(dead_code)]
//! \#936. Stamping The Sequence
//! ============================
//!
//! <https://leetcode.com/problems/stamping-the-sequence>
//!
//! You are given two strings `stamp` and `target`.
//! Initially, there is a string `s` of length `target.length` with all `s[i] == '?'`.
//!
//! In one turn, you can place `stamp` over `s` and replace every letter in the `s`
//! with the corresponding letter from `stamp`.
//!
//! For example, if `stamp = "abc"` and `target = "abcba"`, then s is `"?????"` initially.
//! In one turn you can:
//!
//! - place `stamp` at index `0` of s to obtain `"abc??"`,
//! - place `stamp` at index `1` of s to obtain `"?abc?"`, or
//! - place `stamp` at index `2` of s to obtain `"??abc"`.
//!
//! Note that `stamp` must be fully contained in the boundaries of `s` in order to stamp
//! (i.e., you cannot place `stamp` at index `3` of `s`).
//!
//! We want to convert `s` to `target` using __at most__ `10 * target.length` turns.
//!
//! Return _an array of the index of the left-most letter being stamped at each turn_.
//! If we cannot obtain `target` from `s` within `10 * target.length` turns, return an empty array.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_08::c2022_08_21::*;
//! let stamp = "abc".to_string();
//! let target = "ababc".to_string();
//! assert_eq!(Solution::moves_to_stamp(stamp, target), [0, 2]);
//! ```
//!
//! __Explanation:__ Initially s = `"?????"`.
//!
//! - Place stamp at index 0 to get `"abc??"`.
//! - Place stamp at index 2 to get `"ababc"`.
//!
//! `[1,0,2]` would also be accepted as an answer, as well as some other answers.
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_21::*;
//! let stamp = "abca".to_string();
//! let target = "aabcaca".to_string();
//! assert_eq!(Solution::moves_to_stamp(stamp, target), [3, 0, 1]);
//! ```
//!
//! __Explanation:__ Initially s = `"???????"`.
//!
//! - Place stamp at index `3` to get `"???abca"`.
//! - Place stamp at index `0` to get `"abcabca"`.
//! - Place stamp at index `1` to get `"aabcaca"`.
//!
//! ##### Constraints
//!
//! - `1 <= stamp.length <= target.length <= 1000`
//! - `stamp` and `target` consist of lowercase English letters.

pub struct Solution;
impl Solution {
    pub fn moves_to_stamp(stamp: String, mut target: String) -> Vec<i32> {
        const EMPTY: u8 = b'?';

        fn can_unstamp(mut i: usize, stamp: &[u8], target: &[u8]) -> bool {
            let mut seen_non_e = false;

            for &b in stamp {
                if target[i] == b {
                    seen_non_e = true;
                } else if target[i] != EMPTY {
                    return false;
                }
                i += 1;
            }

            seen_non_e
        }

        fn unstamp(stamp: &[u8], target: &mut [u8]) -> Vec<i32> {
            let mut unstamped_idxs = vec![];
            let mut i_opt = Some(target.len() - stamp.len());

            while let Some(i) = i_opt {
                if can_unstamp(i, stamp, target) {
                    for j in i..i + stamp.len() {
                        target[j] = EMPTY;
                    }
                    unstamped_idxs.push(i as i32);
                    i_opt = i.checked_sub(stamp.len());
                } else {
                    i_opt = i.checked_sub(1);
                }
            }

            unstamped_idxs
        }

        let mut result = vec![];

        unsafe {
            let stamp = stamp.as_bytes();
            let target = target.as_bytes_mut();
            loop {
                let mut unstamped = unstamp(stamp, target);
                if unstamped.is_empty() {
                    break;
                } else {
                    result.append(&mut unstamped);
                }
            }
        }

        if target.bytes().all(|b| b == EMPTY) {
            result.reverse();
            result
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: Vec<i32> = Vec::new();

    #[test]
    fn a_b() {
        assert_eq!(Solution::moves_to_stamp("a".into(), "b".into()), EMPTY);
    }

    #[test]
    fn abc_abcdabc() {
        let stamp = "abc".to_string();
        let target = "abcdabc".to_string();
        assert_eq!(Solution::moves_to_stamp(stamp, target), EMPTY);
    }

    #[test]
    fn a_a() {
        assert_eq!(Solution::moves_to_stamp("a".into(), "a".into()), [0]);
    }
    #[test]
    fn a_aa() {
        assert_eq!(Solution::moves_to_stamp("a".into(), "aa".into()), [0, 1]);
    }
    #[test]
    fn a_aaa() {
        assert_eq!(
            Solution::moves_to_stamp("a".into(), "aaa".into()),
            [0, 1, 2]
        );
    }

    #[test]
    fn abc_ababc() {
        let stamp = "abc".to_string();
        let target = "ababc".to_string();
        assert_eq!(Solution::moves_to_stamp(stamp, target), [0, 2]);
    }

    #[test]
    fn abc_abcabc() {
        let stamp = "abc".to_string();
        let target = "abcabc".to_string();
        assert_eq!(Solution::moves_to_stamp(stamp, target), [0, 3]);
    }

    #[test]
    fn abca_aabcaca() {
        let stamp = "abca".to_string();
        let target = "aabcaca".to_string();
        assert_eq!(Solution::moves_to_stamp(stamp, target), [0, 3, 1]);
    }

    #[test]
    fn a_a_repeat_1000() {
        let stamp = "a".to_string();
        let target = "a".repeat(1000);
        let expected = (0..1000).collect::<Vec<_>>();
        assert_eq!(Solution::moves_to_stamp(stamp, target), expected);
    }
}
