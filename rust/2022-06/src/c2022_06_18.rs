#![allow(dead_code)]
//! \#745. Prefix and Suffix Search
//! ===============================
//!
//! Design a special dictionary with some words that searchs the words in it by a prefix and a suffix.
//!
//! Implement the `WordFilter` class:
//!
//! - `WordFilter(string[] words)` Initializes the object with the words in the dictionary.
//! - `f(string prefix, string suffix)` Returns _the index of the word in the dictionary_,
//!                                     which has the prefix `prefix` and the suffix `suffix`.
//!                                     If there is more than one valid index, return __the largest__ of them.
//!                                     If there is no such word in the dictionary, return `-1`.
//!
//! __Constraints:__
//!
//! - `1 <= words.length <= 15000`
//! - `1 <= words[i].length <= 10`
//! - `1 <= prefix.length, suffix.length <= 10`
//! - `words[i]`, `prefix` and `suffix` consist of lower-case English letters only.
//! - At most `15000` calls will be made to the function `f`.
//!
//! https://leetcode.com/problems/prefix-and-suffix-search

use std::iter::once;

const DEL: u8 = b'z' + 1;

#[derive(Default)]
struct Trie {
    idx: usize,
    children: [Option<Box<Trie>>; 27],
}

struct WordFilter {
    trie: Trie,
}
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::default();
        for (idx, w) in words.into_iter().enumerate() {
            for suffix_start in 0..w.len() {
                let mut curr = &mut trie;
                for b in w[suffix_start..].bytes().chain(once(DEL)).chain(w.bytes()) {
                    let i = (b - b'a') as usize;
                    if curr.children[i].is_none() {
                        curr.children[i] = Some(Default::default());
                    }
                    curr = curr.children[i].as_mut().unwrap();
                    curr.idx = idx;
                }
            }
        }
        Self { trie }
    }
    fn f(&self, prefix: String, suffix: String) -> i32 {
        let mut curr = &self.trie;
        for b in suffix.bytes().chain(once(DEL)).chain(prefix.bytes()) {
            let i = (b - b'a') as usize;
            if let Some(n) = curr.children[i].as_ref() {
                curr = n;
            } else {
                return -1;
            }
        }
        curr.idx as i32
    }
}

mod solution_1_should_fail {
    use std::collections::HashMap;
    struct WordFilter {
        prefixes: HashMap<String, Vec<i32>>,
        suffixes: HashMap<String, Vec<i32>>,
    }
    impl WordFilter {
        fn new(words: Vec<String>) -> Self {
            let mut prefixes: HashMap<String, Vec<i32>> = HashMap::new();
            let mut suffixes: HashMap<String, Vec<i32>> = HashMap::new();
            for (i, w) in words.into_iter().enumerate() {
                let i = i as i32;
                for p in 1..=w.len() {
                    prefixes.entry(w[..p].to_string()).or_default().push(i);
                }
                for s in 0..w.len() {
                    suffixes.entry(w[s..].to_string()).or_default().push(i);
                }
            }
            Self { prefixes, suffixes }
        }
        fn f(&self, prefix: String, suffix: String) -> i32 {
            self.prefixes
                .get(&prefix)
                .zip(self.suffixes.get(&suffix))
                .and_then(|(pws, sws)| {
                    let (mut pi, mut si) = (pws.len() - 1, sws.len() - 1);
                    while pi < pws.len() && si < sws.len() {
                        if pws[pi] == sws[si] {
                            return Some(pws[pi]);
                        } else if pws[pi] < sws[si] {
                            si = si.wrapping_sub(1);
                        } else {
                            pi = pi.wrapping_sub(1);
                        }
                    }
                    None
                })
                .unwrap_or(-1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs { ($($x:expr),*) => { vec![$($x.to_string()),*] } }

    #[test]
    fn apple() {
        let wf = WordFilter::new(vs!["apple"]);
        assert_eq!(wf.f("a".into(), "e".into()), 0);
        assert_eq!(wf.f("a".into(), "a".into()), -1);
        // Explanation
        // WordFilter wordFilter = new WordFilter(["apple"]);
        // wordFilter.f("a", "e"); // return 0, because the word at index 0 has prefix = "a" and suffix = 'e".
    }
    #[test]
    fn abc_def() {
        let wf = WordFilter::new(vs!["abc", "def"]);
        assert_eq!(wf.f("abc".into(), "abc".into()), 0);
        assert_eq!(wf.f("ab".into(), "bc".into()), 0);
        assert_eq!(wf.f("a".into(), "bc".into()), 0);
        assert_eq!(wf.f("ab".into(), "c".into()), 0);
        assert_eq!(wf.f("a".into(), "c".into()), 0);

        assert_eq!(wf.f("def".into(), "def".into()), 1);
        assert_eq!(wf.f("de".into(), "ef".into()), 1);
        assert_eq!(wf.f("d".into(), "ef".into()), 1);
        assert_eq!(wf.f("de".into(), "f".into()), 1);
        assert_eq!(wf.f("d".into(), "f".into()), 1);

        assert_eq!(wf.f("a".into(), "f".into()), -1);
        assert_eq!(wf.f("b".into(), "f".into()), -1);
        assert_eq!(wf.f("a".into(), "b".into()), -1);
    }
    #[test]
    fn abc_abcbc() {
        let wf = WordFilter::new(vs!["abc", "abcbc"]);
        assert_eq!(wf.f("abc".into(), "abc".into()), 0);
        assert_eq!(wf.f("abc".into(), "bc".into()), 1);
        assert_eq!(wf.f("ab".into(), "bc".into()), 1);
    }
    #[test]
    fn aaaaaaaaaa_x_7500_chain_bbbbbbbbbb_x_7500() {
        let words = (0..15000)
            .map(|i| {
                if i < 7500 {
                    "a".repeat(10)
                } else {
                    "b".repeat(10)
                }
            })
            .collect();
        //println!("words:\n{words:?}");
        let wf = WordFilter::new(words);
        //let calls = vec![vec!["a".to_string(), "b".to_string()]; 15000];
        //let calls = vec!["f".to_string(); 15000];
        //println!("calls:\n{calls:?}");
        for _ in 0..15000 {
            assert_eq!(wf.f("a".into(), "b".into()), -1);
        }
    }
}
