#![allow(dead_code)]
/// Non-negative Integers without Consecutive Ones
/// ==============================================
///
/// Given a positive integer `n`, return the number of the integers in the range `[0, n]`
/// whose binary representations __do not__ contain consecutive ones.
///
/// __Constraints:__
///
/// `1 <= n <= 1_000_000_000`
///
/// https://leetcode.com/explore/challenge/card/july-leetcoding-challenge-2021/611/week-4-july-22nd-july-28th/3826/
struct Solution;
impl Solution {
    /// https://protegejj.gitbook.io/oj-practices/chapter1/dynamic-programming/600-non-negative-integers-without-consecutive-ones
    pub fn find_integers(n: i32) -> i32 {
        let nb = format!("{:b}", n).into_bytes();
        let nl = nb.len();
        let mut l0 = vec![1; nl];
        let mut l1 = vec![1; nl];
        for i in 1..nl {
            l0[i] = l0[i - 1] + l1[i - 1];
            l1[i] = l0[i - 1];
        }
        let mut result = l0[nl - 1] + l1[nl - 1];
        for i in 0..nl - 1 {
            match (nb[i], nb[i + 1]) {
                (b'0', b'0') => result -= l1[nl - i - 2],
                (b'1', b'1') => break,
                _ => (),
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_05_produces_5() {
        assert_eq!(Solution::find_integers(5), 5);
        // Explanation:
        // Here are the non-negative integers <= 5 with their corresponding binary representations:
        // 0: 000
        // 1: 001
        // 2: 010
        // 3: 011
        // 4: 100
        // 5: 101
        // Among them, only integer 3 disobeys the rule (two consecutive ones) and the other 5 satisfy the rule.
    }
    #[test]
    fn n_01_produces_2() {
        assert_eq!(Solution::find_integers(1), 2);
    }
    #[test]
    fn n_02_produces_3() {
        assert_eq!(Solution::find_integers(2), 3);
    }
    #[test]
    fn n_03_produces_3() {
        assert_eq!(Solution::find_integers(3), 3);
    }
    #[test]
    fn n_04_produces_4() {
        assert_eq!(Solution::find_integers(4), 4);
    }
    #[test]
    fn n_06_produces_5() {
        assert_eq!(Solution::find_integers(6), 5);
    }
    #[test]
    fn n_07_produces_5() {
        assert_eq!(Solution::find_integers(7), 5);
    }
    #[test]
    fn n_08_produces_6() {
        assert_eq!(Solution::find_integers(8), 6);
    }
    #[test]
    fn n_09_produces_7() {
        assert_eq!(Solution::find_integers(9), 7);
    }
    #[test]
    fn n_10_produces_8() {
        assert_eq!(Solution::find_integers(10), 8);
    }
    #[test]
    fn n_11_produces_8() {
        assert_eq!(Solution::find_integers(11), 8);
    }
    #[test]
    fn n_12_produces_8() {
        assert_eq!(Solution::find_integers(12), 8);
    }
    #[test]
    fn n_13_produces_8() {
        assert_eq!(Solution::find_integers(13), 8);
    }
    #[test]
    fn n_14_produces_8() {
        assert_eq!(Solution::find_integers(14), 8);
    }
    #[test]
    fn n_15_produces_8() {
        assert_eq!(Solution::find_integers(15), 8);
    }
    #[test]
    fn n_16_produces_9() {
        assert_eq!(Solution::find_integers(16), 9);
    }
    #[test]
    fn n_17_produces_10() {
        assert_eq!(Solution::find_integers(17), 10);
    }
    #[test]
    fn n_18_produces_11() {
        assert_eq!(Solution::find_integers(18), 11);
    }
    #[test]
    fn n_19_produces_11() {
        assert_eq!(Solution::find_integers(19), 11);
    }
    #[test]
    fn n_20_produces_12() {
        assert_eq!(Solution::find_integers(20), 12);
    }
    #[test]
    fn n_21_produces_13() {
        assert_eq!(Solution::find_integers(21), 13);
    }
    #[test]
    fn n_22_produces_13() {
        assert_eq!(Solution::find_integers(22), 13);
    }
    #[test]
    fn n_23_produces_13() {
        assert_eq!(Solution::find_integers(23), 13);
    }
    #[test]
    fn n_24_produces_13() {
        assert_eq!(Solution::find_integers(24), 13);
    }
    #[test]
    fn n_25_produces_13() {
        assert_eq!(Solution::find_integers(25), 13);
    }
    #[test]
    fn n_26_produces_13() {
        assert_eq!(Solution::find_integers(26), 13);
    }
    #[test]
    fn n_27_produces_13() {
        assert_eq!(Solution::find_integers(27), 13);
    }
    #[test]
    fn n_28_produces_13() {
        assert_eq!(Solution::find_integers(28), 13);
    }
    #[test]
    fn n_29_produces_13() {
        assert_eq!(Solution::find_integers(29), 13);
    }
    #[test]
    fn n_30_produces_13() {
        assert_eq!(Solution::find_integers(30), 13);
    }
    #[test]
    fn n_31_produces_13() {
        assert_eq!(Solution::find_integers(31), 13);
    }
    #[test]
    fn n_32_produces_14() {
        assert_eq!(Solution::find_integers(32), 14);
        // 00: 000000 01: 000001 02: 000010 03: 000011 04: 000100 05: 000101 06: 000110 07: 000111
        // 08: 001000 09: 001001 10: 001010 11: 001011 12: 001100 13: 001101 14: 001110 15: 001111
        // 16: 010000 17: 010001 18: 010010 19: 010011 20: 010100 21: 010101 22: 010110 23: 010111
        // 24: 011000 25: 011001 26: 011010 27: 011011 28: 011100 29: 011101 30: 011110 31: 011111
        // 32: 100000
    }

    #[test]
    fn n_1_000_000_000_produces_2_178_309() {
        assert_eq!(Solution::find_integers(1_000_000_000), 2_178_309);
    }
}
