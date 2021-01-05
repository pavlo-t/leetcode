#![allow(dead_code)]

/// ### Longest Substring with At Most K Distinct Characters
///
/// https://leetcode.com/explore/featured/card/december-leetcoding-challenge/573/week-5-december-29th-december-31st/3584/
struct Solution;

impl Solution {
    pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
        if k == 0 {
            0
        } else {
            use std::collections::HashMap;

            let chars = s.chars().collect::<Vec<_>>();

            let mut counts: HashMap<char, usize> = HashMap::new();
            let mut max_len = 0;
            let mut len = 0;

            let mut l = 0;

            for &rc in &chars {
                *counts.entry(rc).or_default() += 1;
                len += 1;
                while counts.len() > k as usize {
                    let lc = chars[l];
                    if counts[&lc] == 1 {
                        counts.remove(&lc);
                    } else {
                        counts.entry(lc).and_modify(|v| *v -= 1);
                    }
                    l += 1;
                    len -= 1;
                }
                max_len = max_len.max(len);
            }

            max_len
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_s_eceba_k2_is_3() {
        assert_eq!(Solution::length_of_longest_substring_k_distinct("eceba".to_string(), 2), 3);
        // Explanation: The substring is "ece" with length 3.
    }

    #[test]
    fn example2_s_aa_k1_is_2() {
        assert_eq!(Solution::length_of_longest_substring_k_distinct("aa".to_string(), 1), 2);
        // Explanation: The substring is "aa" with length 2.
    }

    #[test]
    fn s_a_repeat_50000_k0_is_0() {
        let s = String::from("a".repeat(50000));
        let k = 0;
        assert_eq!(Solution::length_of_longest_substring_k_distinct(s, k), 0);
    }

    #[test]
    fn s_ab_repeat_25000_k1_is_1() {
        let s = String::from("ab".repeat(25000));
        let k = 1;
        assert_eq!(Solution::length_of_longest_substring_k_distinct(s, k), 1);
    }

    #[test]
    fn s_ab_repeat_25000_k2_is_50000() {
        let s = String::from("ab".repeat(25000));
        let k = 2;
        assert_eq!(Solution::length_of_longest_substring_k_distinct(s, k), 50000);
    }

    #[test]
    fn s_a_repeat_50000_k1_is_50000() {
        let s = String::from("a".repeat(50000));
        let k = 1;
        assert_eq!(Solution::length_of_longest_substring_k_distinct(s, k), 50000);
    }
}
