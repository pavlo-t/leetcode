#![allow(dead_code)]
/// Custom Sort String
/// ==================
///
/// `order` and `str` are strings composed of lowercase letters.
/// In `order`, no letter occurs more than once.
///
/// `order` was sorted in some custom order previously.
/// We want to permute the characters of `str` so that they match the order that `order` was sorted.
/// More specifically, if `x` occurs before `y` in `order`, then `x` should occur before `y` in the returned string.
///
/// Return any permutation of `str` (as a string) that satisfies this property.
///
/// __Note:__
///
/// - `order` has length at most `26`, and no character is repeated in `order`.
/// - `str` has length at most `200`.
/// - `order` and `str` consist of lowercase letters only.
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/609/week-2-july-8th-july-14th/3813/
struct Solution;
impl Solution {
    pub fn custom_sort_string(order: String, str: String) -> String {
        let mut b_to_o = vec![26usize; 26];
        let mut o_to_b = vec![0u8; 26];
        for (o, &b) in order.as_bytes().iter().enumerate() {
            let i = (b - b'a') as usize;
            b_to_o[i] = o;
            o_to_b[o] = b;
        }
        if order.len() < 26 {
            let mut o = order.len();
            for i in 0..26 {
                if b_to_o[i] == 26 {
                    b_to_o[i] = o;
                    o_to_b[o] = b'a' + i as u8;
                    o += 1;
                }
            }
        }

        let mut encoded = str
            .as_bytes()
            .iter()
            .map(|&b| b_to_o[(b - b'a') as usize])
            .collect::<Vec<_>>();
        encoded.sort_unstable();

        String::from_utf8(encoded.into_iter().map(|o| o_to_b[o]).collect::<Vec<_>>()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn o_cba_s_abcd_produces_cbad() {
        let order = "cba".to_string();
        let str = "abcd".to_string();
        assert_eq!(Solution::custom_sort_string(order, str), "cbad");
        // Explanation:
        // "a", "b", "c" appear in order, so the order of "a", "b", "c" should be "c", "b", and "a".
        // Since "d" does not appear in order, it can be at any position in the returned string.
        // "dcba", "cdba", "cbda" are also valid outputs.
    }
    #[test]
    fn o_cba_s_abbcccdddd_produces_cccbbadddd() {
        let order = "cba".to_string();
        let str = "abbcccdddd".to_string();
        assert_eq!(Solution::custom_sort_string(order, str), "cccbbadddd");
    }
}
