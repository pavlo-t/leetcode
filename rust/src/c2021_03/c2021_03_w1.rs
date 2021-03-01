#![allow(dead_code)]

use std::collections::HashMap;

/// # Single-Row Keyboard
///
/// There is a special keyboard with __all keys in a single row__.
///
/// Given a string `keyboard` of length 26 indicating the layout of the keyboard
/// (indexed from 0 to 25), initially your finger is at index 0.
/// To type a character, you have to move your finger to the index of the desired character.
/// The time taken to move your finger from index `i` to index `j` is `|i - j|`.
///
/// You want to type a string `word`.
/// Write a function to calculate how much time it takes to type it with one finger.
///
/// __Constraints:__
///
/// - `keyboard.length == 26`
/// - `keyboard` contains each English lowercase letter exactly once in some order.
/// - `1 <= word.length <= 10_000`
/// - `word[i]` is an English lowercase letter.
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/588/week-1-march-1st-march-7th/3656/
struct Solution;

impl Solution {
    pub fn calculate_time(keyboard: String, word: String) -> i32 {
        let mut map = vec![0; 26];
        for (i, &c) in keyboard.as_bytes().into_iter().enumerate() {
            map[(c - b'a') as usize] = i;
        }
        let mut last_i = 0;
        let mut result = 0;
        for &c in word.as_bytes() {
            let i = map[(c - b'a') as usize] as i32;
            result += (last_i - i).abs();
            last_i = i;
        }
        result
    }

    pub fn calculate_time_hash_map(keyboard: String, word: String) -> i32 {
        let mut map = HashMap::new();
        for (i, &c) in keyboard.as_bytes().iter().enumerate() {
            map.insert(c, i);
        }
        let mut last_i = 0;
        let mut result = 0;
        for c in word.as_bytes() {
            let i = map[c] as i32;
            result += (last_i - i).abs();
            last_i = i;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let keyboard = "abcdefghijklmnopqrstuvwxyz".to_string();
        let word = "cba".to_string();
        assert_eq!(Solution::calculate_time(keyboard, word), 4);
        // Explanation:
        // The index moves from 0 to 2 to write 'c' then to 1 to write 'b' then to 0 again to write 'a'.
        // Total time = 2 + 1 + 1 = 4.
    }

    #[test]
    fn example2() {
        let keyboard = "pqrstuvwxyzabcdefghijklmno".to_string();
        let word = "leetcode".to_string();
        assert_eq!(Solution::calculate_time(keyboard, word), 73);
    }
}
