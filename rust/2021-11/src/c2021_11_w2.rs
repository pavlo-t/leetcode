#![allow(dead_code)]
/// 425. Word Squares
/// =================
///
/// Given an array of __unique__ strings `words`,
/// return _all the [__word squares__](https://en.wikipedia.org/wiki/Word_square) you can build from `words`_.
/// The same word from `words` can be used __multiple times__.
/// You can return the answer in __any order__.
///
/// A sequence of strings forms a valid __word square__ if the `k`th row and column read the same string,
/// where `0 <= k < max(numRows, numColumns)`.
///
/// For example, the word sequence `["ball","area","lead","lady"]` forms a word square
/// because each word reads the same both horizontally and vertically.
///
/// __Constraints:__
///
/// - `1 <= words.length <= 1000`
/// - `1 <= words[i].length <= 5`
/// - All `words[i]` have the same length.
/// - `words[i]` consists of only lowercase English letters.
/// - All `words[i]` are __unique__.
///
/// https://leetcode.com/problems/word-squares/
struct Solution;
impl Solution {
    /// 20:40-20:57
    pub fn word_squares(words: Vec<String>) -> Vec<Vec<String>> {
        println!("word_squares({:?})", words);
        use std::collections::HashMap;

        #[derive(Default, Debug)]
        struct Trie {
            children: HashMap<u8, Box<Trie>>,
            words: Vec<usize>,
        }
        impl Trie {
            #[rustfmt::skip]
            fn new() -> Self { Self::default() }

            fn add(&mut self, w: &[u8], i: usize) {
                let mut curr = self;
                curr.words.push(i);
                for &b in w {
                    curr = curr.children.entry(b).or_default();
                    curr.words.push(i);
                }
            }

            fn words_by_pref(&self, pref: &[u8]) -> Option<&[usize]> {
                let mut curr = self;
                for b in pref {
                    if let Some(c) = curr.children.get(b) {
                        curr = c;
                    } else {
                        return None;
                    }
                }
                Some(&curr.words)
            }
        }

        fn bts(
            ws: &[&[u8]],
            prefixes: &Trie,
            curr: &mut Vec<usize>,
            rsf: &mut Vec<Vec<String>>,
        ) {
            let n = ws[0].len();
            if curr.len() == n {
                let r = curr
                    .iter()
                    .map(|&i| ws[i].iter().map(|&b| b).collect::<Vec<_>>())
                    .map(String::from_utf8)
                    .map(|r| r.unwrap())
                    .collect::<Vec<_>>();
                rsf.push(r);
            } else {
                let cl = curr.len();
                let pref = (0..cl).map(|r| ws[curr[r]][cl]).collect::<Vec<u8>>();
                if let Some(is) = prefixes.words_by_pref(&pref) {
                    for &i in is {
                        curr.push(i);
                        bts(ws, prefixes, curr, rsf);
                        curr.pop();
                    }
                }
            }
        }

        let ws = words.iter().map(|w| w.as_bytes()).collect::<Vec<_>>();
        let mut prefixes = Trie::new();
        for (i, w) in ws.iter().enumerate() {
            prefixes.add(w, i);
        }
        //println!("  prefixes({:?})", prefixes);

        let mut results = vec![];
        bts(&ws, &prefixes, &mut vec![], &mut results);
        results
    }

    /// 19:10-20:00
    pub fn word_squares_my_hash_table(words: Vec<String>) -> Vec<Vec<String>> {
        println!("word_squares({:?})", words);
        use std::collections::HashMap;

        fn bts(
            ws: &[&[u8]],
            prefixes: &HashMap<Vec<u8>, Vec<usize>>,
            curr: &mut Vec<usize>,
            rsf: &mut Vec<Vec<String>>,
        ) {
            let n = ws[0].len();
            if curr.len() == n {
                let r = curr
                    .iter()
                    .map(|&i| ws[i].iter().map(|&b| b).collect::<Vec<_>>())
                    .map(String::from_utf8)
                    .map(|r| r.unwrap())
                    .collect::<Vec<_>>();
                rsf.push(r);
            } else {
                let cl = curr.len();
                let pref = (0..cl).map(|r| ws[curr[r]][cl]).collect::<Vec<u8>>();
                if let Some(is) = prefixes.get(&pref) {
                    for &i in is {
                        curr.push(i);
                        bts(ws, prefixes, curr, rsf);
                        curr.pop();
                    }
                }
            }
        }

        let ws = words.iter().map(|w| w.as_bytes()).collect::<Vec<_>>();
        let mut prefixes: HashMap<Vec<u8>, Vec<usize>> = HashMap::with_capacity(ws.len());
        for (i, w) in ws.iter().enumerate() {
            prefixes.entry(vec![]).or_default().push(i);
            for pl in 1..w.len() {
                let pref = w.iter().take(pl).map(|&b| b).collect::<Vec<u8>>();
                prefixes.entry(pref).or_default().push(i);
            }
        }
        //println!("  prefixes({:?})", prefixes);

        let mut results = vec![];
        bts(&ws, &prefixes, &mut vec![], &mut results);
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($s:tt),*) => {vec![$($s.to_string()),*]};}

    #[test]
    fn area_lead_wall_lady_ball() {
        let w = vs!["area", "lead", "wall", "lady", "ball"];
        #[rustfmt::skip]
        let e = [
            ["wall",
             "area",
             "lead",
             "lady"],
            ["ball",
             "area",
             "lead",
             "lady"],
        ];
        assert_eq!(Solution::word_squares(w), e);
        // Explanation:
        // The output consists of two word squares.
        // The order of output does not matter (just the order of words in each word square matters).
    }
    #[test]
    fn abat_baba_atan_atal() {
        let w = vs!["abat", "baba", "atan", "atal"];
        #[rustfmt::skip]
        let e = [
            ["baba",
             "abat",
             "baba",
             "atan"],
            ["baba",
             "abat",
             "baba",
             "atal"],
        ];
        assert_eq!(Solution::word_squares(w), e);
        // Explanation:
        // The output consists of two word squares. The order of output does not matter (just the order of words in each word square matters).
    }

    fn i32_to_word(i: i32) -> String {
        String::from_utf8(
            format!("{:0<5}", i)
                .into_bytes()
                .into_iter()
                .map(|b| b - b'0' + b'a')
                .collect(),
        )
        .unwrap()
    }

    mod performance {
        use super::*;

        //#[ignore]
        #[test]
        fn ws_1000x5() {
            use std::collections::HashSet;
            let mut ws = HashSet::with_capacity(1000);
            let mut i = 0;
            while ws.len() < 1000 {
                let w = i32_to_word(i);
                i += 1;
                ws.insert(w);
            }
            let mut ws = ws.into_iter().collect::<Vec<_>>();
            ws.sort_unstable();
            let r = Solution::word_squares(ws);
            assert_eq!(r.len(), 734_248);
        }
    }
}
