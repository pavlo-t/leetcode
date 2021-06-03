#![allow(dead_code)]
/// Longest String Chain
/// ====================
///
/// Given a list of words, each word consists of English lowercase letters.
///
/// Let's say `word1` is a predecessor of `word2` if and only if we can add exactly one letter
/// anywhere in `word1` to make it equal to `word2`.
/// For example, `"abc"` is a predecessor of `"abac"`.
///
/// A _word chain_ is a sequence of words `[word_1, word_2, ..., word_k]` with `k >= 1`,
/// where `word_1` is a predecessor of `word_2`, `word_2` is a predecessor of `word_3`, and so on.
///
/// Return the longest possible length of a word chain with words chosen from the given list of `words`.
///
/// __Constraints:__
///
/// - `1 <= words.length <= 1000`
/// - `1 <= words[i].length <= 16`
/// - `words[i]` only consists of English lowercase letters.
///
/// https://leetcode.com/explore/featured/card/may-leetcoding-challenge-2021/600/week-3-may-15th-may-21st/3746/
struct Solution;
impl Solution {
    pub fn longest_str_chain_2(words: Vec<String>) -> i32 {
        fn is_predecessor(p: &[u8], w: &[u8]) -> bool {
            let (mut pi, mut wi) = (0, 0);
            while pi < p.len() && wi < w.len() {
                if p[pi] == w[wi] {
                    pi += 1;
                    wi += 1;
                } else if pi == wi {
                    wi += 1;
                } else {
                    return false;
                }
            }
            true
        }

        let mut words = words;
        words.sort_unstable_by_key(|w| w.len());
        let mut dp = vec![1; words.len()];
        let mut longest = 1;

        for wi in 1..words.len() {
            for pi in (0..wi).rev() {
                if words[pi].len() == words[wi].len() - 1 {
                    if is_predecessor(words[pi].as_bytes(), words[wi].as_bytes()) {
                        dp[wi] = dp[pi] + 1;
                        longest = longest.max(dp[wi]);
                    }
                } else {
                    break;
                }
            }
        }

        longest
    }

    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        const MAX_LEN: usize = 16;

        if words.len() == 1 {
            1
        } else {
            fn is_predecessor(p: &[u8], w: &[u8]) -> bool {
                let (mut pi, mut wi) = (0, 0);
                while pi < p.len() && wi < w.len() {
                    if p[pi] == w[wi] {
                        pi += 1;
                        wi += 1;
                    } else if pi == wi {
                        wi += 1;
                    } else {
                        return false;
                    }
                }
                true
            }

            fn get_longest_chain(
                pl: usize,
                pi: usize,
                wls: &[Vec<String>],
                cache: &mut [Vec<i32>],
            ) -> i32 {
                if cache[pl][pi] > 0 {
                    cache[pl][pi]
                } else {
                    cache[pl][pi] = wls[pl + 1]
                        .iter()
                        .enumerate()
                        .filter(|&(_, w)| is_predecessor(wls[pl][pi].as_bytes(), w.as_bytes()))
                        .map(|(wi, _)| 1 + get_longest_chain(pl + 1, wi, wls, cache))
                        .max()
                        .unwrap_or(1);
                    cache[pl][pi]
                }
            }

            let mut wls = vec![vec![]; MAX_LEN + 2];
            words.into_iter().for_each(|w| wls[w.len()].push(w));

            let mut cache = vec![];
            wls.iter().for_each(|v| cache.push(vec![0; v.len()]));

            let mut longest = 1;
            for pl in 1..=MAX_LEN {
                for pi in 0..wls[pl].len() {
                    longest = longest.max(get_longest_chain(pl, pi, &wls, &mut cache))
                }
            }

            longest
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs { ($($x:tt), *) => { vec![$($x.to_string()), *] }; }

    #[test]
    fn example1_produces_4() {
        let words = vs!["a", "b", "ba", "bca", "bda", "bdca"];
        assert_eq!(Solution::longest_str_chain(words), 4);
        // Explanation: One of the longest word chain is "a","ba","bda","bdca".
    }
    #[test]
    fn example2_produces_5() {
        let words = vs!["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"];
        assert_eq!(Solution::longest_str_chain(words), 5);
    }
    #[test]
    fn w_abc_produces_1() {
        assert_eq!(Solution::longest_str_chain(vs!["abc"]), 1);
    }
    #[test]
    fn ws_words_from_1_to_16_len_produces_16() {
        let words = (1..17).map(|l| "a".repeat(l)).collect();
        assert_eq!(Solution::longest_str_chain(words), 16);
    }
    #[test]
    fn ws_words_from_1_to_16_len_produces_1() {
        let words = (1..17)
            .map(|l| ((b'a' + l as u8) as char).to_string().repeat(l as usize))
            .collect();
        assert_eq!(Solution::longest_str_chain(words), 1);
    }

    mod performance {
        use super::*;

        #[test]
        fn ws_1k_words_from_1_to_16_len_produces_16() {
            let words = (1..17).flat_map(|l| vec!["a".repeat(l); 63]).collect();
            assert_eq!(Solution::longest_str_chain(words), 16);
        }
        #[test]
        fn ws_1k_words_from_1_to_16_len_produces_1() {
            let words = (1..17)
                .flat_map(|l| vec![((b'a' + l as u8) as char).to_string().repeat(l as usize); 63])
                .collect();
            assert_eq!(Solution::longest_str_chain(words), 1);
        }
    }
}
