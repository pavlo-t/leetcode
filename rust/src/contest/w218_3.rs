#![allow(dead_code, unused_imports)]

/// ### 5620. Concatenation of Consecutive Binary Numbers
///
/// https://leetcode.com/contest/weekly-contest-218/problems/concatenation-of-consecutive-binary-numbers/
struct Solution;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let n = n as i64;
        let mut result = 0i64;

        for i in 1..=n {
            let b = format!("{:b}", i).len();
            result = ((result << b) + i) % MOD;
        }

        result as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::concatenated_binary(1), 1);
        // Explanation: "1" in binary corresponds to the decimal value 1.
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::concatenated_binary(3), 27);
        // Explanation: In binary, 1, 2, and 3 corresponds to "1", "10", and "11".
        // After concatenating them, we have "11011", which corresponds to the decimal value 27.
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::concatenated_binary(12), 505379714);
        // Explanation: The concatenation results in "1101110010111011110001001101010111100".
        // The decimal value of that is 118505380540.
        // After modulo 109 + 7, the result is 505379714.
    }

    #[test]
    fn test_100k() {
        assert_eq!(Solution::concatenated_binary(100000), 757631812);
    }
}