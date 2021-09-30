#![allow(dead_code)]

/// # Strobogrammatic Number
///
/// Given a string `num` which represents an integer,
/// return `true` _if_ `num` _is a __strobogrammatic number___.
///
/// A __strobogrammatic number__ is a number that looks the same when rotated `180` degrees
/// (looked at upside down).
///
/// __Constraints:__
///
/// - `1 <= num.length <= 50`
/// - `num` consists of only digits.
/// - `num` does not contain any leading zeros except for zero itself.
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/589/week-2-march-8th-march-14th/3664/
struct Solution;

impl Solution {
    pub fn is_strobogrammatic(num: String) -> bool {
        use std::collections::HashMap;
        let mut map = HashMap::with_capacity(5);
        map.insert(b'0', b'0');
        map.insert(b'1', b'1');
        map.insert(b'6', b'9');
        map.insert(b'8', b'8');
        map.insert(b'9', b'6');
        let bytes = num.as_bytes();
        let mut l = 0;
        let mut r = bytes.len() - 1;
        while l < r {
            match map.get(&bytes[l]) {
                Some(e) if &bytes[r] == e => (),
                _ => return false,
            }
            l += 1;
            r -= 1;
        }
        if l == r {
            &bytes[l] == map.get(&bytes[l]).unwrap_or(&b'E')
        } else {
            true
        }
    }

    pub fn is_strobogrammatic_my(num: String) -> bool {
        let bytes = num.as_bytes();
        let mut l = 0;
        let mut r = bytes.len() - 1;
        while l < r {
            match (bytes[l], bytes[r]) {
                (b'0', b'0') | (b'1', b'1') | (b'8', b'8') | (b'6', b'9') | (b'9', b'6') => (),
                _ => return false,
            }
            l += 1;
            r -= 1;
        }
        if l == r {
            match bytes[l] {
                b'0' | b'1' | b'8' => true,
                _ => false,
            }
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_n69_is_strobogrammatic() {
        assert!(Solution::is_strobogrammatic("69".to_string()));
    }
    #[test]
    fn example2_n88_is_strobogrammatic() {
        assert!(Solution::is_strobogrammatic("88".to_string()));
    }
    #[test]
    fn example3_n962_is_not_strobogrammatic() {
        assert!(!Solution::is_strobogrammatic("962".to_string()));
    }
    #[test]
    fn n1_is_strobogrammatic() {
        assert!(Solution::is_strobogrammatic("1".to_string()));
    }

    #[test]
    fn n2_is_not_strobogrammatic() {
        assert!(!Solution::is_strobogrammatic("2".to_string()));
    }
    #[test]
    fn n3_is_not_strobogrammatic() {
        assert!(!Solution::is_strobogrammatic("3".to_string()));
    }
    #[test]
    fn n4_is_not_strobogrammatic() {
        assert!(!Solution::is_strobogrammatic("4".to_string()));
    }
    #[test]
    fn n5_is_not_strobogrammatic() {
        assert!(!Solution::is_strobogrammatic("5".to_string()));
    }
    #[test]
    fn n6_is_not_strobogrammatic() {
        assert!(!Solution::is_strobogrammatic("6".to_string()));
    }
    #[test]
    fn n7_is_not_strobogrammatic() {
        assert!(!Solution::is_strobogrammatic("7".to_string()));
    }
    #[test]
    fn n8_is_strobogrammatic() {
        assert!(Solution::is_strobogrammatic("8".to_string()));
    }
    #[test]
    fn n9_is_not_strobogrammatic() {
        assert!(!Solution::is_strobogrammatic("9".to_string()));
    }
    #[test]
    fn n0_is_strobogrammatic() {
        assert!(Solution::is_strobogrammatic("0".to_string()));
    }
    #[test]
    fn n01869069810_is_strobogrammatic() {
        assert!(Solution::is_strobogrammatic("01869069810".to_string()));
    }
}
