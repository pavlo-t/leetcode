#![allow(dead_code)]
/// \#474. Ones and Zeroes
/// ======================
///
/// You are given an array of binary strings `strs` and two integers `m` and `n`.
///
/// Return _the size of the largest subset of `strs` such that
/// there are __at most__ `m` `0`'s and `n` `1`'s in the subset_.
///
/// A set `x` is a __subset__ of a set `y` if all elements of `x` are also elements of `y`.
///
/// __Constraints:__
///
/// - `1 <= strs.length <= 600`
/// - `1 <= strs[i].length <= 100`
/// - `strs[i]` consists only of digits `'0'` and `'1'`.
/// - `1 <= m, n <= 100`
///
/// https://leetcode.com/problems/ones-and-zeroes/
struct Solution;
impl Solution {
    pub fn find_max_form_rec(strs: Vec<String>, m: i32, n: i32) -> i32 {
        println!("{strs:?},{m},{n}");
        fn count_01(s: String) -> (i32, i32) {
            s.chars().fold((0, 0), |(c0, c1), ch| match ch {
                '0' => (c0 + 1, c1),
                _ => (c0, c1 + 1),
            })
        }
        let counts = strs.into_iter().map(count_01).collect::<Vec<_>>();
        println!("counts:{counts:?}");

        fn rec(
            i: usize,
            rsf: i32,
            rsf0: i32,
            rsf1: i32,
            max0: i32,
            max1: i32,
            counts: &[(i32, i32)],
        ) -> i32 {
            if i == counts.len() {
                rsf
            } else {
                let (cur0, cur1) = (rsf0 + counts[i].0, rsf1 + counts[i].1);
                if cur0 <= max0 && cur1 <= max1 {
                    let take = rec(i + 1, rsf + 1, cur0, cur1, max0, max1, counts);
                    let skip = rec(i + 1, rsf, rsf0, rsf1, max0, max1, counts);
                    take.max(skip)
                } else {
                    rec(i + 1, rsf, rsf0, rsf1, max0, max1, counts)
                }
            }
        }

        rec(0, 0, 0, 0, m, n, &counts)
    }

    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        println!("{strs:?},{m},{n}");
        fn count_01(s: String) -> (usize, usize) {
            s.chars().fold((0, 0), |(c0, c1), ch| match ch {
                '0' => (c0 + 1, c1),
                _ => (c0, c1 + 1),
            })
        }
        let counts = strs.into_iter().map(count_01).collect::<Vec<_>>();
        println!("counts:{counts:?}");

        fn rec(
            i: usize,
            rsf0: usize,
            rsf1: usize,
            max0: usize,
            max1: usize,
            counts: &[(usize, usize)],
            memo: &mut Vec<Vec<Vec<i32>>>,
        ) -> i32 {
            if i == counts.len() {
                0
            } else {
                if memo[i][rsf0][rsf1] == -1 {
                    memo[i][rsf0][rsf1] = {
                        let (cur0, cur1) = (rsf0 + counts[i].0, rsf1 + counts[i].1);
                        let skip = rec(i + 1, rsf0, rsf1, max0, max1, counts, memo);
                        if cur0 <= max0 && cur1 <= max1 {
                            let take = 1 + rec(i + 1, cur0, cur1, max0, max1, counts, memo);
                            take.max(skip)
                        } else {
                            skip
                        }
                    }
                }
                memo[i][rsf0][rsf1]
            }
        }

        let (m, n) = (m as usize, n as usize);
        let mut memo = vec![vec![vec![-1; n + 1]; m + 1]; counts.len()];

        rec(0, 0, 0, m, n, &counts, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vs { ($($x:expr),*) => { vec![$($x.to_string()),*] }; }

    #[test]
    fn s_10_0001_111001_1_0_m_5_n_3() {
        let s = vs!["10", "0001", "111001", "1", "0"];
        assert_eq!(Solution::find_max_form(s, 5, 3), 4);
        // Explanation: The largest subset with at most 5 0's and 3 1's is {"10", "0001", "1", "0"}, so the answer is 4.
        // Other valid but smaller subsets include {"0001", "1"} and {"10", "1", "0"}.
        // {"111001"} is an invalid subset because it contains 4 1's, greater than the maximum of 3.
    }
    #[test]
    fn s_10_0_1_m_1_n_1() {
        let s = vs!["10", "0", "1"];
        assert_eq!(Solution::find_max_form(s, 1, 1), 2);
        // Explanation: The largest subset is {"0", "1"}, so the answer is 2.
    }

    #[test]
    fn test_67() {
        let s = vs![
            "011", "1", "11", "0", "010", "1", "10", "1", "1", "0", "0", "0", "01111", "011", "11",
            "00", "11", "10", "1", "0", "0", "0", "0", "101", "001110", "1", "0", "1", "0", "0",
            "10", "00100", "0", "10", "1", "1", "1", "011", "11", "11", "10", "10", "0000", "01",
            "1", "10", "0"
        ];
        assert_eq!(Solution::find_max_form(s, 44, 39), 45);
    }

    //#[ignore]
    #[test]
    fn s_600x100x0_m_60000_n_0() {
        let s = (0..600).map(|_| "0".repeat(100)).collect();
        assert_eq!(Solution::find_max_form(s, 60000, 0), 600);
    }
}
