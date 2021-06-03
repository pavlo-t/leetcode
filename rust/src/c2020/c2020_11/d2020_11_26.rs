// @formatter:off
struct Solution;
// @formatter:on
impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        if s.len() < k as usize {
            0
        } else {
            let invalid_chars: Vec<_> =
                s.chars()
                    .fold(vec![('0', 0); 26], |mut acc, c| {
                        let i = c as usize - 'a' as usize;
                        acc[i].0 = c;
                        acc[i].1 += 1;
                        acc
                    })
                    .iter()
                    .filter_map(|&(c, x)| if x > 0 && x < k { Some(c) } else { None })
                    .collect();

            if invalid_chars.len() == 0 {
                s.len() as i32
            } else {
                s.split(|c| invalid_chars.contains(&c))
                    .map(|s| Self::longest_substring(s.to_string(), k))
                    .max()
                    .unwrap_or_else(|| 0)
            }
        }
    }
}
// @formatter:off

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_aaabb3_is_3() {
        assert_eq!(Solution::longest_substring("aaabb".to_string(), 3), 3);
    }
    #[test]
    fn test_ababbc2_is_5() {
        assert_eq!(Solution::longest_substring("ababbc".to_string(), 2), 5);
    }

    #[test]
    fn test_weitong2_is_0() {
        assert_eq!(Solution::longest_substring("weitong".to_string(), 2), 0);
    }
    #[test]
    fn test_a2_is_0() {
        assert_eq!(Solution::longest_substring("a".to_string(), 2), 0);
    }
    #[test]
    fn test_ababcab2_is_4() {
        assert_eq!(Solution::longest_substring("ababcab".to_string(), 2), 4);
    }
    #[test]
    fn test_ababab2_is_6() {
        assert_eq!(Solution::longest_substring("ababab".to_string(), 2), 6);
    }
    #[test]
    fn test_cababab2_is_6() {
        assert_eq!(Solution::longest_substring("cababab".to_string(), 2), 6);
    }
    #[test]
    fn test_10000a2_is_10000() {
        let s = std::iter::repeat('a').take(10000).collect();
        assert_eq!(Solution::longest_substring(s, 2), 10000);
    }
    #[test]
    fn test_10000random2_is_10000() {
        let mut rng = rand::thread_rng();
        let mut s = String::new();
        for _ in 0..10000 {
            let c = rng.gen_range('a' as u8, 'z' as u8) as char;
            s.push(c);
        }
        assert!(Solution::longest_substring(s, 2) >= 0);
    }
}
