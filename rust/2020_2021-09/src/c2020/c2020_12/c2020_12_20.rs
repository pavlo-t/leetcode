#![allow(dead_code)]

struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut k = k as i64;
        let mut len = 0i64;
        let mut chars = 0;

        for c in s.chars() {
            chars += 1;
            if let Some(m) = c.to_digit(10) {
                len *= m as i64;
                if len > k { break; }
            } else {
                len += 1;
                if len == k { return c.to_string(); }
            };
        }

        let parsed = &s[..chars];

        for c in parsed.chars().rev() {
            k %= len;
            if let Some(m) = c.to_digit(10) {
                len /= m as i64;
            } else if k == 0 {
                return c.to_string();
            } else {
                len -= 1;
            }
        }

        panic!("Illegal input: {}", s);
    }

    // https://leetcode.com/problems/decoded-string-at-index/solution/
    pub fn decode_at_index_solution(s: String, k: i32) -> String {
        let mut len = 0i64;

        for c in s.chars() {
            if let Some(m) = c.to_digit(10) {
                len *= m as i64;
            } else {
                len += 1;
            };
        }

        let mut k = k as i64;

        for c in s.chars().rev() {
            k %= len;
            if let Some(m) = c.to_digit(10) {
                len /= m as i64;
            } else if k == 0 {
                return c.to_string();
            } else {
                len -= 1;
            }
        }

        panic!("Illegal input: {}", s);
    }
}


#[cfg(test)]
// @formatter:off
mod tests {
    use super::*;

    #[test] fn example1_leet2code3_k10_is_o() {
        assert_eq!(Solution::decode_at_index("leet2code3".to_string(), 10), "o");
        //Explanation:
        //The decoded string is "leetleetcodeleetleetcodeleetleetcode".
        //The 10th letter in the string is "o".
    }
    #[test] fn example2_ha22_k5_is_h() {
        assert_eq!(Solution::decode_at_index("ha22".to_string(), 5), "h");
        //Explanation:
        //The decoded string is "hahahaha". The 5th letter is "h".
    }
    #[test] fn example3_a2345678999999999999999_k1_is_a() {
        assert_eq!(Solution::decode_at_index("a2345678999999999999999".to_string(), 1), "a");
        //Explanation:
        //The decoded string is "a" repeated 8301530446056247680 times. The 1st letter is "a".
    }

    #[test] fn test8_a23_k6_is_a() {
        assert_eq!(Solution::decode_at_index("a23".to_string(), 6), "a");
    }
    #[test] fn test34_y959q969u3hb22odq595_k222280369_is_y() {
        assert_eq!(Solution::decode_at_index("y959q969u3hb22odq595".to_string(), 222280369), "y");
    }

    #[test] fn test_abc3_k1_is_a() { assert_eq!(Solution::decode_at_index("abc3".to_string(), 1), "a"); }
    #[test] fn test_abc3_k2_is_b() { assert_eq!(Solution::decode_at_index("abc3".to_string(), 2), "b"); }
    #[test] fn test_abc3_k3_is_c() { assert_eq!(Solution::decode_at_index("abc3".to_string(), 3), "c"); }
    #[test] fn test_abc3_k4_is_a() { assert_eq!(Solution::decode_at_index("abc3".to_string(), 4), "a"); }
    #[test] fn test_abc3_k5_is_b() { assert_eq!(Solution::decode_at_index("abc3".to_string(), 5), "b"); }
    #[test] fn test_abc3_k6_is_c() { assert_eq!(Solution::decode_at_index("abc3".to_string(), 6), "c"); }
    #[test] fn test_abc3_k7_is_a() { assert_eq!(Solution::decode_at_index("abc3".to_string(), 7), "a"); }
    #[test] fn test_abc3_k8_is_b() { assert_eq!(Solution::decode_at_index("abc3".to_string(), 8), "b"); }
    #[test] fn test_abc3_k9_is_c() { assert_eq!(Solution::decode_at_index("abc3".to_string(), 9), "c"); }

    #[test] fn test_a2345678999999999999999_k100_000_000_is_a() {
        assert_eq!(Solution::decode_at_index("a2345678999999999999999".to_string(), 100_000_000), "a");
    }
    #[test] fn test_a2345678999999999999999_k1_000_000_000_is_a() {
        assert_eq!(Solution::decode_at_index("a2345678999999999999999".to_string(), 1_000_000_000), "a");
    }
}
