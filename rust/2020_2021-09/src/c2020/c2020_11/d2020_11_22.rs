#![allow(dead_code)]
struct Solution;
impl Solution {
    const MORSE_CODE: [&'static str; 26] = [
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];

    fn encode(word: &String) -> String {
        word.chars()
            .flat_map(|c| Self::MORSE_CODE[c as usize - 'a' as usize].chars())
            .collect::<String>()
    }

    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        use std::collections::HashSet;

        words
            .iter()
            .map(Self::encode)
            .collect::<HashSet<String>>()
            .len() as i32
    }
}

//noinspection DuplicatedCode
#[cfg(test)]
mod tests {
    use super::*;

    fn random_word(max_len: usize) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

        let mut rng = rand::thread_rng();
        let len = max_len;

        (0..len)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    #[test]
    fn test_gin_zen_gig_msg() {
        let words = vec!["gin", "zen", "gig", "msg"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(Solution::unique_morse_representations(words), 2);
    }

    #[test]
    fn test_empty() {
        let words = Vec::new();
        assert_eq!(Solution::unique_morse_representations(words), 0);
    }

    #[test]
    fn test_100_12z() {
        let mut words = Vec::new();
        for _ in 0..100 {
            words.push(String::from("zzzzzzzzzzzz"));
        }
        assert_eq!(Solution::unique_morse_representations(words), 1);
    }

    #[test]
    fn test_100_random_12char_words() {
        let mut words = Vec::new();
        for _ in 0..100 {
            words.push(random_word(12));
        }

        println!("words: {:?}", words);

        let result = Solution::unique_morse_representations(words);

        println!("result:{:?}", result);
        assert!(result >= 1);
        assert!(result <= 100);
    }
}
