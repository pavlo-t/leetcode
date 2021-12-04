#![allow(dead_code)]
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
/// 1032. Stream of Characters
/// ==========================
///
/// Design an algorithm that accepts a stream of characters
/// and checks if a suffix of these characters is a string of a given array of strings `words`.
///
/// For example, if `words = ["abc", "xyz"]`
/// and the stream added the four characters (one by one) `'a'`, `'x'`, `'y'`, and `'z'`,
/// your algorithm should detect that the suffix `"xyz"` of the characters `"axyz"` matches `"xyz"` from `words`.
///
/// Implement the `StreamChecker` class:
///
/// - `StreamChecker(String[] words)` Initializes the object with the strings array `words`.
/// - `boolean query(char letter)` Accepts a new character from the stream and returns `true`
///   if any non-empty suffix from the stream forms a word that is in `words`.
///
/// __Constraints:__
///
/// - `1 <= words.length <= 2000`
/// - `1 <= words[i].length <= 2000`
/// - `words[i]` consists of lowercase English letters.
/// - `letter` is a lowercase English letter.
/// - At most `40_000` calls will be made to query.
///
/// https://leetcode.com/problems/stream-of-characters/
#[derive(Debug)]
struct StreamCheckerMy {
    words: Rc<RefCell<Trie>>,
    pointers: Vec<Rc<RefCell<Trie>>>,
}
impl StreamCheckerMy {
    fn new(words: Vec<String>) -> Self {
        Self {
            words: Trie::new(words).wrap(),
            pointers: Vec::new(),
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.pointers.push(self.words.clone());
        let mut is_end = false;
        let mut next_pointers = vec![];
        while let Some(p) = self.pointers.pop() {
            if let Some(np) = p.borrow().next_pointer(letter) {
                if np.borrow().is_end {
                    is_end = true;
                }
                next_pointers.push(np);
            }
        }
        self.pointers = next_pointers;
        is_end
    }
}

#[derive(Default, Debug)]
struct Trie {
    is_end: bool,
    children: HashMap<char, Rc<RefCell<Trie>>>,
}
impl Trie {
    fn new(words: Vec<String>) -> Self {
        let mut root = Self::default();
        for word in words {
            root.insert(word);
        }
        root
    }
    fn next_pointer(&self, c: char) -> Option<Rc<RefCell<Self>>> {
        self.children.get(&c).map(|ch| ch.clone())
    }
    fn wrap(self: Self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
    fn insert(&mut self, word: String) {
        let mut chars = word.chars();
        let c = chars.next().unwrap();
        if !self.children.contains_key(&c) {
            self.children.insert(c, Self::default().wrap());
        }
        let mut curr = self.children.get(&c).unwrap().clone();
        for c in chars {
            if !curr.borrow().children.contains_key(&c) {
                curr.borrow_mut().children.insert(c, Self::default().wrap());
            }
            let next = curr.borrow().children.get(&c).unwrap().clone();
            curr = next;
        }
        curr.borrow_mut().is_end = true;
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// From other submissions https://leetcode.com/submissions/detail/596810997/
#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end: bool,
}
impl TrieNode {
    fn new() -> Box<Self> {
        Box::new(Self::default())
    }
}

use std::collections::LinkedList;
struct StreamChecker {
    root: Box<TrieNode>,
    read_chars: LinkedList<u8>,
    max_depth: usize,
}
impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut trie_root = TrieNode::new();
        let mut max_depth = 0;

        for word in words {
            if word.len() > max_depth {
                max_depth = word.len();
            }

            let mut current_node = &mut trie_root;
            for token in word
                .bytes()
                .rev()
                .map(|token| (token - b'a') as usize)
            {
                if current_node.children[token].is_none() {
                    current_node.children[token] = Some(TrieNode::new());
                }
                current_node = current_node.children[token].as_mut().unwrap();
            }
            current_node.is_end = true;
        }

        Self {
            root: trie_root,
            read_chars: LinkedList::new(),
            max_depth,
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.read_chars.push_back(letter as u8);
        if self.read_chars.len() > self.max_depth {
            self.read_chars.pop_front();
        }

        let mut current_node = &self.root;
        for token in self
            .read_chars
            .iter()
            .rev()
            .map(|&token| (token - b'a') as usize)
        {
            if let None = current_node.children[token] {
                break;
            }
            current_node = current_node.children[token].as_ref().unwrap();
            if current_node.is_end {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($x:expr),*) => {vec![$($x.to_string()),*]};}

    #[test]
    fn example1() {
        let mut sc = StreamChecker::new(vs!["cd", "f", "kl"]);
        assert!(!sc.query('a'));
        assert!(!sc.query('b'));
        assert!(!sc.query('c'));
        assert!(sc.query('d')); // because 'cd' is in the wordlist
        assert!(!sc.query('e'));
        assert!(sc.query('f')); // because 'f' is in the wordlist
        assert!(!sc.query('g'));
        assert!(!sc.query('h'));
        assert!(!sc.query('i'));
        assert!(!sc.query('j'));
        assert!(!sc.query('k'));
        assert!(sc.query('l')); //because 'kl' is in the wordlist
    }

    //#[ignore]
    #[test]
    fn words_2000x2000a_query_40000xa() {
        let mut sc = StreamChecker::new(vec!["a".repeat(2000); 2000]);
        for i in 1..2000 {
            assert!(
                !sc.query('a'),
                "expected sc.query('a') to return false on {} iteration",
                i
            );
        }
        for i in 2000..=40000 {
            assert!(
                sc.query('a'),
                "expected sc.query('a') to return true on {} iteration",
                i
            );
        }
    }
    //#[ignore]
    #[test]
    fn words_2000x5a_to_j1995a_query_40000xa() {
        use std::collections::HashSet;
        fn i32_to_word(i: i32) -> String {
            String::from_utf8(
                format!("{:0<2000}", i)
                    .into_bytes()
                    .into_iter()
                    .map(|b| b - b'0' + b'a')
                    .collect(),
            )
            .unwrap()
        }
        let mut words = HashSet::new();
        let mut i = 0;
        while words.len() < 2000 {
            let w = i32_to_word(i);
            words.insert(w);
            i += 1;
        }
        std::thread::sleep(std::time::Duration::from_millis(10000));
        let words = words.into_iter().collect();
        let mut sc = StreamChecker::new(words);
        std::thread::sleep(std::time::Duration::from_millis(10000));
        for c in (0..40000).map(|i| (i % 10) as u8 + b'a').map(|b| b as char) {
            assert!(!sc.query(c));
        }
    }
    //#[ignore]
    #[test]
    fn words_2000x1995a5a_to_j_query_40000xa() {
        use std::collections::HashSet;
        fn i32_to_word(i: i32) -> String {
            String::from_utf8(
                format!("{:0<2000}", i)
                    .into_bytes()
                    .into_iter()
                    .rev()
                    .map(|b| b - b'0' + b'a')
                    .collect(),
            )
            .unwrap()
        }
        let mut words = HashSet::new();
        let mut i = 0;
        while words.len() < 2000 {
            let w = i32_to_word(i);
            words.insert(w);
            i += 1;
        }
        std::thread::sleep(std::time::Duration::from_millis(10000));
        let words = words.into_iter().collect();
        let mut sc = StreamChecker::new(words);
        std::thread::sleep(std::time::Duration::from_millis(10000));
        for c in (0..40000).map(|i| (i % 10) as u8 + b'a').map(|b| b as char) {
            assert!(!sc.query(c));
        }
    }
}
