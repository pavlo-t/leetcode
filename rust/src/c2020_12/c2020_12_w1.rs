#![allow(dead_code)]

struct SolutionIterative;
impl SolutionIterative {
    pub fn shortest_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
        let mut result = i32::MAX;
        let mut i1 = -1;
        let mut i2 = -1;
        for i in 0..words.len() {
            if words[i] == word1 {
                i1 = i as i32;
            } else if words[i] == word2 {
                i2 = i as i32;
            }
            if i1 >= 0 && i2 >= 0 {
                result = result.min((i1 - i2).abs());
            }
        }

        result
    }
}

struct Solution;
impl Solution {
    pub fn shortest_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
        words
            .iter()
            .fold((i32::MAX, i32::MIN, i32::MIN), |(rsf, c1, c2), w| {
                if w == &word1 {
                    (if c2 > 0 { rsf.min(c2) } else { rsf }, 1, c2 + 1)
                } else if w == &word2 {
                    (if c1 > 0 { rsf.min(c1) } else { rsf }, c1 + 1, 1)
                } else {
                    (rsf, c1 + 1, c2 + 1)
                }
            }).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let words = vec!["practice".to_string(), "makes".to_string(),
                         "perfect".to_string(), "coding".to_string(), "makes".to_string()];
        let word1 = "coding".to_string();
        let word2 = "practice".to_string();

        assert_eq!(Solution::shortest_distance(words, word1, word2), 3);
    }

    #[test]
    fn example2() {
        let words = vec!["practice".to_string(), "makes".to_string(),
                         "perfect".to_string(), "coding".to_string(), "makes".to_string()];
        let word1 = "makes".to_string();
        let word2 = "coding".to_string();

        assert_eq!(Solution::shortest_distance(words, word1, word2), 1);
    }
}