#![allow(dead_code)]
/// 902. Numbers At Most N Given Digit Set
/// ======================================
///
/// Given an array of `digits` which is sorted in __non-decreasing__ order.
/// You can write numbers using each `digits[i]` as many times as we want.
/// For example, if `digits = ['1','3','5']`, we may write numbers such as `'13'`, `'551'`, and `'1351315'`.
///
/// Return _the number of positive integers that can be generated that are less than or equal to a given integer `n`_.
///
/// __Constraints:__
///
/// - `1 <= digits.length <= 9`
/// - `digits[i].length == 1`
/// - `digits[i]` is a digit from `'1'` to `'9'`.
/// - All the values in `digits` are __unique__.
/// - `digits` is sorted in __non-decreasing__ order.
/// - `1 <= n <= 1_000_000_000`
///
/// https://leetcode.com/problems/numbers-at-most-n-given-digit-set/
struct Solution;
impl Solution {
    /// Adopted from `/rust/2020_2021-09/src/c2020/c2020_11/d2020_11_21.rs`
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        println!("at_most_n_given_digit_set({:?}, {})", digits, n);
        let digits = digits
            .into_iter()
            .map(|s| s.chars().nth(0).unwrap())
            .collect::<Vec<_>>();
        let n = n.to_string();
        let (d_len, n_len) = (digits.len(), n.len());
        let mut result = 0;

        for i in 1..n_len {
            result += d_len.pow(i as u32);
        }

        for (i, n_char) in n.chars().enumerate() {
            if i == n_len - 1 {
                result += digits.iter().filter(|&&d| d <= n_char).count();
            } else {
                let pow = d_len.pow((n_len - i - 1) as u32);
                let less_than_curr = digits.iter().filter(|&&d| d < n_char).count();
                result += pow * less_than_curr;
                if !digits.iter().any(|&d| d == n_char) {
                    break;
                }
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs {($($x:expr),*) => {vec![$($x.to_string()),*]};}

    #[test]
    fn d1357_n100() {
        let d = vs!["1", "3", "5", "7"];
        let n = 100;
        assert_eq!(Solution::at_most_n_given_digit_set(d, n), 20);
        // Output: 20
        // Explanation:
        // The 20 numbers that can be written are:
        // 1, 3, 5, 7, 11, 13, 15, 17, 31, 33, 35, 37, 51, 53, 55, 57, 71, 73, 75, 77.
    }
    #[test]
    fn d149_n1_000_000_000() {
        let d = vs!["1", "4", "9"];
        let n = 1_000_000_000;
        assert_eq!(Solution::at_most_n_given_digit_set(d, n), 29523);
        // Explanation:
        // We can write 3 one digit numbers, 9 two digit numbers, 27 three digit numbers,
        // 81 four digit numbers, 243 five digit numbers, 729 six digit numbers,
        // 2187 seven digit numbers, 6561 eight digit numbers, and 19683 nine digit numbers.
        // In total, this is 29523 integers that can be written using the digits array.
    }
    #[test]
    fn d7_n8() {
        let d = vs!["7"];
        let n = 8;
        assert_eq!(Solution::at_most_n_given_digit_set(d, n), 1);
    }
    #[test]
    fn d8_n8() {
        let d = vs!["8"];
        let n = 8;
        assert_eq!(Solution::at_most_n_given_digit_set(d, n), 1);
    }
    #[test]
    fn d9_n8() {
        let d = vs!["9"];
        let n = 8;
        assert_eq!(Solution::at_most_n_given_digit_set(d, n), 0);
    }
    #[test]
    fn d12_n11() {
        let d = vs!["1", "2"];
        let n = 11;
        assert_eq!(Solution::at_most_n_given_digit_set(d, n), 3);
    }
}
