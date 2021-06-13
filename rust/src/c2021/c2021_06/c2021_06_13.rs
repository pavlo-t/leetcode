#![allow(dead_code)]
/// Palindrome Pairs
/// ================
///
/// Given a list of __unique__ words,
/// return all the pairs of the __distinct__ indices `(i, j)` in the given list,
/// so that the concatenation of the two words `words[i] + words[j]` is a palindrome.
///
/// __Constraints:__
///
/// - `1 <= words.length <= 5000`
/// - `0 <= words[i].length <= 300`
/// - `words[i]` consists of lower-case English letters.
///
/// https://leetcode.com/explore/featured/card/june-leetcoding-challenge-2021/604/week-2-june-8th-june-14th/3777/
struct Solution;
impl Solution {
    ///  Approach 3: Using a Trie
    ///
    /// https://leetcode.com/problems/palindrome-pairs/solution/
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;

        #[derive(Debug)]
        struct TrieNode {
            word_ending: i32,
            next: HashMap<u8, TrieNode>,
            palindrome_prefix_remaining: Vec<i32>,
        }
        impl TrieNode {
            fn new() -> Self {
                Self {
                    word_ending: -1,
                    next: HashMap::new(),
                    palindrome_prefix_remaining: Vec::new(),
                }
            }
        }

        fn is_palindrome(w: &[u8]) -> bool {
            w.len() <= 1 || {
                let (mut l, mut r) = (0, w.len() - 1);
                while l < r {
                    if w[l] != w[r] {
                        return false;
                    }
                    l += 1;
                    r -= 1;
                }
                true
            }
        }

        let mut root = TrieNode::new();

        for (i, w) in words.iter().enumerate() {
            let word_id = i as i32;
            let bs = w.as_bytes();
            let mut curr = &mut root;
            for (j, b) in w.bytes().enumerate().rev() {
                if is_palindrome(&bs[..=j]) {
                    curr.palindrome_prefix_remaining.push(word_id);
                }
                let e = curr.next.entry(b).or_insert(TrieNode::new());
                curr = e;
            }
            curr.word_ending = word_id;
        }

        let mut result = vec![];

        'words_iter: for (i, w) in words.iter().enumerate() {
            let word_id = i as i32;
            let bs = w.as_bytes();
            let mut curr = &root;
            for (j, b) in w.bytes().enumerate() {
                if curr.word_ending >= 0 && is_palindrome(&bs[j..]) {
                    result.push(vec![word_id, curr.word_ending]);
                }
                if let Some(n) = curr.next.get(&b) {
                    curr = n;
                } else {
                    continue 'words_iter;
                }
            }
            if curr.word_ending >= 0 && curr.word_ending != word_id {
                result.push(vec![word_id, curr.word_ending]);
            }
            for &j in curr.palindrome_prefix_remaining.iter() {
                result.push(vec![word_id, j]);
            }
        }

        result
    }

    /// Approach 2: Hashing
    ///
    /// https://leetcode.com/problems/palindrome-pairs/solution/
    pub fn palindrome_pairs_hash_map(words: Vec<String>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;

        fn is_palindrome(w: &[u8]) -> bool {
            let (mut l, mut r) = (0, w.len() - 1);
            while l < r {
                if w[l] != w[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }
            true
        }

        fn rev_string(w: &[u8]) -> String {
            String::from_utf8(w.iter().rev().map(|b| b.to_owned()).collect()).unwrap()
        }

        let mut wis = HashMap::new();
        for (i, w) in words.into_iter().enumerate() {
            wis.insert(w, i as i32);
        }

        let mut result = vec![];
        for (w, &i) in wis.iter() {
            let w = w.as_bytes();

            // Add reversed string
            let wr = rev_string(w);
            if let Some(&j) = wis.get(&wr) {
                if i != j {
                    result.push(vec![i, j]);
                }
            }

            // Add reversed suffixes (palindromic prefix)
            for r in 0..w.len() {
                if is_palindrome(&w[..=r]) {
                    let wr = rev_string(&w[r + 1..]);
                    if let Some(&j) = wis.get(&wr) {
                        if i != j {
                            result.push(vec![j, i]);
                        }
                    }
                }
            }

            // Add reversed prefixes (palindromic suffix)
            for l in 0..w.len() {
                if is_palindrome(&w[l..]) {
                    let wr = rev_string(&w[..l]);
                    if let Some(&j) = wis.get(&wr) {
                        if i != j {
                            result.push(vec![i, j]);
                        }
                    }
                }
            }
        }
        result
    }

    pub fn palindrome_pairs_my_brute_force_optimized(words: Vec<String>) -> Vec<Vec<i32>> {
        fn is_palindrome(w: &[u8]) -> bool {
            let (mut l, mut r) = (0, w.len() - 1);
            while l < r {
                if w[l] != w[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }
            true
        }

        fn is_palindrome_pair(wl: &[u8], wr: &[u8]) -> bool {
            match (wl.len(), wr.len()) {
                (0, 0) => true,
                (0, _) => is_palindrome(wr),
                (_, 0) => is_palindrome(wl),
                (ll, lr) => {
                    let mut l = 0;
                    let mut r = Some(lr - 1);
                    while l < ll && r.is_some() {
                        if wl[l] != wr[r.unwrap()] {
                            return false;
                        }
                        l += 1;
                        r = r.and_then(|i| i.checked_sub(1));
                    }
                    if l < ll {
                        is_palindrome(&wl[l..])
                    } else if let Some(r) = r {
                        is_palindrome(&wr[..=r])
                    } else {
                        true
                    }
                }
            }
        }

        let mut result = vec![];

        for i in 0..words.len() {
            for j in 0..words.len() {
                if i != j && is_palindrome_pair(words[i].as_bytes(), words[j].as_bytes()) {
                    result.push(vec![i as i32, j as i32]);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($s:tt),*) => {vec![$($s.to_string()),*]};}

    #[test]
    fn w_abcd_dcba_lls_s_sssll_produces_01_10_32_24() {
        let words = vs!["abcd", "dcba", "lls", "s", "sssll"];
        // let e = [[0, 1], [1, 0], [3, 2], [2, 4]];
        let e = [[0, 1], [1, 0], [2, 4], [3, 2]];
        assert_eq!(Solution::palindrome_pairs(words), e);
        // Explanation: The palindromes are ["dcbaabcd","abcddcba","slls","llssssll"]
    }
    #[test]
    fn w_bat_tab_cat_produces_01_10() {
        let words = vs!["bat", "tab", "cat"];
        let e = [[0, 1], [1, 0]];
        assert_eq!(Solution::palindrome_pairs(words), e);
        // Explanation: The palindromes are ["battab","tabbat"]
    }
    #[test]
    fn w_a_empty_produces_01_10() {
        let words = vs!["a", ""];
        let e = [[0, 1], [1, 0]];
        assert_eq!(Solution::palindrome_pairs(words), e);
    }
    #[test]
    fn w_a_produces_empty() {
        let words = vs!["a"];
        let e: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::palindrome_pairs(words), e);
    }

    #[test]
    fn w_a_abc_aba_empty_produces_03_30_23_32() {
        let words = vs!["a", "abc", "aba", ""];
        //let e = [[0, 3], [3, 0], [2, 3], [3, 2]];
        let e = [[0, 3], [2, 3], [3, 0], [3, 2]];
        assert_eq!(Solution::palindrome_pairs(words), e);
    }

    mod performance {
        use super::*;

        #[test]
        fn w_300a_until_300r_produces_269_100_combinations() {
            let words = (b'a'..b'r')
                .flat_map(|b| {
                    (1..=300)
                        .map(|i| b.to_string().repeat(i))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            println!("words.len()={}", words.len());
            let r = Solution::palindrome_pairs(words);
            assert_eq!(r.len(), 269_100);
        }
    }
}
