#![allow(dead_code)]
/// \#1461. Check If a String Contains All Binary Codes of Size K
/// =============================================================
///
/// Given a binary string `s` and an integer `k`,
/// return _`true` if every binary code of length `k` is a substring of `s`_.
/// Otherwise, return `false`.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 500_000`
/// - `s[i]` is either `'0'` or `'1'`.
/// - `1 <= k <= 20`
///
/// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k
struct Solution;
impl Solution {
    /// 18:16‥18:24
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        if s.len() < k {
            return false;
        }
        let mut seen_codes = vec![false; 1 << k];
        for l in 0..=s.len() - k {
            let r = l + k;
            let num = usize::from_str_radix(&s[l..r], 2).unwrap();
            seen_codes[num] = true;
        }
        for n in 0..(1 << k) {
            if !seen_codes[n] {
                return false;
            }
        }
        true
    }
    /// 18:05‥18:16
    pub fn has_all_codes_hash_set(s: String, k: i32) -> bool {
        use std::collections::HashSet;

        if s.len() < k as usize {
            return false;
        }
        let mut seen_codes: HashSet<i32> = HashSet::new();
        for l in 0..=s.len() - k as usize {
            let r = l + k as usize;
            let num = i32::from_str_radix(&s[l..r], 2).unwrap();
            seen_codes.insert(num);
        }
        for n in 0..(1 << k) {
            if !seen_codes.contains(&n) {
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
    fn s00110110_k2() {
        let s = "00110110".to_string();
        assert_eq!(Solution::has_all_codes(s, 2), true);
        // Explanation: The binary codes of length 2 are "00", "01", "10" and "11".
        // They can be all found as substrings at indices 0, 1, 3 and 2 respectively.
    }
    #[test]
    fn s0110_k1() {
        let s = "0110".to_string();
        assert_eq!(Solution::has_all_codes(s, 1), true);
        // Explanation: The binary codes of length 1 are "0" and "1", it is clear that both exist as a substring.
    }
    #[test]
    fn s0110_k2() {
        let s = "0110".to_string();
        assert_eq!(Solution::has_all_codes(s, 2), false);
        // Explanation: The binary code "00" is of length 2 and does not exist in the array.
    }

    #[test]
    fn s0_k20() {
        assert_eq!(Solution::has_all_codes("0".into(), 20), false);
    }
    #[test]
    fn s0101_k13() {
        assert_eq!(Solution::has_all_codes("0101".into(), 13), false);
    }

    #[test]
    fn s500000x0_k1() {
        let s = "0".repeat(500000);
        assert_eq!(Solution::has_all_codes(s, 1), false);
    }
    #[test]
    fn s500000x1_k1() {
        let s = "1".repeat(500000);
        assert_eq!(Solution::has_all_codes(s, 1), false);
    }
    #[test]
    fn s500000x1_k20() {
        let s = "1".repeat(500000);
        assert_eq!(Solution::has_all_codes(s, 20), false);
    }
}
