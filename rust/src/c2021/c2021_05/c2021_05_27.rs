#![allow(dead_code)]

/// Maximum Product of Word Lengths
/// ===============================
///
/// Given a string array `words`,
/// return _the maximum value of_ `length(word[i]) * length(word[j])`
/// _where the two words do not share common letters_.
/// If no such two words exist, return `0`.
///
/// __Constraints:__
///
/// - `2 <= words.length <= 1000`
/// - `1 <= words[i].length <= 1000`
/// - `words[i]` consists only of lowercase English letters.
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/601/week-4-may-22nd-may-28th/3757/
struct Solution;
impl Solution {
    pub fn max_product_bitmask(words: Vec<String>) -> i32 {
        use std::collections::{HashMap, HashSet};

        fn bitmask(s: &HashSet<u8>) -> i32 {
            let mut result = 0;
            s.iter().for_each(|&b| result |= 1 << (b - b'a'));
            result
        }

        let mut wls: HashMap<i32, (HashSet<u8>, usize)> = HashMap::new();
        words
            .into_iter()
            .map(|w| (w.len(), w))
            .map(|(l, w)| (l, w.into_bytes().into_iter().collect::<HashSet<_>>()))
            .map(|(l, s)| (l, bitmask(&s), s))
            .for_each(|(l, k, s)| {
                wls.entry(k)
                    .and_modify(|(_, e)| if &l > e { *e = l })
                    .or_insert((s, l));
            });

        let mut max = 0;
        for (a, al) in wls.values() {
            for (b, bl) in wls.values() {
                if a.is_disjoint(b) {
                    max = max.max(al * bl);
                }
            }
        }
        max as i32
    }

    pub fn max_product(words: Vec<String>) -> i32 {
        use std::collections::{HashMap, HashSet};

        let mut wls: HashMap<Vec<u8>, (HashSet<u8>, usize)> = HashMap::new();
        words
            .into_iter()
            .map(|w| (w.len(), w))
            .map(|(l, w)| (l, w.into_bytes().into_iter().collect::<HashSet<_>>()))
            .map(|(l, s)| {
                let mut k = s.iter().map(|b| b.to_owned()).collect::<Vec<_>>();
                k.sort_unstable();
                (l, s, k)
            })
            .for_each(|(l, s, k)| {
                wls.entry(k)
                    .and_modify(|(_, e)| {
                        if &l > e {
                            *e = l
                        }
                    })
                    .or_insert((s, l));
            });

        let mut max = 0;
        for (a, al) in wls.values() {
            for (b, bl) in wls.values() {
                if a.is_disjoint(b) {
                    max = max.max(al * bl);
                }
            }
        }
        max as i32
    }

    pub fn max_product_my_v1(words: Vec<String>) -> i32 {
        use std::collections::HashSet;

        let wls = words
            .into_iter()
            .map(|w| (w.len(), w))
            .map(|(wl, w)| (wl, w.into_bytes().into_iter().collect::<HashSet<_>>()))
            .collect::<Vec<_>>();

        let mut max = 0;
        for (al, a) in wls.iter() {
            for (bl, b) in wls.iter() {
                if a.is_disjoint(b) {
                    max = max.max(al * bl);
                }
            }
        }
        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($s:tt),*) => {vec![$($s.to_string()),*]};}

    #[test]
    fn example1() {
        let words = vs!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"];
        assert_eq!(Solution::max_product(words), 16);
        // Explanation: The two words can be "abcw", "xtfn".
    }
    #[test]
    fn example2() {
        let words = vs!["a", "ab", "abc", "d", "cd", "bcd", "abcd"];
        assert_eq!(Solution::max_product(words), 4);
        // Explanation: The two words can be "ab", "cd".
    }
    #[test]
    fn example3() {
        let words = vs!["a", "aa", "aaa", "aaaa"];
        assert_eq!(Solution::max_product(words), 0);
        // Explanation: No such pair of words.
    }

    #[test]
    fn w_1000a_repeat_1000_produces_0() {
        let words = vec!["a".repeat(1000); 1000];
        assert_eq!(Solution::max_product(words), 0);
    }
    #[test]
    fn w_999_a_repeat_1000_push_b_repeat_1000_produces_1_000_000() {
        let mut words = vec!["a".repeat(1000); 999];
        words.push("b".repeat(1000));
        assert_eq!(Solution::max_product(words), 1_000_000);
    }

    // #[test]
    // fn test_threads() {
    //     use std::thread;
    //     let mut handles = vec![];
    //
    //     for ti in 0..6 {
    //         const MAX: i64 = 1_000_000_001;
    //         let handle = thread::spawn(move || {
    //             let mut i = 0i64;
    //             while i < MAX {
    //                 if i % 100_000_000 == 0 {
    //                     println!("i = {} in thread {}", i, ti);
    //                 }
    //                 i += 1;
    //             }
    //         });
    //         handles.push(handle);
    //     }
    //
    //     for handle in handles {
    //         handle.join().unwrap();
    //     }
    // }
}
