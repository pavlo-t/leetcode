#![allow(dead_code)]

/// # Longest Word in Dictionary through Deleting
///
/// Given a string `s` and a string array `dictionary`, return _the longest string in
/// the dictionary that can be formed by deleting some of the given string characters_.
/// If there is more than one possible result, return the longest word with the smallest
/// lexicographical order.
/// If there is no possible result, return the empty string.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 1000`
/// - `1 <= dictionary.length <= 1000`
/// - `1 <= dictionary[i].length <= 1000`
/// - `s` and `dictionary[i]` consist of lowercase English letters.
///
/// https://leetcode.com/explore/featured/card/february-leetcoding-challenge-2021/587/week-4-february-22nd-february-28th/3649/
struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        dictionary
            .into_iter()
            .filter(|w| {
                if w.len() > s.len() {
                    false
                } else {
                    let w_bytes = w.as_bytes();
                    let mut wi = 0;
                    for &b in s.as_bytes() {
                        if wi == w_bytes.len() {
                            break;
                        }
                        if b == w_bytes[wi] {
                            wi += 1;
                        }
                    }
                    wi == w_bytes.len()
                }
            })
            .fold(String::new(), |rsf, w| {
                if rsf.len() > w.len() {
                    rsf
                } else if rsf.len() < w.len() {
                    w
                } else if rsf < w {
                    rsf
                } else {
                    w
                }
            })
    }

    pub fn find_longest_word_sorting(s: String, mut dictionary: Vec<String>) -> String {
        dictionary.sort_unstable_by(|a, b| {
            if a.len() == b.len() {
                a.cmp(b)
            } else {
                b.len().cmp(&a.len())
            }
        });

        dictionary
            .into_iter()
            .find(|w| {
                if w.len() > s.len() {
                    false
                } else {
                    let w_bytes = w.as_bytes();
                    let mut wi = 0;
                    for &b in s.as_bytes() {
                        if wi == w_bytes.len() {
                            break;
                        }
                        if b == w_bytes[wi] {
                            wi += 1;
                        }
                    }
                    wi == w_bytes.len()
                }
            })
            .unwrap_or(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn example1() {
        let s = "abpcplea".to_string();
        let dictionary = ["ale", "bpcpl", "apple", "monkey", "plea"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(Solution::find_longest_word(s, dictionary), "apple");
    }
    #[test]
    fn example2() {
        let s = "abpcplea".to_string();
        let dictionary = ["a", "b", "c"].iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::find_longest_word(s, dictionary), "a");
    }
    #[test]
    fn example1_1() {
        let s = "abpcplea".to_string();
        let dictionary = ["ale", "paple", "monkey", "plea"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(Solution::find_longest_word(s, dictionary), "plea");
    }

    #[test]
    fn test27() {
        let contents = fs::read_to_string("./src/c2021_02/c2021_02_22_test_27.txt").unwrap();
        let mut dictionary = contents
            .split(|c| c == '\n' || c == ',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let s = dictionary.remove(0);
        let e = "ntgcykxhdfescxxypyew";

        assert_eq!(Solution::find_longest_word(s, dictionary), e);
    }
}
