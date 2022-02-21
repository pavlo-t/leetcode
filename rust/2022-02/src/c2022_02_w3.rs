#![allow(dead_code)]
/// 1100. Find K-Length Substrings With No Repeated Characters
/// ==========================================================
///
/// Given a string `s` and an integer `k`,
/// return _the number of substrings in `s` of length `k` with no repeated characters_.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 10_000`
/// - `s` consists of lowercase English letters.
/// - `1 <= k <= 10_000`
///
/// https://leetcode.com/problems/find-k-length-substrings-with-no-repeated-characters/
struct Solution;
impl Solution {
    pub fn num_k_len_substr_no_repeats(s: String, k: i32) -> i32 {
        let k = k as usize;
        if k > 26 || k > s.len() {
            0
        } else {
            fn no_repeats(counts: &[usize; 26]) -> bool {
                counts.iter().all(|&count| count < 2)
            }
            let mut counts = [0; 26];
            for b in s.bytes().take(k) {
                counts[(b - b'a') as usize] += 1;
            }
            let mut result = if no_repeats(&counts) { 1 } else { 0 };
            let bytes = s.as_bytes();
            for i in k..s.len() {
                counts[(bytes[i] - b'a') as usize] += 1;
                counts[(bytes[i - k] - b'a') as usize] -= 1;
                result += no_repeats(&counts) as i32;
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn havefunonleetcode_5() {
        let s = "havefunonleetcode".to_string();
        assert_eq!(Solution::num_k_len_substr_no_repeats(s, 5), 6);
        // Explanation: There are 6 substrings they are: 'havef','avefu','vefun','efuno','etcod','tcode'.
    }
    #[test]
    fn home_4() {
        let s = "home".to_string();
        assert_eq!(Solution::num_k_len_substr_no_repeats(s, 4), 1);
    }
    #[test]
    fn home_5() {
        let s = "home".to_string();
        assert_eq!(Solution::num_k_len_substr_no_repeats(s, 5), 0);
        // Explanation: Notice k can be larger than the length of s.
        // In this case, it is not possible to find any substring.
    }
}
