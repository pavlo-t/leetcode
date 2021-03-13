#![allow(dead_code)]

/// # Check If a String Contains All Binary Codes of Size K
///
/// Given a binary string `s` and an integer `k`.
///
/// Return _True_ if every binary code of length `k` is a substring of `s`.
/// Otherwise, return _False_.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 500_000`
/// - `s` consists of 0's and 1's only.
/// - `1 <= k <= 20`
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/589/week-2-march-8th-march-14th/3669/
struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let mut data = vec![false; 2usize.pow(k as u32)];
        let bytes = s.as_bytes();
        let mut l = 0usize;
        let mut r = k as usize;
        while r <= bytes.len() {
            let ss = std::str::from_utf8(&bytes[l..r]).unwrap();
            let i = usize::from_str_radix(ss, 2).unwrap();
            data[i] = true;

            l += 1;
            r += 1;
        }
        data.iter().all(|&b| b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_s00110110k2_produces_true() {
        let s = "00110110".to_string();
        let k = 2;
        assert!(Solution::has_all_codes(s, k));
        // Explanation:
        // The binary codes of length 2 are "00", "01", "10" and "11".
        // They can be all found as substrings at indicies 0, 1, 3 and 2 respectively.
    }
    #[test]
    fn example2_s00110k2_produces_true() {
        let s = "00110".to_string();
        let k = 2;
        assert!(Solution::has_all_codes(s, k));
    }
    #[test]
    fn example3_s0110k1_produces_true() {
        let s = "0110".to_string();
        let k = 1;
        assert!(Solution::has_all_codes(s, k));
        // Explanation:
        // The binary codes of length 1 are "0" and "1",
        // it is clear that both exist as a substring.
    }
    #[test]
    fn example4_s0110k2_produces_false() {
        let s = "0110".to_string();
        let k = 2;
        assert!(!Solution::has_all_codes(s, k));
        // Explanation: The binary code "00" is of length 2 and doesn't exist in the array.
    }
    #[test]
    fn example5_s0000000001011100k4_produces_false() {
        let s = "0000000001011100".to_string();
        let k = 4;
        assert!(!Solution::has_all_codes(s, k));
    }

    #[test]
    fn performance_true() {
        let mut s = String::new();
        let k = 15;
        for i in 0..2i32.pow(15) {
            s.push_str(&format!("{:015b}", i));
        }
        assert!(Solution::has_all_codes(s, k));
    }
    #[test]
    fn performance_false() {
        let s = "0".repeat(500_000);
        let k = 20;
        assert!(!Solution::has_all_codes(s, k));
    }
}
