#![allow(dead_code)]

/// # Short Encoding of Words
///
/// A __valid encoding__ of an array of `words` is any reference string `s` and array of `indices`
/// indices such that:
///
/// - `words.length == indices.length`
/// - The reference string `s` ends with the `'#'` character.
/// - For each index `indices[i]`, the __substring__ of `s` starting from `indices[i]` and up to
///   (but not including) the next `'#'` character is equal to `words[i]`.
///
/// Given an array of `words`, return _the __length of the shortest reference string__ `s` possible
/// of any __valid encoding__ of `words`_.
///
/// __Constraints:__
///
/// - `1 <= words.length <= 2000`
/// - `1 <= words[i].length <= 7`
/// - `words[i]` consists of only lowercase letters.
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/588/week-1-march-1st-march-7th/3662/
struct Solution;

impl Solution {
    pub fn minimum_length_encoding(mut words: Vec<String>) -> i32 {
        words.sort_unstable_by(|w1, w2| w2.len().cmp(&w1.len()));
        let mut rsf: Vec<String> = Vec::new();
        for w in words {
            if rsf.iter().any(|w2| w2.ends_with(&w)) {
                continue;
            } else {
                rsf.push(w.clone());
            }
        }
        rsf.iter().fold(0, |acc, w| acc + w.len() + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let words = vec!["time".to_string(), "me".to_string(), "bell".to_string()];
        assert_eq!(Solution::minimum_length_encoding(words), 10);
        // Explanation: A valid encoding would be s = "time#bell#" and indices = [0, 2, 5].
        // words[0] = "time", the substring of s starting from indices[0] = 0 to the next '#'
        // words[1] = "me", the substring of s starting from indices[1] = 2 to the next '#'
        // words[2] = "bell", the substring of s starting from indices[2] = 5 to the next '#'
    }

    #[test]
    fn example2() {
        let words = vec!["t".to_string()];
        assert_eq!(Solution::minimum_length_encoding(words), 2);
        // Explanation: A valid encoding would be s = "t#" and indices = [0].
    }

    #[test]
    fn test28() {
        let words = vec![
            "time".to_string(),
            "time".to_string(),
            "time".to_string(),
            "time".to_string(),
        ];
        assert_eq!(Solution::minimum_length_encoding(words), 5);
    }
}
