#![allow(dead_code)]
//! \#804. Unique Morse Code Words
//! ==============================
//!
//! <https://leetcode.com/problems/unique-morse-code-words>
//!
//! International Morse Code defines a standard encoding where each letter is mapped to a series of dots and dashes, as follows:
//!
//! - `'a'` maps to `".-"`,
//! - `'b'` maps to `"-..."`,
//! - `'c'` maps to `"-.-."`, and so on.
//!
//! For convenience, the full table for the `26` letters of the English alphabet is given below:
//!
//! ```text
//! [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."]
//! ```
//!
//! Given an array of strings `words` where each word can be written as a concatenation of the Morse code of each letter.
//!
//! For example, `"cab"` can be written as `"-.-..--..."`,
//! which is the concatenation of `"-.-."`, `".-"`, and `"-..."`.
//! We will call such a concatenation the __transformation__ of a word.
//!
//! Return _the number of different __transformations__ among all words we have_.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_08::c2022_08_17::*;
//! # use c2022_08::vs;
//! let words = vs!["gin", "zen", "gig", "msg"];
//! assert_eq!(Solution::unique_morse_representations(words), 2);
//! ```
//!
//! __Explanation:__ The transformation of each word is:
//!
//! - `"gin" -> "--...-."`
//! - `"zen" -> "--...-."`
//! - `"gig" -> "--...--."`
//! - `"msg" -> "--...--."`
//!
//! There are `2` different transformations: `"--...-."` and `"--...--."`.
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_17::*;
//! # use c2022_08::vs;
//! assert_eq!(Solution::unique_morse_representations(vs!["a"]), 1);
//! ```
//!
//! ##### Constraints
//!
//! - `1 <= words.length <= 100`
//! - `1 <= words[i].length <= 12`
//! - `words[i]` consists of lowercase English letters.

pub struct Solution;
impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        use std::collections::HashSet;

        const ENCODED_LETTERS: [&str; 26] = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];

        fn encode(word: String) -> String {
            word.bytes()
                .map(|b| (b - b'a') as usize)
                .fold(String::new(), |mut encoded, i| {
                    encoded.push_str(ENCODED_LETTERS[i]);
                    encoded
                })
        }

        words.into_iter().map(encode).collect::<HashSet<_>>().len() as i32
    }
}
