#![allow(dead_code)]
/// Shifting Letters
/// ================
///
/// You are given a string `s` of lowercase English letters and an integer array `shifts` of the same length.
///
/// Call the `shift()` of a letter, the next letter in the alphabet, (wrapping around so that `'z'` becomes `'a'`).
///
/// For example, `shift('a') = 'b'`, `shift('t') = 'u'`, and `shift('z') = 'a'`.
///
/// Now for each `shifts[i] = x`, we want to shift the first `i + 1` letters of `s`, `x` times.
///
/// Return _the final string after all such shifts to s are applied_.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 100_000`
/// - `s` consists of lowercase English letters.
/// - `shifts.length == s.length`
/// - `0 <= shifts[i] <= 1_000_000_000`
///
/// https://leetcode.com/explore/challenge/card/september-leetcoding-challenge-2021/637/week-2-september-8th-september-14th/3968/
struct Solution;
impl Solution {
    /// https://leetcode.com/problems/shifting-letters/solution/1076680
    pub fn shifting_letters(mut s: String, shifts: Vec<i32>) -> String {
        unsafe {
            let bs = s.as_mut_vec();
            let mut shift = 0;
            for i in (0..shifts.len()).rev() {
                shift = (shift + shifts[i]) % 26;
                bs[i] = (bs[i] - b'a' + shift as u8) % 26 + b'a';
            }
        }
        s
    }
    pub fn shifting_letters_my(mut s: String, mut shifts: Vec<i32>) -> String {
        let mut shift = 0;
        for i in (0..shifts.len()).rev() {
            shift = (shift + shifts[i]) % 26;
            shifts[i] = shift;
        }
        unsafe {
            let bs = s.as_mut_vec();
            for i in 0..shifts.len() {
                bs[i] += shifts[i] as u8;
                if bs[i] > b'z' {
                    bs[i] -= 26;
                }
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abc_3_5_9_produces_rpl() {
        let s = "abc".to_string();
        let shifts = vec![3, 5, 9];
        assert_eq!(Solution::shifting_letters(s, shifts), "rpl");
        // Explanation: We start with "abc".
        // After shifting the first 1 letters of s by 3, we have "dbc".
        // After shifting the first 2 letters of s by 5, we have "igc".
        // After shifting the first 3 letters of s by 9, we have "rpl", the answer.
    }
    #[test]
    fn aaa_1_2_3_produces_gfd() {
        let s = "aaa".to_string();
        let shifts = vec![1, 2, 3];
        assert_eq!(Solution::shifting_letters(s, shifts), "gfd");
    }
    #[test]
    fn aaa_27_28_29_produces_gfd() {
        let s = "aaa".to_string();
        let shifts = vec![27, 28, 29];
        assert_eq!(Solution::shifting_letters(s, shifts), "gfd");
    }
    #[test]
    fn zzz_25_25_27_produces_yza() {
        let s = "zzz".to_string();
        let shifts = vec![25, 25, 27];
        assert_eq!(Solution::shifting_letters(s, shifts), "yza");
    }
}
