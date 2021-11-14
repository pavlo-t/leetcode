#![allow(dead_code)]
/// 1048. Longest String Chain
/// ==========================
///
/// You are given an array of `words` where each word consists of lowercase English letters.
///
/// `wordA` is a __predecessor__ of `wordB` if and only if we can insert __exactly one__ letter
/// anywhere in `wordA` __without changing the order of the other characters__ to make it equal to `wordB`.
///
/// For example, `"abc"` is a __predecessor__ of `"abac"`, while `"cba"` is __not a predecessor__ of `"bcad"`.
///
/// A __word chain__ is a sequence of words `[word1, word2, ..., wordk]` with `k >= 1`,
/// where `word1` is a __predecessor__ of `word2`, `word2` is a __predecessor__ of `word3`, and so on.
/// A single word is trivially a __word chain__ with `k == 1`.
///
/// Return _the __length__ of the __longest possible word chain__ with words chosen from the given list of `words`_.
///
/// __Constraints:__
///
/// - `1 <= words.length <= 1000`
/// - `1 <= words[i].length <= 16`
/// - `words[i]` only consists of lowercase English letters.
///
/// https://leetcode.com/problems/longest-string-chain/
struct Solution;
impl Solution {
    /// 15:54-16:17
    pub fn longest_str_chain_my_1(words: Vec<String>) -> i32 {
        println!("longest_str_chain({:?})", words);
        use std::collections::HashMap;

        const WORD_MAX_LEN: i32 = 16;

        fn is_next(p: &[u8], n: &[u8]) -> bool {
            if p.len() != n.len() - 1 {
                false
            } else {
                let mut diff = 0;
                let (mut pi, mut ni) = (0, 0);
                while pi < p.len() && ni < n.len() {
                    if p[pi] == n[ni] {
                        pi += 1;
                        ni += 1;
                    } else {
                        ni += 1;
                        diff += 1;
                        if diff > 1 {
                            return false;
                        }
                    }
                }
                pi == p.len() && (ni == n.len() || (diff == 0 && ni == n.len() - 1))
            }
        }

        let mut nexts: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, p) in words.iter().enumerate() {
            for (j, n) in words.iter().enumerate() {
                if i != j && is_next(p.as_bytes(), n.as_bytes()) {
                    nexts.entry(i).or_default().push(j);
                }
            }
        }

        fn longest_chain(i: usize, nexts: &HashMap<usize, Vec<usize>>) -> i32 {
            if let Some(nss) = nexts.get(&i) {
                1 + nss.iter().map(|&i| longest_chain(i, nexts)).max().unwrap()
            } else {
                1
            }
        }
        let mut max_chain_len = 1;
        for i in 0..words.len() {
            max_chain_len = max_chain_len.max(longest_chain(i, &nexts));
            if max_chain_len == WORD_MAX_LEN {
                break;
            }
        }
        max_chain_len
    }
    /// 15:54
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        println!("longest_str_chain({:?})", words);
        use std::collections::HashMap;

        const WORD_MAX_LEN: i32 = 16;

        fn is_next(p: &[u8], n: &[u8]) -> bool {
            if p.len() != n.len() - 1 {
                false
            } else {
                let mut diff = 0;
                let (mut pi, mut ni) = (0, 0);
                while pi < p.len() && ni < n.len() {
                    if p[pi] == n[ni] {
                        pi += 1;
                        ni += 1;
                    } else {
                        ni += 1;
                        diff += 1;
                        if diff > 1 {
                            return false;
                        }
                    }
                }
                pi == p.len() && (ni == n.len() || (diff == 0 && ni == n.len() - 1))
            }
        }

        let mut nexts: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, p) in words.iter().enumerate() {
            for (j, n) in words.iter().enumerate() {
                if i != j && is_next(p.as_bytes(), n.as_bytes()) {
                    nexts.entry(i).or_default().push(j);
                }
            }
        }

        fn longest_chain(i: usize, nexts: &HashMap<usize, Vec<usize>>, memo: &mut Vec<i32>) -> i32 {
            if memo[i] != -1 {
                memo[i]
            } else {
                memo[i] = if let Some(nss) = nexts.get(&i) {
                    1 + nss
                        .iter()
                        .map(|&i| longest_chain(i, nexts, memo))
                        .max()
                        .unwrap()
                } else {
                    1
                };
                memo[i]
            }
        }
        let mut max_chain_len = 1;
        let mut memo = vec![-1; words.len()];
        for i in 0..words.len() {
            max_chain_len = max_chain_len.max(longest_chain(i, &nexts, &mut memo));
            if max_chain_len == WORD_MAX_LEN {
                break;
            }
        }
        max_chain_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs { ($($s:expr),*) => { vec![$($s.to_string()),*] }; }

    #[test]
    fn a_b_ba_bca_bda_bdca() {
        let w = vs!["a", "b", "ba", "bca", "bda", "bdca"];
        assert_eq!(Solution::longest_str_chain(w), 4);
        // Explanation: One of the longest word chains is ["a","ba","bda","bdca"].
    }
    #[test]
    fn xbc_pcxbcf_xb_cxbc_pcxbc() {
        let w = vs!["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"];
        assert_eq!(Solution::longest_str_chain(w), 5);
        // Explanation: All the words can be put in a word chain ["xb", "xbc", "cxbc", "pcxbc", "pcxbcf"].
    }
    #[test]
    fn abcd_dbqca() {
        let w = vs!["abcd", "dbqca"];
        assert_eq!(Solution::longest_str_chain(w), 1);
        // Explanation: The trivial word chain ["abcd"] is one of the longest word chains.
        // ["abcd","dbqca"] is not a valid word chain because the ordering of the letters is changed.
    }

    #[test]
    fn a_to_q_repeat_1000() {
        let w = vec!["abcdefghijklmnopq".to_string(); 1000];
        assert_eq!(Solution::longest_str_chain(w), 1);
    }
    #[test]
    fn a_ab_abc_etc_to_q_repeat_1000() {
        let mut w = vec!["abcdefghijklmnopq".to_string(); 1000];
        w.push("abcdefghijklmnop".to_string());
        w.push("abcdefghijklmno".to_string());
        w.push("abcdefghijklmn".to_string());
        w.push("abcdefghijklm".to_string());
        w.push("abcdefghijkl".to_string());
        w.push("abcdefghijk".to_string());
        w.push("abcdefghij".to_string());
        w.push("abcdefghi".to_string());
        w.push("abcdefgh".to_string());
        w.push("abcdefg".to_string());
        w.push("abcdef".to_string());
        w.push("abcde".to_string());
        w.push("abcd".to_string());
        w.push("abc".to_string());
        assert_eq!(Solution::longest_str_chain(w), 15);
    }
    #[test]
    fn test_79() {
        let w = vs![
            "klmnowx",
            "abcdefgiklmno",
            "fgij",
            "bcfghijklmno",
            "fgjpqrst",
            "uy",
            "abceklmnouvw",
            "pqrstuwy",
            "fghijlno",
            "mnouvwxy",
            "klmnopqt",
            "klmnopqrstuy",
            "aeuvw",
            "muvw",
            "abcdeklmnow",
            "fhijpqrst",
            "mpqrst",
            "klmnoprt",
            "fghijklno",
            "abcdelmo",
            "klnuvwxy",
            "klmnopst",
            "abcdeklmnov",
            "fghj",
            "luvwxy",
            "ghklmnopqrst",
            "pqrstwx",
            "abcdklmno",
            "cdefghij",
            "pqrs",
            "efghijklmno",
            "fghjklmno",
            "adeklmno",
            "rs",
            "kuvwxy",
            "ghij",
            "befghijklmno",
            "ln",
            "hijklmnopqrst",
            "ghpqrst",
            "fgiklmnopqrst",
            "pqrtuvwxy",
            "pqrsty",
            "jklmnopqrst",
            "lnouvwxy",
            "klmnoqsuvwxy",
            "abcdeghklmno",
            "fi",
            "fghijlnpqrst",
            "abdklmnouvw",
            "uwx",
            "abcdekln",
            "klmno",
            "abcdekn",
            "abcdemuvw",
            "pqs",
            "fghijpqt",
            "klmnopqrstuw",
            "n",
            "nopqrstuvwxy",
            "abcdefghj",
            "fghiklmnopqrst",
            "klmnorst",
            "abcdemnouvw",
            "fgh",
            "pqt",
            "abfghij",
            "o",
            "nouvw",
            "abcdklmnouvw",
            "abeklmno",
            "abcden",
            "klmnopqrstwxy",
            "q",
            "fghijklmnoprt",
            "klmnovx",
            "abceuvw",
            "klmnopsuvwxy",
            "hj",
            "abcdefgh",
            "fhjklmno",
            "klmnoquvwxy",
            "wxy",
            "klmnopqrstuvwy",
            "kln",
            "abcdegklmno",
            "mno",
            "gklmno",
            "klnouvw",
            "fghijklmnoqr",
            "fghijpqrst",
            "mnuvwxy",
            "ghipqrst",
            "klmnoqrtuvwxy",
            "acdfghij",
            "uwy",
            "fghjklmnopqrst",
            "mnpqrstuvwxy",
            "abcdeknouvw",
            "abcdefghijklmno",
            "klmnorsuvwxy",
            "abcdeh",
            "klmnost",
            "iklmnopqrst",
            "abcdegijklmno",
            "fghijklmopqrst",
            "fghijklmnors",
            "pqrstux",
            "abcdefghijlm",
            "abcdem",
            "klmn",
            "opqrst",
            "ghjklmnopqrst",
            "cdfghij",
            "kluvwxy",
            "ceklmno",
            "abcdeghijklmno",
            "lmo",
            "bklmno",
            "fghijs",
            "cdeklmnouvw",
            "abcdeknuvw",
            "cdklmnouvw",
            "abcdeklmnovw",
            "klmnopr",
            "fghijklmnopqrst",
            "klmnopqtuvwxy",
            "abcdefhijklmno",
            "abcdeuv",
            "abcdefhklmno",
            "x",
            "abcdeouvw",
            "fjklmno",
            "a",
            "klmnopqrstuv",
            "abdklmno",
            "fghijlm",
            "bcefghijklmno",
            "quvwxy",
            "fghi",
            "klmnopqrstuwx",
            "r",
            "klmnuvw",
            "kn",
            "abcdeklmn",
            "abcdeklmno",
            "ps",
            "klmnoqrt",
            "pstuvwxy",
            "klmnopqrsty",
            "lmn",
            "d",
            "abcdefghijmo",
            "fghijmnopqrst",
            "ghiklmno",
            "mouvwxy",
            "abcdeghj",
            "fghijklmnopr",
            "kmnouvwxy",
            "fghijklmnopqst",
            "klmnox",
            "nouvwxy",
            "adefghijklmno",
            "kmo",
            "klmnovy",
            "klmnopqrstuvwxy",
            "cde",
            "y",
            "klmnouy",
            "fgklmnopqrst",
            "nuvwxy",
            "kluvw",
            "abcdefghijkno",
            "abcdekl",
            "fghijno",
            "ceuvw",
            "abcdelo",
            "bcdklmno",
            "gij",
            "abcdeijklmno",
            "klmnopqrt",
            "abcdeklm",
            "pqruvwxy",
            "klnuvw",
            "fijklmno",
            "knpqrstuvwxy",
            "fghijklmnor",
            "ace",
            "abcdekluvw",
            "deklmnouvw",
            "lpqrstuvwxy",
            "abcdefhiklmno",
            "fijpqrst",
            "klmnopqrstvwy",
            "mn",
            "kmpqrst",
            "ipqrst",
            "fghijkmnpqrst",
            "uvxy",
            "bklmnouvw",
            "fghijkmnopqrst",
            "fghijklnopqrst",
            "kopqrstuvwxy",
            "pqrstuwxy",
            "abdeuvw",
            "acefghij",
            "jklmno",
            "k",
            "fghijqs",
            "abcdefghijno",
            "fghijklmnort",
            "adeuvw",
            "vwx",
            "gjklmno",
            "hi",
            "abcdefghijm",
            "nuvw",
            "fklmnopqrst",
            "abcdehklmno",
            "fghijklmnoqrt",
            "klmnoruvwxy",
            "abcdemnuvw",
            "klmnovw",
            "klmnopqrstvw",
            "klmnoqtuvwxy",
            "klmnoy",
            "wx",
            "ouvwxy",
            "lmopqrst",
            "fghjpqrst",
            "lnuvwxy",
            "vy",
            "abdfghijklmno",
            "kmnouvw",
            "uvy",
            "klmnoqst",
            "klmnort",
            "pqrstuxy",
            "qs",
            "lopqrstuvwxy",
            "mnouvw",
            "abdeklmno",
            "abcdelnouvw",
            "pruvwxy",
            "qrsuvwxy",
            "cklmno",
            "bc",
            "acdfghijklmno",
            "j",
            "fghijpqs",
            "fghijknopqrst",
            "fgijklmno",
            "fghijnopqrst",
            "mpqrstuvwxy",
            "knopqrst",
            "acdeuvw",
            "lnouvw",
            "fghijklmn",
            "klmnouvxy",
            "abcdefghiklmno",
            "abcdefghij",
            "hij",
            "abcdekuvw",
            "klmnopqs",
            "aklmnouvw",
            "acdefghijklmno",
            "cfghij",
            "fghijpqst",
            "cfghijklmno",
            "abcdefhi",
            "kmouvwxy",
            "pquvwxy",
            "pqrstuy",
            "pqrstuwx",
            "ce",
            "klmnopqrstv",
            "deklmno",
            "klmnouvw",
            "abcdeno",
            "fghijkmo",
            "knouvwxy",
            "fpqrst",
            "hklmno",
            "fghijkmopqrst",
            "abcdefghjklmno",
            "kmpqrstuvwxy",
            "dklmnouvw",
            "abcdefghijkmo",
            "abcdel",
            "giklmnopqrst",
            "buvw",
            "klmo",
            "klmnoqrsuvwxy",
            "uvwy",
            "abcdeluvw",
            "klmnostuvwxy",
            "bceklmno",
            "fghijm",
            "defghijklmno",
            "ijklmno",
            "pqr",
            "abcduvw",
            "fghijklmnopqt",
            "hijklmno",
            "klmnoprsuvwxy",
            "aeklmno",
            "lo",
            "klmnouxy",
            "cdklmno",
            "fghijpr",
            "muvwxy",
            "lmnouvwxy",
            "abce",
            "kmnopqrst",
            "pt",
            "klmnoptuvwxy",
            "abde",
            "abcdeghiklmno",
            "klmnopqsuvwxy",
            "klmnouvx",
            "efghij",
            "klmnouwy",
            "bceklmnouvw",
            "klmnuvwxy",
            "ad",
            "klmnoux",
            "gjpqrst",
            "abcdegij",
            "fghijkpqrst",
            "fghijlmnopqrst",
            "abcdefj",
            "fghijklmnops",
            "fghijklmnoqs",
            "ghi",
            "fghijqst",
            "abcdemo",
            "abcdefghijkl",
            "fghklmnopqrst",
            "no",
            "klmnopqrstuwxy",
            "klmnopqrstvwx",
            "abcdeklmnou",
            "abdefghijklmno",
            "klmnopqrstxy",
            "acfghij",
            "pqrstuvwy",
            "kuvw",
            "abcdehi",
            "de",
            "fghijpq",
            "lmuvwxy",
            "abcdelmouvw",
            "abdfghij",
            "knuvwxy",
            "acde",
            "cuvw",
            "uw",
            "kmopqrstuvwxy",
            "abcfghij",
            "fghij",
            "abcdeklmo",
            "abcdehij",
            "abcdek",
            "fghijklmnoqrst",
            "pqrstvx",
            "klmnopqrstvx",
            "abc",
            "klmnoqrstuvwxy",
            "acklmnouvw",
            "afghijklmno",
            "abcdeklmnuvw",
            "abcdei",
            "fgiklmno",
            "klmnopqrstuvw",
            "abcklmnouvw",
            "abcdelmnuvw",
            "abcdehj",
            "abcdefghijlo",
            "fghpqrst",
            "acfghijklmno",
            "abcdeg",
            "klpqrst",
            "klmouvw",
            "bdfghijklmno",
            "aefghijklmno",
            "fijklmnopqrst",
            "fghijps",
            "fghijklnpqrst",
            "fghijklmnop",
            "fklmno",
            "klmnopqrstw",
            "abcdfghijklmno",
            "fj",
            "lnopqrst",
            "fghijkmpqrst",
            "fghijlnopqrst",
            "ux",
            "fgjklmno",
            "fghijklmnopqr",
            "abcdeghjklmno",
            "abcdefghijkmn",
            "abcdegiklmno",
            "abcdefiklmno",
            "acdeklmno",
            "klmnops",
            "fghijklmnopqrt",
            "fghijklmnoqt",
            "abcdegj",
            "acdklmno",
            "abcdeghi",
            "abcdelmuvw",
            "abcdefg",
            "fghijkl",
            "gjklmnopqrst",
            "hipqrst",
            "klmnopqstuvwxy",
            "lnuvw",
            "ghijpqrst",
            "pqrstuvwxy",
            "klmnovwx",
            "klmnoqstuvwxy",
            "bcklmnouvw",
            "abdeklmnouvw",
            "fghijklmnopq",
            "abcdefghijk",
            "abcdefghijklno",
            "fiklmnopqrst",
            "klmnopq",
            "npqrstuvwxy",
            "klmnopqrstwx",
            "abcdefi",
            "lpqrst",
            "ghijklmnopqrst",
            "bd",
            "bcuvw",
            "hjklmnopqrst",
            "adfghijklmno",
            "klmnosuvwxy",
            "louvwxy",
            "bce",
            "gpqrst",
            "fghijklmnot",
            "wy",
            "pqrstvwxy",
            "fghijkln",
            "lmouvw",
            "gi",
            "fghijn",
            "fhklmnopqrst",
            "ghj",
            "klpqrstuvwxy",
            "abcdefgklmno",
            "fghijlmo",
            "fghijo",
            "fhpqrst",
            "klmnoprs",
            "abcefghij",
            "abcdeklno",
            "abcdefghijlmno",
            "kmnpqrst",
            "fghijklmnoq",
            "abcdefhij",
            "fghijklmnopt",
            "klmnopt",
            "abcdefklmno",
            "abcdelno",
            "pqrstw",
            "fghijlopqrst",
            "bfghij",
            "abuvw",
            "abcdefghijln",
            "ac",
            "aefghij",
            "prs",
            "lmnopqrstuvwxy",
            "abcdefghijn",
            "klmnopqrstux",
            "luvw",
            "giklmno",
            "kpqrst",
            "jpqrst",
            "fghijrt",
            "fghijlo",
            "abcdefij",
            "fghijklmnorst",
            "fghijqrst",
            "tuvwxy",
            "fghijlmopqrst",
            "klmnouwxy",
            "bdeuvw",
            "fghijprt",
            "klmnoprst",
            "pqrstuvy",
            "fghijpqrt",
            "fgj",
            "pqrstvxy",
            "abdefghij",
            "abcdegh",
            "abefghij",
            "lno",
            "klmnopuvwxy",
            "klmnouvwxy",
            "fghijrst",
            "cd",
            "euvw",
            "hijpqrst",
            "pqrst",
            "klopqrst",
            "gijpqrst",
            "klmnopqrs",
            "fghijk",
            "klopqrstuvwxy",
            "rtuvwxy",
            "klmnorstuvwxy",
            "stuvwxy",
            "abcdevw",
            "cdefghijklmno",
            "bdeklmno",
            "pqstuvwxy",
            "fghipqrst",
            "fghijpst",
            "kmn",
            "mo",
            "abcdeuw",
            "qst",
            "fghijklmnost",
            "klnopqrst",
            "abcdekm",
            "abcdefgj",
            "klmnopqrstuvy",
            "kouvw",
            "abcdelmnouvw",
            "abefghijklmno",
            "fgipqrst",
            "klm",
            "klmnopstuvwxy",
            "abcdekmno",
            "fghijmno",
            "pqrstvw",
            "kmouvw",
            "cdeklmno",
            "st",
            "fij",
            "fhiklmno",
            "abcdefghklmno",
            "lmnuvwxy",
            "klmnovwy",
            "klmnow",
            "mnuvw",
            "fghijklmnpqrst",
            "klmnoxy",
            "fghklmno",
            "b",
            "lopqrst",
            "pqrstuvw",
            "abcdekmouvw",
            "abcdeklmnouw",
            "acduvw",
            "klmnoq",
            "bde",
            "pqrstxy",
            "qrs",
            "bcdeklmnouvw",
            "abcdekmo",
            "abcdefghijkn",
            "kno",
            "abcdefghijklm",
            "abcdefgij",
            "kmopqrst",
            "kmnuvwxy",
            "pqrstuvwx",
            "fghijkmn",
            "acdefghij",
            "fghijmn",
            "qr",
            "l",
            "kmnopqrstuvwxy",
            "abcdefghijl",
            "afghij",
            "auvw",
            "abcdew",
            "klmnopqrstwy",
            "adfghij",
            "abcdeghij",
            "lmopqrstuvwxy",
            "abcdefghijklmn",
            "uvx",
            "km",
            "abcdefijklmno",
            "abcdeij",
            "pq",
            "lnpqrst",
            "bdfghij",
            "abcdefhj",
            "mopqrstuvwxy",
            "fghijklmnopqs",
            "fghijqt",
            "ade",
            "abcdejklmno",
            "fghijklmpqrst",
            "pqrstuvxy",
            "mouvw",
            "mopqrst",
            "lmnopqrst",
            "abcdefhjklmno",
            "acdklmnouvw",
            "bcklmno",
            "nopqrst",
            "qrt",
            "fghijlmnpqrst",
            "pqrstuv",
            "klmnos",
            "pqrstwxy",
            "fgjklmnopqrst",
            "pqrstuw",
            "klmnopqrstuvxy",
            "fhijklmno",
            "abcdekmnuvw",
            "ghjklmno",
            "klmnoprtuvwxy",
            "abcdefghijlmn",
            "klmouvwxy",
            "abcdegjklmno",
            "p",
            "hiklmnopqrst",
            "rstuvwxy",
            "gh",
            "cdfghijklmno",
            "klmnou",
            "iklmno",
            "klmnopqrstvxy",
            "lmpqrstuvwxy",
            "prtuvwxy",
            "klmnovwxy",
            "pqst",
            "klmnortuvwxy",
            "lmnpqrstuvwxy",
            "fghijmpqrst",
            "ghiklmnopqrst",
            "klmpqrst",
            "klmnoprstuvwxy",
            "ghklmno",
            "fghijopqrst",
            "klmnoqruvwxy",
            "abcdeklmouvw",
            "ae",
            "abcfghijklmno",
            "vw",
            "fghijr",
            "kmnpqrstuvwxy",
            "fghijklpqrst",
            "vwxy",
            "uxy",
            "cklmnouvw",
            "abcdefghijkm",
            "uvw",
            "bcd",
            "bcefghij",
            "abcdeiklmno",
            "klmnot",
            "cduvw",
            "fghijmo",
            "eklmnouvw",
            "klmnopqrstuwy",
            "kmnuvw",
            "lmouvwxy",
            "abcdefghijlmo",
            "cefghij",
            "klmnoqs",
            "klmnouwx",
            "kmuvwxy",
            "ceklmnouvw",
            "fghijklmnoqrs",
            "abduvw",
            "abceklmno",
            "ghijklmno",
            "abcdefghijmn",
            "bcdfghij",
            "pqrstvwx",
            "fghijlpqrst",
            "gijklmnopqrst",
            "abcdelnuvw",
            "pqrstvwy",
            "pr",
            "fghijklmnos",
            "c",
            "fghijklmno",
            "abcdefghijkmno",
            "abcuvw",
            "abcdev",
            "abeuvw",
            "pqrstuvx",
            "klo",
            "fghijln",
            "klmnpqrstuvwxy",
            "abcdefghijlno",
            "fghijklmnopst",
            "acd",
            "gipqrst",
            "w",
            "abcdenuvw",
            "deuvw",
            "abcdeklmnouvw",
            "fg",
            "cdeuvw",
            "lmnuvw",
            "abcdenouvw",
            "klmnoqt",
            "hklmnopqrst",
            "klmnouvwy",
            "gijklmno",
            "klmnopqrst",
            "fghijmopqrst",
            "kmno",
            "f",
            "uv",
            "fhij",
            "knuvw",
            "klmnor",
            "klnouvwxy",
            "bcdeuvw",
            "bdklmno",
            "abklmnouvw",
            "abcdehjklmno",
            "bcde",
            "fghijklmo",
            "abcdefghijklmo",
            "ko",
            "abcdeklmuvw",
            "bcdefghijklmno",
            "beuvw",
            "qstuvwxy",
            "aeklmnouvw",
            "h",
            "t",
            "kmuvw",
            "fhi",
            "fjklmnopqrst",
            "hiklmno",
            "fhipqrst",
            "qrtuvwxy",
            "abcdeklnuvw",
            "fghijqrt",
            "pqrstx",
            "bdefghijklmno",
            "gj",
            "abcdelmno",
            "abcdefjklmno",
            "fhjpqrst",
            "klnpqrst",
            "fgijklmnopqrst",
            "abcdelm",
            "fghijrs",
            "klmnouw",
            "fhklmno",
            "abcdeln",
            "fgijpqrst",
            "fghijklmnoqst",
            "abeklmnouvw",
            "klmuvw",
            "i",
            "lmno",
            "fghijklo",
            "fghijkm",
            "g",
            "adklmnouvw",
            "be",
            "abcdefgjklmno",
            "prst",
            "puvwxy",
            "qrstuvwxy",
            "klmnov",
            "klmnopqrstuvwx",
            "acefghijklmno",
            "fghijpqrs",
            "fhj",
            "abklmno",
            "abcd",
            "fiklmno",
            "ijpqrst",
            "fghijnpqrst",
            "lmnouvw",
            "suvwxy",
            "knouvw",
            "duvw",
            "fghijkmno",
            "bdefghij",
            "aceklmno",
            "fgklmno",
            "fghijst",
            "adefghij",
            "fghijklmnopqrs",
            "bcdefghij",
            "abcdefghijko",
            "klmnotuvwxy",
            "klmnopruvwxy",
            "klmnovxy",
            "qtuvwxy",
            "fhjklmnopqrst",
            "klmnoqrst",
            "prt",
            "fghijt",
            "uvwx",
            "abcdekmnouvw",
            "dfghij",
            "abcdehiklmno",
            "klmnopqrstu",
            "adeklmnouvw",
            "abcdefghijmno",
            "klmnopquvwxy",
            "abcdefghijkln",
            "mnpqrst",
            "fghijlmn",
            "lnopqrstuvwxy",
            "pqrt",
            "fghijpt",
            "abcde",
            "ij",
            "bcdklmnouvw",
            "fghijkn",
            "pst",
            "bdklmnouvw",
            "abcdeko",
            "fh",
            "fjpqrst",
            "fghijqr",
            "beklmno",
            "fghijklopqrst",
            "lmuvw",
            "klouvw",
            "fghijpqr",
            "abcdegi",
            "psuvwxy",
            "klnopqrstuvwxy",
            "abcdelmn",
            "fhiklmnopqrst",
            "fghijprst",
            "lnpqrstuvwxy",
            "qruvwxy",
            "aduvw",
            "abcdekmuvw",
            "adklmno",
            "abcdeu",
            "cefghijklmno",
            "lm",
            "hjklmno",
            "abcdefghijklo",
            "klmnoqrs",
            "fghijko",
            "ouvw",
            "pqrstu",
            "mnopqrst",
            "louvw",
            "prsuvwxy",
            "abcdekno",
            "klmnopqruvwxy",
            "kpqrstuvwxy",
            "abcdefgijklmno",
            "klmnors",
            "abcdefghijo",
            "npqrst",
            "knopqrstuvwxy",
            "vx",
            "fghijqrs",
            "klmnopqrstuxy",
            "u",
            "aceuvw",
            "klmnopqrtuvwxy",
            "uvwxy",
            "fipqrst",
            "klmnowxy",
            "befghij",
            "gklmnopqrst",
            "abe",
            "mnopqrstuvwxy",
            "fghijklmnoprs",
            "abcdeo",
            "eklmno",
            "fgi",
            "fghijlmpqrst",
            "qt",
            "abcdefghi",
            "bcduvw",
            "bduvw",
            "klouvwxy",
            "kouvwxy",
            "fghiklmno",
            "abcdemno",
            "pqsuvwxy",
            "ab",
            "klmnopqrstvwxy",
            "pqrstv",
            "abfghijklmno",
            "beklmnouvw",
            "fghijklm",
            "pqrsuvwxy",
            "acdeklmnouvw",
            "vwy",
            "ghjpqrst",
            "fghijl",
            "klmnopqrstvy",
            "abcdelouvw",
            "fhijklmnopqrst",
            "klmpqrstuvwxy",
            "abcdef",
            "bcfghij",
            "fghijkno",
            "kopqrst",
            "rsuvwxy",
            "abcdekmn",
            "aklmno",
            "dfghijklmno",
            "abcdeklouvw",
            "klmnpqrst",
            "abcdeklmnouv",
            "knpqrst",
            "aceklmnouvw",
            "kl",
            "fghijprs",
            "rt",
            "klmnopqrsuvwxy",
            "bfghijklmno",
            "klmnouv",
            "abcdefh",
            "s",
            "klmnouvwx",
            "fghijp",
            "uwxy",
            "fghijq",
            "klmnoqr",
            "vxy",
            "fgpqrst",
            "fghijlmno",
            "defghij",
            "pqtuvwxy",
            "dklmno",
            "klno",
            "klmnowy",
            "abcdemn",
            "abcdeklnouvw",
            "ptuvwxy",
            "abcdehijklmno",
            "acuvw",
            "bcdfghijklmno",
            "abcdemouvw",
            "abcdefgi",
            "opqrstuvwxy",
            "v",
            "lmpqrst",
            "abcdeuvw",
            "abcdekouvw",
            "qrst",
            "klmopqrstuvwxy",
            "prstuvwxy",
            "lmnpqrst",
            "pqrstvy",
            "abcdfghij",
            "klmnop",
            "ijklmnopqrst",
            "klnpqrstuvwxy",
            "klmnopqr",
            "xy",
            "abcdeklo",
            "abd",
            "klmnouvy",
            "klmuvwxy",
            "abcefghijklmno",
            "fghijkopqrst",
            "fghijklmnoprst",
            "ruvwxy",
            "m",
            "e",
            "abcdej",
            "hpqrst",
            "bceuvw",
            "fghijmnpqrst",
            "pqrstwy",
            "klmnopqrstx",
            "qsuvwxy",
            "fghijknpqrst",
            "bcdeklmno",
            "klmopqrst",
            "bdeklmnouvw",
            "klmnopqst",
            "klmnopqrstuvx",
            "acklmno",
            "hjpqrst",
            "abcklmno",
            "rst"
        ];
        assert_eq!(Solution::longest_str_chain(w), 15);
    }
}
