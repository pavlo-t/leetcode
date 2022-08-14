#![allow(dead_code)]
//! \#126. Word Ladder II
//! =====================
//!
//! <https://leetcode.com/problems/word-ladder-ii>
//!
//! A __transformation sequence__ from word `begin_word` to word `end_word` using a dictionary `word_list`
//! is a sequence of words `begin_word -> s1 -> s2 -> ... -> sk` such that:
//!
//! - Every adjacent pair of words differs by a single letter.
//! - Every `si` for `1 <= i <= k` is in `word_list`. Note that `begin_word` does not need to be in `word_list`.
//! - `sk == end_word`
//!
//! Given two words, `begin_word` and `end_word`, and a dictionary `word_list`,
//! return _all the __shortest transformation sequences__ from `begin_word` to `end_word`,
//! or an empty list if no such sequence exists.
//! Each sequence should be returned as a list of the words `[begin_word, s1, s2, ..., sk]`_.
//!
//! ##### Examples
//!
//! ###### Example 1:
//!
//! ```
//! # use c2022_08::c2022_08_14::*;
//! # macro_rules! vs { ($($x:expr),*) => { vec![$($x.to_string()),*] }; }
//! let begin_word = "hit".to_string();
//! let end_word = "cog".to_string();
//! let word_list = vs!["hot","dot","dog","lot","log","cog"];
//!
//! assert_eq!(Solution::find_ladders(begin_word, end_word, word_list), vec![
//!     vs!["hit","hot","dot","dog","cog"],
//!     vs!["hit","hot","lot","log","cog"]
//! ]);
//! ```
//!
//! __Explanation:__ There are `2` shortest transformation sequences:
//!
//! - `"hit" -> "hot" -> "dot" -> "dog" -> "cog"`
//! - `"hit" -> "hot" -> "lot" -> "log" -> "cog"`
//!
//! ###### Example 2:
//!
//! ```
//! # use c2022_08::c2022_08_14::*;
//! # macro_rules! vs { ($($x:expr),*) => { vec![$($x.to_string()),*] }; }
//! let begin_word = "hit".to_string();
//! let end_word = "cog".to_string();
//! let word_list = vs!["hot","dot","dog","lot","log"];
//! let expected: Vec<Vec<String>> = vec![];
//!
//! assert_eq!(Solution::find_ladders(begin_word, end_word, word_list), expected);
//! ```
//!
//! __Explanation:__ The `end_word` `"cog"` is not in `word_list`, therefore there is no valid transformation sequence.
//!
//! ##### Constraints:
//!
//! - `1 <= begin_word.length <= 5`
//! - `end_word.length == begin_word.length`
//! - `1 <= word_list.length <= 500`
//! - `word_list[i].length == begin_word.length`
//! - `begin_word`, `end_word`, and `word_list[i]` consist of lowercase English letters.
//! - `begin_word != end_word`
//! - All the words in `word_list` are __unique__.

pub struct Solution;
impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        mut word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        if let Some(end_idx) = word_list.iter().position(|w| w == &end_word) {
            fn bts(
                curr: usize,
                goal: usize,
                transformations: &[Vec<usize>],
                path: &mut Vec<usize>,
                result: &mut Vec<Vec<usize>>,
            ) {
                if curr == goal {
                    result.push(path.clone());
                } else {
                    for &next in &transformations[curr] {
                        path.push(next);
                        bts(next, goal, transformations, path, result);
                        path.pop();
                    }
                }
            }

            let begin_idx = word_list
                .iter()
                .position(|w| w == &begin_word)
                .unwrap_or_else(|| {
                    word_list.push(begin_word.clone());
                    word_list.len() - 1
                });

            let n = word_list.len();

            let transformations: Vec<Vec<usize>> = {
                let mut transformations = vec![vec![]; n];

                let init_seen = |idx: usize| {
                    let mut seen = vec![false; n];
                    seen[idx] = true;
                    seen
                };

                let mut todo = vec![begin_idx];
                let mut seen = init_seen(begin_idx);

                let mut other_todo = vec![end_idx];
                let mut other_seen = init_seen(end_idx);

                let mut from_end = false;
                let mut done = false;

                while !todo.is_empty() {
                    if !other_todo.is_empty() && todo.len() > other_todo.len() {
                        std::mem::swap(&mut todo, &mut other_todo);
                        std::mem::swap(&mut seen, &mut other_seen);
                        from_end = !from_end;
                    }

                    let mut next_todo = vec![];
                    let mut next_seen = seen.clone();
                    while let Some(i) = todo.pop() {
                        for j in (0..n).filter(|&j| !seen[j]) {
                            if Self::is_valid_transformation(&word_list[i], &word_list[j]) {
                                if from_end {
                                    transformations[j].push(i);
                                } else {
                                    transformations[i].push(j);
                                }

                                if !next_seen[j] {
                                    next_todo.push(j);
                                }
                                next_seen[j] = true;

                                if other_seen[j] {
                                    done = true;
                                }
                            }
                        }
                    }

                    if !done {
                        todo = other_todo;
                        seen = other_seen;
                        other_todo = next_todo;
                        other_seen = next_seen;

                        from_end = !from_end;
                    }
                }
                transformations
            };

            let mut path = vec![begin_idx];
            let mut sequences = vec![];

            bts(
                begin_idx,
                end_idx,
                &transformations,
                &mut path,
                &mut sequences,
            );

            sequences
                .into_iter()
                .map(|sequence| sequence.into_iter().map(|i| word_list[i].clone()).collect())
                .collect()
        } else {
            vec![]
        }
    }

    fn is_valid_transformation(a: &str, b: &str) -> bool {
        let mut diffs = 0;
        for (ca, cb) in a.chars().zip(b.chars()) {
            if ca != cb {
                diffs += 1;
                if diffs > 1 {
                    return false;
                }
            }
        }
        diffs == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    macro_rules! vs { ($($x:expr),*) => { vec![$($x.to_string()),*] }; }

    const EMPTY: Vec<Vec<String>> = vec![];

    #[test]
    fn hit_cog_hot_dot_dog_lot_log_cog() {
        let bw = "hit".to_string();
        let ew = "cog".to_string();
        let wl = vs!["hot", "dot", "dog", "lot", "log", "cog"];

        assert_eq!(
            Solution::find_ladders(bw, ew, wl),
            vec![
                vs!["hit", "hot", "dot", "dog", "cog"],
                vs!["hit", "hot", "lot", "log", "cog"]
            ]
        );
    }

    #[test]
    fn a_b_a_b_c() {
        assert_eq!(
            Solution::find_ladders("a".into(), "c".into(), vs!["a", "b", "c"]),
            vec![vs!["a", "c"]]
        );
    }

    #[test]
    fn a_a_a_b_c() {
        assert_eq!(
            Solution::find_ladders("a".into(), "a".into(), vs!["a", "b", "c"]),
            vec![vs!["a"]]
        );
    }

    #[test]
    fn hot_dog_hot_dog() {
        let bw = "hot".to_string();
        let ew = "dog".to_string();
        let wl = vs!["hot", "dog"];

        assert_eq!(Solution::find_ladders(bw, ew, wl), EMPTY);
    }

    #[test]
    fn hit_cog_hot_dot_dog_lot_log() {
        let bw = "hit".to_string();
        let ew = "cog".to_string();
        let wl = vs!["hot", "dot", "dog", "lot", "log"];

        assert_eq!(Solution::find_ladders(bw, ew, wl), EMPTY);
    }

    fn add_1(word: &String) -> Option<String> {
        let mut result = word.clone();
        unsafe {
            let vec = result.as_mut_vec();
            let n = vec.len();
            if let Some(last_non_z_idx) = vec.iter().rposition(|&b| b != b'z') {
                vec[last_non_z_idx] += 1;
                for i in last_non_z_idx + 1..n {
                    vec[i] = b'a';
                }
            } else {
                return None;
            }
        }
        Some(result)
    }

    //#[ignore]
    #[test]
    fn aa_zz_aa_to_zz() {
        let bw = "aa".to_string();
        let ew = "zz".to_string();
        let wl = iter::successors(Some("aa".to_string()), add_1).collect();

        assert_eq!(
            Solution::find_ladders(bw, ew, wl),
            vec![vs!["aa", "az", "zz"], vs!["aa", "za", "zz"]]
        );
    }

    //#[ignore]
    #[test]
    fn aaaaa_zzzzz_aaaaa_to_zzzzz_take_500() {
        let bw = "aaaaa".to_string();
        let ew = "zzzzz".to_string();
        let wl = iter::successors(Some("aaaaa".to_string()), add_1)
            .take(500)
            .collect();

        assert_eq!(Solution::find_ladders(bw, ew, wl), EMPTY);
    }

    //#[ignore]
    #[test]
    fn test_22() {
        let bw = "cet".to_string();
        let ew = "ism".to_string();
        let wl = vs![
            "kid", "tag", "pup", "ail", "tun", "woo", "erg", "luz", "brr", "gay", "sip", "kay",
            "per", "val", "mes", "ohs", "now", "boa", "cet", "pal", "bar", "die", "war", "hay",
            "eco", "pub", "lob", "rue", "fry", "lit", "rex", "jan", "cot", "bid", "ali", "pay",
            "col", "gum", "ger", "row", "won", "dan", "rum", "fad", "tut", "sag", "yip", "sui",
            "ark", "has", "zip", "fez", "own", "ump", "dis", "ads", "max", "jaw", "out", "btu",
            "ana", "gap", "cry", "led", "abe", "box", "ore", "pig", "fie", "toy", "fat", "cal",
            "lie", "noh", "sew", "ono", "tam", "flu", "mgm", "ply", "awe", "pry", "tit", "tie",
            "yet", "too", "tax", "jim", "san", "pan", "map", "ski", "ova", "wed", "non", "wac",
            "nut", "why", "bye", "lye", "oct", "old", "fin", "feb", "chi", "sap", "owl", "log",
            "tod", "dot", "bow", "fob", "for", "joe", "ivy", "fan", "age", "fax", "hip", "jib",
            "mel", "hus", "sob", "ifs", "tab", "ara", "dab", "jag", "jar", "arm", "lot", "tom",
            "sax", "tex", "yum", "pei", "wen", "wry", "ire", "irk", "far", "mew", "wit", "doe",
            "gas", "rte", "ian", "pot", "ask", "wag", "hag", "amy", "nag", "ron", "soy", "gin",
            "don", "tug", "fay", "vic", "boo", "nam", "ave", "buy", "sop", "but", "orb", "fen",
            "paw", "his", "sub", "bob", "yea", "oft", "inn", "rod", "yam", "pew", "web", "hod",
            "hun", "gyp", "wei", "wis", "rob", "gad", "pie", "mon", "dog", "bib", "rub", "ere",
            "dig", "era", "cat", "fox", "bee", "mod", "day", "apr", "vie", "nev", "jam", "pam",
            "new", "aye", "ani", "and", "ibm", "yap", "can", "pyx", "tar", "kin", "fog", "hum",
            "pip", "cup", "dye", "lyx", "jog", "nun", "par", "wan", "fey", "bus", "oak", "bad",
            "ats", "set", "qom", "vat", "eat", "pus", "rev", "axe", "ion", "six", "ila", "lao",
            "mom", "mas", "pro", "few", "opt", "poe", "art", "ash", "oar", "cap", "lop", "may",
            "shy", "rid", "bat", "sum", "rim", "fee", "bmw", "sky", "maj", "hue", "thy", "ava",
            "rap", "den", "fla", "auk", "cox", "ibo", "hey", "saw", "vim", "sec", "ltd", "you",
            "its", "tat", "dew", "eva", "tog", "ram", "let", "see", "zit", "maw", "nix", "ate",
            "gig", "rep", "owe", "ind", "hog", "eve", "sam", "zoo", "any", "dow", "cod", "bed",
            "vet", "ham", "sis", "hex", "via", "fir", "nod", "mao", "aug", "mum", "hoe", "bah",
            "hal", "keg", "hew", "zed", "tow", "gog", "ass", "dem", "who", "bet", "gos", "son",
            "ear", "spy", "kit", "boy", "due", "sen", "oaf", "mix", "hep", "fur", "ada", "bin",
            "nil", "mia", "ewe", "hit", "fix", "sad", "rib", "eye", "hop", "haw", "wax", "mid",
            "tad", "ken", "wad", "rye", "pap", "bog", "gut", "ito", "woe", "our", "ado", "sin",
            "mad", "ray", "hon", "roy", "dip", "hen", "iva", "lug", "asp", "hui", "yak", "bay",
            "poi", "yep", "bun", "try", "lad", "elm", "nat", "wyo", "gym", "dug", "toe", "dee",
            "wig", "sly", "rip", "geo", "cog", "pas", "zen", "odd", "nan", "lay", "pod", "fit",
            "hem", "joy", "bum", "rio", "yon", "dec", "leg", "put", "sue", "dim", "pet", "yaw",
            "nub", "bit", "bur", "sid", "sun", "oil", "red", "doc", "moe", "caw", "eel", "dix",
            "cub", "end", "gem", "off", "yew", "hug", "pop", "tub", "sgt", "lid", "pun", "ton",
            "sol", "din", "yup", "jab", "pea", "bug", "gag", "mil", "jig", "hub", "low", "did",
            "tin", "get", "gte", "sox", "lei", "mig", "fig", "lon", "use", "ban", "flo", "nov",
            "jut", "bag", "mir", "sty", "lap", "two", "ins", "con", "ant", "net", "tux", "ode",
            "stu", "mug", "cad", "nap", "gun", "fop", "tot", "sow", "sal", "sic", "ted", "wot",
            "del", "imp", "cob", "way", "ann", "tan", "mci", "job", "wet", "ism", "err", "him",
            "all", "pad", "hah", "hie", "aim"
        ];

        assert_eq!(
            Solution::find_ladders(bw, ew, wl),
            vec![
                vs!["cet", "cot", "con", "ion", "inn", "ins", "its", "ito", "ibo", "ibm", "ism"],
                vs!["cet", "cat", "can", "ian", "inn", "ins", "its", "ito", "ibo", "ibm", "ism"]
            ]
        );
    }

    //#[ignore]
    #[test]
    fn test_33() {
        let bw = "aaaaa".to_string();
        let ew = "ggggg".to_string();
        let wl = vs![
            "aaaaa", "caaaa", "cbaaa", "daaaa", "dbaaa", "eaaaa", "ebaaa", "faaaa", "fbaaa",
            "gaaaa", "gbaaa", "haaaa", "hbaaa", "iaaaa", "ibaaa", "jaaaa", "jbaaa", "kaaaa",
            "kbaaa", "laaaa", "lbaaa", "maaaa", "mbaaa", "naaaa", "nbaaa", "oaaaa", "obaaa",
            "paaaa", "pbaaa", "bbaaa", "bbcaa", "bbcba", "bbdaa", "bbdba", "bbeaa", "bbeba",
            "bbfaa", "bbfba", "bbgaa", "bbgba", "bbhaa", "bbhba", "bbiaa", "bbiba", "bbjaa",
            "bbjba", "bbkaa", "bbkba", "bblaa", "bblba", "bbmaa", "bbmba", "bbnaa", "bbnba",
            "bboaa", "bboba", "bbpaa", "bbpba", "bbbba", "abbba", "acbba", "dbbba", "dcbba",
            "ebbba", "ecbba", "fbbba", "fcbba", "gbbba", "gcbba", "hbbba", "hcbba", "ibbba",
            "icbba", "jbbba", "jcbba", "kbbba", "kcbba", "lbbba", "lcbba", "mbbba", "mcbba",
            "nbbba", "ncbba", "obbba", "ocbba", "pbbba", "pcbba", "ccbba", "ccaba", "ccaca",
            "ccdba", "ccdca", "cceba", "cceca", "ccfba", "ccfca", "ccgba", "ccgca", "cchba",
            "cchca", "cciba", "ccica", "ccjba", "ccjca", "cckba", "cckca", "cclba", "cclca",
            "ccmba", "ccmca", "ccnba", "ccnca", "ccoba", "ccoca", "ccpba", "ccpca", "cccca",
            "accca", "adcca", "bccca", "bdcca", "eccca", "edcca", "fccca", "fdcca", "gccca",
            "gdcca", "hccca", "hdcca", "iccca", "idcca", "jccca", "jdcca", "kccca", "kdcca",
            "lccca", "ldcca", "mccca", "mdcca", "nccca", "ndcca", "occca", "odcca", "pccca",
            "pdcca", "ddcca", "ddaca", "ddada", "ddbca", "ddbda", "ddeca", "ddeda", "ddfca",
            "ddfda", "ddgca", "ddgda", "ddhca", "ddhda", "ddica", "ddida", "ddjca", "ddjda",
            "ddkca", "ddkda", "ddlca", "ddlda", "ddmca", "ddmda", "ddnca", "ddnda", "ddoca",
            "ddoda", "ddpca", "ddpda", "dddda", "addda", "aedda", "bddda", "bedda", "cddda",
            "cedda", "fddda", "fedda", "gddda", "gedda", "hddda", "hedda", "iddda", "iedda",
            "jddda", "jedda", "kddda", "kedda", "lddda", "ledda", "mddda", "medda", "nddda",
            "nedda", "oddda", "oedda", "pddda", "pedda", "eedda", "eeada", "eeaea", "eebda",
            "eebea", "eecda", "eecea", "eefda", "eefea", "eegda", "eegea", "eehda", "eehea",
            "eeida", "eeiea", "eejda", "eejea", "eekda", "eekea", "eelda", "eelea", "eemda",
            "eemea", "eenda", "eenea", "eeoda", "eeoea", "eepda", "eepea", "eeeea", "ggggg",
            "agggg", "ahggg", "bgggg", "bhggg", "cgggg", "chggg", "dgggg", "dhggg", "egggg",
            "ehggg", "fgggg", "fhggg", "igggg", "ihggg", "jgggg", "jhggg", "kgggg", "khggg",
            "lgggg", "lhggg", "mgggg", "mhggg", "ngggg", "nhggg", "ogggg", "ohggg", "pgggg",
            "phggg", "hhggg", "hhagg", "hhahg", "hhbgg", "hhbhg", "hhcgg", "hhchg", "hhdgg",
            "hhdhg", "hhegg", "hhehg", "hhfgg", "hhfhg", "hhigg", "hhihg", "hhjgg", "hhjhg",
            "hhkgg", "hhkhg", "hhlgg", "hhlhg", "hhmgg", "hhmhg", "hhngg", "hhnhg", "hhogg",
            "hhohg", "hhpgg", "hhphg", "hhhhg", "ahhhg", "aihhg", "bhhhg", "bihhg", "chhhg",
            "cihhg", "dhhhg", "dihhg", "ehhhg", "eihhg", "fhhhg", "fihhg", "ghhhg", "gihhg",
            "jhhhg", "jihhg", "khhhg", "kihhg", "lhhhg", "lihhg", "mhhhg", "mihhg", "nhhhg",
            "nihhg", "ohhhg", "oihhg", "phhhg", "pihhg", "iihhg", "iiahg", "iiaig", "iibhg",
            "iibig", "iichg", "iicig", "iidhg", "iidig", "iiehg", "iieig", "iifhg", "iifig",
            "iighg", "iigig", "iijhg", "iijig", "iikhg", "iikig", "iilhg", "iilig", "iimhg",
            "iimig", "iinhg", "iinig", "iiohg", "iioig", "iiphg", "iipig", "iiiig", "aiiig",
            "ajiig", "biiig", "bjiig", "ciiig", "cjiig", "diiig", "djiig", "eiiig", "ejiig",
            "fiiig", "fjiig", "giiig", "gjiig", "hiiig", "hjiig", "kiiig", "kjiig", "liiig",
            "ljiig", "miiig", "mjiig", "niiig", "njiig", "oiiig", "ojiig", "piiig", "pjiig",
            "jjiig", "jjaig", "jjajg", "jjbig", "jjbjg", "jjcig", "jjcjg", "jjdig", "jjdjg",
            "jjeig", "jjejg", "jjfig", "jjfjg", "jjgig", "jjgjg", "jjhig", "jjhjg", "jjkig",
            "jjkjg", "jjlig", "jjljg", "jjmig", "jjmjg", "jjnig", "jjnjg", "jjoig", "jjojg",
            "jjpig", "jjpjg", "jjjjg", "ajjjg", "akjjg", "bjjjg", "bkjjg", "cjjjg", "ckjjg",
            "djjjg", "dkjjg", "ejjjg", "ekjjg", "fjjjg", "fkjjg", "gjjjg", "gkjjg", "hjjjg",
            "hkjjg", "ijjjg", "ikjjg", "ljjjg", "lkjjg", "mjjjg", "mkjjg", "njjjg", "nkjjg",
            "ojjjg", "okjjg", "pjjjg", "pkjjg", "kkjjg", "kkajg", "kkakg", "kkbjg", "kkbkg",
            "kkcjg", "kkckg", "kkdjg", "kkdkg", "kkejg", "kkekg", "kkfjg", "kkfkg", "kkgjg",
            "kkgkg", "kkhjg", "kkhkg", "kkijg", "kkikg", "kkljg", "kklkg", "kkmjg", "kkmkg",
            "kknjg", "kknkg", "kkojg", "kkokg", "kkpjg", "kkpkg", "kkkkg", "ggggx", "gggxx",
            "ggxxx", "gxxxx", "xxxxx", "xxxxy", "xxxyy", "xxyyy", "xyyyy", "yyyyy", "yyyyw",
            "yyyww", "yywww", "ywwww", "wwwww", "wwvww", "wvvww", "vvvww", "vvvwz", "avvwz",
            "aavwz", "aaawz", "aaaaz"
        ];

        assert_eq!(
            Solution::find_ladders(bw, ew, wl),
            vec![vs![
                "aaaaa", "aaaaz", "aaawz", "aavwz", "avvwz", "vvvwz", "vvvww", "wvvww", "wwvww",
                "wwwww", "ywwww", "yywww", "yyyww", "yyyyw", "yyyyy", "xyyyy", "xxyyy", "xxxyy",
                "xxxxy", "xxxxx", "gxxxx", "ggxxx", "gggxx", "ggggx", "ggggg"
            ]]
        );
    }
}
