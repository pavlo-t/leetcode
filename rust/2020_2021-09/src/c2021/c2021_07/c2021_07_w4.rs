#![allow(dead_code)]
/// Alien Dictionary
/// ================
///
/// There is a new alien language that uses the English alphabet.
/// However, the order among the letters is unknown to you.
///
/// You are given a list of strings `words` from the alien language's dictionary,
/// where the strings in `words` are __sorted lexicographically__ by the rules of this new language.
///
/// Return _a string of the unique letters in the new alien language sorted in __lexicographically increasing order__
/// by the new language's rules.
/// If there is no solution, return `""`.
/// If there are multiple solutions, return __any of them___.
///
/// A string `s` is __lexicographically smaller__ than a string `t` if at the first letter where they differ,
/// the letter in `s` comes before the letter in `t` in the alien language.
/// If the first `min(s.length, t.length)` letters are the same,
/// then `s` is smaller if and only if `s.length < t.length`.
///
/// __Constraints:__
///
/// - `1 <= words.length <= 100`
/// - `1 <= words[i].length <= 100`
/// - `words[i]` consists of only lowercase English letters.
///
/// https://leetcode.com/explore/challenge/card/july-leetcoding-challenge-2021/611/week-4-july-22nd-july-28th/3822/
struct Solution;
impl Solution {
    /// https://rustgym.com/leetcode/269
    pub fn alien_order(words: Vec<String>) -> String {
        use std::collections::{HashMap, HashSet, VecDeque};

        let mut chars = words
            .iter()
            .flat_map(|w| w.chars())
            .collect::<HashSet<_>>()
            .into_iter()
            .fold(HashMap::new(), |mut rsf: HashMap<char, Vec<char>>, c| {
                rsf.entry(c).or_default();
                rsf
            });
        let size = chars.len();
        for (w1, w2) in words.windows(2).map(|w| (&w[0], &w[1])) {
            if w1 != w2 {
                if w1.starts_with(w2) {
                    return String::new();
                }
                if let Some((c1, c2)) = w1.chars().zip(w2.chars()).find(|(c1, c2)| c1 != c2) {
                    chars.get_mut(&c1).unwrap().push(c2);
                }
            }
        }

        let mut depth = HashMap::new();
        chars.keys().for_each(|&c| { depth.insert(c, 0); });
        chars.values().for_each(|cs| cs.iter().for_each(|c| *depth.get_mut(c).unwrap() += 1));

        let mut q = VecDeque::new();
        depth.iter().for_each(|(&k, &v)| { if v == 0 { q.push_back(k); } });

        let mut result = String::new();
        while let Some(c) = q.pop_front() {
            result.push(c);
            for &c in chars.get(&c).unwrap().iter() {
                let cd = depth.get_mut(&c).unwrap();
                *cd -= 1;
                if cd == &0 {
                    q.push_back(c);
                }
            }
        }

        if result.len() == size {
            result
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs { ($($s:tt),*) => {vec![$($s.to_string()),*]}; }

    #[test]
    fn wrt_wrf_er_ett_rftt_produces_wertf() {
        let words = vs!["wrt", "wrf", "er", "ett", "rftt"];
        assert_eq!(Solution::alien_order(words), "wertf");
    }
    #[test]
    fn z_x_produces_zx() {
        let words = vs!["z", "x"];
        assert_eq!(Solution::alien_order(words), "zx");
    }
    #[test]
    fn z_x_z_produces_empty_string() {
        let words = vs!["z", "x", "z"];
        assert_eq!(Solution::alien_order(words), "");
        //Explanation: The order is invalid, so return "".
    }

    #[test]
    fn wrt_wrtf_produces_frtw() {
        let words = vs!["wrt", "wrtf"];
        assert_eq!(Solution::alien_order(words), "frtw");
    }
    #[test]
    fn wrtf_wrt_produces_empty_string() {
        let words = vs!["wrtf", "wrt"];
        assert_eq!(Solution::alien_order(words), "");
    }
    #[test]
    fn bc_bd_de_df_fa_fb_produces_abcdef() {
        let words = vs!["bc", "bd", "de", "df", "fa", "fb"];
        assert_eq!(Solution::alien_order(words), "abcdef");
        // bdf + cd -> bcdf + ef -> bcdef + ab -> abcdef
    }
}
