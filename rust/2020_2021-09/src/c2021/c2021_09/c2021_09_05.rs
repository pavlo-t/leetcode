#![allow(dead_code)]
/// Orderly Queue
/// =============
///
/// You are given a string `s` and an integer `k`.
/// You can choose one of the first `k` letters of s and append it at the end of the string.
///
/// Return _the lexicographically smallest string you could have after applying the mentioned step any number of moves_.
///
/// __Constraints:__
///
/// - `1 <= k <= s.length <= 1000`
/// - `s` consist of lowercase English letters.
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/636/week-1-september-1st-september-7th/3964/
struct Solution;
impl Solution {
    /// https://leetcode.com/problems/orderly-queue/solution/
    pub fn orderly_queue_leetcode(s: String, k: i32) -> String {
        let mut bs = s.into_bytes();
        if k == 1 {
            let mut result = bs.clone();
            for i in 0..bs.len() {
                let tmp = bs[i..]
                    .iter()
                    .chain(bs[..i].iter())
                    .map(|&b| b)
                    .collect::<Vec<_>>();
                result = result.min(tmp);
            }
            unsafe { String::from_utf8_unchecked(result) }
        } else {
            bs.sort_unstable();
            unsafe { String::from_utf8_unchecked(bs) }
        }
    }

    pub fn orderly_queue(s: String, k: i32) -> String {
        let mut bs = s.into_bytes();
        if k > 1 {
            bs.sort_unstable();
            unsafe { String::from_utf8_unchecked(bs) }
        } else {
            let min = bs.iter().map(|&b| b).min().unwrap();
            let idxs = bs
                .iter()
                .enumerate()
                .filter(|(_, &b)| b == min)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();

            if idxs.len() == 1 {
                bs.rotate_left(idxs[0]);
            } else if idxs.len() < bs.len() {
                let mut result = bs.clone();
                for i in idxs {
                    let local = bs[i..]
                        .iter()
                        .chain(bs[..i].iter())
                        .map(|&b| b)
                        .collect::<Vec<_>>();
                    result = result.min(local);
                }
                bs = result;
            }
            unsafe { String::from_utf8_unchecked(bs) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn k1_cba_produces_acb() {
        let s = "cba".to_string();
        let k = 1;
        let e = "acb";
        assert_eq!(Solution::orderly_queue(s, k), e);
        // Explanation:
        // In the first move, we move the 1st character 'c' to the end, obtaining the string "bac".
        // In the second move, we move the 1st character 'b' to the end, obtaining the final result "acb".
    }
    #[test]
    fn k3_baaca_produces_aaabc() {
        let s = "baaca".to_string();
        let k = 3;
        let e = "aaabc";
        assert_eq!(Solution::orderly_queue(s, k), e);
        // Explanation:
        // In the first move, we move the 1st character 'b' to the end, obtaining the string "aacab".
        // In the second move, we move the 3rd character 'c' to the end, obtaining the final result "aaabc".
    }

    #[test]
    fn k1_ccc_produces_ccc() {
        let s = "ccc".to_string();
        let k = 1;
        let e = "ccc";
        assert_eq!(Solution::orderly_queue(s, k), e);
    }
    #[test]
    fn k1_ccac_produces_accc() {
        let s = "ccac".to_string();
        let k = 1;
        let e = "accc";
        assert_eq!(Solution::orderly_queue(s, k), e);
    }
    #[test]
    fn k1_ccabcacb_produces_abcacbcc() {
        let s = "ccabcacb".to_string();
        let k = 1;
        let e = "abcacbcc";
        assert_eq!(Solution::orderly_queue(s, k), e);
    }
    #[test]
    fn k1_ccacbabc_produces_abcccacb() {
        let s = "ccacbabc".to_string();
        let k = 1;
        let e = "abcccacb";
        assert_eq!(Solution::orderly_queue(s, k), e);
    }

    #[test]
    fn k2_ccc_produces_ccc() {
        let s = "ccc".to_string();
        let k = 2;
        let e = "ccc";
        assert_eq!(Solution::orderly_queue(s, k), e);
    }
    #[test]
    fn k2_cba_produces_abc() {
        let s = "cba".to_string();
        let k = 2;
        let e = "abc";
        assert_eq!(Solution::orderly_queue(s, k), e);
        // cba -> cab -> abc
    }
    #[test]
    fn k2_dcba_produces_abcd() {
        let s = "dcba".to_string();
        let k = 2;
        let e = "abcd";
        assert_eq!(Solution::orderly_queue(s, k), e);
        // dcba dbac bacd acdb adbc abcd
    }
    #[test]
    fn k2_edcba_produces_abcde() {
        let s = "edcba".to_string();
        let k = 2;
        let e = "abcde";
        assert_eq!(Solution::orderly_queue(s, k), e);
        // edcba ecbad ebadc eadcb adcbe acbed cbeda cedab edabc eabcd abcde
    }
    #[test]
    fn k2_fedcba_produces_abcdef() {
        let s = "fedcba".to_string();
        let k = 2;
        let e = "abcdef";
        assert_eq!(Solution::orderly_queue(s, k), e);
    }

    #[test]
    fn k1_1b999a_produces_999a1b() {
        let mut s = "b".to_string();
        (0..999).for_each(|_| s.push('a'));
        let k = 1;
        let mut e = "a".repeat(999);
        e.push('b');
        assert_eq!(Solution::orderly_queue(s, k), e);
    }
}
