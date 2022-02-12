#![allow(dead_code)]
/// 127. Word Ladder
/// ================
///
/// A __transformation sequence__ from word `beginWord` to word `endWord` using a dictionary `wordList`
/// is a sequence of words `beginWord -> s1 -> s2 -> ... -> sk` such that:
///
/// - Every adjacent pair of words differs by a single letter.
/// - Every `si` for `1 <= i <= k` is in `wordList`. Note that `beginWord` does not need to be in `wordList`.
/// - `sk == endWord`
///
/// Given two words, `beginWord` and `endWord`, and a dictionary `wordList`,
/// return _the __number of words__ in the __shortest transformation sequence__ from `beginWord` to `endWord`,
/// or `0` if no such sequence exists_.
///
/// __Constraints:__
///
/// - `1 <= beginWord.length <= 10`
/// - `endWord.length == beginWord.length`
/// - `1 <= wordList.length <= 5000`
/// - `wordList[i].length == beginWord.length`
/// - `beginWord`, `endWord`, and `wordList[i]` consist of lowercase English letters.
/// - `beginWord != endWord`
/// - All the words in `wordList` are __unique__.
///
/// https://leetcode.com/problems/word-ladder/
struct Solution;
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if let Some(target) = word_list.iter().position(|w| w == &end_word) {
            use std::collections::HashSet;

            fn is_next_step(a: &str, b: &str) -> bool {
                let mut diff = 0;
                for (a, b) in a.chars().zip(b.chars()) {
                    if a != b {
                        if diff > 0 {
                            return false;
                        }
                        diff += 1;
                    }
                }
                diff == 1
            }

            let mut have_not_seen = (0..word_list.len()).collect::<HashSet<_>>();
            let mut curr = word_list
                .iter()
                .enumerate()
                .filter(|(_, w)| is_next_step(&begin_word, w))
                .map(|(i, _)| {
                    have_not_seen.remove(&i);
                    i
                })
                .collect::<Vec<_>>();
            let mut next = vec![];
            let mut i_next = vec![];
            let mut num_words = 2;
            while !curr.is_empty() {
                while let Some(i) = curr.pop() {
                    if i == target {
                        return num_words;
                    }
                    for &j in have_not_seen.iter() {
                        if is_next_step(&word_list[i], &word_list[j]) {
                            i_next.push(j);
                        }
                    }
                    for j in i_next.iter() {
                        have_not_seen.remove(j);
                    }
                    next.append(&mut i_next);
                }
                num_words += 1;
                std::mem::swap(&mut curr, &mut next);
            }

            0
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($x:expr),*) => { vec![$($x.to_string()),*] };}

    #[test]
    fn hit_cog_hot_dot_dog_lot_log_cog() {
        let b = "hit".to_string();
        let e = "cog".to_string();
        let w = vs!["hot", "dot", "dog", "lot", "log", "cog"];
        assert_eq!(Solution::ladder_length(b, e, w), 5);
        // Explanation: One shortest transformation sequence is "hit" -> "hot" -> "dot" -> "dog" -> cog", which is 5 words long.
    }
    #[test]
    fn hit_cog_hot_dot_dog_lot_log() {
        let b = "hit".to_string();
        let e = "cog".to_string();
        let w = vs!["hot", "dot", "dog", "lot", "log"];
        assert_eq!(Solution::ladder_length(b, e, w), 0);
        // Explanation: The endWord "cog" is not in wordList, therefore there is no valid transformation sequence.
    }
    #[test]
    fn hit_cog_hot_dot_log_cog() {
        let b = "hit".to_string();
        let e = "cog".to_string();
        let w = vs!["hot", "dot", "log", "cog"];
        assert_eq!(Solution::ladder_length(b, e, w), 0);
    }
    #[test]
    fn dog_cog_hot_dot_log_cog() {
        let b = "dog".to_string();
        let e = "cog".to_string();
        let w = vs!["hot", "dot", "log", "cog"];
        assert_eq!(Solution::ladder_length(b, e, w), 2);
    }
    #[test]
    fn dog_cog_hot_dot_dog_lot_log_cog() {
        let b = "dog".to_string();
        let e = "cog".to_string();
        let w = vs!["hot", "dot", "dog", "lot", "log", "cog"];
        assert_eq!(Solution::ladder_length(b, e, w), 2);
    }
    #[test]
    fn lot_cog_hot_dot_dog_lot_log_cog() {
        let b = "lot".to_string();
        let e = "cog".to_string();
        let w = vs!["hot", "dot", "dog", "lot", "log", "cog"];
        assert_eq!(Solution::ladder_length(b, e, w), 3);
    }
}
