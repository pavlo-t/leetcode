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
struct WordFilter {}
impl WordFilter {
    fn new(_words: Vec<String>) -> Self {
        todo!()
    }
    fn f(&self, _prefix: String, _suffix: String) -> i32 {
        todo!()
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
}
