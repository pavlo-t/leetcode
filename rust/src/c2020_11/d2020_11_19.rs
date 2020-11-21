struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut data = vec![(1, String::new())];
        let mut reading_digit = false;

        for c in s.chars() {
            match c {
                '0'..='9' if reading_digit => {
                    let mut last = data.last_mut().unwrap();
                    last.0 = last.0 * 10 + c.to_digit(10).unwrap();
                }
                '0'..='9' => {
                    reading_digit = true;
                    data.push((c.to_digit(10).unwrap(), String::new()));
                }
                '[' => { reading_digit = false; }
                ']' => {
                    let (cnt, pat) = data.pop().unwrap();
                    let sub_s = std::iter::repeat(pat).take(cnt as usize).collect::<String>();
                    let last = data.last_mut().unwrap();

                    last.1.push_str(&sub_s);
                }
                'a'..='z' => {
                    let last = data.last_mut().unwrap();
                    last.1.push(c)
                }
                _ => panic!("Unsupported char: {}", c),
            };
        }

        let (_, result) = data.first().unwrap().to_owned();

        result
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