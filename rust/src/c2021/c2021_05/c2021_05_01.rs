#![allow(dead_code)]
/// Prefix and Suffix Search
/// ========================
///
/// Design a special dictionary which has some words and allows you to search the words in it by a
/// prefix and a suffix.
///
/// Implement the `WordFilter` class:
///
/// - `WordFilter(string[] words)` Initializes the object with the `words` in the dictionary.
/// - `f(string prefix, string suffix)` Returns _the index of the word in the dictionary_
///   which has the prefix `prefix` and the suffix `suffix`.
///   If there is more than one valid index, return __the largest__ of them.
///   If there is no such word in the dictionary, return `-1`.
///
/// __Constraints:__
///
/// - `1 <= words.length <= 15000`
/// - `1 <= words[i].length <= 10`
/// - `1 <= prefix.length, suffix.length <= 10`
/// - `words[i]`, `prefix` and `suffix` consist of lower-case English letters only.
/// - At most `15000` calls will be made to the function `f`.
///
/// https://leetcode.com/explore/featured/card/may-leetcoding-challenge-2021/598/week-1-may-1st-may-7th/3728/
struct WordFilter {
    trie: TrieNode,
}
impl WordFilter {
    /// Approach #3: Trie of Suffix Wrapped Words `[Accepted]`
    ///
    /// https://leetcode.com/problems/prefix-and-suffix-search/solution/
    fn new(words: Vec<String>) -> Self {
        let mut trie = TrieNode::new();

        for (weight, mut word) in words.into_iter().enumerate() {
            word.push('{');
            let word = word.as_bytes();

            for i in 0..word.len() {
                let mut cur = &mut trie;
                cur.weight = weight as i32;
                for j in i..(2 * word.len() - 1) {
                    let k = (word[j % word.len()] - b'a') as usize;
                    if cur.children[k].is_none() {
                        cur.children[k] = Some(Box::new(TrieNode::new()));
                    }
                    cur = cur.children[k].as_mut().unwrap();
                    cur.weight = weight as i32;
                }
            }
        }

        Self { trie }
    }
    fn f(&self, prefix: String, suffix: String) -> i32 {
        let delimiter = std::iter::once(b'{');
        let mut cur = &self.trie;

        for b in suffix.bytes().chain(delimiter).chain(prefix.bytes()) {
            let i = (b - b'a') as usize;
            if let Some(c) = cur.children[i].as_ref() {
                cur = c;
            } else {
                return -1;
            }
        }

        cur.weight
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 27],
    weight: i32,
}
impl TrieNode {
    fn new() -> Self {
        Self {
            children: Default::default(),
            weight: 0,
        }
    }
}

struct WordFilterBruteForce {
    ws: Vec<String>,
}
impl WordFilterBruteForce {
    fn new(words: Vec<String>) -> Self {
        Self { ws: words }
    }
    fn f(&self, prefix: String, suffix: String) -> i32 {
        for (i, w) in self.ws.iter().enumerate().rev() {
            if w.starts_with(&prefix) && w.ends_with(&suffix) {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs { ($($s:tt),*) => { vec![$($s.to_string()),*] }; }

    #[test]
    fn example1() {
        let wf = WordFilter::new(vs!["apple"]);
        assert_eq!(wf.f("a".to_string(), "e".to_string()), 0);
        // Explanation
        // WordFilter wordFilter = new WordFilter(["apple"]);
        // wordFilter.f("a", "e");
        // return 0, because the word at index 0 has prefix = "a" and suffix = 'e".
    }

    #[test]
    fn test() {
        let wf = WordFilter::new(vec!["apple".to_string(), "ae".to_string()]);
        assert_eq!(wf.f("asdf".to_string(), "e".to_string()), -1);
        assert_eq!(wf.f("ap".to_string(), "e".to_string()), 0);
        assert_eq!(wf.f("a".to_string(), "e".to_string()), 1);
    }

    mod performance {
        use super::*;

        #[test]
        fn w_15k_a_repeat_10_15k_calls_p_10a_s_10a() {
            let words = vec!["a".repeat(10); 15_000];
            let wf = WordFilter::new(words);
            for _ in 0..15_000 {
                assert_eq!(wf.f("a".repeat(10), "a".repeat(10)), 14_999);
            }
        }
        #[test]
        fn w_15k_a_repeat_10_15k_calls_p_aaaaaaaaab_s_aaaaaaaaab() {
            let words = vec!["a".repeat(10); 15_000];
            let wf = WordFilter::new(words);
            for _ in 0..15_000 {
                let p = "aaaaaaaaab".to_string();
                assert_eq!(wf.f(p.clone(), p), -1);
            }
        }
        #[test]
        fn w_15k_10len_words_15k_calls_p_z_repeat_10() {
            let words = (0..15_000)
                .map(|i| {
                    let mut s = i.to_string();
                    unsafe {
                        for b in s.as_bytes_mut() {
                            *b += b'a' - b'0';
                        }
                    }
                    let mut p = "a".repeat(10 - s.len());
                    p.push_str(&s);
                    p
                })
                .collect::<Vec<_>>();
            let wf = WordFilter::new(words);
            for _ in 0..15_000 {
                let p = "z".repeat(10);
                assert_eq!(wf.f(p.clone(), p), -1);
            }
        }
    }
}
