#![allow(dead_code)]
/// Palindrome Partitioning II
/// ==========================
///
/// Given a string `s`, partition `s` such that every substring of the partition is a palindrome.
///
/// Return _the minimum cuts needed_ for a palindrome partitioning of `s`.
///
/// Constraints:
///
/// - `1 <= s.length <= 2000`
/// - `s` consists of lower-case English letters only.
///
/// https://leetcode.com/explore/featured/card/august-leetcoding-challenge-2021/613/week-1-august-1st-august-7th/3872/
struct Solution;
impl Solution {
    /// https://www.geeksforgeeks.org/palindrome-partitioning-dp-17/
    pub fn min_cut(s: String) -> i32 {
        let bs = s.as_bytes();
        let n = bs.len();
        let mut ps = vec![vec![false; n]; n];
        (0..n).for_each(|i| ps[i][i] = true);
        for lr in 2..=n {
            for l in 0..=n - lr {
                let r = l + lr - 1;
                ps[l][r] = bs[l] == bs[r] && (lr < 4 || ps[l + 1][r - 1]);
            }
        }
        let mut cs = vec![std::i32::MAX; n];
        for r in 0..n {
            if ps[0][r] {
                cs[r] = 0;
            } else {
                for l in 0..r {
                    if ps[l + 1][r] {
                        cs[r] = cs[r].min(cs[l] + 1);
                    }
                }
            }
        }
        cs[n - 1]
    }

    pub fn min_cut_rec_with_memo(s: String) -> i32 {
        fn is_palindrome(s: &[u8]) -> bool {
            let (mut l, mut r) = (0, s.len() - 1);
            while l < r {
                if s[l] != s[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }
            true
        }
        fn min_cut_rec(l: usize, r: usize, s: &[u8], d: &mut Vec<Vec<i32>>) -> i32 {
            //println!("mcr(l:{},r:{},s:{})", l, r, String::from_utf8(s[l..=r].iter().map(|&b| b).collect::<Vec<_>>()).unwrap());
            if d[l][r] < 0 {
                if is_palindrome(&s[l..=r]) {
                    d[l][r] = 0;
                } else {
                    d[l][r] = (l..r)
                        .map(|m| 1 + min_cut_rec(l, m, s, d) + min_cut_rec(m + 1, r, s, d))
                        .min()
                        .unwrap();
                }
            }
            d[l][r]
        }
        let n = s.len();
        let mut data = vec![vec![-1; n]; n];
        (0..n).for_each(|i| data[i][i] = 0);
        min_cut_rec(0, n - 1, s.as_bytes(), &mut data)
    }

    pub fn min_cut_rec_brute_force(s: String) -> i32 {
        fn is_palindrome(s: &[u8]) -> bool {
            let (mut l, mut r) = (0, s.len() - 1);
            while l < r {
                if s[l] != s[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }
            true
        }
        fn min_cut_rec(l: usize, r: usize, s: &[u8]) -> i32 {
            if is_palindrome(&s[l..=r]) {
                0
            } else {
                (l..r)
                    .map(|m| 1 + min_cut_rec(l, m, s) + min_cut_rec(m + 1, r, s))
                    .min()
                    .unwrap()
            }
        }
        min_cut_rec(0, s.len() - 1, s.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aab_produces_1() {
        assert_eq!(Solution::min_cut("aab".to_string()), 1);
        // Explanation: The palindrome partitioning ["aa","b"] could be produced using 1 cut.
    }
    #[test]
    fn a_produces_0() {
        assert_eq!(Solution::min_cut("a".to_string()), 0);
    }
    #[test]
    fn ab_produces_1() {
        assert_eq!(Solution::min_cut("ab".to_string()), 1);
    }
    #[test]
    fn aaba_produces_1() {
        assert_eq!(Solution::min_cut("aaba".to_string()), 1);
    }
    #[test]
    fn ababbbabbababa_produces_3() {
        assert_eq!(Solution::min_cut("ababbbabbababa".to_string()), 3);
    }
    #[test]
    fn ax2000_produces_0() {
        assert_eq!(Solution::min_cut("a".repeat(2000)), 0);
    }
    #[test]
    fn a_to_z_x20_eq_520_chars_produces_519() {
        let s = String::from_utf8((b'a'..=b'z').collect::<Vec<_>>())
            .unwrap()
            .repeat(20);
        assert_eq!(Solution::min_cut(s), 519);
    }
    #[test]
    fn a_to_z_x77_produces_0() {
        let s = String::from_utf8((b'a'..=b'z').collect::<Vec<_>>())
            .unwrap()
            .repeat(40);
        assert_eq!(Solution::min_cut(s), 1039);
    }
}
