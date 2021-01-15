#![allow(dead_code)]

/// ### Word Ladder
/// https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3598/
struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32 {
        use std::collections::{HashMap, VecDeque};

        word_list.push(begin_word);
        let start = word_list.len() - 1;
        let mut end = None;
        let mut map = HashMap::new();
        fn get_key(w: &str, i: usize) -> String {
            let (l, _) = w.split_at(i);
            let (_, r) = w.split_at(i + 1);
            l.to_owned() + "*" + r
        }

        for (i, w) in word_list.iter().enumerate() {
            if w == &end_word {
                end = Some(i);
            }
            for j in 0..w.len() {
                (*map.entry(get_key(w, j)).or_insert(vec![])).push(i);
            }
        }
        if end.is_none() {
            return 0;
        }
        let end = end.unwrap();

        let mut queue = VecDeque::new();
        let mut seen = vec![false; word_list.len()];
        queue.push_back((start, 1));
        seen[start] = true;

        while let Some((i, steps)) = queue.pop_front() {
            if i == end {
                return steps;
            }
            let w = &word_list[i];
            for j in 0..w.len() {
                for &k in map.get(&get_key(w, j)).unwrap().iter() {
                    if !seen[k] {
                        queue.push_back((k, steps + 1));
                        seen[k] = true;
                    }
                }
            }
        }

        0
    }

    pub fn ladder_length_partition(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32 {
        use std::collections::VecDeque;

        let mut queue = VecDeque::new();
        queue.push_back((begin_word, 1));

        while let Some((word, steps)) = queue.pop_front() {
            if word == end_word {
                return steps;
            }
            let word_bytes = word.as_bytes();
            let (enqueue, next_words): (Vec<String>, Vec<String>) =
                word_list
                    .into_iter()
                    .partition(|w| {
                        let mut diff = 0;
                        let wb = w.as_bytes();
                        for i in 0..w.len() {
                            if word_bytes[i] != wb[i] {
                                diff += 1;
                            }
                            if diff > 1 {
                                break;
                            }
                        }
                        diff == 1
                    });
            word_list = next_words;
            enqueue.into_iter().for_each(|w| queue.push_back((w, steps + 1)));
        }

        0
    }
    pub fn ladder_length_linked_list(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32 {
        use std::collections::LinkedList;

        let mut queue = LinkedList::new();
        queue.push_back((begin_word, 1));

        while let Some((word, steps)) = queue.pop_front() {
            if word == end_word {
                return steps;
            }
            let word_bytes = word.as_bytes();
            let mut enqueue = LinkedList::new();
            word_list = word_list.into_iter()
                .fold(Vec::new(), |mut nwl, w| {
                    let mut diff = 0;
                    let wb = w.as_bytes();
                    for i in 0..w.len() {
                        if word_bytes[i] != wb[i] {
                            diff += 1;
                        }
                        if diff > 1 {
                            break;
                        }
                    }
                    if diff == 1 {
                        enqueue.push_back((w, steps + 1));
                    } else {
                        nwl.push(w);
                    }
                    nwl
                });
            queue.append(&mut enqueue);
        }

        0
    }
    pub fn ladder_length_fold(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32 {
        use std::collections::VecDeque;

        let mut queue = VecDeque::new();
        queue.push_back((begin_word, 1));

        while let Some((word, steps)) = queue.pop_front() {
            if word == end_word {
                return steps;
            }
            let word_bytes = word.as_bytes();
            word_list = word_list.into_iter()
                .fold(Vec::new(), |mut nwl, w| {
                    let mut diff = 0;
                    let wb = w.as_bytes();
                    for i in 0..w.len() {
                        if word_bytes[i] != wb[i] {
                            diff += 1;
                        }
                        if diff > 1 {
                            break;
                        }
                    }
                    if diff == 1 {
                        queue.push_back((w, steps + 1));
                    } else {
                        nwl.push(w);
                    }
                    nwl
                });
        }

        0
    }
}

//noinspection DuplicatedCode
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_bw_hit_ew_cog_wl_hot_dot_dog_lot_log_cog_is_5() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = ["hot", "dot", "dog", "lot", "log", "cog"]
            .iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 5);
        // Explanation: As one shortest transformation is "hit" -> "hot" -> "dot" -> "dog" -> "cog", return its length 5.
    }

    #[test]
    fn example2_bw_hit_ew_cog_wl_hot_dot_dog_lot_log_is_0() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = ["hot", "dot", "dog", "lot", "log"]
            .iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 0);
        // Explanation: The endWord "cog" is not in wordList, therefore no possible transformation.
    }

    #[test]
    fn bw_a_ew_b_wl_b_is_2() {
        let begin_word = "a".to_string();
        let end_word = "b".to_string();
        let word_list = vec!["b".to_string()];
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 2);
    }

    #[test]
    fn bw_a_repeat100_ew_b_repeat100_wl_100_words_is_101() {
        let begin_word = "a".repeat(100);
        let end_word = "b".repeat(100);
        let word_list = (0..100)
            .map(|i| {
                let mut s = "a".repeat(i);
                s.push_str(&"b".repeat(100 - i));
                s
            })
            .collect();
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 101);
    }

    #[test]
    fn bw_a_repeat100_ew_b_repeat100_wl_2000_words_is_101() {
        let begin_word = "a".repeat(100);
        let end_word = "b".repeat(100);
        let word_list = (0..100)
            .map(|i| {
                let mut s = "a".repeat(i);
                s.push_str(&"b".repeat(100 - i));
                s
            })
            .flat_map(|s| vec![s; 20])
            .collect();
        assert_eq!(Solution::ladder_length(begin_word, end_word, word_list), 101);
    }
}
