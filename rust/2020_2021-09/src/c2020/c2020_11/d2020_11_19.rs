#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut data = vec![(1, String::new())];
        let mut current_k = 0;

        for c in s.chars() {
            match c {
                '0'..='9' if current_k == 0 => {
                    current_k = c.to_digit(10).unwrap();
                }
                '0'..='9' => {
                    current_k = current_k * 10 + c.to_digit(10).unwrap();
                }
                '[' => {
                    data.push((current_k, String::new()));
                    current_k = 0;
                }
                'a'..='z' => {
                    data.last_mut().unwrap().1.push(c);
                }
                ']' => {
                    let (k, pat) = data.pop().unwrap();
                    data.last_mut().unwrap().1.push_str(&pat.repeat(k as usize));
                }
                _ => panic!("Unsupported char: {}", c),
            };
        }

        data.first().unwrap().to_owned().1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_a_2_bc_() {
        assert_eq!(Solution::decode_string("3[a]2[bc]".to_string()), "aaabcbc".to_string())
    }

    #[test]
    fn test_3_a2_c__() {
        assert_eq!(Solution::decode_string("3[a2[c]]".to_string()), "accaccacc".to_string())
    }

    #[test]
    fn test_a2_b3_c_d_e() {
        let s = "a2[b3[c]d]e".to_string();
        let e = "abcccdbcccde".to_string();
        assert_eq!(Solution::decode_string(s), e)
    }

    #[test]
    fn test_2_abc_3_cd_ef() {
        assert_eq!(Solution::decode_string("2[abc]3[cd]ef".to_string()), "abcabccdcdcdef".to_string())
    }

    #[test]
    fn test_abc3_cd_xyz() {
        let s = String::from("abc3[cd]xyz");
        let expected = String::from("abccdcdcdxyz");
        assert_eq!(Solution::decode_string(s), expected);
    }

    #[test]
    fn test_100_leetcode_() {
        let s = String::from("100[leetcode]");
        let expected: String = std::iter::repeat("leetcode").take(100).collect();
        assert_eq!(Solution::decode_string(s), expected);
    }
}
