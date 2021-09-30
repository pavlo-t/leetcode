#![allow(dead_code)]
/// Word Ladder II
/// ==============
///
/// A __transformation sequence__ from word `begin_word` to word `end_word` using a dictionary `word_list`
/// is a sequence of words `begin_word -> s1 -> s2 -> ... -> sk` such that:
///
/// - Every adjacent pair of words differs by a single letter.
/// - Every `si` for `1 <= i <= k` is in wordList. Note that `begin_word` does not need to be in `word_list`.
/// - `sk == end_word`
///
/// Given two words, `begin_word` and `end_word`, and a dictionary `word_list`,
/// return _all the __shortest transformation sequences__ from_ `begin_word` _to_ `end_word`,
/// _or an empty list if no such sequence exists.
/// Each sequence should be returned as a list of the words_ `[begin_word, s1, s2, ..., sk]`.
///
/// Constraints:
///
/// - `1 <= begin_word.length <= 5`
/// - `end_word.length == begin_word.length`
/// - `1 <= word_list.length <= 1000`
/// - `word_list[i].length == begin_word.length`
/// - `begin_word`, `end_word`, and `word_list[i]` consist of lowercase English letters.
/// - `begin_word != end_word`
/// - All the words in `word_list` are __unique__.
///
/// https://leetcode.com/explore/challenge/card/july-leetcoding-challenge-2021/611/week-4-july-22nd-july-28th/3825/
struct Solution;
impl Solution {
    fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        use std::collections::{HashMap, HashSet};
        use std::iter::FromIterator;

        fn bfs(
            set1: HashSet<String>,
            set2: HashSet<String>,
            flipped: bool,
            map: &mut HashMap<String, HashSet<String>>,
            dict: &mut HashSet<String>,
        ) {
            if !set1.is_empty() {
                if set1.len() > set2.len() {
                    bfs(set2, set1, !flipped, map, dict);
                } else {
                    for s in set1.iter().chain(set2.iter()) {
                        dict.remove(s);
                    }
                    let mut next: HashSet<String> = HashSet::new();
                    let mut done = false;
                    for word in set1.iter() {
                        for next_word in connected_words(&word.to_string()) {
                            let (key, value) = match flipped {
                                true => (next_word.clone(), word.clone()),
                                _ => (word.clone(), next_word.clone()),
                            };
                            if set2.contains(&next_word) {
                                done = true;
                                map.entry(key).or_default().insert(value);
                            } else if dict.contains(&next_word) {
                                next.insert(next_word);
                                map.entry(key).or_default().insert(value);
                            }
                        }
                    }
                    if !done {
                        bfs(set2, next, !flipped, map, dict);
                    }
                }
            }
        }

        fn dfs(
            start: &str,
            path: &mut Vec<String>,
            all: &mut Vec<Vec<String>>,
            map: &HashMap<String, HashSet<String>>,
            end: &str,
        ) {
            if start == end {
                all.push(path.to_vec());
            } else {
                if let Some(nei) = map.get(start) {
                    for next in nei.iter() {
                        path.push(next.to_string());
                        dfs(next, path, all, map, end);
                        path.pop();
                    }
                }
            }
        }

        fn connected_words(word: &str) -> Vec<String> {
            let mut res = vec![];
            for i in 0..word.len() {
                let mut s: Vec<char> = word.chars().collect();
                for j in 0..26 {
                    let c = (b'a' + j as u8) as char;
                    s[i] = c;
                    let new_word: String = s.iter().collect();
                    res.push(new_word);
                }
            }
            res
        }

        let mut dict: HashSet<String> = HashSet::new();
        for word in word_list {
            dict.insert(word);
        }
        if !dict.contains(&end_word) {
            return vec![];
        }
        let set1: HashSet<String> = HashSet::from_iter(vec![begin_word.clone()]);
        let set2: HashSet<String> = HashSet::from_iter(vec![end_word.clone()]);
        let mut map: HashMap<String, HashSet<String>> = HashMap::new();
        bfs(set1, set2, false, &mut map, &mut dict);
        println!("{:?}", map);
        let mut path = vec![begin_word.clone()];
        let mut res = vec![];
        dfs(&begin_word, &mut path, &mut res, &map, &end_word);
        res
    }

    /// https://rustgym.com/leetcode/126
    fn find_ladders_rustgym(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        use std::collections::{HashMap, HashSet};
        use std::iter::FromIterator;

        fn bfs(
            set1: HashSet<String>,
            set2: HashSet<String>,
            flipped: bool,
            map: &mut HashMap<String, HashSet<String>>,
            dict: &mut HashSet<String>,
        ) {
            if set1.is_empty() {
                return;
            }
            if set1.len() > set2.len() {
                bfs(set2, set1, !flipped, map, dict);
                return;
            }
            for s in set1.iter() {
                dict.remove(s);
            }
            for s in set2.iter() {
                dict.remove(s);
            }
            let mut next: HashSet<String> = HashSet::new();
            let mut done = false;
            for word in set1.iter() {
                for next_word in connected_words(&word.to_string()) {
                    let key = if flipped {
                        next_word.to_string()
                    } else {
                        word.to_string()
                    };
                    let value = if flipped {
                        word.to_string()
                    } else {
                        next_word.to_string()
                    };
                    if set2.contains(&next_word) {
                        done = true;
                        map.entry(key).or_default().insert(value);
                    } else if dict.contains(&next_word) {
                        next.insert(next_word);
                        map.entry(key).or_default().insert(value);
                    }
                }
            }
            if !done {
                bfs(set2, next, !flipped, map, dict);
            }
        }

        fn dfs(
            start: &str,
            path: &mut Vec<String>,
            all: &mut Vec<Vec<String>>,
            map: &HashMap<String, HashSet<String>>,
            end: &str,
        ) {
            if start == end {
                all.push(path.to_vec());
            } else {
                if let Some(nei) = map.get(start) {
                    for next in nei.iter() {
                        path.push(next.to_string());
                        dfs(next, path, all, map, end);
                        path.pop();
                    }
                }
            }
        }

        fn connected_words(word: &str) -> Vec<String> {
            let n = word.len();
            let mut res = vec![];
            for i in 0..n {
                let mut s: Vec<char> = word.chars().collect();
                for j in 0..26 {
                    let c = (b'a' + j as u8) as char;
                    s[i] = c;
                    let new_word: String = s.iter().collect();
                    res.push(new_word);
                }
            }
            res
        }

        let mut dict: HashSet<String> = HashSet::new();
        for word in word_list {
            dict.insert(word);
        }
        if !dict.contains(&end_word) {
            return vec![];
        }
        let set1: HashSet<String> = HashSet::from_iter(vec![begin_word.to_string()]);
        let set2: HashSet<String> = HashSet::from_iter(vec![end_word.to_string()]);
        let mut map: HashMap<String, HashSet<String>> = HashMap::new();
        bfs(set1, set2, false, &mut map, &mut dict);
        let mut path = vec![begin_word.to_string()];
        let mut res = vec![];
        dfs(&begin_word, &mut path, &mut res, &map, &end_word);
        res
    }

    pub fn find_ladders_optimization_use_idx_instead_of_strings(
        begin_word: String,
        end_word: String,
        mut word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        use std::mem::swap;

        if let Some(end_i) = word_list.iter().position(|w| w == &end_word) {
            fn are_neighbors(w1: &str, w2: &str) -> bool {
                let bs1 = w1.as_bytes();
                let bs2 = w2.as_bytes();
                let mut diff = 0;
                for i in 0..bs1.len() {
                    if bs1[i] != bs2[i] {
                        if diff != 0 {
                            return false;
                        }
                        diff += 1;
                    }
                }
                diff == 1
            }

            word_list.push(begin_word.clone());
            let mut neighbors = vec![vec![]; word_list.len()];

            for i in 0..neighbors.len() - 1 {
                for j in i..neighbors.len() {
                    if are_neighbors(&word_list[i], &word_list[j]) {
                        neighbors[i].push(j);
                        neighbors[j].push(i);
                    }
                }
            }

            let mut q1 = vec![vec![neighbors.len() - 1]];
            let mut q2 = vec![];
            let mut found = false;
            while !q1.is_empty() {
                while let Some(ps) = q1.pop() {
                    let ns = &neighbors[ps[ps.len() - 1]];
                    for &i in ns {
                        if i == end_i {
                            found = true;
                        }
                        if !ps.contains(&i) {
                            let mut n = ps.clone();
                            n.push(i);
                            q2.push(n);
                        }
                    }
                }
                if found {
                    return q2
                        .into_iter()
                        .filter(|is| is.last() == Some(&end_i))
                        .map(|is| is.into_iter().map(|i| word_list[i].clone()).collect())
                        .collect();
                }
                swap(&mut q1, &mut q2);
            }
            vec![]
        } else {
            vec![]
        }
    }

    pub fn find_ladders_my_brute_force(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        use std::iter;
        use std::mem::swap;

        fn use_word(w: &str, ts: &[String]) -> Option<Vec<String>> {
            let w = w.to_string();
            let wbs = w.as_bytes();
            let lbs = ts[ts.len() - 1].as_bytes();
            let mut diff = 0;
            for i in 0..wbs.len() {
                if wbs[i] != lbs[i] {
                    diff += 1;
                }
            }
            if diff != 1 || (ts.len() > 1 && ts.contains(&w)) {
                None
            } else {
                Some(ts.iter().map(|s| s.clone()).chain(iter::once(w)).collect())
            }
        }

        let mut q1 = vec![vec![begin_word]];
        let mut q2 = vec![];
        let mut found = false;
        while !q1.is_empty() {
            while let Some(pr) = q1.pop() {
                for w in word_list.iter() {
                    if let Some(nr) = use_word(w, &pr) {
                        q2.push(nr);
                        if w == &end_word {
                            found = true;
                        }
                    }
                }
            }
            if found {
                return q2
                    .into_iter()
                    .filter(|v| v.last() == Some(&end_word))
                    .collect();
            }
            swap(&mut q1, &mut q2);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($s:tt),*) => {vec![$($s.to_string()),*]};}

    #[test]
    fn b_hit_e_cog_wl_hot_dot_dog_lot_log_cog_produces_2_vecs_of_5() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vs!["hot", "dot", "dog", "lot", "log", "cog"];
        let e = vec![
            vs!["hit", "hot", "dot", "dog", "cog"],
            vs!["hit", "hot", "lot", "log", "cog"],
        ];
        let mut result = Solution::find_ladders(begin_word, end_word, word_list);
        result.sort();
        assert_eq!(result, e);
        // Explanation: There are 2 shortest transformation sequences:
        // "hit" -> "hot" -> "dot" -> "dog" -> "cog"
        // "hit" -> "hot" -> "lot" -> "log" -> "cog"
    }
    #[test]
    fn b_hit_e_cog_wl_hot_dot_dog_lot_log_produces_empty_vec() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = vs!["hot", "dot", "dog", "lot", "log"];
        let e: Vec<Vec<String>> = vec![];
        assert_eq!(Solution::find_ladders(begin_word, end_word, word_list), e);
        // Explanation: The endWord "cog" is not in wordList, therefore there is no valid transformation sequence.
    }

    #[test]
    fn b_abc_e_def_wl_abf_aec_dbc_aef_dbf_dec_def_produces_6_vecs_of_4() {
        let begin_word = "abc".to_string();
        let end_word = "def".to_string();
        let word_list = vs!["abf", "aec", "dbc", "aef", "dbf", "dec", "def"];
        let e = vec![
            vs!["abc", "abf", "aef", "def"],
            vs!["abc", "abf", "dbf", "def"],
            vs!["abc", "aec", "aef", "def"],
            vs!["abc", "aec", "dec", "def"],
            vs!["abc", "dbc", "dbf", "def"],
            vs!["abc", "dbc", "dec", "def"],
        ];
        let mut result = Solution::find_ladders(begin_word, end_word, word_list);
        result.sort();
        assert_eq!(result, e);
    }

    #[test]
    fn b_cet_e_ism_wl_599_words_test_22() {
        let begin_word = "cet".to_string();
        let end_word = "ism".to_string();
        let word_list = vs![
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
            "all", "pad", "hah", "hie", "aim", "ike", "jed", "ego", "mac", "baa", "min", "com",
            "ill", "was", "cab", "ago", "ina", "big", "ilk", "gal", "tap", "duh", "ola", "ran",
            "lab", "top", "gob", "hot", "ora", "tia", "kip", "han", "met", "hut", "she", "sac",
            "fed", "goo", "tee", "ell", "not", "act", "gil", "rut", "ala", "ape", "rig", "cid",
            "god", "duo", "lin", "aid", "gel", "awl", "lag", "elf", "liz", "ref", "aha", "fib",
            "oho", "tho", "her", "nor", "ace", "adz", "fun", "ned", "coo", "win", "tao", "coy",
            "van", "man", "pit", "guy", "foe", "hid", "mai", "sup", "jay", "hob", "mow", "jot",
            "are", "pol", "arc", "lax", "aft", "alb", "len", "air", "pug", "pox", "vow", "got",
            "meg", "zoe", "amp", "ale", "bud", "gee", "pin", "dun", "pat", "ten", "mob"
        ];
        let e = vec![
            vs!["cet", "cat", "can", "ian", "inn", "ins", "its", "ito", "ibo", "ibm", "ism"],
            vs!["cet", "cot", "con", "ion", "inn", "ins", "its", "ito", "ibo", "ibm", "ism"],
            vs!["cet", "get", "gee", "gte", "ate", "ats", "its", "ito", "ibo", "ibm", "ism"],
        ];

        let mut result = Solution::find_ladders(begin_word, end_word, word_list);
        result.sort();
        assert_eq!(result, e);
    }
}
