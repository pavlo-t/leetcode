#![allow(dead_code)]
/// 249. Group Shifted Strings
/// ==========================
///
/// We can shift a string by shifting each of its letters to its successive letter.
/// For example, `"abc"` can be shifted to be `"bcd"`.
///
/// We can keep shifting the string to form a sequence.
/// For example, we can keep shifting `"abc"` to form the sequence: `"abc" -> "bcd" -> ... -> "xyz"`.
///
/// Given an array of strings `strings`, group all `strings[i]` that belong to the same shifting sequence.
/// You may return the answer in __any order__.
///
/// __Constraints:__
///
/// - `1 <= strings.length <= 200`
/// - `1 <= strings[i].length <= 50`
/// - `strings[i]` consists of lowercase English letters.
///
/// https://leetcode.com/problems/group-shifted-strings/
struct Solution;
impl Solution {
    pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        fn group_key(s: &str) -> String {
            let diff = s.bytes().nth(0).map(|b| b - b'a').unwrap();
            //s.bytes()
            //    .map(|b| b - diff)
            //    .map(|b| if b < b'a' { b + 26 } else { b })
            //    .map(|b| b as char)
            //    .collect()
            s.bytes()
                .map(|b| match b - diff {
                    b @ b'a'..=b'z' => b,
                    b => b + 26,
                } as char)
                .collect()
        }

        let mut groups: HashMap<String, Vec<String>> = HashMap::new();
        for s in strings {
            let k = group_key(&s);
            groups.entry(k).or_default().push(s);
        }
        groups.into_iter().map(|(_, group)| group).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;

    macro_rules! vs  { ($($x:expr),*) => { vec![$($x.to_string()),*] }; }
    macro_rules! vvs { ($($x:tt),*)   => { vec![$(vs!$x),*] }; }

    #[test]
    fn abc_bcd_acef_xyz_az_ba_a_z() {
        let s = vs!["abc", "bcd", "acef", "xyz", "az", "ba", "a", "z"];
        let e = vvs![["acef"], ["a", "z"], ["abc", "bcd", "xyz"], ["az", "ba"]]
            .into_iter()
            .collect::<HashSet<_>>();
        let r = Solution::group_strings(s)
            .into_iter()
            .collect::<HashSet<_>>();
        assert_eq!(r, e);
    }
    #[test]
    fn a() {
        let s = vs!["a"];
        let e = vvs![["a"]];
        assert_eq!(Solution::group_strings(s), e);
    }
}
