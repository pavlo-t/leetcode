#![allow(dead_code)]
//! \#820. Short Encoding of Words
//! ==============================
//!
//! A __valid encoding__ of an array of `words` is any reference string `s` and array of indices `indices` such that:
//!
//! - `words.length == indices.length`
//! - The reference string `s` ends with the `'#'` character.
//! - For each index `indices[i]`, the __substring__ of `s` starting from `indices[i]`
//!   and up to (but not including) the next `'#'` character is equal to `words[i]`.
//!
//! Given an array of `words`, return
//! _the __length of the shortest reference string__ `s` possible of any __valid encoding__ of `words`_.
//!
//! __Constraints:__
//!
//! - `1 <= words.length <= 2000`
//! - `1 <= words[i].length <= 7`
//! - `words[i]` consists of only lowercase letters.
//!
//! <https://leetcode.com/problems/short-encoding-of-words>

pub struct Solution;
impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        #[derive(Default)]
        struct Trie {
            children: [Option<Box<Trie>>; 26],
        }
        impl Trie {
            fn new(words: &[String]) -> Self {
                let mut trie = Trie::default();
                for w in words {
                    trie.add(w);
                }
                trie
            }
            fn add(&mut self, s: &str) {
                let mut curr = self;
                for b in s.bytes().rev() {
                    let i = (b - b'a') as usize;
                    if curr.children[i].is_none() {
                        curr.children[i] = Some(Box::new(Trie::default()));
                    }
                    curr = curr.children[i].as_mut().unwrap();
                }
            }
            fn get(&self, s: &str) -> i32 {
                let mut curr = self;
                for b in s.bytes().rev() {
                    let i = (b - b'a') as usize;
                    curr = curr.children[i].as_ref().unwrap();
                }
                if curr.children.iter().all(|c| c.is_none()) {
                    s.len() as i32 + 1
                } else {
                    0
                }
            }
        }
        use std::collections::HashSet;

        let words = words
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let trie = Trie::new(&words);
        let mut result = 0;
        for w in &words {
            result += trie.get(w);
        }
        result
    }
    pub fn minimum_length_encoding_brute_force(words: Vec<String>) -> i32 {
        use std::collections::HashSet;
        fn has_superstring(s: &str, words: &[String]) -> bool {
            words
                .iter()
                .take_while(|w| w.len() > s.len())
                .any(|w| w.ends_with(s))
        }
        let mut words = words
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        words.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
        let mut result = 0;
        for w in &words {
            if !has_superstring(w, &words) {
                result += w.len() as i32 + 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs { ($($x:expr),*) => { vec![$($x.to_string()),*] }; }

    #[test]
    fn t() {
        let w = vs!["t"];
        assert_eq!(Solution::minimum_length_encoding(w), 2);
        // Explanation: A valid encoding would be s = "t#" and indices = [0].
    }
    #[test]
    fn time_me_bell() {
        let w = vs!["time", "me", "bell"];
        assert_eq!(Solution::minimum_length_encoding(w), 10);
        // Explanation: A valid encoding would be s = "time#bell#" and indices = [0, 2, 5].
        // words[0] = "time", the substring of s starting from indices[0] = 0 to the next '#'
        // words[1] = "me", the substring of s starting from indices[1] = 2 to the next '#'
        // words[2] = "bell", the substring of s starting from indices[2] = 5 to the next '#'
    }
    #[test]
    fn me_time_bell() {
        let w = vs!["me", "time", "bell"];
        assert_eq!(Solution::minimum_length_encoding(w), 10);
    }
    #[test]
    fn me_time_bell_bell() {
        let w = vs!["me", "time", "bell", "bell"];
        assert_eq!(Solution::minimum_length_encoding(w), 10);
    }

    #[test]
    fn a_x_7_x_2000() {
        let w = vec!["a".repeat(7); 2000];
        assert_eq!(Solution::minimum_length_encoding(w), 8);
    }

    struct AtoZIterator {
        curr: String,
    }
    #[rustfmt::skip]
    impl AtoZIterator {
        fn new()                  -> Self { Self::new_from("a".to_string()) }
        fn new_from(curr: String) -> Self { Self { curr } }
    }
    impl Iterator for AtoZIterator {
        type Item = String;
        fn next(&mut self) -> Option<Self::Item> {
            let result = self.curr.clone();
            unsafe {
                let bs = self.curr.as_mut_vec();
                let n = bs.len();
                let mut i = n - 1;
                while i < n && bs[i] == b'z' {
                    i = i.wrapping_sub(1);
                }
                if i < n {
                    bs[i] += 1;
                    for i in i + 1..n {
                        bs[i] = b'a';
                    }
                } else {
                    for i in 0..n {
                        bs[i] = b'a';
                    }
                    bs.push(b'a');
                }
            }
            Some(result)
        }
    }

    #[test]
    fn a_b_c_to_zzzzzzz_take_2000() {
        let words = AtoZIterator::new();
        let w = words.take(2000).collect::<Vec<_>>();
        assert_eq!(Solution::minimum_length_encoding(w), 5192);
    }
    #[test]
    fn a_b_c_to_zzzzzzz_skip_300000_take_2000() {
        let words = AtoZIterator::new();
        let w = words.skip(300000).take(2000).collect::<Vec<_>>();
        println!("words: {w:?}");
        assert_eq!(Solution::minimum_length_encoding(w), 10000);
    }
    #[test]
    fn a_b_c_to_zzzzzzz_take_1000_appended_aaaaaaa_to_zzzzzzz_take_1000() {
        let w1 = AtoZIterator::new();
        let w2 = AtoZIterator::new_from("aaaaaaa".into());
        let ws = w1.take(1000).chain(w2.take(1000)).collect::<Vec<_>>();
        println!("words: {ws:?}");
        assert_eq!(Solution::minimum_length_encoding(ws), 8000);
    }
}
