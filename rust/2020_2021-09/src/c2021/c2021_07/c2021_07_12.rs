#![allow(dead_code)]
/// Isomorphic Strings
/// ==================
///
/// Given two strings `s` and `t`, _determine if they are isomorphic_.
///
/// Two strings `s` and `t` are isomorphic if the characters in `s` can be replaced to get `t`.
///
/// All occurrences of a character must be replaced with another character while preserving the order of characters.
/// No two characters may map to the same character, but a character may map to itself.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 50_000`
/// - `t.length == s.length`
/// - `s` and `t` consist of any valid ascii character.
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/609/week-2-july-8th-july-14th/3811/
struct Solution;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let sbs = s.as_bytes();
        let tbs = t.as_bytes();

        let mut st: [usize; 257] = [256; 257];
        let mut ts: [usize; 257] = [256; 257];

        for i in 0..sbs.len() {
            let (sb, tb) = (sbs[i], tbs[i]);
            let (si, ti) = (sb as usize, tb as usize);
            if st[si] == 256 && ts[ti] == 256 {
                st[si] = ti;
                ts[ti] = si;
            } else if st[si] != ti || ts[ti] != si {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s_egg_t_add_is_isomorphic() {
        let s = "egg".to_string();
        let t = "add".to_string();
        assert!(Solution::is_isomorphic(s, t));
    }
    #[test]
    fn s_foo_t_bar_is_not_isomorphic() {
        let s = "foo".to_string();
        let t = "bar".to_string();
        assert!(!Solution::is_isomorphic(s, t));
    }
    #[test]
    fn s_paper_t_title_is_isomorphic() {
        let s = "paper".to_string();
        let t = "title".to_string();
        assert!(Solution::is_isomorphic(s, t));
    }
}
