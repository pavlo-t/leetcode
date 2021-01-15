#![allow(dead_code)]

/// ### Check If Two String Arrays are Equivalent
/// https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3597/
struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut i1 = 0;
        let mut j1 = 0;
        let mut i2 = 0;
        let mut j2 = 0;

        while i1 < word1.len() && i2 < word2.len() {
            if word1[i1].as_bytes()[j1] != word2[i2].as_bytes()[j2] {
                return false;
            }

            if j1 < word1[i1].len() - 1 {
                j1 += 1;
            } else {
                i1 += 1;
                j1 = 0;
            }

            if j2 < word2[i2].len() - 1 {
                j2 += 1;
            } else {
                i2 += 1;
                j2 = 0;
            }
        }

        i1 == word1.len() && i2 == word2.len()
    }

    pub fn array_strings_are_equal_join(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.join("") == word2.join("")
    }
}

#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use super::*;

    #[test]
    fn example1_w1_ab_c_w2_a_bc_is_true() {
        let word1 = vec!["ab".to_string(), "c".to_string()];
        let word2 = vec!["a".to_string(), "bc".to_string()];
        assert!(Solution::array_strings_are_equal(word1, word2));
        // Explanation:
        // word1 represents string "ab" + "c" -> "abc"
        // word2 represents string "a" + "bc" -> "abc"
        // The strings are the same, so return true.
    }

    #[test]
    fn example2_w1_a_cb_w2_ab_c_is_false() {
        let word1 = vec!["a".to_string(), "cb".to_string()];
        let word2 = vec!["ab".to_string(), "c".to_string()];
        assert!(!Solution::array_strings_are_equal(word1, word2));
    }

    #[test]
    fn example3_w1_abc_d_defg_w2_abcddefg_is_true() {
        let word1 = vec!["abc".to_string(), "d".to_string(), "defg".to_string()];
        let word2 = vec!["abcddefg".to_string()];
        assert!(Solution::array_strings_are_equal(word1, word2));
    }

    #[test]
    fn w1_n_w2_n_is_true() {
        assert!(Solution::array_strings_are_equal(vec![], vec![]));
    }

    // #[test]
    // fn test_lifetimes_static() {
    //     const S1: &str = "s1";
    //     const S2: &str = "s2";
    //     fn tst(b: bool) -> &'static str {
    //         if b { S1 } else { S2 }
    //     }
    //     assert_eq!(tst(true), "s1");
    //     assert_eq!(tst(false), "s2");
    // }
    //
    // #[test]
    // fn test_lifetimes_3rd_elision_rule_self() {
    //     #[derive(Debug)]
    //     struct ImportantExcerpt<'a> {
    //         part: &'a str,
    //     }
    //
    //     impl<'a> ImportantExcerpt<'a> {
    //         fn announce_and_return_part(&self, announcement: &str) -> &str {
    //             println!("Attention please: {}", announcement);
    //             self.part
    //         }
    //     }
    //
    //     let novel = String::from("Call me Ishmael. Some years ago...");
    //     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    //     let i = ImportantExcerpt { part: first_sentence };
    //     println!("{:?}", i);
    //     let part = i.announce_and_return_part("Hello");
    //     println!("announce_and_return_part(\"Hello\") -> {:?}", part);
    // }
}
