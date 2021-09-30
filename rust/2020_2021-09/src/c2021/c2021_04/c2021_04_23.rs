#![allow(dead_code)]
/// Count Binary Substrings
/// =======================
///
/// Give a string `s`,
/// count the number of non-empty (contiguous) substrings that have the same number of 0's and 1's,
/// and all the 0's and all the 1's in these substrings are grouped consecutively.
///
/// Substrings that occur multiple times are counted the number of times they occur.
///
/// __Note:__
///
/// - `s.length` will be between `1` and `50_000`.
/// - `s` will only consist of "0" or "1" characters.
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/596/week-4-april-22nd-april-28th/3718/
struct Solution;
impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let (r, pl, cl, _) = s.chars().skip(1).fold(
            (0, 0, 1, s.chars().next().unwrap()),
            |(rsf, pl, cl, pc), c| {
                if c == pc {
                    (rsf, pl, cl + 1, c)
                } else {
                    (rsf + cl.min(pl), cl, 1, c)
                }
            },
        );
        r + pl.min(cl)
    }

    pub fn count_binary_substrings_my_fold(s: String) -> i32 {
        s.chars()
            .fold((0, 0, 0, 'x'), |(rsf, pl, cl, pc), c| {
                if c == pc {
                    (rsf + if pl > cl { 1 } else { 0 }, pl, cl + 1, c)
                } else {
                    (rsf + if cl > 0 { 1 } else { 0 }, cl, 1, c)
                }
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_s00110011_produces_6() {
        let s = "00110011".to_string();
        assert_eq!(Solution::count_binary_substrings(s), 6);
        // Explanation:
        // There are 6 substrings that have equal number of consecutive 1's and 0's:
        // "0011", "01", "1100", "10", "0011", and "01".
        //
        // Notice that some of these substrings repeat and are counted the number of times they occur.
        //
        // Also, "00110011" is not a valid substring because all the 0's (and 1's) are not grouped together.
    }
    #[test]
    fn example2_s10101_produces_4() {
        let s = "10101".to_string();
        assert_eq!(Solution::count_binary_substrings(s), 4);
        // Explanation:
        // There are 4 substrings: "10", "01", "10", "01" that have equal number of consecutive 1's and 0's.
    }

    mod performance {
        use super::*;

        #[test]
        fn s_25k0s_and_25k1s_produces_25k() {
            let s = (0..50000)
                .map(|i| if i < 25000 { '0' } else { '1' })
                .collect();
            assert_eq!(Solution::count_binary_substrings(s), 25000);
        }
        #[test]
        fn s_01repeat25k_produces_49999() {
            let s = (0..50000)
                .map(|i| if i % 2 == 0 { '0' } else { '1' })
                .collect();
            assert_eq!(Solution::count_binary_substrings(s), 49999);
        }
    }
}
