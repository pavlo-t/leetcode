#![allow(dead_code)]
/// 394. Decode String
/// ==================
///
/// Given an encoded string, return its decoded string.
///
/// The encoding rule is:
/// `k[encoded_string]`, where the `encoded_string` inside the square brackets is being repeated exactly `k` times.
/// Note that `k` is guaranteed to be a positive integer.
///
/// You may assume that the input string is always valid;
/// No extra white spaces, square brackets are well-formed, etc.
///
/// Furthermore, you may assume that the original data does not contain any digits
/// and that digits are only for those repeat numbers, `k`.
/// For example, there won't be input like `3a` or `2[4]`.
///
/// __Constraints:__
///
/// - `1 <= s.length <= 30`
/// - `s` consists of lowercase English letters, digits, and square brackets `'[]'`.
/// - `s` is guaranteed to be __a valid__ input.
/// - All the integers in `s` are in the range `[1, 300]`.
///
/// https://leetcode.com/problems/decode-string/
struct Solution;
impl Solution {
    /// from `/rust/2020_2021-09/src/c2020/c2020_11/d2020_11_19.rs`
    pub fn decode_string(s: String) -> String {
        println!("decode_string({:?})", s);
        let mut stack = vec![(1, String::new())];
        let mut reps = 0;

        for c in s.chars() {
            match c {
                '0'..='9' => reps = reps * 10 + c.to_digit(10).unwrap() as usize,
                '[' => {
                    stack.push((reps, String::new()));
                    reps = 0;
                }
                ']' => {
                    let (reps, chars) = stack.pop().unwrap();
                    stack.last_mut().unwrap().1.push_str(&chars.repeat(reps));
                }
                c => stack.last_mut().unwrap().1.push(c),
            }
        }

        stack[0].1.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s_10_a_() {
        let s = "10[a]".to_string();
        let e = "aaaaaaaaaa";
        assert_eq!(Solution::decode_string(s), e);
    }
    #[test]
    fn s_3_a_2_bc_() {
        let s = "3[a]2[bc]".to_string();
        let e = "aaabcbc";
        assert_eq!(Solution::decode_string(s), e);
    }
    #[test]
    fn s_3_a2_c__() {
        let s = "3[a2[c]]".to_string();
        let e = "accaccacc";
        assert_eq!(Solution::decode_string(s), e);
    }
    #[test]
    fn s_2_abc_3_cd_ef() {
        let s = "2[abc]3[cd]ef".to_string();
        let e = "abcabccdcdcdef";
        assert_eq!(Solution::decode_string(s), e);
    }
    #[test]
    fn s_abc_3_cd_xyz() {
        let s = "abc3[cd]xyz".to_string();
        let e = "abccdcdcdxyz";
        assert_eq!(Solution::decode_string(s), e);
    }
}
