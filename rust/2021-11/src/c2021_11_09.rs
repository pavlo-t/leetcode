#![allow(dead_code)]
/// 1178. Number of Valid Words for Each Puzzle
/// ===========================================
///
/// With respect to a given `puzzle` string, a `word` is _valid_ if both the following conditions are satisfied:
///
/// - `word` contains the first letter of `puzzle`.
/// - For each letter in `word`, that letter is in `puzzle`.
///
/// For example,
/// if the puzzle is `"abcdefg"`, then valid words are `"faced"`, `"cabbage"`, and `"baggage"`, while
/// invalid words are `"beefed"` (does not include `'a'`) and `"based"` (includes `'s'` which is not in the puzzle).
///
/// Return _an array `answer`, where `answer[i]` is the number of words in the given word list `words`
/// that is valid with respect to the puzzle puzzles[i]_.
///
/// __Constraints:__
///
/// - `1 <= words.length <= 100_000`
/// - `4 <= words[i].length <= 50`
/// - `1 <= puzzles.length <= 10_000`
/// - `puzzles[i].length == 7`
/// - `words[i]` and `puzzles[i]` consist of lowercase English letters.
/// - Each `puzzles[i]` does not contain repeated characters.
///
/// https://leetcode.com/problems/number-of-valid-words-for-each-puzzle/
struct Solution;
impl Solution {
    /// Approach 1: Hashing (Bitmask)
    /// https://leetcode.com/problems/number-of-valid-words-for-each-puzzle/solution/
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        //println!("find_num_of_valid_words({:?}, {:?})", words, puzzles);
        use std::collections::HashMap;

        #[rustfmt::skip] struct Puzzle { first: i32, all: i32 }
        #[rustfmt::skip] fn encode_byte(b: u8) -> i32 { 1 << (b - b'a') }
        #[rustfmt::skip] fn encode_word(w: String) -> i32 { w.bytes().fold(0, |rsf, b| rsf | encode_byte(b)) }
        #[rustfmt::skip] fn encode_puzzle(p: String) -> Puzzle {
            Puzzle { first: encode_byte(p.as_bytes()[0]), all: encode_word(p) }
        }

        let mut ws: HashMap<i32, i32> = HashMap::new();
        words
            .into_iter()
            .map(encode_word)
            .filter(|w| w.count_ones() <= 7)
            .for_each(|w| *ws.entry(w).or_default() += 1);

        puzzles
            .into_iter()
            .map(encode_puzzle)
            .map(|p| {
                let submask = p.all ^ p.first;
                let mut subset = submask;
                let mut result = ws.get(&p.first).unwrap_or(&0).to_owned();
                while subset > 0 {
                    if let Some(count) = ws.get(&(subset | p.first)) {
                        result += count;
                    }
                    subset = (subset - 1) & submask;
                }
                result
            })
            .collect()
    }

    pub fn find_num_of_valid_words_my(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        //println!("find_num_of_valid_words({:?}, {:?})", words, puzzles);
        #[rustfmt::skip] struct Puzzle { first: i32, all: i32 }
        #[rustfmt::skip] fn encode_byte(b: u8) -> i32 { 1 << (b - b'a') }
        #[rustfmt::skip] fn encode_word(w: String) -> i32 { w.bytes().fold(0, |rsf, b| rsf | encode_byte(b)) }
        #[rustfmt::skip] fn encode_puzzle(p: String) -> Puzzle {
            Puzzle { first: encode_byte(p.as_bytes()[0]), all: encode_word(p) }
        }

        let ps: Vec<Puzzle> = puzzles.into_iter().map(encode_puzzle).collect();
        let ws: Vec<i32> = words.into_iter().map(encode_word).collect();

        ps.into_iter()
            .map(|p| {
                ws.iter()
                    .filter(|&w| w & p.first != 0 && w | p.all == p.all)
                    .count() as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($s:tt),*) => {vec![$($s.to_string()),*]};}

    #[test]
    fn w_aaaa_p_abcdefg() {
        let w = vs!["aaaa"];
        let p = vs!["abcdefg"];
        let e = [1];
        //   abcdefg          abcdefg
        // p 1111111   w|p => 1111111  == p
        // w 1000000  <= aaaa
        //
        //   abcdefgh         abcdefgh
        // p 11111110  w|p => 11111111 != p
        // w 10000001 <= aaah
        assert_eq!(Solution::find_num_of_valid_words(w, p), e);
    }
    #[test]
    fn w_bbbb_p_abcdefg() {
        let w = vs!["bbbb"];
        let p = vs!["abcdefg"];
        let e = [0];
        assert_eq!(Solution::find_num_of_valid_words(w, p), e);
    }
    #[test]
    fn w_bbba_p_abcdefg() {
        let w = vs!["bbba"];
        let p = vs!["abcdefg"];
        let e = [1];
        assert_eq!(Solution::find_num_of_valid_words(w, p), e);
    }
    #[test]
    fn w_bbha_p_abcdefg() {
        let w = vs!["bbha"];
        let p = vs!["abcdefg"];
        let e = [0];
        assert_eq!(Solution::find_num_of_valid_words(w, p), e);
    }
    #[test]
    fn example1() {
        let w = vs!["aaaa", "asas", "able", "ability", "actt", "actor", "access"];
        let p = vs!["aboveyz", "abrodyz", "abslute", "absoryz", "actresz", "gaswxyz"];
        let e = [1, 1, 3, 2, 4, 0];
        assert_eq!(Solution::find_num_of_valid_words(w, p), e);
        // Explanation:
        // 1 valid word for "aboveyz" : "aaaa"
        // 1 valid word for "abrodyz" : "aaaa"
        // 3 valid words for "abslute" : "aaaa", "asas", "able"
        // 2 valid words for "absoryz" : "aaaa", "asas"
        // 4 valid words for "actresz" : "aaaa", "asas", "actt", "access"
        // There are no valid words for "gaswxyz" cause none of the words in the list contains letter 'g'.
    }
    #[test]
    fn example2() {
        let w = vs!["apple", "pleas", "please"];
        let p = vs!["aelwxyz", "aelpxyz", "aelpsxy", "saelpxy", "xaelpsy"];
        let e = [0, 1, 3, 2, 0];
        assert_eq!(Solution::find_num_of_valid_words(w, p), e);
    }

    //#[ignore]
    #[test]
    fn w_100000x_a_repeat_50_p_10000x_abcdefg() {
        let w = vec!["a".repeat(50); 100000];
        let p = vec!["abcdefg".to_string(); 10000];
        let e = [100000; 10000];
        assert_eq!(Solution::find_num_of_valid_words(w, p), e);
    }

    //#[ignore]
    #[test]
    fn w_100000x50letters_p_10000x7letters() {
        use std::collections::HashSet;

        fn i32_to_word(i: i32) -> String {
            String::from_utf8(
                format!("{:05}", i)
                    .into_bytes()
                    .into_iter()
                    .map(|b| b - b'0' + b'a')
                    .collect(),
            )
            .unwrap()
        }
        fn i32_to_puzzle(i: i32) -> String {
            let mut s = String::with_capacity(7);
            let mut l = 0;
            while s.len() < 7 {
                if (1 << l) & i != 0 {
                    s.push((b'a' + l as u8) as char);
                }
                l += 1;
            }
            s
        }

        let mut ws = HashSet::with_capacity(100000);
        let mut i = 0;
        while ws.len() < 100000 {
            let w = i32_to_word(i);
            ws.insert(w.repeat(10));
            i += 1;
        }
        let mut ws = ws.into_iter().collect::<Vec<_>>();
        ws.sort_unstable();
        //println!(" words: {:?}", ws);
        //println!("   len: {:?}", ws.len());

        let mut ps = Vec::with_capacity(10000);
        let mut i = 0i32;
        while ps.len() < 10000 {
            if i.count_ones() == 7 {
                let p = i32_to_puzzle(i);
                ps.push(p);
            }
            i += 1;
        }
        //println!(" puzzles: {:?}", ps);
        //println!("   len: {:?}", ps.len());

        let result = Solution::find_num_of_valid_words(ws, ps);
        assert_eq!(result.iter().sum::<i32>(), 17982700);
    }
}
