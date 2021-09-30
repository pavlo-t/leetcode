#![allow(dead_code)]

/// # Word Subsets
///
/// We are given two arrays `A` and `B` of words.
/// Each word is a string of lowercase letters.
///
/// Now, say that word `b` is a subset of word `a` if every letter in `b` occurs in `a`,
/// __including multiplicity__.
/// For example, `"wrr"` is a subset of `"warrior"`, but is not a subset of `"world"`.
///
/// Now say a word `a` from `A` is _universal_ if for every `b` in `B`, `b` is a subset of `a`.
///
/// Return a list of all universal words in `A`.
/// You can return the words in any order.
///
/// __Note:__
///
/// - `1 <= A.length, B.length <= 10000`
/// - `1 <= A[i].length, B[i].length <= 10`
/// - `A[i]` and `B[i]` consist only of lowercase letters.
/// - All words in `A[i]` are unique: there isn't `i != j` with `A[i] == A[j]`.
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/591/week-4-march-22nd-march-28th/3685/
struct Solution;
impl Solution {
    pub fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
        let mut b_map = vec![0; 26];
        for w in b {
            let mut w_map = vec![0; 26];
            for &c in w.as_bytes() {
                w_map[(c - b'a') as usize] += 1;
            }
            for (i, &count) in w_map.iter().enumerate() {
                if b_map[i] < count {
                    b_map[i] = count;
                }
            }
        }

        let mut result = Vec::new();
        for w in a {
            let mut w_map = vec![0; 26];
            for &c in w.as_bytes() {
                w_map[(c - b'a') as usize] += 1;
            }
            if b_map
                .iter()
                .enumerate()
                .all(|(c, &count)| w_map[c] >= count)
            {
                result.push(w);
            }
        }

        result
    }
    pub fn word_subsets_hash_map(a: Vec<String>, b: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let mut b_map = HashMap::new();
        let mut w_map = HashMap::new();
        for w in b {
            w_map.clear();
            for &c in w.as_bytes() {
                *w_map.entry(c).or_insert(0) += 1;
            }
            for (&c, &count) in &w_map {
                let max = b_map.entry(c).or_insert(0);
                if *max < count {
                    *max = count;
                }
            }
        }

        let mut result = Vec::new();
        for w in a {
            w_map.clear();
            for &c in w.as_bytes() {
                *w_map.entry(c).or_insert(0) += 1;
            }
            if b_map
                .iter()
                .all(|(c, count)| w_map.get(c).unwrap_or(&0) >= count)
            {
                result.push(w);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let a = ["amazon", "apple", "facebook", "google", "leetcode"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let b = ["e", "o"].iter().map(|s| s.to_string()).collect();
        let e = ["facebook", "google", "leetcode"];
        assert_eq!(Solution::word_subsets(a, b), e);
    }
    #[test]
    fn example2() {
        let a = ["amazon", "apple", "facebook", "google", "leetcode"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let b = ["l", "e"].iter().map(|s| s.to_string()).collect();
        let e = ["apple", "google", "leetcode"];
        assert_eq!(Solution::word_subsets(a, b), e);
    }
    #[test]
    fn example3() {
        let a = ["amazon", "apple", "facebook", "google", "leetcode"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let b = ["e", "oo"].iter().map(|s| s.to_string()).collect();
        let e = ["facebook", "google"];
        assert_eq!(Solution::word_subsets(a, b), e);
    }
    #[test]
    fn example4() {
        let a = ["amazon", "apple", "facebook", "google", "leetcode"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let b = ["lo", "eo"].iter().map(|s| s.to_string()).collect();
        let e = ["google", "leetcode"];
        assert_eq!(Solution::word_subsets(a, b), e);
    }
    #[test]
    fn example5() {
        let a = ["amazon", "apple", "facebook", "google", "leetcode"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let b = ["ec", "oc", "ceo"].iter().map(|s| s.to_string()).collect();
        let e = ["facebook", "leetcode"];
        assert_eq!(Solution::word_subsets(a, b), e);
    }

    mod performance {
        use super::*;

        #[test]
        fn a10k10a_b10k10a_produces_10k10a() {
            let a = vec!["a".repeat(10); 10_000];
            let b = a.clone();
            let e = a.clone();
            assert_eq!(Solution::word_subsets(a, b), e);
        }
    }
}
