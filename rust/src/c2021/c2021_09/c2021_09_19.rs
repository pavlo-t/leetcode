#![allow(dead_code)]
/// Distinct Subsequences
/// =====================
///
/// Given two strings `s` and `t`, return _the number of distinct subsequences of `s` which equals `t`_.
///
/// A string's __subsequence__ is a new string formed from the original string by deleting some (can be none)
/// of the characters without disturbing the remaining characters' relative positions.
/// (i.e., `"ACE"` is a subsequence of `"ABCDE"` while `"AEC"` is not).
///
/// It is guaranteed the answer fits on a 32-bit signed integer.
///
/// __Constraints:__
///
/// - `1 <= s.length, t.length <= 1000`
/// - `s` and `t` consist of English letters.
///
/// https://leetcode.com/explore/challenge/card/september-leetcoding-challenge-2021/638/week-3-september-15th-september-21st/3980/
struct Solution;
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        println!("num_distinct({}, {})", s, t);
        fn rec(si: usize, ti: usize, s: &[u8], t: &[u8], memo: &mut Vec<Vec<i32>>) -> i32 {
            if ti >= t.len() {
                1
            } else if si >= s.len() {
                0
            } else if memo[si][ti] >= 0 {
                memo[si][ti]
            } else if s[si] == t[ti] {
                memo[si][ti] = rec(si + 1, ti + 1, s, t, memo) + rec(si + 1, ti, s, t, memo);
                memo[si][ti]
            } else {
                memo[si][ti] = rec(si + 1, ti, s, t, memo);
                memo[si][ti]
            }
        }
        let mut memo = vec![vec![-1; t.len()]; s.len()];
        rec(0, 0, s.as_bytes(), t.as_bytes(), &mut memo)
    }
    pub fn num_distinct_brute_force(s: String, t: String) -> i32 {
        println!("num_distinct({}, {})", s, t);
        fn rec(si: usize, ti: usize, s: &[u8], t: &[u8]) -> i32 {
            if ti >= t.len() {
                1
            } else if si >= s.len() {
                0
            } else if s[si] == t[ti] {
                rec(si + 1, ti + 1, s, t) + rec(si + 1, ti, s, t)
            } else {
                rec(si + 1, ti, s, t)
            }
        }
        rec(0, 0, s.as_bytes(), t.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rabbbit_rabbit_3() {
        let s = "rabbbit".to_string();
        let t = "rabbit".to_string();
        assert_eq!(Solution::num_distinct(s, t), 3);
        // Explanation:
        // As shown below, there are 3 ways you can generate "rabbit" from S.
        // rabbbit  rabbbit  rabbbit
        // ---- --  --- ---  -- ----
    }
    #[test]
    fn babgbag_bag_5() {
        let s = "babgbag".to_string();
        let t = "bag".to_string();
        assert_eq!(Solution::num_distinct(s, t), 5);
        // Explanation:
        // As shown below, there are 5 ways you can generate "bag" from S.
        // babgbag  babgbag  babgbag  babgbag  babgbag
        // -- -     --    -  -    --    -  --      ---
    }

    mod performance {
        use super::*;

        #[test]
        fn ax100_ax99_100() {
            let s = "a".repeat(100);
            let t = "a".repeat(99);
            assert_eq!(Solution::num_distinct(s, t), 100);
        }
        #[test]
        fn ax1000_ax997_166_167_000() {
            let s = "a".repeat(1000);
            let t = "a".repeat(997);
            assert_eq!(Solution::num_distinct(s, t), 166_167_000);
        }
    }
}
