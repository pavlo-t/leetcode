#![allow(dead_code)]
/// # Reconstruct Original Digits from English
///
/// Given a __non-empty__ string containing an out-of-order English representation of digits `0-9`,
/// output the digits in ascending order.
///
/// __Note:__
///
/// 1. Input contains only lowercase English letters.
/// 2. Input is guaranteed to be valid and can be transformed to its original digits.
///    That means invalid inputs such as "abc" or "zerone" are not permitted.
/// 3. Input length is less than 50,000.
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/591/week-4-march-22nd-march-28th/3687/
struct Solution;
impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut counts = vec![0; 10];
        for &b in s.as_bytes() {
            match b {
                b'z' => counts[0] += 1,
                b'o' => counts[1] += 1,
                b'w' => counts[2] += 1,
                b't' => counts[3] += 1,
                b'u' => counts[4] += 1,
                b'f' => counts[5] += 1,
                b'x' => counts[6] += 1,
                b's' => counts[7] += 1,
                b'g' => counts[8] += 1,
                b'i' => counts[9] += 1,
                _ => (),
            }
        }

        counts[1] -= counts[0] + counts[2] + counts[4];
        counts[3] -= counts[2] + counts[8];
        counts[5] -= counts[4];
        counts[7] -= counts[6];
        counts[9] -= counts[5] + counts[6] + counts[8];

        let mut result = String::new();
        for (i, &c) in counts.iter().enumerate() {
            result.push_str(&i.to_string().repeat(c));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::original_digits("owoztneoer".to_string()), "012");
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::original_digits("fviefuro".to_string()), "45");
    }

    #[test]
    fn test_0123456789() {
        let s = "zeroonetwothreefourfivesixseveneightnine".to_string();
        assert_eq!(Solution::original_digits(s), "0123456789");
    }

    mod performance {
        use super::*;

        #[test]
        fn test_10k_three() {
            let s = "three".repeat(10_000);
            assert_eq!(Solution::original_digits(s), "3".repeat(10_000));
        }
    }
}
