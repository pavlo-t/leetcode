#![allow(dead_code)]
/// 402. Remove K Digits
/// ====================
///
/// Given string num representing a non-negative integer `num`, and an integer `k`,
/// return _the smallest possible integer after removing `k` digits from `num`_.
///
/// __Constraints:__
///
/// - `1 <= k <= num.length <= 100_000`
/// - `num` consists of only digits.
/// - `num` does not have any leading zeros except for the zero itself.
///
/// https://leetcode.com/problems/remove-k-digits/
struct Solution;
impl Solution {
    /// Approach:2 (Stack)
    /// https://www.geeksforgeeks.org/build-lowest-number-by-removing-n-digits-from-a-given-number/
    pub fn remove_kdigits(num: String, mut k: i32) -> String {
        let mut stack = vec![];
        for b in num.bytes() {
            while !stack.is_empty() && k > 0 && stack.last().unwrap() > &b {
                stack.pop();
                k -= 1;
            }
            if !stack.is_empty() || b != b'0' {
                stack.push(b);
            }
        }
        while !stack.is_empty() && k > 0 {
            stack.pop();
            k -= 1;
        }

        if stack.is_empty() {
            "0".to_string()
        } else {
            let mut result = String::new();
            while let Some(b) = stack.pop() {
                result.push(b as char);
            }
            result.chars().rev().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_1432219_k_3() {
        let n = "1432219".to_string();
        let k = 3;
        let e = "1219".to_string();
        assert_eq!(Solution::remove_kdigits(n, k), e);
        // Explanation: Remove the three digits 4, 3, and 2 to form the new number 1219 which is the smallest.
    }
    #[test]
    fn n_10200_k_1() {
        let n = "10200".to_string();
        let k = 1;
        let e = "200".to_string();
        assert_eq!(Solution::remove_kdigits(n, k), e);
        // Explanation: Remove the leading 1 and the number is 200. Note that the output must not contain leading zeroes.
    }
    #[test]
    fn n_10_k_2() {
        let n = "10".to_string();
        let k = 2;
        let e = "0".to_string();
        assert_eq!(Solution::remove_kdigits(n, k), e);
        // Explanation: Remove all the digits from the number and it is left with nothing which is 0.
    }
    #[test]
    fn n_112_k_1() {
        let n = "112".to_string();
        let k = 1;
        let e = "11".to_string();
        assert_eq!(Solution::remove_kdigits(n, k), e);
    }
    #[test]
    fn n_23456781_k_1() {
        let n = "23456781".to_string();
        let k = 3;
        let e = "23451".to_string();
        assert_eq!(Solution::remove_kdigits(n, k), e);
    }
}
