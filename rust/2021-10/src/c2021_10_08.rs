#![allow(dead_code)]
/// 208. Implement Trie (Prefix Tree)
/// =================================
///
/// A [trie] (pronounced as "try") or __prefix tree__ is a tree data structure
/// used to efficiently store and retrieve keys in a dataset of strings.
/// There are various applications of this data structure, such as autocomplete and spellchecker.
///
/// Implement the Trie class:
///
/// - `Trie()`                            Initializes the trie object.
/// - `void insert(String word)`          Inserts the string `word` into the trie.
/// - `boolean search(String word)`       Returns `true` if the string `word` is in the trie
///                                       (i.e., was inserted before), and `false` otherwise.
/// - `boolean startsWith(String prefix)` Returns `true` if there is a previously inserted string `word`
///                                       that has the prefix `prefix`, and `false` otherwise.
///
/// [trie]: https://en.wikipedia.org/wiki/Trie
///
/// __Constraints:__
///
/// - `1 <= word.length, prefix.length <= 2000`
/// - `word` and `prefix` consist only of lowercase English letters.
/// - At most `30_000` calls __in total__ will be made to `insert`, `search`, and `startsWith`.
///
/// https://leetcode.com/problems/implement-trie-prefix-tree/
#[derive(Debug, Default)]
struct Trie {
    word_end: bool,
    data: [Option<Box<Self>>; 26],
}
impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut curr = self;
        for i in word.bytes().map(|b| (b - b'a') as usize) {
            if curr.data[i].is_none() {
                curr.data[i] = Some(Box::new(Self::new()));
            }
            curr = curr.data[i].as_mut().unwrap();
        }
        curr.word_end = true;
    }

    fn search(&self, word: String) -> bool {
        self.find(word).map(|t| t.word_end).unwrap_or(false)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.find(prefix).is_some()
    }

    fn find(&self, prefix: String) -> Option<&Self> {
        let mut curr = self;
        for i in prefix.bytes().map(|b| (b - b'a') as usize) {
            if curr.data[i].is_none() {
                return None;
            } else {
                curr = curr.data[i].as_ref().unwrap();
            }
        }
        Some(curr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut t = Trie::new();
        t.insert("apple".to_string());
        assert!(t.search("apple".to_string()));
        assert!(!t.search("app".to_string()));
        assert!(t.starts_with("app".to_string()));
        t.insert("app".to_string());
        assert!(t.search("app".to_string()));
    }

    fn i32_to_word(i: i32) -> String {
        String::from_utf8(
            format!("{:0<5}", i)
                .into_bytes()
                .into_iter()
                .map(|b| b - b'0' + b'a')
                .collect(),
        )
        .unwrap()
    }

    mod performance {
        use super::*;

        //#[ignore]
        #[test]
        fn insert_00300x2000() {
            let mut t = Trie::new();
            for i in 1..=300 {
                let w = i32_to_word(i).repeat(400);
                t.insert(w);
            }
        }

        /// my Trie: 11 sec 408 ms; 3.7 GiB
        // #[ignore]
        #[test]
        fn insert_10kx2k_search_10kx2k_starts_with_10kx2k() {
            let mut t = Trie::new();
            let mut words = vec![];
            for i in 1..=10000 {
                words.push(i32_to_word(i).repeat(400));
            }

            for w in words.clone() {
                t.insert(w);
            }
            for w in words.clone() {
                assert!(t.search(w));
            }
            for w in words.clone() {
                assert!(t.starts_with(w));
            }
        }
    }

    // mod produce_test_cases {
    //     use super::*;
    //
    //     const LEN: i32 = 300;
    //
    //     #[test]
    //     fn produce_commands() {
    //         let mut commands = vec!["Trie"];
    //         for _ in 1..=LEN {
    //             commands.push("insert");
    //         }
    //         println!("{:?}", commands);
    //     }
    //
    //     #[test]
    //     fn produce_params() {
    //         let mut words = vec![vec![]];
    //         for i in 1..=LEN {
    //             let w = i32_to_word(i).repeat(400);
    //             words.push(vec![w]);
    //             //t.insert(w);
    //         }
    //         println!("{:?}", words);
    //     }
    // }
}
