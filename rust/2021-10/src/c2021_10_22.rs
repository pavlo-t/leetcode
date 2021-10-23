#![allow(dead_code)]
/// 451. Sort Characters By Frequency
/// =================================
///
/// Given a string `s`, sort it in __decreasing order__ based on the __frequency__ of the characters.
/// The __frequency__ of a character is the number of times it appears in the string.
///
/// Return _the sorted string_.
/// If there are multiple answers, return _any of them_.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 50_000`
/// - `s` consists of uppercase and lowercase English letters and digits.
///
/// https://leetcode.com/problems/sort-characters-by-frequency/
struct Solution;
impl Solution {
    // 18:13-18:36
    pub fn frequency_sort(mut s: String) -> String {
        use std::collections::HashMap;
        let mut counts = HashMap::new();
        for b in s.as_bytes() {
            *counts.entry(b).or_insert(0) += 1;
        }
        let mut counts = counts.iter().map(|(&&k, &v)| (v, k)).collect::<Vec<_>>();
        counts.sort_unstable_by(|a, b| b.cmp(a));
        unsafe {
            let v = s.as_mut_vec();
            let mut i = 0;
            for (c, b) in counts {
                for _ in 0..c {
                    v[i] = b;
                    i += 1;
                }
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! check {
        ($s:expr, $e:expr) => {
            let r = Solution::frequency_sort($s);
            let e = $e;
            #[rustfmt::skip]
            assert!(e.iter().any(|e| e == &r), "\n  Expected any of: {:?}\n  Got: \"{}\"\n", e, r);
        };
    }

    #[test]
    fn tree() {
        check!("tree".to_string(), ["eert", "eetr"]);
        // Explanation: 'e' appears twice while 'r' and 't' both appear once.
        // So 'e' must appear before both 'r' and 't'. Therefore "eetr" is also a valid answer.
    }
    #[test]
    fn cccaaa() {
        check!("cccaaa".to_string(), ["aaaccc", "cccaaa"]);
        // Explanation: Both 'c' and 'a' appear three times, so both "cccaaa" and "aaaccc" are valid answers.
        // Note that "cacaca" is incorrect, as the same characters must be together.
    }
    #[allow(non_snake_case)]
    #[test]
    fn Aabb() {
        check!("Aabb".to_string(), ["bbAa", "bbaA"]);
        // Explanation: "bbaA" is also a valid answer, but "Aabb" is incorrect.
        // Note that 'A' and 'a' are treated as two different characters.
    }

    #[allow(non_snake_case)]
    #[test]
    fn a_to_Z_repeat_1000() {
        #[rustfmt::skip]
        let mut s = (b'a'..=b'z').chain(b'A'..=b'Z').map(|b| b as char).collect::<String>();
        s = s.repeat(1000);
        #[rustfmt::skip]
        let e = (b'a'..=b'z').rev().chain((b'A'..=b'Z').rev()).map(|b| b as char)
            .flat_map(|c| vec![c; 1000]).collect::<String>();
        check!(s, [e]);
    }
}
