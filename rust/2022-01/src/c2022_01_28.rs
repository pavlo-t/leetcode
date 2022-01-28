#![allow(dead_code)]
/// 211. Design Add and Search Words Data Structure
/// ===============================================
///
/// Design a data structure that supports adding new words and finding if a string matches any previously added string.
///
/// Implement the `WordDictionary` class:
///
/// - `WordDictionary()` Initializes the object.
/// - `void addWord(word)` Adds `word` to the data structure, it can be matched later.
/// - `bool search(word)` Returns `true` if there is any string in the data structure that matches `word`
///   or `false` otherwise. `word` may contain dots `'.'` where dots can be matched with any letter.
///
/// __Constraints:__
///
/// - `1 <= word.length <= 500`
/// - `word` in addWord consists lower-case English letters.
/// - `word` in search consist of  '.' or lower-case English letters.
/// - At most `50000` calls will be made to `addWord` and `search`.
///
/// https://leetcode.com/problems/design-add-and-search-words-data-structure/
#[derive(Debug, Default)]
struct WordDictionary {
    children: [Option<Box<WordDictionary>>; 26],
    word_end: bool,
}
impl WordDictionary {
    fn new() -> Self {
        Self::default()
    }
    fn add_word(&mut self, word: String) {
        let mut curr = self;
        for i in word.bytes().map(|b| (b - b'a') as usize) {
            if curr.children[i].is_none() {
                curr.children[i] = Some(Box::new(WordDictionary::new()));
            }
            curr = curr.children[i].as_mut().unwrap();
        }
        curr.word_end = true;
    }
    fn search(&self, word: String) -> bool {
        self.search_str(&word)
    }
    fn search_str(&self, word: &str) -> bool {
        if word.len() == 0 {
            self.word_end
        } else {
            let next_word = &word[1..];
            match word.as_bytes()[0] {
                b'.' => self.children.iter().any(|c| {
                    c.as_ref()
                        .map(|next| next.search_str(next_word))
                        .unwrap_or(false)
                }),
                b => self.children[(b - b'a') as usize]
                    .as_ref()
                    .map(|next| next.search_str(next_word))
                    .unwrap_or(false),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut wd = WordDictionary::new();

        assert_eq!(wd.word_end, false);
        assert_eq!(wd.children.iter().filter(|c| c.is_some()).count(), 0);

        wd.add_word("bad".into());
        //println!(" wd: {:#?}", wd);
        wd.add_word("bat".into());
        //println!(" wd: {:#?}", wd);
        wd.add_word("dad".into());
        //println!(" wd: {:#?}", wd);
        wd.add_word("mad".into());
        //println!(" wd: {:#?}", wd);
        assert!(!wd.search("pad".into()));
        assert!(wd.search("bad".into()));
        assert!(wd.search(".ad".into()));
        assert!(wd.search("b..".into()));
    }
}
