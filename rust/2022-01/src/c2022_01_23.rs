#![allow(dead_code)]
/// 1291. Sequential Digits
/// =======================
///
/// An integer has _sequential digits_ if and only if each digit in the number is one more than the previous digit.
///
/// Return a __sorted__ list of all the integers in the range `[low, high]` inclusive that have sequential digits.
///
/// __Constraints:__
///
/// - `10 <= low <= high <= 1_000_000_000`
///
/// https://leetcode.com/problems/sequential-digits/
struct Solution;
impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut result = vec![];

        'outer: for len in 2..=9 {
            let (mut curr, diff) = {
                let mut start = 0;
                let mut digit = 1;
                let mut diff = 1;
                for _ in 1..len {
                    start *= 10;
                    start += digit;
                    digit += 1;
                    diff *= 10;
                    diff += 1;
                }
                (start, diff)
            };

            for _ in 0..10 - len {
                curr += diff;
                if curr >= low && curr <= high {
                    result.push(curr);
                } else if curr > high {
                    break 'outer;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l_10_h_10() {
        let e: Vec<i32> = vec![];
        assert_eq!(Solution::sequential_digits(10, 10), e);
    }
    #[rustfmt::skip] #[test] fn l_12_h_12() { assert_eq!(Solution::sequential_digits(12, 12), vec![12]); }
    #[rustfmt::skip] #[test] fn l_12_h_23() { assert_eq!(Solution::sequential_digits(12, 23), vec![12,23]); }

    #[test]
    fn l_100_h_300() {
        assert_eq!(Solution::sequential_digits(100, 300), [123, 234]);
    }
    #[test]
    fn l_1000_h_13000() {
        let e = [1234, 2345, 3456, 4567, 5678, 6789, 12345];
        assert_eq!(Solution::sequential_digits(1000, 13000), e);
    }
}
