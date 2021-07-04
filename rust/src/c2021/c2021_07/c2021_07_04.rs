#![allow(dead_code)]
/// Count Vowels Permutation
/// ========================
///
/// Given an integer `n`,
/// your task is to count how many strings of length `n` can be formed under the following rules:
///
/// - Each character is a lower case vowel (`'a'`, `'e'`, `'i'`, `'o'`, `'u'`)
/// - Each vowel `'a'` may only be followed by an `'e'`.
/// - Each vowel `'e'` may only be followed by an `'a'` or an `'i'`.
/// - Each vowel `'i'` __may not__ be followed by another `'i'`.
/// - Each vowel `'o'` may only be followed by an `'i'` or a `'u'`.
/// - Each vowel `'u'` may only be followed by an `'a'`.
///
/// Since the answer may be too large, return it modulo `10^9 + 7`.
///
/// __Constraints:__
///
/// - `1 <= n <= 20_000`
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/608/week-1-july-1st-july-7th/3802/
struct Solution;
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut dp = [1, 1, 1, 1, 1];
        for _ in 1..n {
            let [pa, pe, pi, po, pu] = dp;
            dp[0] = ((pe + pi) % MOD + pu) % MOD;
            dp[1] = (pa + pi) % MOD;
            dp[2] = (pe + po) % MOD;
            dp[3] = pi;
            dp[4] = (pi + po) % MOD;
        }
        dp.iter().fold(0, |acc, &v| (acc + v) % MOD)
    }

    pub fn count_vowel_permutation_dp_vars(n: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let (mut a, mut e, mut i, mut o, mut u) = (1, 1, 1, 1, 1);
        for _ in 1..n {
            let (pa, pe, pi, po, pu) = (a, e, i, o, u);
            a = ((pe + pi) % MOD + pu) % MOD;
            e = (pa + pi) % MOD;
            i = (pe + po) % MOD;
            o = pi;
            u = (pi + po) % MOD;
        }
        [a, e, i, o, u].iter().fold(0, |acc, &v| (acc + v) % MOD)
    }
    pub fn count_vowel_permutation_dp_vec(n: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = n as usize;
        let mut dp = vec![vec![1; 5]; n];
        // a: 0 -> 1
        // e: 1 -> 0,2
        // i: 2 -> 0,1,3,4
        // o: 3 -> 2,4
        // u: 4 -> 0
        for c in 1..n {
            let p = c - 1;
            dp[c][0] = ((dp[p][1] + dp[p][2]) % MOD + dp[p][4]) % MOD;
            dp[c][1] = (dp[p][0] + dp[p][2]) % MOD;
            dp[c][2] = (dp[p][1] + dp[p][3]) % MOD;
            dp[c][3] = dp[p][2];
            dp[c][4] = (dp[p][2] + dp[p][3]) % MOD;
        }
        dp.last().unwrap().iter().fold(0, |acc, &v| (acc + v) % MOD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n1_produces_5() {
        assert_eq!(Solution::count_vowel_permutation(1), 5);
        // Explanation: All possible strings are: "a", "e", "i" , "o" and "u".
    }
    #[test]
    fn n2_produces_10() {
        assert_eq!(Solution::count_vowel_permutation(2), 10);
        // Explanation: All possible strings are: "ae", "ea", "ei", "ia", "ie", "io", "iu", "oi", "ou" and "ua".
    }
    #[test]
    fn n3_produces_19() {
        assert_eq!(Solution::count_vowel_permutation(3), 19);
    }
    #[test]
    fn n4_produces_35() {
        assert_eq!(Solution::count_vowel_permutation(4), 35);
    }
    #[test]
    fn n5_produces_68() {
        assert_eq!(Solution::count_vowel_permutation(5), 68);
    }
    #[test]
    fn n6_produces_129() {
        assert_eq!(Solution::count_vowel_permutation(6), 129);
    }
    #[test]
    fn n7_produces_249() {
        assert_eq!(Solution::count_vowel_permutation(7), 249);
    }
    #[test]
    fn n8_produces_474() {
        assert_eq!(Solution::count_vowel_permutation(8), 474);
    }
    #[test]
    fn n9_produces_911() {
        assert_eq!(Solution::count_vowel_permutation(9), 911);
    }

    mod performance {
        use super::*;

        #[test]
        fn n20000_produces_759_959_057() {
            assert_eq!(Solution::count_vowel_permutation(20000), 759_959_057);
        }
    }
}
