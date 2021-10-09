#![allow(dead_code)]
/// 212. Word Search II
/// ===================
///
/// Given an `m x n` board of characters and a list of strings `words`,
/// return _all words on the board_.
///
/// Each word must be constructed from letters of sequentially adjacent cells,
/// where __adjacent cells__ are horizontally or vertically neighboring.
/// The same letter cell may not be used more than once in a word.
///
/// __Constraints:__
///
/// - `1 <= board.length, board[i].length <= 12`
/// - `board[i][j]` is a lowercase English letter.
/// - `1 <= words.length <= 30_000`
/// - `1 <= words[i].length <= 10`
/// - `words[i]` consists of lowercase English letters.
/// - All the strings of `words` are unique.
///
/// https://leetcode.com/problems/word-search-ii/
struct Solution;
impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        println!("find_words({:?}, {:?})", board, words);
        use std::collections::HashSet;

        #[derive(Debug, Default)]
        struct Trie {
            word: Option<String>,
            data: [Option<Box<Self>>; 26],
        }
        impl Trie {
            fn insert(&mut self, w: String) {
                let mut curr = self;
                for i in w.bytes().map(|b| (b - b'a') as usize) {
                    if curr.data[i].is_none() {
                        curr.data[i] = Some(Box::new(Self::default()));
                    }
                    curr = curr.data[i].as_mut().unwrap();
                }
                curr.word = Some(w);
            }
            fn get(&self, b: char) -> Option<&Box<Self>> {
                self.data[(b as u8 - b'a') as usize].as_ref()
            }
        }
        let mut t = Trie::default();
        for w in words {
            t.insert(w);
        }

        fn bts(
            r: usize,
            c: usize,
            t: &Trie,
            b: &[Vec<char>],
            used: &mut Vec<Vec<bool>>,
            result: &mut HashSet<String>,
        ) {
            if let Some(t) = t.get(b[r][c]) {
                if !used[r][c] {
                    if let Some(w) = t.word.as_ref() {
                        if !result.contains(w) {
                            result.insert(w.clone());
                        }
                    }
                    used[r][c] = true;
                    for &(r, c) in [
                        (r.wrapping_sub(1), c),
                        (r + 1, c),
                        (r, c.wrapping_sub(1)),
                        (r, c + 1),
                    ]
                    .iter()
                    .filter(|&&(r, c)| r < b.len() && c < b[0].len())
                    {
                        bts(r, c, t, b, used, result);
                    }
                    used[r][c] = false;
                }
            }
        }

        let (m, n) = (board.len(), board[0].len());
        let mut used = vec![vec![false; n]; m];
        let mut result = HashSet::new();
        for r in 0..m {
            for c in 0..n {
                bts(r, c, &t, &board, &mut used, &mut result);
            }
        }
        result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($x:tt),*) => {vec![$($x.to_string()),*]};}
    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn b_oaan_etae_ihkr_iflv_w_oath_pea_eat_rain() {
        let b = vv![
            ['o', 'a', 'a', 'n'],
            ['e', 't', 'a', 'e'],
            ['i', 'h', 'k', 'r'],
            ['i', 'f', 'l', 'v']
        ];
        let w = vs!["oath", "pea", "eat", "rain"];
        let e = vs!["eat", "oath"];
        let mut r = Solution::find_words(b, w);
        r.sort_unstable();
        assert_eq!(r, e);
    }
    #[test]
    fn b_ab_cd_w_abcd() {
        let b = vv![['a', 'b'], ['c', 'd']];
        let w = vs!["abcb"];
        let e: Vec<String> = vec![];
        assert_eq!(Solution::find_words(b, w), e);
    }

    #[test]
    fn b_12x12xa_w_10a() {
        let b = vec![vec!['a'; 12]; 12];
        let w = vs!["aaaaaaaaaa"];
        let e = vs!["aaaaaaaaaa"];
        assert_eq!(Solution::find_words(b, w), e);
    }
    #[test]
    fn b_12x12xa_w_10a_9a1b_9a1c_9a1d_9a1e_9a1f() {
        let b = vec![vec!['a'; 12]; 12];
        let w = vs![
            "aaaaaaaaaa",
            "aaaaaaaaab",
            "aaaaaaaaac",
            "aaaaaaaaad",
            "aaaaaaaaae",
            "aaaaaaaaaf"
        ];
        let e = vs!["aaaaaaaaaa"];
        assert_eq!(Solution::find_words(b, w), e);
    }
}
