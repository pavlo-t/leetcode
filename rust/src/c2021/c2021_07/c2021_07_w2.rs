#![allow(dead_code)]
/// Reverse Words in a String II
/// ============================
///
/// Given a character array `s`, reverse the order of the __words__.
///
/// A __word__ is defined as a sequence of non-space characters.
/// The __words__ in `s` will be separated by a single space.
///
/// Your code must solve the problem __in-place__, i.e. without allocating extra space.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 100_000`
/// - `s[i]` is an English letter (uppercase or lowercase), digit, or space `' '`.
/// - There is __at least one__ word in `s`.
/// - `s` does not contain leading or trailing spaces.
/// - All the words in `s` are guaranteed to be separated by a single space.
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/609/week-2-july-8th-july-14th/3806/
struct Solution;
impl Solution {
    pub fn reverse_words(s: &mut Vec<char>) {
        s.reverse();

        let mut ws = 0;
        while ws < s.len() {
            let mut l = ws;
            let mut r = ws + 1;
            while r < s.len() && s[r] != ' ' {
                r += 1;
            }
            ws = r + 1;
            r -= 1;
            while l < r {
                s.swap(l, r);
                l += 1;
                r -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ab_c_def_becomes_def_c_ab() {
        let mut s = vec!['a', 'b', ' ', 'c', ' ', 'd', 'e', 'f'];
        Solution::reverse_words(&mut s);
        assert_eq!(s, ['d', 'e', 'f', ' ', 'c', ' ', 'a', 'b']);
    }
    #[test]
    fn the_sky_is_blue_becomes_blue_is_sky_the() {
        let mut s = vec![
            't', 'h', 'e', ' ', 's', 'k', 'y', ' ', 'i', 's', ' ', 'b', 'l', 'u', 'e',
        ];
        Solution::reverse_words(&mut s);
        let e = [
            'b', 'l', 'u', 'e', ' ', 'i', 's', ' ', 's', 'k', 'y', ' ', 't', 'h', 'e',
        ];
        assert_eq!(s, e);
    }
    #[test]
    fn a_stays_a() {
        let mut s = vec!['a'];
        Solution::reverse_words(&mut s);
        assert_eq!(s, ['a']);
    }

    mod performance {
        use super::*;

        #[test]
        fn abcde_repeat_20000_stays_the_same() {
            let mut s = (0..20000).flat_map(|_| vec!['a', 'b', 'c', 'd', 'e', ' ']).collect::<Vec<_>>();
            s.pop();
            let e = s.clone();
            Solution::reverse_words(&mut s);
            assert_eq!(s, e);
        }
        #[test]
        fn a_repeat_100000_stays_the_same() {
            let mut s = (0..100000).map(|_| 'a').collect::<Vec<_>>();
            s.pop();
            let e = s.clone();
            Solution::reverse_words(&mut s);
            assert_eq!(s, e);
        }
    }
}
