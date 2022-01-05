#![allow(dead_code)]
/// 131. Palindrome Partitioning
/// ============================
///
/// Given a string `s`, partition `s` such that every substring of the partition is a __palindrome__.
/// Return all possible palindrome partitioning of `s`.
///
/// A __palindrome__ string is a string that reads the same backward as forward.
///
///
/// https://leetcode.com/problems/palindrome-partitioning/
struct Solution;
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        println!("partition({})", s);

        fn is_palindrome(mut l: usize, mut r: usize, bs: &[u8]) -> bool {
            while l < r {
                if bs[l] != bs[r] {
                    return false;
                }
                l += 1;
                r = r.saturating_sub(1);
            }
            true
        }
        fn rec(l: usize, r: usize, bs: &[u8]) -> Vec<Vec<String>> {
            if l == bs.len() {
                vec![vec![]]
            } else if r == bs.len() {
                vec![]
            } else if is_palindrome(l, r, bs) {
                let curr = std::str::from_utf8(&bs[l..=r]).unwrap();
                let mut next_take = rec(r + 1, r + 1, bs)
                    .into_iter()
                    .map(|mut strings| {
                        strings.push(curr.to_string());
                        strings
                    })
                    .collect::<Vec<_>>();
                let mut next_skip = rec(l, r + 1, bs);
                next_take.append(&mut next_skip);
                next_take
            } else {
                rec(l, r + 1, bs)
            }
        }

        let bs = s.as_bytes();
        rec(0, 0, bs)
            .into_iter()
            .map(|mut strings| {
                strings.reverse();
                strings
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($x:expr),*) => {vec![$($x.to_string()),*]};}
    macro_rules! vvs {($($x:tt),*) => {vec![$(vs!$x),*]};}

    #[test]
    fn a() {
        let s = "a".to_string();
        let e = vvs![["a"]];
        assert_eq!(Solution::partition(s), e);
    }
    #[test]
    fn aa() {
        let s = "aa".to_string();
        let e = vvs![["a", "a"], ["aa"]];
        assert_eq!(Solution::partition(s), e);
    }
    #[test]
    fn aaa() {
        let s = "aaa".to_string();
        let e = vvs![["a", "a", "a"], ["a", "aa"], ["aa", "a"], ["aaa"]];
        assert_eq!(Solution::partition(s), e);
    }
    #[test]
    fn aab() {
        let s = "aab".to_string();
        let e = vvs![["a", "a", "b"], ["aa", "b"]];
        assert_eq!(Solution::partition(s), e);
    }

    #[test]
    fn aaaaaaaaaaaaaaaa() {
        let s = "aaaaaaaaaaaaaaaa".to_string();
        let result = Solution::partition(s);
        assert_eq!(result.len(), 32768);
    }
}
