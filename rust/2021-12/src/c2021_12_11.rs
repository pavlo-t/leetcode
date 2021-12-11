#![allow(dead_code)]
/// 878. Nth Magical Number
/// =======================
///
/// A positive integer is _magical_ if it is divisible by either `a` or `b`.
///
/// Given the three integers `n`, `a`, and `b`, return the `n`th magical number.
/// Since the answer may be very large, __return it modulo__ `10**9 + 7`.
///
/// __Constraints:__
///
/// - `1 <= n <= 1_000_000_000`
/// - `2 <= a, b <= 40_000`
///
/// https://leetcode.com/problems/nth-magical-number/
struct Solution;
impl Solution {
    pub fn nth_magical_number_my_time_limit_exceeded(n: i32, a: i32, b: i32) -> i32 {
        println!("nth_magical_number({}, {}, {})", n, a, b);
        const MOD: i64 = 1_000_000_007;

        let (a, b) = if a < b { (a, b) } else { (b, a) };
        let (a, b) = (a as i64, b as i64);

        if b % a == 0 {
            ((a * n as i64) % MOD) as i32
        } else {
            let mut i = 1;
            let (mut x, mut y) = (a, b);
            while i < n {
                if x <= y {
                    x += a;
                } else {
                    y += b;
                    if y % a == 0 {
                        y += b;
                    }
                }
                i += 1;
            }
            (x.min(y) % MOD) as i32
        }
    }

    /// Provided numbers a=3,b=11:
    ///
    /// ```text
    /// idx:   1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39
    /// as: 0  3  6  9    12 15 18 21    24 27 30 33 36 39 42    45 48 51 54    57 60 63 66 69 72 75    78 81 84 87    90 93 96 99
    /// bs: 0          11             22          33          44             55          66          77             88          99
    /// ```
    ///
    /// There is a pattern from `i=0` to `i=12` that repeats itself in further indexes:
    ///
    /// ```text
    ///                                         3*11=33       33+12=45                 66+0=66
    /// as: 0  3  6  9    12 15 18 21    24 27 30  0  3  6  9    12 15 18 21    24 27 30  0  3  6  9    12 15 18 21    24 27 30  0
    /// bs: 0          11             22           0          11             22           0          11             22           0
    ///                                          33+0      33+11=44       33+22=55
    /// ```
    ///
    /// There is a pattern that repeats itself idx [0,13)
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        println!("nth_magical_number({}, {}, {})", n, a, b);
        const MOD: i64 = 1_000_000_007;

        let n = n as i64;
        let (a, b) = if a < b { (a, b) } else { (b, a) };
        let (a, b) = (a as i64, b as i64);

        if b % a == 0 {
            ((a * n) % MOD) as i32
        } else {
            let ab = a * b;
            let mut pattern = vec![0];
            let (mut x, mut y) = (a, b);
            while x < ab || y < ab {
                pattern.push(x.min(y));
                if x <= y {
                    x += a;
                } else {
                    y += b;
                    if y % a == 0 {
                        y += b;
                    }
                }
            }
            let len = pattern.len() as i64;
            let result = (n / len) * ab + pattern[(n % len) as usize];
            (result % MOD) as i32
        }
    }

    /// Approach 2: Binary Search
    /// https://leetcode.com/problems/nth-magical-number/solution/
    pub fn nth_magical_number_l(n: i32, a: i32, b: i32) -> i32 {
        println!("nth_magical_number({}, {}, {})", n, a, b);
        const MOD: i64 = 1_000_000_007;

        #[rustfmt::skip] fn gcd(x: i64, y: i64) -> i64 {
            if x == 0 { y } else { gcd(y % x, x) }
        }

        let (n, a, b) = (n as i64, a as i64, b as i64);
        let l = a / gcd(a, b) * b;
        let mut lo = 0i64;
        let mut hi = n * a.min(b);
        while lo < hi {
            let mi = lo + (hi - lo) / 2;
            // If there are not enough magic numbers below mi...
            if mi / a + mi / b - mi / l < n {
                lo = mi + 1;
            } else {
                hi = mi;
            }
        }
        (lo % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n1a2b3() { assert_eq!(Solution::nth_magical_number(1, 2, 3), 2); }
    /// ```text
    /// final_index:    1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36
    /// a_index:        1     2  3  4     5  6  7     8  9 10    11 12 13    14 15 16    17 18 19    20 21 22    23 24 25    26 27
    ///
    /// a_seq:       0  2     4  6  8    10 12 14    16 18 20    22 24 26    28 30 32    34 36 38    40 42 44    46 48 50    52 54
    /// b_seq:       0     3     6     9    12    15    18    21    24    27    30    33    36    39    42    45    48    51    54
    ///
    /// b_index:           1     2     3     4     5     6     7     8     9    10    11    12    13    14    15    16    17    18
    /// both_have:               1           2           3           4           5           6           7           8
    /// a_only_idx:     1     2     3     4     5     6     7     8     9    10    11    12    13    14    15    16    17    18
    /// b_only_idx:        1           2           3           4           5           6           7           8           9
    /// ```
    /// `2,3,4,6`
    #[rustfmt::skip] #[test] fn n4a2b3() { assert_eq!(Solution::nth_magical_number(4, 2, 3), 6); }
    /// `2,4,6,8,10`
    #[rustfmt::skip] #[test] fn n5a2b4() { assert_eq!(Solution::nth_magical_number(5, 2, 4), 10); }
    /// ```text
    /// i: 1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27
    ///    4     8 12 16    20 24 28    32 36 40    44 48 52    56 60 64    68 72 76    80
    ///       6    12    18    24    30    36    42    48    54    60    66    72    78
    /// ```
    /// `4,6,*8,12,16,18,20`
    #[rustfmt::skip] #[test] fn n3a6b4() { assert_eq!(Solution::nth_magical_number(3, 6, 4), 8); }
    /// ```text
    /// i: 1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27
    ///    2  4  6  8 10 12 14 16 18 20 22 24 26 28 30 32 34 36 38 40 42 44 46 48 50 52 54
    ///          6       12       18       24       30       36       42       48       54
    /// ```
    /// `2,4,6,8,10,12,14,16,18,20`
    #[rustfmt::skip] #[test] fn n10a2b6() { assert_eq!(Solution::nth_magical_number(10, 2, 6), 20); }
    /// ```text
    /// i:     1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39
    /// ai:    1  2  3     4  5  6  7     8  9 10 11 12 13 14    15 16 17 18    19 20 21 22 23 24 25    26 27 28 29    30 31 32 33
    /// as: 0  3  6  9    12 15 18 21    24 27 30 33 36 39 42    45 48 51 54    57 60 63 66 69 72 75    78 81 84 87    90 93 96 99
    /// bs: 0          11             22          33          44             55          66          77             88          99
    /// bi:             1              2           3           4              5           6           7              8           9
    ///
    ///                                          33+0          33+12                    66+0
    /// as: 0  3  6  9    12 15 18 21    24 27 30  0  3  6  9    12 15 18 21    24 27 30  0  3  6  9    12 15 18 21    24 27 30  0
    /// bs: 0          11             22           0          11             22           0          11             22           0
    ///                                          33+0                                   66+0
    /// ```
    /// `3,6,9,11,12,15,18,21,22`
    #[rustfmt::skip] #[test] fn n9a3b11() { assert_eq!(Solution::nth_magical_number(9, 3, 11), 22); }

    //#[ignore]
    #[rustfmt::skip] #[test] fn n1_000_000_000_a3_b2() {
        assert_eq!(Solution::nth_magical_number(1_000_000_000, 3, 2), 499_999_993);
    }
    //#[ignore]
    #[rustfmt::skip] #[test] fn n1_000_000_000_a39999_b40000() {
        assert_eq!(Solution::nth_magical_number(1_000_000_000, 39999, 40000), 999_860_007);
    }
}
